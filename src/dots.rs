use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::window::{WindowMode, WindowResolution};
use bevy_spatial::kdtree::KDTree2;
use bevy_spatial::*;
use std::collections::HashMap;
use std::time::Duration;

mod input;
mod setup;
mod update;

use setup::*;
use update::*;

/** RESOURCES **/

#[derive(Resource)]
pub struct MeshHandles {
    particle: Mesh2dHandle,
}

#[derive(Resource)]
pub struct CameraConfig {
    pan_speed: f32,
    zoom_speed: f32,
}

/** EVENTS **/

#[derive(Event)]
pub struct EmissionEvent {
    pub position: Vec2,
    pub color: Color,
}

/** COMPONENTS **/

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct Velocity {
    velocity: Vec2,
}

pub struct Charge {
    inner: f32,
    outer: f32,
    color: Color,
}

#[derive(Component)]
pub struct Charges(pub Vec<Charge>);

impl Charges {
    fn rgb(outer: (f32, f32, f32)) -> Self {
        let (r, g, b) = outer;
        Self(vec![
            Charge {
                inner: r,
                outer: 0.0,
                color: Color::RED,
            },
            Charge {
                inner: g,
                outer: 0.0,
                color: Color::GREEN,
            },
            Charge {
                inner: b,
                outer: 0.0,
                color: Color::BLUE,
            },
        ])
    }

    fn to_rgb(&self) -> (f32, f32, f32) {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        for charge in &self.0 {
            r += charge.outer * charge.color.r();
            g += charge.outer * charge.color.g();
            b += charge.outer * charge.color.b();
        }
        (r, g, b)
    }
}

#[derive(Component)]
pub struct Border {
    area: Rect,
}

/** BUNDLES **/

#[derive(Bundle)]
pub struct DotBundle {
    dot: Dot,
    velocity: Velocity,
    charges: Charges,
    cm2d_bundle: ColorMesh2dBundle,
}

type NNTree = KDTree2<Dot>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(3840.0, 2160.0),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            AutomaticUpdate::<Dot>::new()
                .with_frequency(Duration::from_millis(15))
                .with_spatial_ds(SpatialStructure::KDTree2),
        )
        .add_event::<EmissionEvent>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_meshes)
        .add_systems(Startup, setup_environment)
        .add_systems(Update, control_camera)
        .add_systems(Update, control_dots)
        .add_systems(Update, update_velocity)
        .add_systems(Update, update_border)
        .add_systems(Update, update_charge)
        .add_systems(Update, update_colorize)
        .run();
}
