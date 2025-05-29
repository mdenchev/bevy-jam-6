//! The title screen that appears after the splash screen.

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{menus::Menu, screens::Screen};

const TITLE_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Title),
        |mut clear_color: ResMut<ClearColor>| {
            *clear_color = ClearColor(TITLE_BACKGROUND_COLOR);
        },
    );
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);
}
