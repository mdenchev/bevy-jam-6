use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    text::TextColor,
    ui::{AlignItems, JustifyContent, Node, UiRect, Val},
};
use bevy_auto_plugin::auto_plugin::*;

use crate::game::scenes::LevelData;

#[derive(Component, Clone, Copy)]
pub struct TempleHealthUi;

#[derive(Component, Clone, Copy)]
pub struct BallThrowsLeftUI;

#[derive(Component, Clone, Copy)]
pub struct BallTimerUI;

#[derive(Component, Clone, Copy)]
pub struct KillCountUI;

fn update_temple_health(
    level_data: Res<LevelData>,
    temple_text: Single<&mut Text, With<TempleHealthUi>>,
) {
    let mut text = temple_text.into_inner();
    *text = Text::new(format!(" {}", level_data.temple_health));
}

fn update_kill_count(level_data: Res<LevelData>, text: Single<&mut Text, With<KillCountUI>>) {
    let mut text = text.into_inner();
    *text = Text::new(format!("{}", level_data.kill_count));
}

fn update_ball_count(
    level_data: Res<LevelData>,
    count: Single<&mut Text, With<BallThrowsLeftUI>>,
    timer: Single<&mut Text, (With<BallTimerUI>, Without<BallThrowsLeftUI>)>,
) {
    let mut count_text = count.into_inner();
    *count_text = Text::new(format!("{}", level_data.balls_left));

    let mut timer_text = timer.into_inner();
    *timer_text = Text::new(format!(
        "  ({:.2})s",
        level_data.time_to_new_ball.as_secs_f32()
    ));
}

fn spawn_hud_elements(mut commands: Commands, asset_server: Res<AssetServer>) {
    let temple_image = asset_server.load_with_settings(
        "images/temple.png",
        |settings: &mut ImageLoaderSettings| {
            // Need to use nearest filtering to avoid bleeding between the slices with tiling
            settings.sampler = ImageSampler::nearest();
        },
    );
    commands.spawn((
        Node {
            width: Val::Px(600.0),
            height: Val::Px(200.0),
            left: Val::Px(20.0),
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(20.0)),
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexEnd,
            ..default()
        },
        children![
            ImageNode {
                image: temple_image.clone(),
                ..default()
            },
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                TempleHealthUi,
            ),
            // Spacer; todo figure out flex param to get same effect
            (Text::new("        "),),
            // TODO replace with icon
            (
                Text::new("Kills: "),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
            ),
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                KillCountUI,
            ),
            // Spacer; todo figure out flex param to get same effect
            (Text::new("        "),),
            // TODO replace with icon
            (
                Text::new("Balls: "),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
            ),
            (
                Text::new(""),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                BallThrowsLeftUI,
            ),
            (
                Text::new("  "),
                TextColor::WHITE,
                BorderColor(Color::BLACK),
                BallTimerUI,
            ),
        ],
    ));
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_hud_elements);
    app.add_systems(Update, update_temple_health);
    app.add_systems(Update, update_kill_count);
    app.add_systems(Update, update_ball_count);
}
