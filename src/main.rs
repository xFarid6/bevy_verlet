#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::FixedTimestep,
    window::PresentMode, input::{keyboard::KeyboardInput, mouse::MouseMotion}, render::view::window,
};
use rand::{thread_rng, Rng};
use std::rc::Rc;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_hanabi::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: Vec2 = Vec2::new(0.0, -9.8 * 100.0);

#[derive(Component, Clone, Copy)]
struct VerletObject {
    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
}

impl VerletObject {
    fn new() -> Self {
        Self {
            position_current: Vec2::new(0.0, 0.0),
            position_old: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
        }
    }

    fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;

    }

    fn update_position(&mut self, dt: f32) {
        let velocity: Vec2 = self.position_current - self.position_old;
        self.position_old = self.position_current;

        self.position_current += velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::new(0.0, 0.0);
        
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Bevy Simulation".to_string(),
                    width: 1280.,
                    height: 720.,
                    present_mode: PresentMode::AutoVsync,
                    resizable: true,
                    ..default()
                },
                ..default()
            })
        )
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(HanabiPlugin)
        .add_plugin(LdtkPlugin)
        //.add_plugin(RapierPlugin)
        
        .add_startup_system(setup)
        
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_controls)
        .add_system(update_verlet)

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn(Camera2dBundle::default());

    let mut circles = vec![];
    let circle_handle = asset_server.load("circle.png");

    circles.push((
        SpriteBundle {
            texture: circle_handle.clone(), 
            transform: Transform::from_scale(Vec3::splat(0.1)),
            sprite: Sprite::default(),
            ..default()
        },
        VerletObject::new(),
        )
    );

    commands.spawn_batch(circles);

}

fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
) {

}

fn update_verlet(
    time: Res<Time>,
    objects: &mut Query<(&mut Transform, &mut VerletObject)>,
    win: ResMut<Windows>,    
) {
    let sub_steps = 2;
    let dt = time.delta_seconds();
    let sub_dt: f32 = dt / sub_steps as f32;

    for i in (0..sub_steps).rev() {
        apply_gravity(&mut objects);
        apply_constraints(&mut objects, &win);
        solve_collisions(&mut objects);
        update_positions(&mut objects, sub_dt);
    }
}

fn apply_gravity(objects: &mut Query<(&mut Transform, &mut VerletObject)>) {
    for (_, mut obj) in objects.iter_mut() {
        obj.accelerate(GRAVITY);
    }
}

fn apply_constraints(
    objects: &mut Query<(&mut Transform, &mut VerletObject)>,
    win: &ResMut<Windows>
) {
    let screen = win.get_primary().unwrap();
    let bottom = - screen.height() / 2.0;
    let top = screen.height() / 2.0;
    let left = - screen.width() / 2.0;
    let right = screen.width() / 2.0;

    let radius = 26.;
       
    for (_, mut obj) in objects.iter_mut() {
        // Bottom
        if obj.position_current.y < bottom + radius {
            obj.position_current.y = bottom + radius;
        }
        // Top
        if obj.position_current.y > top - radius {
            obj.position_current.y = top - radius;
        }
        // Left
        if obj.position_current.x < left + radius {
            obj.position_current.x = left + radius;
        }
        // Right
        if obj.position_current.x > right - radius {
            obj.position_current.x = right - radius;
        }
    }
}

fn solve_collisions(objects: &mut Query<(&mut Transform, &mut VerletObject)>,) {

}


fn update_positions(objects: &mut Query<(&mut Transform, &mut VerletObject)>, sub_dt: f32) {
    for (mut transform, mut obj) in objects.iter_mut() {
        obj.update_position(sub_dt);
        transform.translation = Vec3::new(obj.position_current.x, obj.position_current.y, 0.0);
    }
}
