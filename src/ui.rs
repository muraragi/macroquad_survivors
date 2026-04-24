use bevy_ecs::prelude::*;
use macroquad::{prelude::*, ui::*};

use crate::{GameState, GameStateChange, resources::ScreenSize};

pub fn get_skin(bg_color: &Color) -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(255, 255, 255, 255)) // yellow text
        .font_size(48)
        .build();

    let window_style = root_ui()
        .style_builder()
        .color(*bg_color)
        .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .color(RED)
        .text_color(*bg_color)
        .font_size(64)
        .build();

    Skin {
        label_style,
        window_style,
        button_style,
        ..root_ui().default_skin()
    }
}

pub fn render_menu(
    screen_size: Res<ScreenSize>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    let window_size = Vec2::new(screen_size.width / 3.0, screen_size.height / 3.0);
    let btn_size = Vec2::new(window_size.x - 130.0, 50.0);

    root_ui().window(
        hash!(),
        vec2(
            screen_width() / 2.0 - window_size.x / 2.0,
            screen_height() / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            ui.label(Vec2::new(15.0, 0.0), "MACROQUAD SURVIVORS");
            if widgets::Button::new("PLAY")
                .size(btn_size)
                .position(vec2(65.0, 75.0))
                .ui(ui)
            {
                *game_state = GameState::Running;
                commands.trigger(GameStateChange(*game_state));
            }
            if widgets::Button::new("QUIT")
                .size(btn_size)
                .position(vec2(65.0, 150.0))
                .ui(ui)
            {
                std::process::exit(0);
            }
        },
    );
}
