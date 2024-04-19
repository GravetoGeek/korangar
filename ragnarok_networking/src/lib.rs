use std::net::Ipv4Addr;

use derive_new::new;
use ragnarok_bytes::{
    ByteConvertable, ByteStream, ConversionError, ConversionResult, ConversionResultExt, FixedByteSize, FromBytes, ToBytes,
};
#[cfg(feature = "derive")]
pub use ragnarok_procedural::{IncomingPacket, OutgoingPacket};
#[cfg(not(feature = "derive"))]
use ragnarok_procedural::{IncomingPacket, OutgoingPacket};

// To make proc macros work in korangar_interface.
extern crate self as ragnarok_networking;

/// Base trait that all incoming packets implement.
/// All packets in Ragnarok online consist of a header, two bytes in size,
/// followed by the packet data. If the packet does not have a fixed size,
/// the first two bytes will be the size of the packet in bytes *including* the
/// header. Packets are sent in little endian.
pub trait IncomingPacket: Clone {
    const IS_PING: bool;
    const HEADER: u16;

    /// Read packet **without the header**. To read the packet with the header,
    /// use [`IncomingPacketExt::packet_from_bytes`].
    fn payload_from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self>;
}

/// Base trait that all outgoing packets implement.
/// All packets in Ragnarok online consist of a header, two bytes in size,
/// followed by the packet data. If the packet does not have a fixed size,
/// the first two bytes will be the size of the packet in bytes *including* the
/// header. Packets are sent in little endian.
pub trait OutgoingPacket: Clone {
    const IS_PING: bool;

    fn packet_to_bytes(&self) -> ConversionResult<Vec<u8>>;
}

/// Extension trait for reading incoming packets with the header.
pub trait IncomingPacketExt: IncomingPacket {
    /// Read packet **with the header**. To read the packet without the header,
    /// use [`IncomingPacket::payload_from_bytes`].
    fn packet_from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self>;
}

impl<T> IncomingPacketExt for T
where
    T: IncomingPacket,
{
    fn packet_from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        let header = u16::from_bytes(byte_stream)?;

        if header != Self::HEADER {
            return Err(ConversionError::from_message("mismatched header"));
        }

        Self::payload_from_bytes(byte_stream)
    }
}

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ClientTick(pub u32);

// TODO: move to login
#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct AccountId(pub u32);

// TODO: move to character
#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct CharacterId(pub u32);

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct PartyId(pub u32);

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct EntityId(pub u32);

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct SkillId(pub u16);

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct SkillLevel(pub u16);

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ServerAddress(pub [u8; 4]);

impl From<ServerAddress> for Ipv4Addr {
    fn from(value: ServerAddress) -> Self {
        value.0.into()
    }
}

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct TilePosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct LargeTilePosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ColorBGRA {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ColorRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// Item index is always actual index + 2.
#[derive(Clone, Copy, Debug, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ItemIndex(u16);

impl FromBytes for ItemIndex {
    fn from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        u16::from_bytes(byte_stream).map(|raw| Self(raw - 2))
    }
}

impl ToBytes for ItemIndex {
    fn to_bytes(&self) -> ConversionResult<Vec<u8>> {
        u16::to_bytes(&(self.0 + 2))
    }
}

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ItemId(pub u32);

#[derive(Copy, Clone, Debug, ByteConvertable, FixedByteSize, PartialEq)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum Sex {
    Female,
    Male,
    Both,
    Server,
}

/// Sent by the client to the login server.
/// The very first packet sent when logging in, it is sent after the user has
/// entered email and password.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0064)]
pub struct LoginServerLoginPacket {
    /// Unused
    #[new(default)]
    pub version: [u8; 4],
    #[length_hint(24)]
    pub name: String,
    #[length_hint(24)]
    pub password: String,
    /// Unused
    #[new(default)]
    pub client_type: u8,
}

/// Sent by the login server as a response to [LoginServerLoginPacket]
/// succeeding. After receiving this packet, the client will connect to one of
/// the character servers provided by this packet.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0AC4)]
pub struct LoginServerLoginSuccessPacket {
    #[packet_length]
    pub packet_length: u16,
    pub login_id1: u32,
    pub account_id: AccountId,
    pub login_id2: u32,
    /// Deprecated and always 0 on rAthena
    pub ip_address: u32,
    /// Deprecated and always 0 on rAthena
    pub name: [u8; 24],
    /// Always 0 on rAthena
    pub unknown: u16,
    pub sex: Sex,
    pub auth_token: [u8; 17],
    #[repeating_remaining]
    pub character_server_information: Vec<CharacterServerInformation>,
}

/// Sent by the character server as a response to [CharacterServerLoginPacket]
/// succeeding. Provides basic information about the number of available
/// character slots.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x082D)]
pub struct CharacterServerLoginSuccessPacket {
    /// Always 29 on rAthena
    pub unknown: u16,
    pub normal_slot_count: u8,
    pub vip_slot_count: u8,
    pub billing_slot_count: u8,
    pub poducilble_slot_count: u8,
    pub vaild_slot: u8,
    pub unused: [u8; 20],
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x006B)]
pub struct Packet6b00 {
    pub unused: u16,
    pub maximum_slot_count: u8,
    pub available_slot_count: u8,
    pub vip_slot_count: u8,
    pub unknown: [u8; 20],
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B18)]
pub struct Packet180b {
    /// Possibly inventory related
    pub unknown: u16,
}

#[derive(Clone, Debug, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct WorldPosition {
    pub x: usize,
    pub y: usize,
}

impl FromBytes for WorldPosition {
    fn from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        let coordinates = byte_stream.slice::<Self>(3)?;

        let x = (coordinates[1] >> 6) | (coordinates[0] << 2);
        let y = (coordinates[2] >> 4) | ((coordinates[1] & 0b111111) << 4);
        //let direction = ...

        Ok(Self {
            x: x as usize,
            y: y as usize,
        })
    }
}

impl ToBytes for WorldPosition {
    fn to_bytes(&self) -> ConversionResult<Vec<u8>> {
        let mut coordinates = vec![0, 0, 0];

        coordinates[0] = (self.x >> 2) as u8;
        coordinates[1] = ((self.x << 6) as u8) | (((self.y >> 4) & 0x3F) as u8);
        coordinates[2] = (self.y << 4) as u8;

        Ok(coordinates)
    }
}

#[derive(Clone, Debug, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct WorldPosition2 {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl FromBytes for WorldPosition2 {
    fn from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        let coordinates: Vec<usize> = byte_stream.slice::<Self>(6)?.iter().map(|byte| *byte as usize).collect();

        let x1 = (coordinates[1] >> 6) | (coordinates[0] << 2);
        let y1 = (coordinates[2] >> 4) | ((coordinates[1] & 0b111111) << 4);
        let x2 = (coordinates[3] >> 2) | ((coordinates[2] & 0b1111) << 6);
        let y2 = coordinates[4] | ((coordinates[3] & 0b11) << 8);
        //let direction = ...

        Ok(Self { x1, y1, x2, y2 })
    }
}

/// Sent by the map server as a response to [MapServerLoginPacket] succeeding.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02EB)]
pub struct MapServerLoginSuccessPacket {
    pub client_tick: ClientTick,
    pub position: WorldPosition,
    /// Always [5, 5] on rAthena
    pub ignored: [u8; 2],
    pub font: u16,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum LoginFailedReason {
    #[numeric_value(1)]
    ServerClosed,
    #[numeric_value(2)]
    AlreadyLoggedIn,
    #[numeric_value(8)]
    AlreadyOnline,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0081)]
pub struct LoginFailedPacket {
    pub reason: LoginFailedReason,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0840)]
pub struct MapServerUnavailablePacket {
    pub packet_length: u16,
    #[length_hint(self.packet_length - 4)]
    pub unknown: String,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum LoginFailedReason2 {
    UnregisteredId,
    IncorrectPassword,
    IdExpired,
    RejectedFromServer,
    BlockedByGMTeam,
    GameOutdated,
    LoginProhibitedUntil,
    ServerFull,
    CompanyAccountLimitReached,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x083E)]
pub struct LoginFailedPacket2 {
    pub reason: LoginFailedReason2,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum CharacterSelectionFailedReason {
    RejectedFromServer,
}

/// Sent by the character server as a response to [SelectCharacterPacket]
/// failing. Provides a reason for the character selection failing.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x006C)]
pub struct CharacterSelectionFailedPacket {
    pub reason: CharacterSelectionFailedReason,
}

/// Sent by the character server as a response to [SelectCharacterPacket]
/// succeeding. Provides a map server to connect to, along with the ID of our
/// selected character.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0AC5)]
pub struct CharacterSelectionSuccessPacket {
    pub character_id: CharacterId,
    #[length_hint(16)]
    pub map_name: String,
    pub map_server_ip: ServerAddress,
    pub map_server_port: u16,
    pub unknown: [u8; 128],
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum CharacterCreationFailedReason {
    CharacterNameAlreadyUsed,
    NotOldEnough,
    #[numeric_value(3)]
    NotAllowedToUseSlot,
    #[numeric_value(255)]
    CharacterCerationFailed,
}

/// Sent by the character server as a response to [CreateCharacterPacket]
/// failing. Provides a reason for the character creation failing.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x006E)]
pub struct CharacterCreationFailedPacket {
    pub reason: CharacterCreationFailedReason,
}

/// Sent by the client to the login server every 60 seconds to keep the
/// connection alive.
#[derive(Clone, Debug, Default, OutgoingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0200)]
#[ping]
pub struct LoginServerKeepalivePacket {
    pub user_id: [u8; 24],
}

#[derive(Clone, Debug, FromBytes, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct CharacterServerInformation {
    pub server_ip: ServerAddress,
    pub server_port: u16,
    #[length_hint(20)]
    pub server_name: String,
    pub user_count: u16,
    pub server_type: u16, // ServerType
    pub display_new: u16, // bool16 ?
    pub unknown: [u8; 128],
}

/// Sent by the client to the character server after after successfully logging
/// into the login server.
/// Attempts to log into the character server using the provided information.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0065)]
pub struct CharacterServerLoginPacket {
    pub account_id: AccountId,
    pub login_id1: u32,
    pub login_id2: u32,
    #[new(default)]
    pub unknown: u16,
    pub sex: Sex,
}

/// Sent by the client to the map server after after successfully selecting a
/// character. Attempts to log into the map server using the provided
/// information.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0436)]
pub struct MapServerLoginPacket {
    pub account_id: AccountId,
    pub character_id: CharacterId,
    pub login_id1: u32,
    pub client_tick: ClientTick,
    pub sex: Sex,
    #[new(default)]
    pub unknown: [u8; 4],
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0283)]
pub struct Packet8302 {
    pub entity_id: EntityId,
}

/// Sent by the client to the character server when the player tries to create
/// a new character.
/// Attempts to create a new character in an empty slot using the provided
/// information.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A39)]
pub struct CreateCharacterPacket {
    #[length_hint(24)]
    pub name: String,
    pub slot: u8,
    pub hair_color: u16, // TODO: HairColor
    pub hair_style: u16, // TODO: HairStyle
    pub start_job: u16,  // TODO: Job
    #[new(default)]
    pub unknown: [u8; 2],
    pub sex: Sex,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct CharacterInformation {
    pub character_id: CharacterId,
    pub experience: i64,
    pub money: i32,
    pub job_experience: i64,
    pub jop_level: i32,
    pub body_state: i32,
    pub health_state: i32,
    pub effect_state: i32,
    pub virtue: i32,
    pub honor: i32,
    pub jobpoint: i16,
    pub health_points: i64,
    pub maximum_health_points: i64,
    pub spell_points: i64,
    pub maximum_spell_points: i64,
    pub movement_speed: i16,
    pub job: i16,
    pub head: i16,
    pub body: i16,
    pub weapon: i16,
    pub level: i16,
    pub sp_point: i16,
    pub accessory: i16,
    pub shield: i16,
    pub accessory2: i16,
    pub accessory3: i16,
    pub head_palette: i16,
    pub body_palette: i16,
    #[length_hint(24)]
    pub name: String,
    pub strength: u8,
    pub agility: u8,
    pub vit: u8,
    pub intelligence: u8,
    pub dexterity: u8,
    pub luck: u8,
    pub character_number: u8,
    pub hair_color: u8,
    pub b_is_changed_char: i16,
    #[length_hint(16)]
    pub map_name: String,
    pub deletion_reverse_date: i32,
    pub robe_palette: i32,
    pub character_slot_change_count: i32,
    pub character_name_change_count: i32,
    pub sex: Sex,
}

/// Sent by the character server as a response to [CreateCharacterPacket]
/// succeeding. Provides all character information of the newly created
/// character.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B6F)]
pub struct CreateCharacterSuccessPacket {
    pub character_information: CharacterInformation,
}

/// Sent by the client to the character server.
/// Requests a list of every character associated with the account.
#[derive(Clone, Debug, Default, OutgoingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09A1)]
pub struct RequestCharacterListPacket {}

/// Sent by the character server as a response to [RequestCharacterListPacket]
/// succeeding. Provides the requested list of character information.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B72)]
pub struct RequestCharacterListSuccessPacket {
    #[packet_length]
    pub packet_length: u16,
    #[repeating_remaining]
    pub character_information: Vec<CharacterInformation>,
}

/// Sent by the client to the map server when the player wants to move.
/// Attempts to path the player towards the provided position.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0881)]
pub struct RequestPlayerMovePacket {
    pub position: WorldPosition,
}

/// Sent by the client to the map server when the player wants to warp.
/// Attempts to warp the player to a specific position on a specific map using
/// the provided information.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0140)]
pub struct RequestWarpToMapPacket {
    #[length_hint(16)]
    pub map_name: String,
    pub position: TilePosition,
}

/// Sent by the map server to the client.
/// Informs the client that an entity is pathing towards a new position.
/// Provides the initial position and destination of the movement, as well as a
/// timestamp of when it started (for synchronization).
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0086)]
pub struct EntityMovePacket {
    pub entity_id: EntityId,
    pub from_to: WorldPosition2,
    pub timestamp: ClientTick,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0088)]
pub struct EntityStopMovePacket {
    pub entity_id: EntityId,
    pub position: TilePosition,
}

/// Sent by the map server to the client.
/// Informs the client that the player is pathing towards a new position.
/// Provides the initial position and destination of the movement, as well as a
/// timestamp of when it started (for synchronization).
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0087)]
pub struct PlayerMovePacket {
    pub timestamp: ClientTick,
    pub from_to: WorldPosition2,
}

/// Sent by the client to the character server when the user tries to delete a
/// character.
/// Attempts to delete a character from the user account using the provided
/// information.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01FB)]
pub struct DeleteCharacterPacket {
    character_id: CharacterId,
    /// This field can be used for email or date of birth, depending on the
    /// configuration of the character server.
    #[length_hint(40)]
    pub email: String,
    /// Ignored by rAthena
    #[new(default)]
    pub unknown: [u8; 10],
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum CharacterDeletionFailedReason {
    NotAllowed,
    CharacterNotFound,
    NotEligible,
}

/// Sent by the character server as a response to [DeleteCharacterPacket]
/// failing. Provides a reason for the character deletion failing.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0070)]
pub struct CharacterDeletionFailedPacket {
    pub reason: CharacterDeletionFailedReason,
}

/// Sent by the character server as a response to [DeleteCharacterPacket]
/// succeeding.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x006F)]
pub struct CharacterDeletionSuccessPacket {}

/// Sent by the client to the character server when the user selects a
/// character. Attempts to select the character in the specified slot.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0066)]
pub struct SelectCharacterPacket {
    pub selected_slot: u8,
}

/// Sent by the map server to the client when there is a new chat message from
/// the server. Provides the message to be displayed in the chat window.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x008E)]
pub struct ServerMessagePacket {
    pub packet_length: u16,
    #[length_hint(self.packet_length - 4)]
    pub message: String,
}

/// Sent by the client to the map server when the user hovers over an entity.
/// Attempts to fetch additional information about the entity, such as the
/// display name.
#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0368)]
pub struct RequestDetailsPacket {
    pub entity_id: EntityId,
}

/// Sent by the map server to the client as a response to
/// [RequestDetailsPacket]. Provides additional information about the player.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A30)]
pub struct RequestPlayerDetailsSuccessPacket {
    pub character_id: CharacterId,
    #[length_hint(24)]
    pub name: String,
    #[length_hint(24)]
    pub party_name: String,
    #[length_hint(24)]
    pub guild_name: String,
    #[length_hint(24)]
    pub position_name: String,
    pub title_id: u32,
}

/// Sent by the map server to the client as a response to
/// [RequestDetailsPacket]. Provides additional information about the entity.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0ADF)]
pub struct RequestEntityDetailsSuccessPacket {
    pub entity_id: EntityId,
    pub group_id: u32,
    #[length_hint(24)]
    pub name: String,
    #[length_hint(24)]
    pub title: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09E7)]
pub struct NewMailStatusPacket {
    pub new_available: u8,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct AchievementData {
    pub acheivement_id: u32,
    pub is_completed: u8,
    pub objectives: [u32; 10],
    pub completion_timestamp: u32,
    pub got_rewarded: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A24)]
pub struct AchievementUpdatePacket {
    pub total_score: u32,
    pub level: u16,
    pub acheivement_experience: u32,
    pub acheivement_experience_to_next_level: u32, // "to_next_level" might be wrong
    pub acheivement_data: AchievementData,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A23)]
pub struct AchievementListPacket {
    #[packet_length]
    pub packet_length: u16,
    pub acheivement_count: u32,
    pub total_score: u32,
    pub level: u16,
    pub acheivement_experience: u32,
    pub acheivement_experience_to_next_level: u32, // "to_next_level" might be wrong
    #[repeating(self.acheivement_count)]
    pub acheivement_data: Vec<AchievementData>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0ADE)]
pub struct CriticalWeightUpdatePacket {
    pub packet_length: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01D7)]
pub struct SpriteChangePacket {
    pub account_id: AccountId,
    pub sprite_type: u8, // TODO: Is it actually the sprite type?
    pub value: u32,
    pub value2: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B08)]
pub struct InventoyStartPacket {
    pub packet_length: u16,
    pub inventory_type: u8,
    #[length_hint(self.packet_length - 5)]
    pub inventory_name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B0B)]
pub struct InventoyEndPacket {
    pub inventory_type: u8,
    pub flag: u8, // maybe char ?
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ItemOptions {
    pub index: u16,
    pub value: u16,
    pub parameter: u8,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct RegularItemInformation {
    pub index: ItemIndex,
    pub item_id: ItemId,
    pub item_type: u8,
    pub amount: u16,
    pub wear_state: u32,
    pub slot: [u32; 4], // card ?
    pub hire_expiration_date: i32,
    pub fags: u8, // bit 1 - is_identified; bit 2 - place_in_etc_tab;
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B09)]
pub struct RegularItemListPacket {
    #[packet_length]
    pub packet_length: u16,
    pub inventory_type: u8,
    #[repeating_remaining]
    pub item_information: Vec<RegularItemInformation>,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct EquippableItemInformation {
    pub index: ItemIndex,
    pub item_id: ItemId,
    pub item_type: u8,
    pub equip_position: EquipPosition,
    pub equipped_position: EquipPosition,
    pub slot: [u32; 4], // card ?
    pub hire_expiration_date: i32,
    pub bind_on_equip_type: u16,
    pub w_item_sprite_number: u16,
    pub option_count: u8,
    pub option_data: [ItemOptions; 5], // fix count
    pub refinement_level: u8,
    pub enchantment_level: u8,
    pub fags: u8, // bit 1 - is_identified; bit 2 - is_damaged; bit 3 - place_in_etc_tab
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B39)]
pub struct EquippableItemListPacket {
    #[packet_length]
    pub packet_length: u16,
    pub inventory_type: u8,
    #[repeating_remaining]
    pub item_information: Vec<EquippableItemInformation>,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct EquippableSwitchItemInformation {
    pub index: ItemIndex,
    pub position: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A9B)]
pub struct EquippableSwitchItemListPacket {
    #[packet_length]
    pub packet_length: u16,
    #[repeating_remaining]
    pub item_information: Vec<EquippableSwitchItemInformation>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x099B)]
pub struct MapTypePacket {
    pub map_type: u16,
    pub flags: u32,
}

/// Sent by the map server to the client when there is a new chat message from
/// ??. Provides the message to be displayed in the chat window, as well as
/// information on how the message should be displayed.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01C3)]
pub struct Broadcast2MessagePacket {
    pub packet_length: u16,
    pub font_color: ColorRGBA,
    pub font_type: u16,
    pub font_size: u16,
    pub font_alignment: u16,
    pub font_y: u16,
    #[length_hint(self.packet_length - 16)]
    pub message: String,
}

/// Sent by the map server to the client when when someone uses the @broadcast
/// command. Provides the message to be displayed in the chat window.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x009A)]
pub struct BroadcastMessagePacket {
    pub packet_length: u16,
    #[length_hint(self.packet_length - 2)]
    pub message: String,
}

/// Sent by the map server to the client when when someone writes in proximity
/// chat. Provides the source player and message to be displayed in the chat
/// window and the speach bubble.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x008D)]
pub struct OverheadMessagePacket {
    pub packet_length: u16,
    pub entity_id: EntityId,
    #[length_hint(self.packet_length - 6)]
    pub message: String,
}

/// Sent by the map server to the client when there is a new chat message from
/// an entity. Provides the message to be displayed in the chat window, the
/// color of the message, and the ID of the entity it originated from.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02C1)]
pub struct EntityMessagePacket {
    pub packet_length: u16,
    pub entity_id: EntityId,
    pub color: ColorBGRA,
    #[length_hint(self.packet_length - 12)]
    pub message: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00C0)]
pub struct DisplayEmotionPacket {
    pub entity_id: EntityId,
    pub emotion: u8,
}

/// Every value that can be set from the server through [UpdateStatusPacket],
/// [UpdateStatusPacket1], [UpdateStatusPacket2], and [UpdateStatusPacket3].
/// All UpdateStatusPackets do the same, they just have different sizes
/// correlating to the space the updated value requires.
#[derive(Clone, Debug)]
pub enum StatusType {
    Weight(u32),
    MaximumWeight(u32),
    MovementSpeed(u32),
    BaseLevel(u32),
    JobLevel(u32),
    Karma(u32),
    Manner(u32),
    StatusPoint(u32),
    SkillPoint(u32),
    Hit(u32),
    Flee1(u32),
    Flee2(u32),
    MaximumHealthPoints(u32),
    MaximumSpellPoints(u32),
    HealthPoints(u32),
    SpellPoints(u32),
    AttackSpeed(u32),
    Attack1(u32),
    Defense1(u32),
    MagicDefense1(u32),
    Attack2(u32),
    Defense2(u32),
    MagicDefense2(u32),
    Critical(u32),
    MagicAttack1(u32),
    MagicAttack2(u32),
    Zeny(u32),
    BaseExperience(u64),
    JobExperience(u64),
    NextBaseExperience(u64),
    NextJobExperience(u64),
    SpUstr(u8),
    SpUagi(u8),
    SpUvit(u8),
    SpUint(u8),
    SpUdex(u8),
    SpUluk(u8),
    Strength(u32, u32),
    Agility(u32, u32),
    Vitality(u32, u32),
    Intelligence(u32, u32),
    Dexterity(u32, u32),
    Luck(u32, u32),
    CartInfo(u16, u32, u32),
    ActivityPoints(u32),
    TraitPoint(u32),
    MaximumActivityPoints(u32),
    Power(u32, u32),
    Stamina(u32, u32),
    Wisdom(u32, u32),
    Spell(u32, u32),
    Concentration(u32, u32),
    Creativity(u32, u32),
    SpUpow(u8),
    SpUsta(u8),
    SpUwis(u8),
    SpUspl(u8),
    SpUcon(u8),
    SpUcrt(u8),
    PhysicalAttack(u32),
    SpellMagicAttack(u32),
    Resistance(u32),
    MagicResistance(u32),
    HealingPlus(u32),
    CriticalDamageRate(u32),
}

impl FromBytes for StatusType {
    fn from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        let status = match u16::from_bytes(byte_stream).trace::<Self>()? {
            0 => u32::from_bytes(byte_stream).map(Self::MovementSpeed),
            1 => u64::from_bytes(byte_stream).map(Self::BaseExperience),
            2 => u64::from_bytes(byte_stream).map(Self::JobExperience),
            3 => u32::from_bytes(byte_stream).map(Self::Karma),
            4 => u32::from_bytes(byte_stream).map(Self::Manner),
            5 => u32::from_bytes(byte_stream).map(Self::HealthPoints),
            6 => u32::from_bytes(byte_stream).map(Self::MaximumHealthPoints),
            7 => u32::from_bytes(byte_stream).map(Self::SpellPoints),
            8 => u32::from_bytes(byte_stream).map(Self::MaximumSpellPoints),
            9 => u32::from_bytes(byte_stream).map(Self::StatusPoint),
            11 => u32::from_bytes(byte_stream).map(Self::BaseLevel),
            12 => u32::from_bytes(byte_stream).map(Self::SkillPoint),
            13 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Strength(a, u32::from_bytes(byte_stream)?))),
            14 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Agility(a, u32::from_bytes(byte_stream)?))),
            15 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Vitality(a, u32::from_bytes(byte_stream)?))),
            16 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Intelligence(a, u32::from_bytes(byte_stream)?))),
            17 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Dexterity(a, u32::from_bytes(byte_stream)?))),
            18 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Luck(a, u32::from_bytes(byte_stream)?))),
            20 => u32::from_bytes(byte_stream).map(Self::Zeny),
            22 => u64::from_bytes(byte_stream).map(Self::NextBaseExperience),
            23 => u64::from_bytes(byte_stream).map(Self::NextJobExperience),
            24 => u32::from_bytes(byte_stream).map(Self::Weight),
            25 => u32::from_bytes(byte_stream).map(Self::MaximumWeight),
            32 => u8::from_bytes(byte_stream).map(Self::SpUstr),
            33 => u8::from_bytes(byte_stream).map(Self::SpUagi),
            34 => u8::from_bytes(byte_stream).map(Self::SpUvit),
            35 => u8::from_bytes(byte_stream).map(Self::SpUint),
            36 => u8::from_bytes(byte_stream).map(Self::SpUdex),
            37 => u8::from_bytes(byte_stream).map(Self::SpUluk),
            41 => u32::from_bytes(byte_stream).map(Self::Attack1),
            42 => u32::from_bytes(byte_stream).map(Self::Attack2),
            43 => u32::from_bytes(byte_stream).map(Self::MagicAttack1),
            44 => u32::from_bytes(byte_stream).map(Self::MagicAttack2),
            45 => u32::from_bytes(byte_stream).map(Self::Defense1),
            46 => u32::from_bytes(byte_stream).map(Self::Defense2),
            47 => u32::from_bytes(byte_stream).map(Self::MagicDefense1),
            48 => u32::from_bytes(byte_stream).map(Self::MagicDefense2),
            49 => u32::from_bytes(byte_stream).map(Self::Hit),
            50 => u32::from_bytes(byte_stream).map(Self::Flee1),
            51 => u32::from_bytes(byte_stream).map(Self::Flee2),
            52 => u32::from_bytes(byte_stream).map(Self::Critical),
            53 => u32::from_bytes(byte_stream).map(Self::AttackSpeed),
            55 => u32::from_bytes(byte_stream).map(Self::JobLevel),
            99 => u16::from_bytes(byte_stream)
                .and_then(|a| Ok(Self::CartInfo(a, u32::from_bytes(byte_stream)?, u32::from_bytes(byte_stream)?))),
            219 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Power(a, u32::from_bytes(byte_stream)?))),
            220 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Stamina(a, u32::from_bytes(byte_stream)?))),
            221 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Wisdom(a, u32::from_bytes(byte_stream)?))),
            222 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Spell(a, u32::from_bytes(byte_stream)?))),
            223 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Concentration(a, u32::from_bytes(byte_stream)?))),
            224 => u32::from_bytes(byte_stream).and_then(|a| Ok(Self::Creativity(a, u32::from_bytes(byte_stream)?))),
            225 => u32::from_bytes(byte_stream).map(Self::PhysicalAttack),
            226 => u32::from_bytes(byte_stream).map(Self::SpellMagicAttack),
            227 => u32::from_bytes(byte_stream).map(Self::Resistance),
            228 => u32::from_bytes(byte_stream).map(Self::MagicResistance),
            229 => u32::from_bytes(byte_stream).map(Self::HealingPlus),
            230 => u32::from_bytes(byte_stream).map(Self::CriticalDamageRate),
            231 => u32::from_bytes(byte_stream).map(Self::TraitPoint),
            232 => u32::from_bytes(byte_stream).map(Self::ActivityPoints),
            233 => u32::from_bytes(byte_stream).map(Self::MaximumActivityPoints),
            247 => u8::from_bytes(byte_stream).map(Self::SpUpow),
            248 => u8::from_bytes(byte_stream).map(Self::SpUsta),
            249 => u8::from_bytes(byte_stream).map(Self::SpUwis),
            250 => u8::from_bytes(byte_stream).map(Self::SpUspl),
            251 => u8::from_bytes(byte_stream).map(Self::SpUcon),
            252 => u8::from_bytes(byte_stream).map(Self::SpUcrt),
            invalid => Err(ConversionError::from_message(format!("invalid status code {invalid}"))),
        };

        status.trace::<Self>()
    }
}

// TODO: make StatusType derivable
#[cfg(feature = "interface")]
impl<App: korangar_interface::application::Application> korangar_interface::elements::PrototypeElement<App> for StatusType {
    fn to_element(&self, display: String) -> korangar_interface::elements::ElementCell<App> {
        format!("{self:?}").to_element(display)
    }
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B0)]
pub struct UpdateStatusPacket {
    #[length_hint(6)]
    pub status_type: StatusType,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0196)]
pub struct StatusChangeSequencePacket {
    pub index: u16,
    pub id: u32,
    pub state: u8,
}

/// Sent by the character server to the client when loading onto a new map.
/// This packet is ignored by Korangar since all of the provided values are set
/// again individually using the UpdateStatusPackets.
#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00BD)]
pub struct InitialStatusPacket {
    pub status_points: u16,
    pub strength: u8,
    pub required_strength: u8,
    pub agility: u8,
    pub required_agility: u8,
    pub vitatity: u8,
    pub required_vitatity: u8,
    pub intelligence: u8,
    pub required_intelligence: u8,
    pub dexterity: u8,
    pub required_dexterity: u8,
    pub luck: u8,
    pub required_luck: u8,
    pub left_attack: u16,
    pub rigth_attack: u16,
    pub rigth_magic_attack: u16,
    pub left_magic_attack: u16,
    pub left_defense: u16,
    pub rigth_defense: u16,
    pub rigth_magic_defense: u16,
    pub left_magic_defense: u16,
    pub hit: u16, // ?
    pub flee: u16,
    pub flee2: u16,
    pub crit: u16,
    pub attack_speed: u16,
    /// Always 0 on rAthena
    pub bonus_attack_speed: u16,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0141)]
pub struct UpdateStatusPacket1 {
    #[length_hint(12)]
    pub status_type: StatusType,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0ACB)]
pub struct UpdateStatusPacket2 {
    #[length_hint(10)]
    pub status_type: StatusType,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00BE)]
pub struct UpdateStatusPacket3 {
    #[length_hint(3)]
    pub status_type: StatusType,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x013A)]
pub struct UpdateAttackRangePacket {
    pub attack_range: u16,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x08D4)]
pub struct SwitchCharacterSlotPacket {
    pub origin_slot: u16,
    pub destination_slot: u16,
    /// 1 instead of default, just in case the sever actually uses this value
    /// (rAthena does not)
    #[new(value = "1")]
    pub remaining_moves: u16,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum Action {
    Attack,
    PickUpItem,
    SitDown,
    StandUp,
    #[numeric_value(7)]
    ContinousAttack,
    /// Unsure what this does
    #[numeric_value(12)]
    TouchSkill,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0437)]
pub struct RequestActionPacket {
    pub npc_id: EntityId,
    pub action: Action,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00F3)]
pub struct GlobalMessagePacket {
    pub packet_length: u16,
    pub message: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0139)]
pub struct RequestPlayerAttackFailedPacket {
    pub target_entity_id: EntityId,
    pub target_position: TilePosition,
    pub position: TilePosition,
    pub attack_range: u16,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0977)]
pub struct UpdateEntityHealthPointsPacket {
    pub entity_id: EntityId,
    pub health_points: u32,
    pub maximum_health_points: u32,
}

/*#[derive(Clone, Debug, ByteConvertable)]
pub enum DamageType {
}*/

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x08C8)]
pub struct DamagePacket {
    pub source_entity_id: EntityId,
    pub destination_entity_id: EntityId,
    pub client_tick: ClientTick,
    pub source_movement_speed: u32,
    pub destination_movement_speed: u32,
    pub damage_amount: u32,
    pub is_special_damage: u8,
    pub amount_of_hits: u16,
    pub damage_type: u8,
    /// Assassin dual wield damage
    pub damage_amount2: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x007F)]
#[ping]
pub struct ServerTickPacket {
    pub client_tick: ClientTick,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0360)]
#[ping]
pub struct RequestServerTickPacket {
    pub client_tick: ClientTick,
}

#[derive(Clone, Debug, PartialEq, Eq, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum SwitchCharacterSlotResponseStatus {
    Success,
    Error,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B70)]
pub struct SwitchCharacterSlotResponsePacket {
    pub unknown: u16, // is always 8 ?
    pub status: SwitchCharacterSlotResponseStatus,
    pub remaining_moves: u16,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0091)]
pub struct ChangeMapPacket {
    #[length_hint(16)]
    pub map_name: String,
    pub position: TilePosition,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum DissapearanceReason {
    OutOfSight,
    Died,
    LoggedOut,
    Teleported,
    TrickDead,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0080)]
pub struct EntityDisappearedPacket {
    pub entity_id: EntityId,
    pub reason: DissapearanceReason,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09FD)]
pub struct MovingEntityAppearedPacket {
    pub packet_length: u16,
    pub object_type: u8,
    pub entity_id: EntityId,
    pub group_id: u32, // may be reversed - or completely wrong
    pub movement_speed: u16,
    pub body_state: u16,
    pub health_state: u16,
    pub effect_state: u32,
    pub job: u16,
    pub head: u16,
    pub weapon: u32,
    pub shield: u32,
    pub accessory: u16,
    pub move_start_time: u32,
    pub accessory2: u16,
    pub accessory3: u16,
    pub head_palette: u16,
    pub body_palette: u16,
    pub head_direction: u16,
    pub robe: u16,
    pub guild_id: u32, // may be reversed - or completely wrong
    pub emblem_version: u16,
    pub honor: u16,
    pub virtue: u32,
    pub is_pk_mode_on: u8,
    pub sex: Sex,
    pub position: WorldPosition2,
    pub x_size: u8,
    pub y_size: u8,
    pub c_level: u16,
    pub font: u16,
    pub maximum_health_points: i32,
    pub health_points: i32,
    pub is_boss: u8,
    pub body: u16,
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09FE)]
pub struct EntityAppearedPacket {
    pub packet_length: u16,
    pub object_type: u8,
    pub entity_id: EntityId,
    pub group_id: u32, // may be reversed - or completely wrong
    pub movement_speed: u16,
    pub body_state: u16,
    pub health_state: u16,
    pub effect_state: u32,
    pub job: u16,
    pub head: u16,
    pub weapon: u32,
    pub shield: u32,
    pub accessory: u16,
    pub accessory2: u16,
    pub accessory3: u16,
    pub head_palette: u16,
    pub body_palette: u16,
    pub head_direction: u16,
    pub robe: u16,
    pub guild_id: u32, // may be reversed - or completely wrong
    pub emblem_version: u16,
    pub honor: u16,
    pub virtue: u32,
    pub is_pk_mode_on: u8,
    pub sex: Sex,
    pub position: WorldPosition,
    pub x_size: u8,
    pub y_size: u8,
    pub c_level: u16,
    pub font: u16,
    pub maximum_health_points: i32,
    pub health_points: i32,
    pub is_boss: u8,
    pub body: u16,
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09FF)]
pub struct EntityAppeared2Packet {
    pub packet_length: u16,
    pub object_type: u8,
    pub entity_id: EntityId,
    pub group_id: u32, // may be reversed - or completely wrong
    pub movement_speed: u16,
    pub body_state: u16,
    pub health_state: u16,
    pub effect_state: u32,
    pub job: u16,
    pub head: u16,
    pub weapon: u32,
    pub shield: u32,
    pub accessory: u16,
    pub accessory2: u16,
    pub accessory3: u16,
    pub head_palette: u16,
    pub body_palette: u16,
    pub head_direction: u16,
    pub robe: u16,
    pub guild_id: u32, // may be reversed - or completely wrong
    pub emblem_version: u16,
    pub honor: u16,
    pub virtue: u32,
    pub is_pk_mode_on: u8,
    pub sex: Sex,
    pub position: WorldPosition,
    pub x_size: u8,
    pub y_size: u8,
    pub state: u8,
    pub c_level: u16,
    pub font: u16,
    pub maximum_health_points: i32,
    pub health_points: i32,
    pub is_boss: u8,
    pub body: u16,
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Copy, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum SkillType {
    #[numeric_value(0)]
    Passive,
    #[numeric_value(1)]
    Attack,
    #[numeric_value(2)]
    Ground,
    #[numeric_value(4)]
    SelfCast,
    #[numeric_value(16)]
    Support,
    #[numeric_value(32)]
    Trap,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct SkillInformation {
    pub skill_id: SkillId,
    pub skill_type: SkillType,
    pub skill_level: SkillLevel,
    pub spell_point_cost: u16,
    pub attack_range: u16,
    #[length_hint(24)]
    pub skill_name: String,
    pub upgraded: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x010F)]
pub struct UpdateSkillTreePacket {
    #[packet_length]
    pub packet_length: u16,
    #[repeating_remaining]
    pub skill_information: Vec<SkillInformation>,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct HotkeyData {
    pub is_skill: u8,
    pub skill_id: u32,
    pub quantity_or_skill_level: SkillLevel,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B20)]
pub struct UpdateHotkeysPacket {
    pub rotate: u8,
    pub tab: u16,
    pub hotkeys: [HotkeyData; 38],
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02C9)]
pub struct UpdatePartyInvitationStatePacket {
    pub allowed: u8, // always 0 on rAthena
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02DA)]
pub struct UpdateShowEquipPacket {
    pub open_equip_window: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02D9)]
pub struct UpdateConfigurationPacket {
    pub config_type: u32,
    pub value: u32, // only enabled and disabled ?
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x08E2)]
pub struct NavigateToMonsterPacket {
    pub target_type: u8, // 3 - entity; 0 - coordinates; 1 - coordinates but fails if you're alweady on the map
    pub flags: u8,
    pub hide_window: u8,
    #[length_hint(16)]
    pub map_name: String,
    pub target_position: TilePosition,
    pub target_monster_id: u16,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum MarkerType {
    DisplayFor15Seconds,
    DisplayUntilLeave,
    RemoveMark,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0144)]
pub struct MarkMinimapPositionPacket {
    pub npc_id: EntityId,
    pub marker_type: MarkerType,
    pub position: LargeTilePosition,
    pub id: u8,
    pub color: ColorRGBA,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B5)]
pub struct NextButtonPacket {
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B6)]
pub struct CloseButtonPacket {
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B7)]
pub struct DialogMenuPacket {
    pub packet_length: u16,
    pub entity_id: EntityId,
    #[length_hint(self.packet_length - 8)]
    pub message: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01F3)]
pub struct DisplaySpecialEffectPacket {
    pub entity_id: EntityId,
    pub effect_id: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x043D)]
pub struct DisplaySkillCooldownPacket {
    pub skill_id: SkillId,
    pub until: ClientTick,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01DE)]
pub struct DisplaySkillEffectAndDamagePacket {
    pub skill_id: SkillId,
    pub source_entity_id: EntityId,
    pub destination_entity_id: EntityId,
    pub start_time: ClientTick,
    pub soruce_delay: u32,
    pub destination_delay: u32,
    pub damage: u32,
    pub level: SkillLevel,
    pub div: u16,
    pub skill_type: u8,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum HealType {
    #[numeric_value(5)]
    Health,
    #[numeric_value(7)]
    SpellPoints,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0A27)]
pub struct DisplayPlayerHealEffect {
    pub heal_type: HealType,
    pub heal_amount: u32,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09CB)]
pub struct DisplaySkillEffectNoDamagePacket {
    pub skill_id: SkillId,
    pub heal_amount: u32,
    pub destination_entity_id: EntityId,
    pub source_entity_id: EntityId,
    pub result: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0983)]
pub struct StatusChangePacket {
    pub index: u16,
    pub entity_id: EntityId,
    pub state: u8,
    pub duration_in_milliseconds: u32,
    pub remaining_in_milliseconds: u32,
    pub value: [u32; 3],
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ObjectiveDetails1 {
    pub hunt_identification: u32,
    pub objective_type: u32,
    pub mob_id: u32,
    pub minimum_level: u16,
    pub maximum_level: u16,
    pub mob_count: u16,
    #[length_hint(24)]
    pub mob_name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09F9)]
pub struct QuestNotificationPacket1 {
    pub quest_id: u32,
    pub active: u8,
    pub start_time: u32,
    pub expire_time: u32,
    pub objective_count: u16,
    /// For some reason this packet always has space for three objective
    /// details, even if none are sent
    pub objective_details: [ObjectiveDetails1; 3],
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct HuntingObjective {
    pub quest_id: u32,
    pub mob_id: u32,
    pub total_count: u16,
    pub current_count: u16,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x08FE)]
pub struct HuntingQuestNotificationPacket {
    #[packet_length]
    pub packet_length: u16,
    #[repeating_remaining]
    pub objective_details: Vec<HuntingObjective>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09FA)]
pub struct HuntingQuestUpdateObjectivePacket {
    #[packet_length]
    pub packet_length: u16,
    pub objective_count: u16,
    #[repeating_remaining]
    pub objective_details: Vec<HuntingObjective>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02B4)]
pub struct QuestRemovedPacket {
    pub quest_id: u32,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct QuestDetails {
    pub hunt_identification: u32,
    pub objective_type: u32,
    pub mob_id: u32,
    pub minimum_level: u16,
    pub maximum_level: u16,
    pub kill_count: u16,
    pub total_count: u16,
    #[length_hint(24)]
    pub mob_name: String,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct Quest {
    #[packet_length]
    pub quest_id: u32,
    pub active: u8,
    pub remaining_time: u32, // TODO: double check these
    pub expire_time: u32,    // TODO: double check these
    pub objective_count: u16,
    #[repeating(self.objective_count)]
    pub objective_details: Vec<QuestDetails>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09F8)]
pub struct QuestListPacket {
    #[packet_length]
    pub packet_length: u16,
    pub quest_count: u32,
    #[repeating(self.quest_count)]
    pub quests: Vec<Quest>,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum VisualEffect {
    BaseLevelUp,
    JobLevelUp,
    RefineFailure,
    RefineSuccess,
    GameOver,
    PharmacySuccess,
    PharmacyFailure,
    BaseLevelUpSuperNovice,
    JobLevelUpSuperNovice,
    BaseLevelUpTaekwon,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x019B)]
pub struct VisualEffectPacket {
    pub entity_id: EntityId,
    pub effect: VisualEffect,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum ExperienceType {
    #[numeric_value(1)]
    BaseExperience,
    JobExperience,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum ExperienceSource {
    Regular,
    Quest,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0ACC)]
pub struct DisplayGainedExperiencePacket {
    pub account_id: AccountId,
    pub amount: u64,
    pub experience_type: ExperienceType,
    pub experience_source: ExperienceSource,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum ImageLocation {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    MiddleFloating,
    MiddleColorless,
    #[numeric_value(255)]
    ClearAll,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x01B3)]
pub struct DisplayImagePacket {
    #[length_hint(64)]
    pub image_name: String,
    pub location: ImageLocation,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0229)]
pub struct StateChangePacket {
    pub entity_id: EntityId,
    pub body_state: u16,
    pub health_state: u16,
    pub effect_state: u32,
    pub is_pk_mode_on: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B41)]
pub struct ItemPickupPacket {
    pub index: ItemIndex,
    pub count: u16,
    pub item_id: ItemId,
    pub is_identified: u8,
    pub is_broken: u8,
    pub cards: [u32; 4],
    pub equip_position: EquipPosition,
    pub item_type: u8,
    pub result: u8,
    pub hire_expiration_date: u32,
    pub bind_on_equip_type: u16,
    pub option_data: [ItemOptions; 5], // fix count
    pub favorite: u8,
    pub look: u16,
    pub refinement_level: u8,
    pub enchantment_level: u8,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum RemoveItemReason {
    Normal,
    ItemUsedForSkill,
    RefinsFailed,
    MaterialChanged,
    MovedToStorage,
    MovedToCart,
    ItemSold,
    ConsumedByFourSpiritAnalysis,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x07FA)]
pub struct RemoveItemFromInventoryPacket {
    pub remove_reason: RemoveItemReason,
    pub index: u16,
    pub amount: u16,
}

// TODO: improve names
#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum QuestEffect {
    Quest,
    Quest2,
    Job,
    Job2,
    Event,
    Event2,
    ClickMe,
    DailyQuest,
    Event3,
    JobQuest,
    JumpingPoring,
    #[numeric_value(9999)]
    None,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum QuestColor {
    Yellow,
    Orange,
    Green,
    Purple,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0446)]
pub struct QuestEffectPacket {
    pub entity_id: EntityId,
    pub position: TilePosition,
    pub effect: QuestEffect,
    pub color: QuestColor,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B4)]
pub struct NpcDialogPacket {
    pub packet_length: u16,
    pub npc_id: EntityId,
    #[length_hint(self.packet_length - 8)]
    pub text: String,
}

#[derive(Clone, Debug, Default, OutgoingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x007D)]
pub struct MapLoadedPacket {}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0187)]
#[ping]
pub struct CharacterServerKeepalivePacket {
    /// rAthena never reads this value, so just set it to 0.
    #[new(value = "AccountId(0)")]
    pub account_id: AccountId,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0090)]
pub struct StartDialogPacket {
    pub npc_id: EntityId,
    #[new(value = "1")]
    pub dialog_type: u8,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B9)]
pub struct NextDialogPacket {
    pub npc_id: EntityId,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0146)]
pub struct CloseDialogPacket {
    pub npc_id: EntityId,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B8)]
pub struct ChooseDialogOptionPacket {
    pub npc_id: EntityId,
    pub option: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum EquipPosition {
    #[numeric_value(0)]
    None,
    #[numeric_value(1)]
    HeadLower,
    #[numeric_value(512)]
    HeadMiddle,
    #[numeric_value(256)]
    HeadTop,
    #[numeric_value(2)]
    RightHand,
    #[numeric_value(32)]
    LeftHand,
    #[numeric_value(16)]
    Armor,
    #[numeric_value(64)]
    Shoes,
    #[numeric_value(4)]
    Garment,
    #[numeric_value(8)]
    LeftAccessory,
    #[numeric_value(128)]
    RigthAccessory,
    #[numeric_value(1024)]
    CostumeHeadTop,
    #[numeric_value(2048)]
    CostumeHeadMiddle,
    #[numeric_value(4196)]
    CostumeHeadLower,
    #[numeric_value(8192)]
    CostumeGarment,
    #[numeric_value(32768)]
    Ammo,
    #[numeric_value(65536)]
    ShadowArmor,
    #[numeric_value(131072)]
    ShadowWeapon,
    #[numeric_value(262144)]
    ShadowShield,
    #[numeric_value(524288)]
    ShadowShoes,
    #[numeric_value(1048576)]
    ShadowRightAccessory,
    #[numeric_value(2097152)]
    ShadowLeftAccessory,
    #[numeric_value(136)]
    LeftRightAccessory,
    #[numeric_value(34)]
    LeftRightHand,
    #[numeric_value(3145728)]
    ShadowLeftRightAccessory,
}

impl EquipPosition {
    pub fn display_name(&self) -> &'static str {
        match self {
            EquipPosition::None => panic!(),
            EquipPosition::HeadLower => "Head lower",
            EquipPosition::HeadMiddle => "Head middle",
            EquipPosition::HeadTop => "Head top",
            EquipPosition::RightHand => "Right hand",
            EquipPosition::LeftHand => "Left hand",
            EquipPosition::Armor => "Armor",
            EquipPosition::Shoes => "Shoes",
            EquipPosition::Garment => "Garment",
            EquipPosition::LeftAccessory => "Left accessory",
            EquipPosition::RigthAccessory => "Right accessory",
            EquipPosition::CostumeHeadTop => "Costume head top",
            EquipPosition::CostumeHeadMiddle => "Costume head middle",
            EquipPosition::CostumeHeadLower => "Costume head lower",
            EquipPosition::CostumeGarment => "Costume garment",
            EquipPosition::Ammo => "Ammo",
            EquipPosition::ShadowArmor => "Shadow ammo",
            EquipPosition::ShadowWeapon => "Shadow weapon",
            EquipPosition::ShadowShield => "Shadow shield",
            EquipPosition::ShadowShoes => "Shadow shoes",
            EquipPosition::ShadowRightAccessory => "Shadow right accessory",
            EquipPosition::ShadowLeftAccessory => "Shadow left accessory",
            EquipPosition::LeftRightAccessory => "Accessory",
            EquipPosition::LeftRightHand => "Two hand weapon",
            EquipPosition::ShadowLeftRightAccessory => "Shadow accessory",
        }
    }
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0998)]
pub struct RequestEquipItemPacket {
    pub inventory_index: ItemIndex,
    pub equip_position: EquipPosition,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum RequestEquipItemStatus {
    Success,
    Failed,
    FailedDueToLevelRequirement,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0999)]
pub struct RequestEquipItemStatusPacket {
    pub inventory_index: ItemIndex,
    pub equipped_position: EquipPosition,
    pub view_id: u16,
    pub result: RequestEquipItemStatus,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00AB)]
pub struct RequestUnequipItemPacket {
    pub inventory_index: ItemIndex,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum RequestUnequipItemStatus {
    Success,
    Failed,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x099A)]
pub struct RequestUnequipItemStatusPacket {
    pub inventory_index: ItemIndex,
    pub equipped_position: EquipPosition,
    pub result: RequestUnequipItemStatus,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum RestartType {
    Respawn,
    Disconnect,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B2)]
pub struct RestartPacket {
    pub restart_type: RestartType,
}

// TODO: check that this can be only 1 and 0, if not ByteConvertable
// should be implemented manually
#[derive(Clone, Debug, ByteConvertable, PartialEq, Eq)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum RestartResponseStatus {
    Nothing,
    Ok,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x00B3)]
pub struct RestartResponsePacket {
    pub result: RestartResponseStatus,
}

// TODO: check that this can be only 1 and 0, if not Named, ByteConvertable
// should be implemented manually
#[derive(Clone, Debug, ByteConvertable, PartialEq, Eq)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum DisconnectResponseStatus {
    Ok,
    Wait10Seconds,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x018B)]
pub struct DisconnectResponsePacket {
    pub result: DisconnectResponseStatus,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0438)]
pub struct UseSkillAtIdPacket {
    pub skill_level: SkillLevel,
    pub skill_id: SkillId,
    pub target_id: EntityId,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0AF4)]
pub struct UseSkillOnGroundPacket {
    pub skill_level: SkillLevel,
    pub skill_id: SkillId,
    pub target_position: TilePosition,
    #[new(default)]
    pub unused: u8,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B10)]
pub struct StartUseSkillPacket {
    pub skill_id: SkillId,
    pub skill_level: SkillLevel,
    pub target_id: EntityId,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B11)]
pub struct EndUseSkillPacket {
    pub skill_id: SkillId,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x07FB)]
pub struct UseSkillSuccessPacket {
    pub source_entity: EntityId,
    pub destination_entity: EntityId,
    pub position: TilePosition,
    pub skill_id: SkillId,
    pub element: u32,
    pub delay_time: u32,
    pub disposable: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0110)]
pub struct ToUseSkillSuccessPacket {
    pub skill_id: SkillId,
    pub btype: i32,
    pub item_id: ItemId,
    pub flag: u8,
    pub cause: u8,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum UnitId {
    #[numeric_value(0x7E)]
    Safetywall,
    Firewall,
    WarpWaiting,
    WarpActive,
    Benedictio,
    Sanctuary,
    Magnus,
    Pneuma,
    Dummyskill,
    FirepillarWaiting,
    FirepillarActive,
    HiddenTrap,
    Trap,
    HiddenWarpNpc,
    UsedTraps,
    Icewall,
    Quagmire,
    Blastmine,
    Skidtrap,
    Anklesnare,
    Venomdust,
    Landmine,
    Shockwave,
    Sandman,
    Flasher,
    Freezingtrap,
    Claymoretrap,
    Talkiebox,
    Volcano,
    Deluge,
    Violentgale,
    Landprotector,
    Lullaby,
    Richmankim,
    Eternalchaos,
    Drumbattlefield,
    Ringnibelungen,
    Rokisweil,
    Intoabyss,
    Siegfried,
    Dissonance,
    Whistle,
    Assassincross,
    Poembragi,
    Appleidun,
    Uglydance,
    Humming,
    Dontforgetme,
    Fortunekiss,
    Serviceforyou,
    Graffiti,
    Demonstration,
    Callfamily,
    Gospel,
    Basilica,
    Moonlit,
    Fogwall,
    Spiderweb,
    Gravitation,
    Hermode,
    Kaensin,
    Suiton,
    Tatamigaeshi,
    Kaen,
    GrounddriftWind,
    GrounddriftDark,
    GrounddriftPoison,
    GrounddriftWater,
    GrounddriftFire,
    Deathwave,
    Waterattack,
    Windattack,
    Earthquake,
    Evilland,
    DarkRunner,
    DarkTransfer,
    Epiclesis,
    Earthstrain,
    Manhole,
    Dimensiondoor,
    Chaospanic,
    Maelstrom,
    Bloodylust,
    Feintbomb,
    Magentatrap,
    Cobalttrap,
    Maizetrap,
    Verduretrap,
    Firingtrap,
    Iceboundtrap,
    Electricshocker,
    Clusterbomb,
    Reverberation,
    SevereRainstorm,
    Firewalk,
    Electricwalk,
    Netherworld,
    PsychicWave,
    CloudKill,
    Poisonsmoke,
    Neutralbarrier,
    Stealthfield,
    Warmer,
    ThornsTrap,
    Wallofthorn,
    DemonicFire,
    FireExpansionSmokePowder,
    FireExpansionTearGas,
    HellsPlant,
    VacuumExtreme,
    Banding,
    FireMantle,
    WaterBarrier,
    Zephyr,
    PowerOfGaia,
    FireInsignia,
    WaterInsignia,
    WindInsignia,
    EarthInsignia,
    PoisonMist,
    LavaSlide,
    VolcanicAsh,
    ZenkaiWater,
    ZenkaiLand,
    ZenkaiFire,
    ZenkaiWind,
    Makibishi,
    Venomfog,
    Icemine,
    Flamecross,
    Hellburning,
    MagmaEruption,
    KingsGrace,
    GlitteringGreed,
    BTrap,
    FireRain,
    Catnippowder,
    Nyanggrass,
    Creatingstar,
    Dummy0,
    RainOfCrystal,
    MysteryIllusion,
    #[numeric_value(269)]
    StrantumTremor,
    ViolentQuake,
    AllBloom,
    TornadoStorm,
    FloralFlareRoad,
    AstralStrike,
    CrossRain,
    PneumaticusProcella,
    AbyssSquare,
    AcidifiedZoneWater,
    AcidifiedZoneGround,
    AcidifiedZoneWind,
    AcidifiedZoneFire,
    LightningLand,
    VenomSwamp,
    Conflagration,
    CaneOfEvilEye,
    TwinklingGalaxy,
    StarCannon,
    GrenadesDropping,
    #[numeric_value(290)]
    Fuumashouaku,
    MissionBombard,
    TotemOfTutelary,
    HyunRoksBreeze,
    Shinkirou, // mirage
    JackFrostNova,
    GroundGravitation,
    #[numeric_value(298)]
    Kunaiwaikyoku,
    #[numeric_value(20852)]
    Deepblindtrap,
    Solidtrap,
    Swifttrap,
    Flametrap,
    #[numeric_value(0xC1)]
    GdLeadership,
    #[numeric_value(0xC2)]
    GdGlorywounds,
    #[numeric_value(0xC3)]
    GdSoulcold,
    #[numeric_value(0xC4)]
    GdHawkeyes,
    #[numeric_value(0x190)]
    Max,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x09CA)]
pub struct NotifySkillUnitPacket {
    pub lenght: u16,
    pub entity_id: EntityId,
    pub creator_id: EntityId,
    pub position: TilePosition,
    pub unit_id: UnitId,
    pub range: u8,
    pub visible: u8,
    pub skill_level: u8,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0117)]
pub struct NotifyGroundSkillPacket {
    pub skill_id: SkillId,
    pub entity_id: EntityId,
    pub level: SkillLevel,
    pub position: TilePosition,
    pub start_time: ClientTick,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0120)]
pub struct SkillUnitDisappearPacket {
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct Friend {
    pub account_id: AccountId,
    pub character_id: CharacterId,
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0202)]
pub struct AddFriendPacket {
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0203)]
pub struct RemoveFriendPacket {
    pub account_id: AccountId,
    pub character_id: CharacterId,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x020A)]
pub struct NotifyFriendRemovedPacket {
    pub account_id: AccountId,
    pub character_id: CharacterId,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0201)]
pub struct FriendListPacket {
    #[packet_length]
    pub packet_length: u16,
    #[repeating_remaining]
    pub friends: Vec<Friend>,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub enum OnlineState {
    Online,
    Offline,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0206)]
pub struct FriendOnlineStatusPacket {
    pub account_id: AccountId,
    pub character_id: CharacterId,
    pub state: OnlineState,
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0207)]
pub struct FriendRequestPacket {
    pub friend: Friend,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u32)]
pub enum FriendRequestResponse {
    Reject,
    Accept,
}

#[derive(Clone, Debug, OutgoingPacket, new)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0208)]
pub struct FriendRequestResponsePacket {
    pub account_id: AccountId,
    pub character_id: CharacterId,
    pub response: FriendRequestResponse,
}

#[derive(Clone, Debug, PartialEq, Eq, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[numeric_type(u16)]
pub enum FriendRequestResult {
    Accepted,
    Rejected,
    OwnFriendListFull,
    OtherFriendListFull,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0209)]
pub struct FriendRequestResultPacket {
    pub result: FriendRequestResult,
    pub friend: Friend,
}

impl FriendRequestResultPacket {
    pub fn into_message(self) -> String {
        // Messages taken from rAthena
        match self.result {
            FriendRequestResult::Accepted => format!("You have become friends with {}.", self.friend.name),
            FriendRequestResult::Rejected => format!("{} does not want to be friends with you.", self.friend.name),
            FriendRequestResult::OwnFriendListFull => "Your Friend List is full.".to_owned(),
            FriendRequestResult::OtherFriendListFull => format!("{}'s Friend List is full.", self.friend.name),
        }
    }
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x02C6)]
pub struct PartyInvitePacket {
    pub party_id: PartyId,
    #[length_hint(24)]
    pub party_name: String,
}

#[derive(Clone, Debug, ByteConvertable, FixedByteSize)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct ReputationEntry {
    pub reputation_type: u64,
    pub points: i64,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0B8D)]
pub struct ReputationPacket {
    #[packet_length]
    pub packet_length: u16,
    pub success: u8,
    #[repeating_remaining]
    pub entries: Vec<ReputationEntry>,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct Aliance {
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, ByteConvertable)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
pub struct Antagonist {
    #[length_hint(24)]
    pub name: String,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x098A)]
pub struct ClanInfoPacket {
    #[packet_length]
    pub packet_length: u16,
    pub clan_id: u32,
    #[length_hint(24)]
    pub clan_name: String,
    #[length_hint(24)]
    pub clan_master: String,
    #[length_hint(16)]
    pub clan_map: String,
    pub aliance_count: u8,
    pub antagonist_count: u8,
    #[repeating(self.aliance_count)]
    pub aliances: Vec<Aliance>,
    #[repeating(self.antagonist_count)]
    pub antagonists: Vec<Antagonist>,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0988)]
pub struct ClanOnlineCountPacket {
    pub online_members: u16,
    pub maximum_members: u16,
}

#[derive(Clone, Debug, IncomingPacket)]
#[cfg_attr(feature = "interface", derive(korangar_interface::elements::PrototypeElement))]
#[header(0x0192)]
pub struct ChangeMapCellPacket {
    position: TilePosition,
    cell_type: u16,
    #[length_hint(16)]
    map_name: String,
}

/// Helper struct for working with unknown incoming packets.
#[derive(Clone, new)]
pub struct UnknownPacket {
    bytes: Vec<u8>,
}

impl IncomingPacket for UnknownPacket {
    const HEADER: u16 = 0;
    const IS_PING: bool = false;

    fn payload_from_bytes<Meta>(byte_stream: &mut ByteStream<Meta>) -> ConversionResult<Self> {
        let _ = byte_stream;
        unimplemented!()
    }
}

#[cfg(feature = "interface")]
impl<App: korangar_interface::application::Application> korangar_interface::elements::PrototypeElement<App> for UnknownPacket {
    fn to_element(&self, display: String) -> korangar_interface::elements::ElementCell<App> {
        use korangar_interface::elements::{ElementWrap, Expandable};

        let mut byte_stream = ByteStream::<()>::without_metadata(&self.bytes);

        let elements = match self.bytes.len() >= 2 {
            true => {
                let signature = u16::from_bytes(&mut byte_stream).unwrap();
                let header = format!("0x{:0>4x}", signature);
                let data = &self.bytes[byte_stream.get_offset()..];

                vec![header.to_element("header".to_owned()), data.to_element("data".to_owned())]
            }
            false => {
                vec![self.bytes.to_element("data".to_owned())]
            }
        };

        Expandable::new(display, elements, false).wrap()
    }
}
