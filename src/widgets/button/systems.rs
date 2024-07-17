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
            .insert(UiTreeBundle::<CustomButtonUi>::from(UiTree::new("Button")))
            .with_children(|ui| {
                let image = ui
                    .spawn((
                        UiLink::<CustomButtonUi>::path("Control/Image"),
                        UiImage2dBundle {
                            texture: button.texture.clone(),
                            sprite: Sprite {
                                color: Color::Srgba(button.color.into()),
                                ..default()
                            },
                            ..default()
                        },
                        Pickable::IGNORE,
                        UiLayout::window_full().pack::<Base>(),
                        ImageScaleMode::Sliced(TextureSlicer {
                            border: BorderRect::rectangle(10.0, 10.0),
                            ..default()
                        }),
                    ))
                    .id();
                let text = ui
                    .spawn((
                        UiLink::<CustomButtonUi>::path("Control/Image/Text"),
                        UiLayout::window()
                            .size(Rl(50.0))
                            .pos(Rl(50.0))
                            .anchor(Anchor::Center)
                            .pack::<Base>(),
                        UiText2dBundle {
                            text: Text::from_section(
                                button.text.to_string(),
                                get_button_text_styles(&assets),
                            ),
                            ..default()
                        },
                        Pickable::IGNORE,
                    ))
                    .id();
                ui.spawn((
                    UiLink::<CustomButtonUi>::path("Control"),
                    UiLayout::window_full().pack::<Base>(),
                    UiZoneBundle::default(),
                    UiAnimator::<Hover>::new()
                        .forward_speed(5.0)
                        .backward_speed(1.0),
                    UiAnimatorPipe::<Hover>::new(vec![text, image]),
                    UiColor::<Base>::new(button.color.into()),
                    UiColor::<Hover>::new(button.color.lighter(0.3).into()),
                    OnHoverSetCursor::new(CursorIcon::Pointer),
                    UiClickEmitter {
                        target: Some(entity),
                    },
                ));
            });
    }
}
