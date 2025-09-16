use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<MatCap>::default());
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MatCap {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    pub alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for MatCap {
    fn fragment_shader() -> ShaderRef {
        "shaders/matcap.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
