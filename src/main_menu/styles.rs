use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub fn draw_main_menu_image(
    asset_server: &AssetServer,
    image_path: String
) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        },
        image: asset_server.load(image_path).into(),
        ..default()
    }
}

pub fn draw_text(
    asset_server: &AssetServer,
    button_text: String,
    font_size: f32
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    button_text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size,
                        color: Color::WHITE
                    }
                )
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}