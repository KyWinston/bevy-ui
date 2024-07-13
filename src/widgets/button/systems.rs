use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

use super::{
    components::{CustomButton, CustomButtonUi},
    styles::get_button_text_styles,
};

pub fn build_button(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &CustomButton), Added<CustomButton>>,
) {
    for (entity, button) in &query {
        commands
            .entity(entity)
            .insert((UiTreeBundle::<CustomButtonUi>::from(UiTree::new("Button")),))
            .with_children(|ui| {
                ui.spawn((
                    UiLink::<CustomButtonUi>::path("Button"),
                    UiImage2dBundle {
                        texture: button.texture.clone(),
                        sprite: Sprite {
                            color: Color::Srgba(button.color.into()),
                            ..default()
                        },
                        ..default()
                    },
                    UiLayout::window_full()
                        .anchor(Anchor::Center)
                        .pack::<Base>(),
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::rectangle(10.0, 10.0),
                        ..default()
                    }),
                    UiAnimator::<Hover>::new()
                        .forward_speed(5.0)
                        .backward_speed(1.0),
                    UiColor::<Base>::new(button.color.into()),
                    UiColor::<Hover>::new(button.color.lighter(0.3).into()),
                    OnHoverSetCursor::new(CursorIcon::Pointer),
                    UiClickEmitter {
                        target: Some(entity),
                    },
                ));
                ui.spawn((
                    UiLink::<CustomButtonUi>::path("Button/Buttontext"),
                    UiLayout::window_full()
                        .anchor(Anchor::Center)
                        .pos(Rl(50.0))
                        .pack::<Base>(),
                    UiText2dBundle {
                        text: Text::from_section(
                            button.text.to_string(),
                            get_button_text_styles(&assets),
                        ),
                        ..default()
                    },
                ));
            });
    }
}
