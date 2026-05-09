use bevy_ecs::prelude::*;
use macroquad::{prelude::*, ui::*};

use crate::{
    GameState, consts,
    movement::Speed,
    observers::GameStateChange,
    player::Player,
    resources::ScreenSize,
    score::{Upgrade, UpgradesToChoose},
    stats::{Damage, Health},
    weapon::Weapon,
};

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
        .text_color(WHITE)
        .text_color_hovered(consts::color::BACKGROUND)
        .font_size(52)
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

pub fn render_level_up_ui(
    screen_size: Res<ScreenSize>,
    upgrades_to_choose: Res<UpgradesToChoose>,
    mut player_stats: Query<(&mut Health, &mut Speed), With<Player>>,
    mut weapon: Query<(&mut Weapon, &mut Damage)>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    if let Ok((mut health, mut speed)) = player_stats.single_mut()
        && let Ok((mut weapon, mut damage)) = weapon.single_mut()
    {
        let window_size = Vec2::new(screen_size.width / 2.0, screen_size.height / 2.0);
        let btn_size = Vec2::new(window_size.x - 150.0, 50.0);

        let level_up_window_skin = get_skin(&Color::new(0.0, 0.0, 0.0, 0.0));
        root_ui().push_skin(&level_up_window_skin);

        root_ui().window(
            hash!(),
            vec2(
                screen_size.width / 2.0 - window_size.x / 2.0,
                screen_size.height / 2.0 - window_size.y / 2.0,
            ),
            window_size,
            |ui| {
                ui.label(Vec2::new((window_size.x / 2.0) - 90.0, 0.0), "LEVEL UP!");

                let buttons_x_pos = (window_size.x / 2.0) - (btn_size.x / 2.0);
                for (i, upgrade) in upgrades_to_choose.0.iter().enumerate() {
                    if widgets::Button::new(upgrade.label())
                        .size(btn_size)
                        .position(vec2(buttons_x_pos, 75.0 * (i + 1) as f32))
                        .ui(ui)
                    {
                        match upgrade {
                            Upgrade::Attack(amount) => damage.0 += amount,
                            Upgrade::Speed(amount) => speed.0 += amount,
                            Upgrade::FireRate(amount) => {
                                weapon.attack_timer.interval =
                                    (weapon.attack_timer.interval - amount).clamp(0.0001, 0.40);
                            }
                            Upgrade::Health(amount) => health.0 += amount,
                            Upgrade::Target => weapon.max_targets += 1,
                        }

                        *game_state = GameState::Running;
                        commands.trigger(GameStateChange(*game_state));
                    }
                }
            },
        );

        root_ui().pop_skin();
    }
}
