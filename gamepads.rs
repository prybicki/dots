use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};
use bevy::prelude::*;
use bevy::window::WindowPosition::Centered;
use bevy::window::{ExitCondition, WindowResolution};
use std::time::Duration;

fn main() {
    let window = Window {
        position: Centered(MonitorSelection::Primary),
        resolution: WindowResolution::new(1600.0, 900.0),
        title: "Gamepads".to_string(),
        decorations: false,
        transparent: true,
        focused: true,
        ..default()
    };
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    });
    App::new()
        .add_plugins(plugins)
        .add_systems(Update, gamepad_system)
        .run();
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
    for gamepad in gamepads.iter() {
        let button_pressed = |button| {
            button_inputs.just_pressed(GamepadButton {
                gamepad,
                button_type: button,
            })
        };

        if button_pressed(GamepadButtonType::North) {
            info!(
                "North face button: strong (low-frequency) with low intensity for rumble for 5 seconds. Press multiple times to increase intensity."
            );
            rumble_requests.send(GamepadRumbleRequest::Add {
                gamepad,
                intensity: GamepadRumbleIntensity::strong_motor(0.1),
                duration: Duration::from_secs(5),
            });
        }

        if button_pressed(GamepadButtonType::East) {
            info!("East face button: maximum rumble on both motors for 5 seconds");
            rumble_requests.send(GamepadRumbleRequest::Add {
                gamepad,
                duration: Duration::from_secs(5),
                intensity: GamepadRumbleIntensity::MAX,
            });
        }

        if button_pressed(GamepadButtonType::South) {
            info!("South face button: low-intensity rumble on the weak motor for 0.5 seconds");
            rumble_requests.send(GamepadRumbleRequest::Add {
                gamepad,
                duration: Duration::from_secs_f32(0.5),
                intensity: GamepadRumbleIntensity::weak_motor(0.25),
            });
        }

        if button_pressed(GamepadButtonType::West) {
            info!("West face button: custom rumble intensity for 5 second");
            rumble_requests.send(GamepadRumbleRequest::Add {
                gamepad,
                intensity: GamepadRumbleIntensity {
                    // intensity low-frequency motor, usually on the left-hand side
                    strong_motor: 0.5,
                    // intensity of high-frequency motor, usually on the right-hand side
                    weak_motor: 0.25,
                },
                duration: Duration::from_secs(5),
            });
        }

        if button_pressed(GamepadButtonType::Start) {
            info!("Start button: Interrupt the current rumble");
            rumble_requests.send(GamepadRumbleRequest::Stop { gamepad });
        }
    }
}
