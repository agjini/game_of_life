use bevy::asset::Handle;
use bevy::pbr::Material;
use bevy::prelude::Image;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};

#[derive(Clone, Copy, ShaderType, Debug, Hash, Eq, PartialEq)]
pub struct Repeats {
    pub horizontal: u32,
    pub vertical: u32,
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "82d336c5-fd6c-41a3-bdd4-267cd4c9be22"]
pub struct RepeatedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub repeats: Repeats,
}

impl TypePath for RepeatedMaterial {
    fn type_path() -> &'static str {
        todo!()
    }

    fn short_type_path() -> &'static str {
        todo!()
    }
}

impl Material for RepeatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/repeated.wgsl".into()
    }
}
