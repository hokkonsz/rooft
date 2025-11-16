use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::color;

pub fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(color::BLACK18))
        .init_state::<CameraView>()
        .init_state::<CameraLock>()
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, zoom, view_transition))
        // ..
        ;
}

fn setup(mut commands: Commands) {
    let translation = Vec3::new(5000., 20000.0, 5000.0);

    commands.spawn((
        Name::from("Camera"),
        Camera3d::default(),
        Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(CameraSettings {
        orbit_distance: translation.length(),
        ..Default::default()
    });
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance_min: f32,
    pub orbit_distance_max: f32,
    pub orbit_distance: f32,
    pub zoom_step: f32,
    pub pitch_limit: f32,
    pub pitch_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            orbit_distance_min: 0.,
            orbit_distance_max: 30000.,
            orbit_distance: 5000.,
            zoom_step: 500.,
            pitch_limit: FRAC_PI_2 - 0.01,
            pitch_speed: 4.,
            yaw_speed: 5.,
        }
    }
}

fn rotate(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_view_curr: Res<State<CameraView>>,
    mut camera_view_next: ResMut<NextState<CameraView>>,
    camera_lock: Res<State<CameraLock>>,
    settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    if *camera_lock == CameraLock::Locked {
        return;
    }

    if !mouse_buttons.pressed(MouseButton::Middle) {
        return;
    }

    if **camera_view_curr != CameraView::Free {
        println!("test");
        camera_view_next.set(CameraView::Free);
    }

    let dt = time.delta_secs();

    let delta_pitch = -mouse_motion.delta.y * settings.pitch_speed * dt;
    let delta_yaw = -mouse_motion.delta.x * settings.yaw_speed * dt;

    let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta_pitch).clamp(-settings.pitch_limit, settings.pitch_limit);
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);

    camera.translation = Vec3::ZERO - camera.forward() * settings.orbit_distance;
}

fn zoom(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut settings: ResMut<CameraSettings>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
) {
    if mouse_wheel.delta.y.abs() == 0.0 {
        return;
    }

    settings.orbit_distance = (settings.orbit_distance + mouse_wheel.delta.y * -settings.zoom_step)
        .clamp(settings.orbit_distance_min, settings.orbit_distance_max);

    camera.translation = Vec3::ZERO - camera.forward() * settings.orbit_distance;
}

fn view_transition(
    mut state_events: EventReader<StateTransitionEvent<CameraView>>,
    mut camera: Single<&mut Transform, With<Camera>>,
    settings: Res<CameraSettings>,
) {
    let Some(transition) = state_events.read().next() else {
        return;
    };

    let Some(new_view) = &transition.entered else {
        return;
    };

    match new_view {
        CameraView::Free => return,
        CameraView::Top => camera.rotation = Quat::from_xyzw(-0.70356244, 0., 0., 0.71063346),
        CameraView::Left => camera.rotation = Quat::from_xyzw(0., -0.70710677, 0., 0.70710677),
        CameraView::Right => camera.rotation = Quat::from_xyzw(0., 0.70710677, 0., 0.70710677),
        CameraView::Front => camera.rotation = Quat::from_xyzw(0., 0., 0., 1.),
        CameraView::Back => camera.rotation = Quat::from_xyzw(0., 1., 0., 0.),
    }

    camera.translation = Vec3::ZERO - camera.forward() * settings.orbit_distance;
}

#[derive(Component, Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum CameraLock {
    #[default]
    Unlocked,
    Locked,
}

#[derive(Component, Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum CameraView {
    #[default]
    Free,
    Top,
    Left,
    Right,
    Front,
    Back,
}

impl CameraView {
    pub const LEN: usize = 6;
    pub const LIST: [CameraView; 6] = [
        CameraView::Free,
        CameraView::Top,
        CameraView::Left,
        CameraView::Right,
        CameraView::Front,
        CameraView::Back,
    ];
}

impl ToString for CameraView {
    fn to_string(&self) -> String {
        match self {
            CameraView::Free => String::from("Free"),
            CameraView::Top => String::from("Top"),
            CameraView::Left => String::from("Left"),
            CameraView::Right => String::from("Right"),
            CameraView::Front => String::from("Front"),
            CameraView::Back => String::from("Back"),
        }
    }
}
