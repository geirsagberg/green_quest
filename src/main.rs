use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelProjection, PixelZoom};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Green Quest".to_string(),
                        resolution: (512., 512.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PixelCameraPlugin)
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .add_systems(Startup, setup)
        .add_systems(Update, place_monstera_on_click)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PixelZoom::Fixed(2)));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("graphics/house.ldtk"),
        transform: Transform::from_translation(Vec3::new(-128., -128., 0.)),
        ..default()
    });
}

fn place_monstera_on_click(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera_transform, camera) = camera_query.single();
        let window = window_query.single();
        let cursor_position = window.cursor_position().unwrap();

        let point = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap();

        commands.spawn(SpriteBundle {
            texture: asset_server.load("graphics/monstera2.png"),
            transform: Transform::from_translation(Vec3::new(point.x - 1., point.y + 12., 10.)),
            ..default()
        });
    }
}
