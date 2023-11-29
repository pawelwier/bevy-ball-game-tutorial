use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands
        .spawn(
    (NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            MainMenu {}
        )
    )
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(900.0),
                height: Val::Px(120.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(draw_main_menu_image(asset_server, "sprites/ball_blue_large.png".to_string()));
            parent.spawn(draw_text(asset_server, "Bevy Ball Game".to_string(), 64.0));
            parent.spawn(draw_main_menu_image(asset_server, "sprites/ball_red_large.png".to_string()));

        });
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..Default::default()
                },
                PlayButton {}
            )
        ).with_children(|parent| {
            parent.spawn(draw_text(asset_server, "Play".to_string(), 32.0));
        });
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..Default::default()
                },
                QuitButton {}
            )
        ).with_children(|parent| {
            parent.spawn(draw_text(asset_server, "Quit".to_string(), 32.0));
        });
    })
    .id();

    main_menu_entity
}