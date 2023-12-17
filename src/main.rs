use bevy::prelude::*;
use bevy::render::settings::Backends;
use bevy::render::settings::RenderCreation;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;

fn main() {
    let mut app = App::new();
    if cfg!(target_family = "wasm") {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }));
    } else if cfg!(target_family = "windows") {
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
        }));
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            update_ui,
            update_window,
            update_viewport,
            update_mouse,
            update_touch,
        ),
    );

    app.run();
}

#[derive(Component)]
struct WholeScreenNode;
#[derive(Component)]
struct UiDetails;
#[derive(Component)]
struct WindowDetails;
#[derive(Component)]
struct ViewportDetails;
#[derive(Component)]
struct MouseDetails;
#[derive(Component)]
struct TouchDetails;

fn setup(
    mut commands: Commands,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            WholeScreenNode,
        ))
        .with_children(|parent| {
            parent.spawn((text_bundle(), UiDetails));
            parent.spawn((text_bundle(), WindowDetails));
            parent.spawn((text_bundle(), ViewportDetails));
            parent.spawn((text_bundle(), MouseDetails));
            parent.spawn((text_bundle(), TouchDetails));
        });

    commands.spawn(Camera2dBundle::default());
}

fn text_bundle() -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            font_size: 30.,
            ..default()
        },
    )
}

fn update_ui(
    ui_scale: Res<UiScale>,
    whole_screen_node: Query<(&Node, &GlobalTransform), With<WholeScreenNode>>,
    mut ui_text: Query<&mut Text, With<UiDetails>>,
) {
    let (wsn, wsngt) = whole_screen_node.single();
    ui_text.single_mut().sections[0].value = format!(
        "UI scale {}, UI rect {:?}",
        ui_scale.0,
        wsn.logical_rect(wsngt)
    );
}
fn update_window(window: Query<&Window>, mut window_text: Query<&mut Text, With<WindowDetails>>) {
    let w = window.single();
    window_text.single_mut().sections[0].value = format!(
        "Window scale {} logical {} {}, physical {} {}",
        w.scale_factor(),
        w.resolution.width(),
        w.resolution.height(),
        w.resolution.physical_width(),
        w.resolution.physical_height()
    );
}
fn update_viewport(
    camera: Query<&Camera>,
    mut viewport_text: Query<&mut Text, With<ViewportDetails>>,
) {
    let c = camera.single();
    viewport_text.single_mut().sections[0].value = format!(
        "Viewport logical {:?}, physical {:?}",
        c.logical_viewport_rect(),
        c.physical_viewport_rect()
    );
}
fn update_mouse(window: Query<&Window>, mut mouse_text: Query<&mut Text, With<MouseDetails>>) {
    if let Some(cursor) = window.single().cursor_position() {
        mouse_text.single_mut().sections[0].value = format!("Mouse position: {}", cursor);
    };
}
fn update_touch(touches: Res<Touches>, mut touch_text: Query<&mut Text, With<TouchDetails>>) {
    for t in touches.iter() {
        touch_text.single_mut().sections[0].value = format!("Touch {}", t.position());
    }
}
