use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<MatCap>::default())
        .add_plugins(MaterialPlugin::<FlatColorMat>::default());
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

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlatColorMat {
    #[uniform(0)]
    pub color: LinearRgba,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for FlatColorMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/flat_color.wgsl".into()
    }
}

#[derive(Debug, Clone)]
pub enum MeshMaterial {
    Standard(Handle<StandardMaterial>),
    MatCap(Handle<MatCap>),
    FlatColor(Handle<FlatColorMat>),
}

#[derive(Debug, Component)]
#[component(on_add = on_add)]
pub struct ChangeMaterial(MeshMaterial);

impl ChangeMaterial {
    pub fn standard(handle: Handle<StandardMaterial>) -> Self {
        Self(MeshMaterial::Standard(handle))
    }

    pub fn mat_cap(handle: Handle<MatCap>) -> Self {
        Self(MeshMaterial::MatCap(handle))
    }

    pub fn flat_color(handle: Handle<FlatColorMat>) -> Self {
        Self(MeshMaterial::FlatColor(handle))
    }
}

fn on_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    // Remove old materials
    world.commands().entity(entity).remove::<(
        MeshMaterial3d<StandardMaterial>,
        MeshMaterial3d<MatCap>,
        MeshMaterial3d<FlatColorMat>,
    )>();

    // Add new material
    match world.get::<ChangeMaterial>(entity).unwrap().0.clone() {
        MeshMaterial::Standard(handle) => {
            world
                .commands()
                .entity(entity)
                .insert(MeshMaterial3d(handle));
        }
        MeshMaterial::MatCap(handle) => {
            world
                .commands()
                .entity(entity)
                .insert(MeshMaterial3d(handle));
        }
        MeshMaterial::FlatColor(handle) => {
            world
                .commands()
                .entity(entity)
                .insert(MeshMaterial3d(handle));
        }
    }

    // Remove component
    world.commands().entity(entity).remove::<ChangeMaterial>();
}
