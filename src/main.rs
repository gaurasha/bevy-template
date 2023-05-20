use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::ExitCondition;
///! This example illustrates how to resize windows, and how to respond to a window being resized.
use bevy::{prelude::*, window::WindowResized};
use bevy_text::{Text, TextAlignment, TextStyle};

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                title: "bevy-template".to_string(),
                // canvas: Some("#bevy".to_string()),
                ..default()
            }),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui)
        .add_system(on_resize_system)
        // .add_system(toggle_resolution)
        // .add_system(update_marker.after(on_resize_system))
        .run();
}

/// Marker component for the text that displays the current reslution.
#[derive(Component)]
struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Spawns the UI
fn setup_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..default()
    })
    .with_children(|parent| {
        // Text where we display current resolution
        parent
            .spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
                ..Default::default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Resolution",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,

                        ..default()
                    }),
                    ResolutionText,
                ));
            });
    });
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
        text.sections[0].value = format!("bevy-template");
        text.alignment = TextAlignment::Center;
        // text.sections[0].
    }
}
