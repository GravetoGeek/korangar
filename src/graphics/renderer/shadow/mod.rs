mod vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/geometry_shadow_vertex_shader.glsl"
    }
}

mod fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/geometry_shadow_fragment_shader.glsl"
    }
}

use std::sync::Arc;
use std::iter;

use vulkano::device::Device;
use vulkano::image::ImageViewAbstract;
use vulkano::pipeline::{ GraphicsPipeline, PipelineBindPoint, Pipeline };
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::viewport::{ Viewport, ViewportState };
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::shader::ShaderModule;
use vulkano::render_pass::Subpass;
use vulkano::sampler::{ Sampler, Filter, SamplerAddressMode };
use vulkano::buffer::{ BufferUsage, BufferAccess };

use crate::types::maths::*;
use crate::types::map::model::Node2;
use crate::graphics::*;

use self::vertex_shader::ty::Constants;
use self::vertex_shader::ty::Matrices;

pub struct GeometryShadowRenderer {
    pipeline: Arc<GraphicsPipeline>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    matrices_buffer: CpuBufferPool<Matrices>,
    linear_sampler: Arc<Sampler>,
}

impl GeometryShadowRenderer {

    pub fn new(device: Arc<Device>, subpass: Subpass, viewport: Viewport) -> Self {

        let vertex_shader = vertex_shader::load(device.clone()).unwrap();
        let fragment_shader = fragment_shader::load(device.clone()).unwrap();
        let pipeline = Self::create_pipeline(device.clone(), subpass, viewport, &vertex_shader, &fragment_shader);

        let matrices_buffer = CpuBufferPool::new(device.clone(), BufferUsage::all());
        let linear_sampler = Sampler::start(device)
            .filter(Filter::Linear)
            .address_mode(SamplerAddressMode::ClampToEdge)
            //.mip_lod_bias(1.0)
            //.lod(0.0..=100.0)
            .build()
            .unwrap();
            //create_sampler!(device, Linear, ClampToEdge);

        Self { pipeline, vertex_shader, fragment_shader, matrices_buffer, linear_sampler }
    }

    pub fn recreate_pipeline(&mut self, device: Arc<Device>, subpass: Subpass, viewport: Viewport) {
        self.pipeline = Self::create_pipeline(device, subpass, viewport, &self.vertex_shader, &self.fragment_shader);
    }

    fn create_pipeline(device: Arc<Device>, subpass: Subpass, viewport: Viewport, vertex_shader: &ShaderModule, fragment_shader: &ShaderModule) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<ModelVertex>())
            .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant(iter::once(viewport)))
            .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
            .depth_stencil_state(DepthStencilState::simple_depth_test())
            .render_pass(subpass)
            .build(device)
            .unwrap()
    }

    pub fn render(&self, camera: &dyn Camera, builder: &mut CommandBuilder, vertex_buffer: ModelVertexBuffer, textures: &Vec<Texture>, transform: &Transform) {

        let layout = self.pipeline.layout().clone();
        let descriptor_layout = layout.descriptor_set_layouts().get(0).unwrap().clone();

        // SUPER DIRTY, PLEASE FIX

        let texture0 = textures[0].clone();

        let texture1 = match textures.len() > 1 {
            true => textures[1].clone(),
            false => texture0.clone(),
        };

        let texture2 = match textures.len() > 2 {
            true => textures[2].clone(),
            false => texture0.clone(),
        };

        let texture3 = match textures.len() > 3 {
            true => textures[3].clone(),
            false => texture0.clone(),
        };

        let texture4 = match textures.len() > 4 {
            true => textures[4].clone(),
            false => texture0.clone(),
        };

        let texture5 = match textures.len() > 5 {
            true => textures[5].clone(),
            false => texture0.clone(),
        };

        let texture6 = match textures.len() > 6 {
            true => textures[6].clone(),
            false => texture0.clone(),
        };

        let texture7 = match textures.len() > 7 {
            true => textures[7].clone(),
            false => texture0.clone(),
        };

        let texture8 = match textures.len() > 8 {
            true => textures[8].clone(),
            false => texture0.clone(),
        };

        let texture9 = match textures.len() > 9 {
            true => textures[9].clone(),
            false => texture0.clone(),
        };

        let view_projection_matix = camera.get_light_matrix();
        let matrices = Matrices {
            view_projection: view_projection_matix.into(),
        };
        let matrices_subbuffer = Arc::new(self.matrices_buffer.next(matrices).unwrap());

        let set = PersistentDescriptorSet::new(descriptor_layout, [
            WriteDescriptorSet::buffer(0, matrices_subbuffer),
            WriteDescriptorSet::sampler(1, self.linear_sampler.clone()),
            WriteDescriptorSet::image_view_array(2, 0, [
                texture0 as Arc<dyn ImageViewAbstract + 'static>,
                texture1 as Arc<dyn ImageViewAbstract + 'static>,
                texture2 as Arc<dyn ImageViewAbstract + 'static>,
                texture3 as Arc<dyn ImageViewAbstract + 'static>,
                texture4 as Arc<dyn ImageViewAbstract + 'static>,
                texture5 as Arc<dyn ImageViewAbstract + 'static>,
                texture6 as Arc<dyn ImageViewAbstract + 'static>,
                texture7 as Arc<dyn ImageViewAbstract + 'static>,
                texture8 as Arc<dyn ImageViewAbstract + 'static>,
                texture9 as Arc<dyn ImageViewAbstract + 'static>,
            ])
        ]).unwrap(); 

        let vertex_count = vertex_buffer.size() as usize / std::mem::size_of::<ModelVertex>();
        let world_matrix = camera.transform_matrix(transform);
        let constants = Constants {
            world: world_matrix.into(),
        };

        builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(PipelineBindPoint::Graphics, layout.clone(), 0, set)
            .push_constants(layout, 0, constants)
            .bind_vertex_buffers(0, vertex_buffer)
            .draw(vertex_count as u32, 1, 0, 0).unwrap();
    }

    pub fn render_node(&self, camera: &dyn Camera, builder: &mut CommandBuilder, node: &Node2, transform: &Transform) {

        let layout = self.pipeline.layout().clone();
        let descriptor_layout = layout.descriptor_set_layouts().get(0).unwrap().clone();

        // SUPER DIRTY, PLEASE FIX

        let texture0 = node.textures[0].clone();

        let texture1 = match node.textures.len() > 1 {
            true => node.textures[1].clone(),
            false => texture0.clone(),
        };

        let texture2 = match node.textures.len() > 2 {
            true => node.textures[2].clone(),
            false => texture0.clone(),
        };

        let texture3 = match node.textures.len() > 3 {
            true => node.textures[3].clone(),
            false => texture0.clone(),
        };

        let texture4 = match node.textures.len() > 4 {
            true => node.textures[4].clone(),
            false => texture0.clone(),
        };

        let texture5 = match node.textures.len() > 5 {
            true => node.textures[5].clone(),
            false => texture0.clone(),
        };

        let texture6 = match node.textures.len() > 6 {
            true => node.textures[6].clone(),
            false => texture0.clone(),
        };

        let texture7 = match node.textures.len() > 7 {
            true => node.textures[7].clone(),
            false => texture0.clone(),
        };

        let texture8 = match node.textures.len() > 8 {
            true => node.textures[8].clone(),
            false => texture0.clone(),
        };

        let texture9 = match node.textures.len() > 9 {
            true => node.textures[9].clone(),
            false => texture0.clone(),
        };

        let view_projection_matix = camera.get_light_matrix();
        let matrices = Matrices {
            view_projection: view_projection_matix.into(),
        };
        let matrices_subbuffer = Arc::new(self.matrices_buffer.next(matrices).unwrap());

        let set = PersistentDescriptorSet::new(descriptor_layout, [
            WriteDescriptorSet::buffer(0, matrices_subbuffer),
            WriteDescriptorSet::sampler(1, self.linear_sampler.clone()),
            WriteDescriptorSet::image_view_array(2, 0, [
                texture0 as Arc<dyn ImageViewAbstract + 'static>,
                texture1 as Arc<dyn ImageViewAbstract + 'static>,
                texture2 as Arc<dyn ImageViewAbstract + 'static>,
                texture3 as Arc<dyn ImageViewAbstract + 'static>,
                texture4 as Arc<dyn ImageViewAbstract + 'static>,
                texture5 as Arc<dyn ImageViewAbstract + 'static>,
                texture6 as Arc<dyn ImageViewAbstract + 'static>,
                texture7 as Arc<dyn ImageViewAbstract + 'static>,
                texture8 as Arc<dyn ImageViewAbstract + 'static>,
                texture9 as Arc<dyn ImageViewAbstract + 'static>,
            ])
        ]).unwrap(); 

        let vertex_count = node.vertex_buffer.size() as usize / std::mem::size_of::<ModelVertex>();

        let world_matrix = Matrix4::from_nonuniform_scale(transform.scale.x, transform.scale.y, transform.scale.z)
            //* Matrix4::from_axis_angle(axis, angle)
            * Matrix4::from_translation(transform.position)
            * node.transform_matrix;
        
        let constants = Constants {
            world: world_matrix.into(),
        };

        builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(PipelineBindPoint::Graphics, layout.clone(), 0, set)
            .push_constants(layout, 0, constants)
            .bind_vertex_buffers(0, node.vertex_buffer.clone())
            .draw(vertex_count as u32, 1, 0, 0).unwrap();
    }
}
