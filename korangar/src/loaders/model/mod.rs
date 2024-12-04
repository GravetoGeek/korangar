use std::sync::Arc;

use cgmath::{EuclideanSpace, Matrix4, Point3, Rad, SquareMatrix, Vector2, Vector3};
use derive_new::new;
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "debug")]
use korangar_debug::logging::{print_debug, Colorize, Timer};
use korangar_util::collision::AABB;
use korangar_util::math::multiply_matrix4_and_point3;
use korangar_util::texture_atlas::AllocationId;
use korangar_util::FileLoader;
use ragnarok_bytes::{ByteReader, FromBytes};
use ragnarok_formats::model::{ModelData, NodeData};
use ragnarok_formats::version::InternalVersion;

use super::error::LoadError;
use super::FALLBACK_MODEL_FILE;
use crate::graphics::{Color, NativeModelVertex};
use crate::loaders::map::DeferredVertexGeneration;
use crate::loaders::texture::TextureAtlasEntry;
use crate::loaders::{GameFileLoader, TextureAtlasFactory};
use crate::world::{Model, Node};

#[derive(new)]
pub struct ModelLoader {
    game_file_loader: Arc<GameFileLoader>,
}

impl ModelLoader {
    fn add_vertices(
        native_vertices: &mut Vec<NativeModelVertex>,
        vertex_positions: &[Point3<f32>],
        texture_coordinates: &[Vector2<f32>],
        texture_index: u16,
        reverse_vertices: bool,
        reverse_normal: bool,
    ) {
        let normal = match reverse_normal {
            true => NativeModelVertex::calculate_normal(vertex_positions[0], vertex_positions[1], vertex_positions[2]),
            false => NativeModelVertex::calculate_normal(vertex_positions[2], vertex_positions[1], vertex_positions[0]),
        };

        if reverse_vertices {
            for (vertex_position, texture_coordinates) in vertex_positions.iter().copied().zip(texture_coordinates).rev() {
                native_vertices.push(NativeModelVertex::new(
                    vertex_position,
                    normal,
                    *texture_coordinates,
                    texture_index as i32,
                    Color::WHITE,
                    0.0, // TODO: actually add wind affinity
                ));
            }
        } else {
            for (vertex_position, texture_coordinates) in vertex_positions.iter().copied().zip(texture_coordinates) {
                native_vertices.push(NativeModelVertex::new(
                    vertex_position,
                    normal,
                    *texture_coordinates,
                    texture_index as i32,
                    Color::WHITE,
                    0.0, // TODO: actually add wind affinity
                ));
            }
        }
    }

    fn make_vertices(node: &NodeData, main_matrix: &Matrix4<f32>, reverse_order: bool, texture_transparency: Vec<bool>) -> Vec<SubMesh> {
        let capacity = node.faces.iter().map(|face| if face.two_sided != 0 { 6 } else { 3 }).sum();
        let mut native_vertices = Vec::with_capacity(capacity);

        let array: [f32; 3] = node.scale.unwrap().into();
        let reverse_node_order = array.into_iter().fold(1.0, |a, b| a * b).is_sign_negative();

        if reverse_node_order {
            panic!("this can actually happen");
        }

        for face in &node.faces {
            let vertex_positions: [Point3<f32>; 3] = std::array::from_fn(|index| {
                let position_index = face.vertex_position_indices[index];
                let position = node.vertex_positions[position_index as usize];
                multiply_matrix4_and_point3(main_matrix, position)
            });

            let texture_coordinates: [Vector2<f32>; 3] = std::array::from_fn(|index| {
                let coordinate_index = face.texture_coordinate_indices[index];
                node.texture_coordinates[coordinate_index as usize].coordinates
            });

            Self::add_vertices(
                &mut native_vertices,
                &vertex_positions,
                &texture_coordinates,
                face.texture_index,
                reverse_order,
                false,
            );

            if face.two_sided != 0 {
                Self::add_vertices(
                    &mut native_vertices,
                    &vertex_positions,
                    &texture_coordinates,
                    face.texture_index,
                    !reverse_order,
                    true,
                );
            }
        }

        if texture_transparency.iter().any(|&t| t) {
            Self::split_disconnected_meshes(&native_vertices, texture_transparency)
        } else {
            vec![SubMesh {
                transparent: false,
                native_vertices,
            }]
        }
    }

    fn calculate_matrices(node: &NodeData, parent_matrix: &Matrix4<f32>) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
        let main = Matrix4::from_translation(node.translation1.unwrap()) * Matrix4::from(node.offset_matrix);
        let scale = node.scale.unwrap();
        let scale_matrix = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        let rotation_matrix = Matrix4::from_axis_angle(node.rotation_axis.unwrap(), Rad(node.rotation_angle.unwrap()));
        let translation_matrix = Matrix4::from_translation(node.translation2);

        let transform = match node.rotation_keyframe_count > 0 {
            true => translation_matrix * scale_matrix,
            false => translation_matrix * rotation_matrix * scale_matrix,
        };

        let box_transform = parent_matrix * translation_matrix * rotation_matrix * scale_matrix;

        (main, transform, box_transform)
    }

    // For nodes with that are transparent, we will split all disconnected meshed,
    // so that we can properly depth sort them to be able to render transparent
    // models correctly.
    fn split_disconnected_meshes(vertices: &[NativeModelVertex], texture_transparency: Vec<bool>) -> Vec<SubMesh> {
        // Step 1: Split opaque and transparent vertices.
        let (transparent_vertices, opaque_vertices): (Vec<NativeModelVertex>, Vec<NativeModelVertex>) = vertices
            .iter()
            .partition(|vertex| texture_transparency[vertex.texture_index as usize]);

        // Step 2: Create an adjacency map for transparent triangles.
        let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
        let triangles: Vec<_> = transparent_vertices.chunks_exact(3).collect();

        for (triangle1_index, triangle1) in triangles.iter().enumerate() {
            for (triangle2_index, triangle2) in triangles.iter().enumerate() {
                if triangle1_index != triangle2_index {
                    let shares_vertex = triangle1.iter().any(|vertex1| {
                        triangle2.iter().any(|vertex2| {
                            const EPSILON: f32 = 1.0;
                            (vertex1.position.x - vertex2.position.x).abs() < EPSILON
                                && (vertex1.position.y - vertex2.position.y).abs() < EPSILON
                                && (vertex1.position.z - vertex2.position.z).abs() < EPSILON
                        })
                    });

                    if shares_vertex {
                        adjacency.entry(triangle1_index).or_default().insert(triangle2_index);
                        adjacency.entry(triangle2_index).or_default().insert(triangle1_index);
                    }
                }
            }
        }

        // Step 3: Find connected vertices using a depth first search.
        let mut visited: HashSet<usize> = HashSet::new();
        let mut submeshes: Vec<SubMesh> = vec![SubMesh {
            transparent: false,
            native_vertices: opaque_vertices,
        }];

        for triangle_index in 0..triangles.len() {
            if visited.contains(&triangle_index) {
                continue;
            }

            let mut native_vertices: Vec<NativeModelVertex> = Vec::new();
            let mut stack = vec![triangle_index];

            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    native_vertices.extend_from_slice(triangles[current]);

                    if let Some(neighbors) = adjacency.get(&current) {
                        stack.extend(neighbors.iter().filter(|&index| !visited.contains(index)));
                    }
                }
            }

            submeshes.push(SubMesh {
                transparent: true,
                native_vertices,
            });
        }

        submeshes
    }

    fn calculate_centroid(vertices: &[NativeModelVertex]) -> Point3<f32> {
        let sum = vertices.iter().fold(Vector3::new(0.0, 0.0, 0.0), |accumulator, vertex| {
            accumulator + vertex.position.to_vec()
        });
        Point3::from_vec(sum / vertices.len() as f32)
    }

    fn process_node_mesh(
        current_node: &NodeData,
        nodes: &[NodeData],
        vertex_offset: &mut usize,
        native_vertices: &mut Vec<NativeModelVertex>,
        model_texture_mapping: &[ModelTexture],
        parent_matrix: &Matrix4<f32>,
        main_bounding_box: &mut AABB,
        reverse_order: bool,
    ) -> Vec<Node> {
        let (main_matrix, transform_matrix, box_transform_matrix) = Self::calculate_matrices(current_node, parent_matrix);

        let box_matrix = box_transform_matrix * main_matrix;
        let bounding_box = AABB::from_vertices(
            current_node
                .vertex_positions
                .iter()
                .map(|position| multiply_matrix4_and_point3(&box_matrix, *position)),
        );
        main_bounding_box.extend(&bounding_box);

        let child_nodes = nodes
            .iter()
            .filter(|node| node.parent_node_name == current_node.node_name)
            .filter(|node| node.parent_node_name != node.node_name)
            .flat_map(|node| {
                Self::process_node_mesh(
                    node,
                    nodes,
                    vertex_offset,
                    native_vertices,
                    model_texture_mapping,
                    &box_transform_matrix,
                    main_bounding_box,
                    reverse_order,
                )
            })
            .collect();

        // Map the node texture index to the model texture index.
        let (node_texture_mapping, texture_transparency): (Vec<i32>, Vec<bool>) = current_node
            .texture_indices
            .iter()
            .map(|&index| {
                let model_texture = model_texture_mapping[index as usize];
                (model_texture.index, model_texture.transparent)
            })
            .unzip();

        let mut sub_meshes = Self::make_vertices(current_node, &main_matrix, reverse_order, texture_transparency);

        let mut sub_nodes: Vec<Node> = sub_meshes
            .iter_mut()
            .map(|mesh| {
                mesh.native_vertices
                    .iter_mut()
                    .for_each(|vertice| vertice.texture_index = node_texture_mapping[vertice.texture_index as usize]);

                // Remember the vertex offset/count and gather node vertices.
                let node_vertex_offset = *vertex_offset;
                let node_vertex_count = mesh.native_vertices.len();
                *vertex_offset += node_vertex_count;
                native_vertices.extend(mesh.native_vertices.iter());

                let centroid = Self::calculate_centroid(&mesh.native_vertices);

                Node::new(
                    transform_matrix,
                    centroid,
                    mesh.transparent,
                    node_vertex_offset,
                    node_vertex_count,
                    vec![],
                    current_node.rotation_keyframes.clone(),
                )
            })
            .collect();

        sub_nodes[0].child_nodes = child_nodes;

        sub_nodes
    }

    pub fn calculate_transformation_matrix(node: &mut Node, is_root: bool, bounding_box: AABB, parent_matrix: Matrix4<f32>) {
        node.transform_matrix = match is_root {
            true => {
                let translation_matrix = Matrix4::from_translation(-Vector3::new(
                    bounding_box.center().x,
                    bounding_box.max().y,
                    bounding_box.center().z,
                ));

                parent_matrix * translation_matrix * node.transform_matrix
            }
            false => parent_matrix * node.transform_matrix,
        };

        node.child_nodes
            .iter_mut()
            .for_each(|child_node| Self::calculate_transformation_matrix(child_node, false, bounding_box, node.transform_matrix));
    }

    pub fn load(
        &mut self,
        texture_atlas_factory: &mut TextureAtlasFactory,
        vertex_offset: &mut usize,
        model_file: &str,
        reverse_order: bool,
    ) -> Result<(Model, DeferredVertexGeneration), LoadError> {
        #[cfg(feature = "debug")]
        let timer = Timer::new_dynamic(format!("load rsm model from {}", model_file.magenta()));

        let bytes = match self.game_file_loader.get(&format!("data\\model\\{model_file}")) {
            Ok(bytes) => bytes,
            Err(_error) => {
                #[cfg(feature = "debug")]
                {
                    print_debug!("Failed to load model: {:?}", _error);
                    print_debug!("Replacing with fallback");
                }

                return self.load(texture_atlas_factory, vertex_offset, FALLBACK_MODEL_FILE, reverse_order);
            }
        };
        let mut byte_reader: ByteReader<Option<InternalVersion>> = ByteReader::with_default_metadata(&bytes);

        let model_data = match ModelData::from_bytes(&mut byte_reader) {
            Ok(model_data) => model_data,
            Err(_error) => {
                #[cfg(feature = "debug")]
                {
                    print_debug!("Failed to load model: {:?}", _error);
                    print_debug!("Replacing with fallback");
                }

                return self.load(texture_atlas_factory, vertex_offset, FALLBACK_MODEL_FILE, reverse_order);
            }
        };

        // TODO: Temporary check until we support more versions.
        // TODO: The model operation to scale keyframe is not implemented yet.
        // TODO: The model operation to translate keyframe is not implemented yet.
        // TODO: The model operation to modify texture keyframe is not implemented yet.
        let version: InternalVersion = model_data.version.into();
        if version.equals_or_above(2, 2) {
            #[cfg(feature = "debug")]
            {
                print_debug!("Failed to load model because version {} is unsupported", version);
                print_debug!("Replacing with fallback");
            }

            return self.load(texture_atlas_factory, vertex_offset, FALLBACK_MODEL_FILE, reverse_order);
        }

        let texture_allocation: Vec<TextureAtlasEntry> = model_data
            .texture_names
            .iter()
            .map(|texture_name| texture_atlas_factory.register(texture_name.as_ref()))
            .collect();

        let texture_mapping: Vec<ModelTexture> = texture_allocation
            .iter()
            .enumerate()
            .map(|(index, entry)| ModelTexture {
                index: index as i32,
                transparent: entry.transparent,
            })
            .collect();

        let root_node_name = &model_data.root_node_name.clone().unwrap();
        let root_node = model_data
            .nodes
            .iter()
            .find(|node_data| &node_data.node_name == root_node_name)
            .expect("failed to find main node");

        let mut native_model_vertices = Vec::<NativeModelVertex>::new();

        let mut bounding_box = AABB::uninitialized();
        let mut root_nodes = Self::process_node_mesh(
            root_node,
            &model_data.nodes,
            vertex_offset,
            &mut native_model_vertices,
            &texture_mapping,
            &Matrix4::identity(),
            &mut bounding_box,
            reverse_order,
        );

        Self::calculate_transformation_matrix(&mut root_nodes[0], true, bounding_box, Matrix4::identity());

        let model = Model::new(
            root_nodes,
            bounding_box,
            #[cfg(feature = "debug")]
            model_data,
        );

        let texture_allocation: Vec<AllocationId> = texture_allocation.iter().map(|entry| entry.allocation_id).collect();

        let deferred = DeferredVertexGeneration {
            native_model_vertices,
            texture_allocation,
        };

        #[cfg(feature = "debug")]
        timer.stop();

        Ok((model, deferred))
    }
}

#[derive(Copy, Clone)]
struct ModelTexture {
    index: i32,
    transparent: bool,
}

struct SubMesh {
    transparent: bool,
    native_vertices: Vec<NativeModelVertex>,
}
