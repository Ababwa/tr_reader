use std::io::{Read, Result};
use byteorder::{ReadBytesExt, LE};
use tr_readable::{Readable, read_vec};

pub const IMG_DIM: usize = 256;
pub const NUM_PIXELS: usize = IMG_DIM * IMG_DIM;

#[derive(Readable)]
pub struct Vertex<T: Readable> {
	pub x: T,
	pub y: T,
	pub z: T,
}

#[derive(Readable)]
pub struct RoomVertex {
	pub vertex: Vertex<i16>,//relative to Room
	#[skip_2]
	pub flags: u16,
	pub color: u16,
}

#[derive(Readable)]
pub struct RoomFace<const N: usize> {
	pub vertex_ids: [u16; N],//ids into Room.vertices
	pub texture_and_flag: u16,//bits 0-14 id into LevelData.object_textures, bit 15 means double-sided
}

#[derive(Readable)]
pub struct Sprite {
	pub vertex_id: u16,//ids into Room.vertices
	pub texture_id: u16,//ids into LevelData.sprite_textures
}

#[derive(Readable)]
pub struct Portal {
	pub adjoining_room_id: u16,//ids into LevelData.rooms
	pub normal: Vertex<i16>,
	pub vertices: [Vertex<i16>; 4],//relative to Room
}

#[derive(Readable)]
pub struct Sector {
	pub floor_data_id: u16,//ids into LevelData.floor_data
	pub bitfields: u16,
	pub room_below_id: u8,//ids into LevelData.Rooms, 255 = none
	pub floor: i8,
	pub room_above_id: u8,//ids into LevelData.Rooms, 255 = none
	pub ceiling: i8,
}

#[derive(Readable)]
pub struct Light {
	pub pos: Vertex<i32>,//world coords
	pub r: u8,//color
	pub g: u8,
	pub b: u8,
	pub light_type: u8,
	#[skip_1]
	pub intensity: u8,
	pub hotspot: f32,
	pub falloff: f32,
	pub length: f32,
	pub cutoff: f32,
	pub direction: Vertex<f32>,//direction
}

#[derive(Readable)]
pub struct RoomStaticMesh {
	pub pos: Vertex<i32>,//world coords
	pub rotation: u16,
	pub color: u16,
	#[skip_2]
	pub static_mesh_id: u16,//ids into LevelData.static_meshes
}

#[derive(Readable)]
pub struct Room {
	pub x: i32,//world coords
	pub z: i32,
	pub y_bottom: i32,
	pub y_top: i32,
	#[skip_4]
	#[list_u16]
	pub vertices: Vec<RoomVertex>,
	#[list_u16]
	pub quads: Vec<RoomFace<4>>,
	#[list_u16]
	pub triangles: Vec<RoomFace<3>>,
	#[list_u16]
	pub sprites: Vec<Sprite>,
	#[list_u16]
	pub portals: Vec<Portal>,
	#[list_2d]
	pub sectors: Vec<Vec<Sector>>,
	pub color: u32,//argb
	#[list_u16]
	pub lights: Vec<Light>,
	#[list_u16]
	pub room_static_meshes: Vec<RoomStaticMesh>,
	pub flip_room_id: u16,//ids into LevelData.rooms, 65535 = none
	pub flags: u16,
	pub water_effect: u8,
	pub reverb: u8,
	pub flip_group: u8,
}

pub enum MeshComponent {
	Normals(Vec<Vertex<i16>>),
	Lights(Vec<u16>),
}

impl Readable for MeshComponent {
	fn read<R: Read>(reader: &mut R) -> Result<Self> {
		let num = reader.read_i16::<LE>()?;
		Ok(if num > 0 {
			MeshComponent::Normals(read_vec(reader, num as usize)?)
		} else {
			MeshComponent::Lights(read_vec(reader, (-num) as usize)?)
		})
	}
}

#[derive(Readable)]
pub struct MeshFace<const N: usize> {
	pub vertex_ids: [u16; N],//ids into Room.vertices
	pub texture_id: u16,//ids into LevelData.object_textures
	pub light_effects: u16,
}

#[derive(Readable)]
pub struct Mesh {
	pub center: Vertex<i16>,
	pub radius: i32,
	#[list_u16]
	pub vertices: Vec<Vertex<i16>>,
	pub component: MeshComponent,
	#[list_u16]
	pub quads: Vec<MeshFace<4>>,
	#[list_u16]
	pub triangles: Vec<MeshFace<3>>,
}

#[derive(Readable)]
pub struct Anim {
	pub frame_offset: u32,//byte offset into LevelData.frames
	pub frame_duration: u8,//30ths of a second
	pub num_frames: u8,
	pub state: u16,
	pub speed: u32,//fixed-point
	pub accel: u32,//fixed-point
	pub lateral_speed: u32,//fixed-point
	pub lateral_accel: u32,//fixed-point
	pub frame_start: u16,
	pub frame_end: u16,
	pub next_anim: u16,
	pub next_frame: u16,
	pub num_state_changes: u16,
	pub state_change_id: u16,//ids into LevelData.state_changes
	pub num_anim_commands: u16,
	pub anim_command_id: u16,//ids into LevelData.anim_commands
}

#[derive(Readable)]
pub struct StateChange {
	pub state: u16,
	pub num_anim_dispatches: u16,
	pub anim_dispatch_id: u16,//ids into LevelData.anim_dispatches
}

#[derive(Readable)]
pub struct AnimDispatch {
	pub low_frame: u16,
	pub high_frame: u16,
	pub next_anim_id: u16,//ids into LevelData.animations
	pub next_frame_id: u16,//ids into LevelData.frames
}

#[derive(Readable)]
pub struct MeshNode {
	pub flags: u8,
	pub x: i8,//relative to parent
	pub y: i8,
	pub z: i8,
}

#[derive(Readable)]
pub struct Model {
	pub id: u32,
	pub num_meshes: u16,
	pub mesh_id: u16,//ids into LevelData.mesh_data.mesh_pointers
	pub mesh_node_id: u32,//ids into LevelData.mesh_nodes
	pub frame_offset: u32,//byte offset into LevelData.frames
	pub anim_id: u16,//ids into LevelData.animations, 65536 = none
}

#[derive(Readable)]
pub struct BoundBox {
	pub x_min: i16,
	pub x_max: i16,
	pub y_min: i16,
	pub y_max: i16,
	pub z_min: i16,
	pub z_max: i16,
}

#[derive(Readable)]
pub struct StaticMesh {
	pub id: u32,
	pub mesh_id: u16,//ids into LevelData.mesh_data.mesh_pointers
	pub visibility: BoundBox,
	pub collision: BoundBox,
	pub flags: u16,//unused
}

#[derive(Readable)]
pub struct SpriteTexture {
	pub atlas: u16,
	#[skip_2]
	pub width: u16,
	pub height: u16,
	pub left: i16,
	pub top: i16,
	pub right: i16,
	pub bottom: i16,
}

#[derive(Readable)]
pub struct SpriteSequence {
	pub sprite_id: u32,
	pub neg_length: i16,
	pub offset: u16,
}

#[derive(Readable)]
pub struct Camera {
	pub pos: Vertex<i32>,//world coords
	pub room_id: u16,//ids into LevelData.rooms
	pub flags: u16,
}

#[derive(Readable)]
pub struct FlybyCamera {
	pub pos: Vertex<i32>,//world coords
	pub direction: Vertex<i32>,
	pub chain: u8,
	pub index: u8,
	pub fov: u16,
	pub roll: i16,
	pub timer: u16,
	pub speed: u16,
	pub flags: u16,
	pub room_id: u32,//ids into LevelData.rooms
}

#[derive(Readable)]
pub struct SoundSource {
	pub pos: Vertex<i32>,//world coords
	pub sound_id: u16,
	pub flags: u16,
}

#[derive(Readable)]
pub struct TRBox {
	pub z_min: u8,//sectors
	pub z_max: u8,
	pub x_min: u8,
	pub x_max: u8,
	pub y: i16,
	pub overlap: u16,
}

#[derive(Readable)]
pub struct ObjectTextureVertex {
	pub x: u16,//fixed-point
	pub y: u16,
}

#[derive(Readable)]
pub struct ObjectTexture {
	pub blend_mode: u16,
	pub atlas_and_flag: u16,//bits 0-14 id into images, bit 15 indicates triangle face
	pub flags: u16,
	pub vertices: [ObjectTextureVertex; 4],
	#[skip_8]
	pub width: u32,
	pub height: u32,
}

#[derive(Readable)]
pub struct Entity {
	pub model_id: u16,//matched to Model.id
	pub room_id: u16,//ids into LevelData.rooms
	pub pos: Vertex<i32>,//world coords
	pub angle: i16,
	pub light_intensity: u16,//65535 = use mesh light
	pub ocb: u16,
	pub flags: u16,
}

#[derive(Readable)]
pub struct Ai {
	pub model_id: u16,//matched to Model.id
	pub room_id: u16,//ids into LevelData.rooms
	pub pos: Vertex<i32>,//world coords
	pub ocb: u16,
	pub flags: u16,
	pub angle: i32,
}

#[derive(Readable)]
pub struct SoundDetail {
	#[skip_2]
	pub volume: u8,
	pub range: u8,//in sectors
	pub chance: u8,
	pub pitch: u8,
	pub flags: u16,
}

#[derive(Readable)]
pub struct LevelData {
	#[skip_4]
	#[list_u16]
	pub rooms: Vec<Room>,
	#[list_u32]
	pub floor_data: Vec<u16>,
	#[meshes]
	pub meshes: Vec<Mesh>,
	#[list_u32]
	pub mesh_pointers: Vec<u32>,
	#[list_u32]
	pub animations: Vec<Anim>,
	#[list_u32]
	pub state_changes: Vec<StateChange>,
	#[list_u32]
	pub anim_dispatches: Vec<AnimDispatch>,
	#[list_u32]
	pub anim_commands: Vec<u16>,
	#[list_u32]
	pub mesh_nodes: Vec<MeshNode>,
	#[list_u32]
	pub frames: Vec<u16>,
	#[list_u32]
	pub models: Vec<Model>,
	#[list_u32]
	pub static_meshes: Vec<StaticMesh>,
	pub spr: [u8; 3],
	#[list_u32]
	pub sprite_textures: Vec<SpriteTexture>,
	#[list_u32]
	pub sprite_sequences: Vec<SpriteSequence>,
	#[list_u32]
	pub cameras: Vec<Camera>,
	#[list_u32]
	pub flyby_cameras: Vec<FlybyCamera>,
	#[list_u32]
	pub sound_sources: Vec<SoundSource>,
	#[list_u32]
	#[save_len]
	pub boxes: Vec<TRBox>,
	#[list_u32]
	pub overlaps: Vec<u16>,
	#[list_saved_10]
	pub zones: Vec<u16>,
	#[list_u32]
	pub animated_textures: Vec<u16>,
	pub animated_textures_uv_count: u8,
	pub tex: [u8; 3],
	#[list_u32]
	pub object_textures: Vec<ObjectTexture>,
	#[list_u32]
	pub entities: Vec<Entity>,
	#[list_u32]
	pub ais: Vec<Ai>,
	#[list_u16]
	pub demo_data: Vec<u8>,
	pub sound_map: Box<[u16; 370]>,
	#[list_u32]
	pub sound_details: Vec<SoundDetail>,
	#[list_u32]
	pub sample_indices: Vec<u32>,
	pub zero: [u8; 6],
}

#[derive(Readable)]
pub struct Sample {
	pub uncompressed: u32,
	#[list_u32]
	pub data: Vec<u8>,
}

#[derive(Readable)]
pub struct Level {
	pub version: u32,
	#[sum]
	pub num_room_images: u16,
	#[sum]
	pub num_obj_images: u16,
	#[sum]
	pub num_bump_maps: u16,
	#[zlib]
	#[list_sum]
	pub images_32: Vec<Box<[u8; 4 * NUM_PIXELS]>>,
	#[zlib]
	#[list_sum]
	pub images_16: Vec<Box<[u8; 2 * NUM_PIXELS]>>,
	#[zlib]
	pub misc_images: Box<[Box<[u8; 4 * NUM_PIXELS]>; 2]>,
	#[zlib]
	pub level_data: LevelData,
	#[list_u32]
	pub samples: Vec<Sample>,
}
