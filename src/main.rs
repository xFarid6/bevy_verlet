// allow dead code
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::FixedTimestep,
    window::PresentMode, input::{keyboard::KeyboardInput, mouse::MouseMotion},
};
use rand::{thread_rng, Rng};

const GRAVITY: f32 = -9.8 * 1000.0;

#[derive(Resource)]
struct BevyCounter {
    pub count: usize,
}

#[derive(Component)]
struct Circle {
    pos: Vec2,
    radius: f32,
    prev_pos: Vec2,
    acceleration: Vec2,
}

fn main() {
    App::new()
        .insert_resource(BevyCounter {
            count: 0,
        })
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
        
        .add_startup_system(setup)

        .add_system(update_circles)
        .add_system(collision_system)
        .add_system(keyboard_system)
        .add_system(mouse_handler)

        .run();
}

fn keyboard_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Circle, &mut Transform)>) {
    for (circle, mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 1.;
        }

        if keyboard_input.pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}

fn mouse_handler(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("Left mouse button pressed");
        let mx = windows.get_primary().unwrap().cursor_position().unwrap().x;
        let my = windows.get_primary().unwrap().cursor_position().unwrap().y;
        println!("Mouse position: {}, {}", mx, my);
        commands.spawn((SpriteBundle {
            texture: asset_server.load("circle.png"),
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        },
        Circle {
            pos: Vec2::new(mx, my),
            radius: 25.,
            prev_pos: Vec2::new(0., 0.),
            acceleration: Vec2::new(0., 0.),
        }));
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        println!("Right mouse button pressed");
    }
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        println!("Middle mouse button pressed");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    let sprite_handle = asset_server.load("circle.png");

    commands.spawn((SpriteBundle {
        texture: sprite_handle.clone(),
        transform: Transform::from_scale(Vec3::splat(0.1)),
        ..default()
    },
    Circle {
        pos: Vec2::new(0., 0.),
        radius: 25.,
        prev_pos: Vec2::new(0., 0.),
        acceleration: Vec2::new(0., 0.),
    }));
}

fn update_circles(mut query: Query<(&mut Circle, &mut Transform)>, time: Res<Time>) {
    for (mut circle, mut transform) in &mut query {
        let dt = time.delta_seconds();

        // gravity
        circle.acceleration.y += GRAVITY * dt;

        let velocity = circle.pos - circle.prev_pos;

        circle.prev_pos = circle.pos;
        circle.pos = circle.pos + velocity + circle.acceleration * dt * dt;
        circle.acceleration = Vec2::new(0., 0.);

        transform.translation.x = circle.pos.x;
        transform.translation.y = circle.pos.y;
    }
}

fn collision_system(windows: Res<Windows>, mut query: Query<(&mut Circle, &mut Transform)>, time: Res<Time>) {
    let window = windows.primary();
    let window_height = window.height() as f32;
    let window_width = window.width() as f32;

    for (mut circle, mut transform) in &mut query {
        // bottom of window
        if transform.translation.y - circle.radius < - window_height / 2. {
            transform.translation.y = - window_height / 2. + circle.radius;
            circle.pos.y = transform.translation.y;
            circle.prev_pos.y = transform.translation.y;
            circle.acceleration.y = 0.;
        }
        // top of window
        if transform.translation.y + circle.radius > window_height / 2. {
            transform.translation.y = window_height / 2. - circle.radius;
            circle.pos.y = transform.translation.y;
            circle.prev_pos.y = transform.translation.y;
            circle.acceleration.y = 0.;
        }
        // left of window
        if transform.translation.x - circle.radius < - window_width / 2. {
            transform.translation.x = - window_width / 2. + circle.radius;
            circle.pos.x = transform.translation.x;
            circle.prev_pos.x = transform.translation.x;
            circle.acceleration.x = 0.;
        }
        // right of window
        if transform.translation.x + circle.radius > window_width / 2. {
            transform.translation.x = window_width / 2. - circle.radius;
            circle.pos.x = transform.translation.x;
            circle.prev_pos.x = transform.translation.x;
            circle.acceleration.x = 0.;
        }
    }

    // collisions with other circles
    let mut circles = query.iter_mut().collect::<Vec<_>>();
    for i in 0..circles.len() {
        let mut object_1 = &mut circles[i];
        let k = i + 1;
        for k in 0..circles.len() {
            let object_2 = &mut circles[k];

            let (mut circle_1, mut transform_1) = &mut object_1;
            let (mut circle_2, mut transform_2) = &mut object_2;

            let collision_axis = circle_1.pos - circle_2.pos;
            let distance = collision_axis.length();
            if distance < circle_1.radius + circle_2.radius {
                let normal = collision_axis.normalize();
                let delta = (circle_1.radius + circle_2.radius) - distance;
                circle_1.pos += normal * delta * 0.5;
            }
        }
    }

}