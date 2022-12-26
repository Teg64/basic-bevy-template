use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const BACKGROUND_COLOR: Color = Color::DARK_GRAY;

fn main() {
    let mut app = App::new();

    // Window Setup
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: format!("{}", env!("CARGO_PKG_NAME")),
                ..default()
            },
            ..default()
        }))
        // Inspector Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Setup
        .add_startup_system(spawn_camera)
        // Systems and Plugins
        .add_startup_system(sample_system);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    {% if camera-type=="3D" %}commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });{% else %}commands.spawn(Camera2dBundle::default());{% endif %}
}

fn sample_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sample_asset.png"),
        ..default()
    })
    .insert(Name::from("Sample Sprite"));
}
