use std::f32::consts::{PI, }; //TAU

use bevy::{
    diagnostic::LogDiagnosticsPlugin, //{FrameTimeDiagnosticsPlugin, }
    prelude::*, 
    window::WindowResizeConstraints, 
    render::camera::{Projection, }, //RenderTarget
    math::vec2,
};
use bevy_rapier3d::{prelude::*, }; //parry::shape::Cone
use bevy_editor_pls::{
    prelude::*,
    EditorState,
};

use crate::player_controller::*;

pub mod player_controller;

//mod player/player_controller;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Menu,
    Playing,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "determination".to_string(),
            width: 1280.0,
            height: 720.0,
            resize_constraints: WindowResizeConstraints {
                min_width: 256.0,
                min_height: 256.0,
                ..default()
            },
            ..default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.25,
        })
        .insert_resource(RapierConfiguration::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_world)
        .add_system(toggle_mouse)
        .run()
}

fn spawn_world (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        material: materials.add(Color::rgb(0.8, 0.3, 0.5).into()),
        transform: Transform::identity(),
        ..default()
    })
    .insert(Collider::cuboid(500.0, 0., 500.0))
    .insert(RigidBody::Fixed)
    .insert(Transform::identity());

    // directional light to add a little depth
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-38.0, 40.0, 34.0),
        ..default()
    });

    // shapes
    // icosphere 
    let trans = Transform::from_xyz(13.6, 0., 22.3);
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { subdivisions: 10, radius: 6.0 })),
        material: materials.add(Color::rgb(0.8, 0.3, 0.5).into()),
        transform: trans,
        ..default()
    })
    .insert(Collider::ball(6.0,)) //3.0, 50.0
    .insert(RigidBody::Fixed)
    .insert(trans);

    // slopes
    let material = materials.add(Color::WHITE.into());

    let (hw, hh, hl) = (1.5, 0.5, 15.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-17.3,-0.1, 29.1).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.1,
                0.9,
                -0.3,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-24.2, 1.4, 16.3).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.2,
                0.7,
                -0.4,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-23.0, 2.8, 9.2).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.3,
                0.5,
                -0.6,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-19.7, 4.0, 4.4).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.4,
                0.3,
                -0.5,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-13.57, 6.8, -0.35).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.5,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));
    
    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(26.5, 6.8, 0.35).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.5,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(hw, hh, hl)));    

    // walls
    let (hw, hh, hl) = (0.25, 3.0, 16.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh,
            material: materials.add(Color::LIME_GREEN.into()),
            transform: Transform::from_xyz(16.9, 3.0, 35.8).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -std::f32::consts::FRAC_PI_4,
                0.0,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));
    
    let (hw, hh, hl) = (0.25, 3.0, 15.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );
    
    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::GOLD.into()),
                transform: Transform::from_xyz(28.3, 3.0, 7.0),
                ..default()
            })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));
    
    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::VIOLET.into()),
                transform: Transform::from_xyz(17.3, 3.0, 2.4).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    -std::f32::consts::FRAC_PI_4,
                    0.0,
                )),
                ..default()
        })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    let (hw, hh, hl) = (0.25, 3.0, 15.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::GRAY.into()),
                transform: Transform::from_xyz(-36.4, 3.0, 29.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    0.3,
                    0.0
                )),
                ..default()
            })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::BISQUE.into()),
                transform: Transform::from_xyz(-45.3, 3.0, 7.0),
                ..default()
            })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));
    
    let (hw, hh, hl) = (0.25, 3.0, 8.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_xyz(-35.7, 3.0, 51.6).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                0.68,
                0.0,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::ALICE_BLUE.into()),
            transform: Transform::from_xyz(-24.8, 3.0, 52.5).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                0.68,
                0.0,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::OLIVE.into()),
                transform: Transform::from_xyz(-40.3, 5.8, -8.3),
                ..default()
            })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));
    
    
    commands
        .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::MIDNIGHT_BLUE.into()),
                transform: Transform::from_xyz(-45.3, 9.5, -23.6),
                ..default()
            })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    // floating objects
    let (hw, hh, hl) = (5.0, 1.0, 5.0);
    let plat_mesh1 = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );
    let plat_mats = materials.add(Color::INDIGO.into());
    let (x1, y1, z1) = (15.5, 8.0,-9.0);
    let trans = Transform::from_xyz(x1, y1, z1);
    let trans2 = Transform::from_xyz(x1 - 20., y1, z1 - (0.8 * z1));
    commands
        .spawn_bundle(PbrBundle {
                mesh: plat_mesh1.clone(),
                material: plat_mats,
                transform: trans,
                ..default()
        })
        .insert_bundle((Name::from("Platform 1"), Collider::cuboid(hw, hh, hl)));

    let plat_mats = materials.add(Color::CRIMSON.into());
    commands
        .spawn_bundle(PbrBundle {
                mesh: plat_mesh1.clone(),
                material: plat_mats,
                transform: trans2,
                ..default()
        })
        .insert_bundle((Name::from("Platform 2"), Collider::cuboid(hw, hh, hl)));

    let (hw, hh, hl) = (0.25, 3.0, 8.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::AQUAMARINE.into()),
            transform: Transform::from_xyz(x1 - 10.5, y1 + 2.0, z1 - (0.4 * z1)),
            ..default()
        })
        .insert_bundle((Name::from("Wall"), ClimbableWall, Collider::cuboid(hw, hh, hl)));
    
    // large blocks
    let (hw, hh, hl) = (35.0, 30.0, 50.0);
    let building_mesh1 = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    let (x1, y1, z1) = (-55.0, hh.clone(), -127.0);
    let trans_build = Transform::from_xyz(x1, y1, z1);

    commands
        .spawn_bundle(PbrBundle {
            mesh: building_mesh1.clone(),
            material: materials.add(Color::SEA_GREEN.into()),
           transform: trans_build.clone(), 
            ..default()
        })
        .insert_bundle((Name::from("Building 1 Bottom"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    let (hw, hh, hl) = (25.0, 80.0, 35.0);
    let building_mesh2 = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );
    let (x2, y2, z2) = (x1, y1 + hh.clone(), z1);
    let trans_build2 = Transform::from_xyz(x2, y2, z2);

    commands
        .spawn_bundle(PbrBundle {
            mesh: building_mesh2.clone(),
            material: materials.add(Color::AZURE.into()),
           transform: trans_build2.clone(), 
            ..default()
        })
        .insert_bundle((Name::from("Building 1 top"), ClimbableWall, Collider::cuboid(hw, hh, hl)));

    // player. modified from subfuse 
    commands
        .spawn()
        .insert(Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero())
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(GravityScale(0.0))
        .insert(Ccd { enabled: true }) // Prevent clipping when going fast
        .insert(Transform::from_xyz(0.0, 3.0, 0.0))
        .insert(LogicalPlayer(0))
        .insert(PlayerControllerInput {
            pitch: 0.0,
            yaw: 110.0f32.to_radians(),
            ..default()
        })
        .insert(PlayerController {
            wallrun_speed: 16.0,
            key_fly: Some(KeyCode::M), 
            key_up: KeyCode::Space,
            key_down: KeyCode::LControl,
            sensitivity: 0.005,
            ..default() 
        });
    commands
    .spawn_bundle(Camera3dBundle {
        camera: Camera {
            //target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: PI / 3.0,
            ..default()
        }),
        ..default()
    })
    .insert(UiCameraConfig { show_ui: true })
    .insert(RenderPlayer(0))
    .insert(PlayerCamera);    
    // // world camera
    // commands.spawn_bundle(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

// pub fn manage_cursor(
//     mut windows: ResMut<Windows>,
//     //btn: Res<Input<MouseButton>>,
//     key: Res<Input<KeyCode>>,
//     mut controllers: Query<&mut PlayerController>,
// ) {
//     let window = windows.get_primary_mut().unwrap();
//     if key.just_pressed(KeyCode::Escape) {
//         //window.set_cursor_lock_mode(false);
//         //window.set_cursor_visibility(true);
//         for mut controller in &mut controllers {
//             if controller.enable_input == true {
//                 controller.enable_input = false;
//                 window.set_cursor_lock_mode(false);
//                 println!("Player input disabled!");
//                 window.set_cursor_visibility(true);
//                 //break
//             } else
//             if controller.enable_input == false {
//                 controller.enable_input = true;
//                 window.set_cursor_lock_mode(true);
//                 window.set_cursor_visibility(false);
//                 println!("Player input enabled!");
//                 //break
//             }  else {
//                 println!("please enable player input!")
//             }
//         }   
//     }
// }
fn toggle_mouse(
    mut windows: ResMut<Windows>,
    keys: Res<Input<KeyCode>>,
    mut player_controller: Query<&mut PlayerController>,
    btn: Res<Input<MouseButton>>,
    #[cfg(debug_assertions)] editor_state: Res<EditorState>,
) {
    let window = windows.primary_mut();
    let mut player_controller = player_controller.single_mut();
    if keys.just_pressed(KeyCode::Tab) {
        let is_locked = window.cursor_locked();
        if is_locked {
            // Unlock
            player_controller.enable_input = false;
            window.set_cursor_visibility(true);
            window.set_cursor_lock_mode(false);
        } else {
            // Lock
            player_controller.enable_input = true;
            window.set_cursor_visibility(false);
            window.set_cursor_lock_mode(true);
            window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));
        }
    }
    if keys.just_pressed(KeyCode::Escape)
        || (!window.cursor_locked() && player_controller.enable_input)
    {
        // Unlock
        player_controller.enable_input = false;
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }

    #[allow(unused_assignments, unused_mut)]
    let mut editor_active = false;

    #[cfg(debug_assertions)]
    {
        editor_active = editor_state.active;
    }

    if btn.just_pressed(MouseButton::Left)
        && (!player_controller.enable_input || window.cursor_visible() || !window.cursor_locked())
        && !editor_active
    {
        // Lock
        player_controller.enable_input = true;
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
        window.set_cursor_position(vec2(window.width() / 2.0, window.height() / 2.0));
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct ClimbableWall;

#[derive(Component)]
pub struct Wall;

// fn sun_follow_camera(
//     camera: Query<&Transform, (With<PlayerCamera>, Without<Sun>)>,
//     mut sun: Query<&mut Transform, (With<Sun>, Without<PlayerCamera>)>,
// ) {
//     for mut sun in &mut sun {
//         for camera in &camera {
//             sun.translation = camera.translation;
//         }
//     }
// }