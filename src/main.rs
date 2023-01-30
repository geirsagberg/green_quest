use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelProjection};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 512.,
                        height: 512.,
                        resizable: false,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(PixelCameraPlugin)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup)
        .add_system(place_monstera_on_click)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = PixelCameraBundle::new(PixelProjection {
        zoom: 2,
        centered: false,
        ..Default::default()
    });
    commands.spawn(camera);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("graphics/house.ldtk"),
        ..default()
    });
}

fn place_monstera_on_click(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&Transform, &Camera)>,
    windows: Res<Windows>,
) {
    let (camera_transform, camera) = camera_query.single();

    let window = windows.get_primary().unwrap();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let position = window.cursor_position().unwrap();

        commands.spawn(SpriteBundle {
            texture: asset_server.load("graphics/monstera2.png"),
            transform: Transform::from_translation(Vec3::new(
                position.x / 2. - 1.,
                position.y / 2. + 12.,
                3.,
            )),
            ..default()
        });
    }
}
