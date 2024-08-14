use crate::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Drag, DragEnd, Pointer},
    prelude::{Listener, On},
    PickableBundle,
};
use palette::FromColor;
use vello::{
    kurbo::{self, RoundedRectRadii},
    peniko,
};

#[derive(Component, Reflect, Clone, Copy, PartialEq, Default)]
struct ColorPickerState {
    is_dragging: bool,
    current_color: Hsva,
}

/// Color picker changed event
#[derive(Component, Debug, Reflect, Clone, Copy, PartialEq, Default)]
pub struct ColorPickerChanged {
    /// Color picked
    pub color: Color,
}

/// Color picker widget
#[derive(Widget, Component, Reflect, Clone, Copy, PartialEq, Default)]
#[auto_update(render)]
#[props(ColorPicker)]
#[state(ColorPickerState)]
pub struct ColorPicker {
    /// Initial color to use
    pub initial_color: Color,
}

/// The color picker bundle
#[derive(Bundle, Clone)]
pub struct ColorPickerBundle {
    /// Color picker
    pub color_picker: ColorPicker,
    /// Internal styles
    pub internal_styles: WoodpeckerStyle,
    /// Internal children
    pub internal_children: WidgetChildren,
    /// Internal render
    pub internal_render: WidgetRender,
}

impl Default for ColorPickerBundle {
    fn default() -> Self {
        Self {
            color_picker: Default::default(),
            internal_styles: Default::default(),
            internal_children: Default::default(),
            internal_render: WidgetRender::Quad,
        }
    }
}

fn render(
    mut commands: Commands,
    current_widget: Res<CurrentWidget>,
    mut hooks: ResMut<HookHelper>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ColorPicker, &mut WoodpeckerStyle, &mut WidgetChildren)>,
    state_query: Query<&ColorPickerState>,
) {
    let Ok((picker, mut styles, mut children)) = query.get_mut(**current_widget) else {
        return;
    };

    let default_state = ColorPickerState {
        is_dragging: false,
        current_color: picker.initial_color.into(),
    };
    let state_entity = hooks.use_state(&mut commands, *current_widget, default_state);

    let state = state_query.get(state_entity).unwrap_or(&default_state);

    *styles = WoodpeckerStyle {
        background_color: colors::BACKGROUND,
        border_radius: Corner::all(20.0),
        width: 320.0.into(),
        ..Default::default()
    };

    let srgba_color: Color = state.current_color.into();
    let srgba_color = Color::Srgba(srgba_color.into());

    // So we can pass it in.
    let widget_entity = **current_widget;
    *children = WidgetChildren::default().with_child::<Clip>(ClipBundle {
        styles: WoodpeckerStyle {
            border_radius: Corner::all(20.0),
            width: Units::Percentage(100.0),
            height: Units::Percentage(100.0),
            flex_direction: WidgetFlexDirection::Column,
            ..Default::default()
        },
        // Main color
        children: WidgetChildren::default().with_child::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    background_color: srgba_color,
                    width: Units::Percentage(100.0),
                    height: 140.0.into(),
                    margin: Edge::all(0.0).bottom(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            WidgetRender::Quad,
        ))
        // Color hex value
        .with_child::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    align_items: Some(WidgetAlignItems::Center),
                    margin: Edge::all(0.0).bottom(20.0).left(20.0).right(20.0),
                    ..Default::default()
                },
                children: WidgetChildren::default().with_child::<Element>((
                    ElementBundle {
                        styles: WoodpeckerStyle {
                            font_size: 22.0,
                            color: Color::WHITE,
                            flex_grow: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    WidgetRender::Text {
                        content: srgba_color.to_srgba().to_hex(),
                        word_wrap: false,
                    }
                ))
                .with_child::<IconButton>((
                    IconButtonBundle {
                        button_styles: IconButtonStyles {
                            normal: WoodpeckerStyle {
                                background_color: Color::WHITE,
                                ..Default::default()
                            },
                            hovered: WoodpeckerStyle {
                                background_color: Color::WHITE.with_alpha(0.8),
                                ..Default::default()
                            },
                            width: 32.0.into(),
                            height: 32.0.into(),
                        },
                        render: WidgetRender::Svg {
                            handle: asset_server.load("embedded://woodpecker_ui/embedded_assets/icons/copy-outline.svg"),
                            color: None, // Set by IconButton
                        },
                        ..Default::default()
                    },
                    On::<Pointer<Click>>::run(move |state_query: Query<&ColorPickerState>| {
                        let Ok(state) = state_query.get(state_entity) else {
                            return;
                        };

                        let color: Color = state.current_color.into();
                        let hex = color.to_srgba().to_hex();

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let Ok(mut clipboard) = arboard::Clipboard::new() else {
                                warn!("no clipboard");
                                return;
                            };
                            let _ = clipboard.set_text(hex);
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let Some(clipboard) = web_sys::window().and_then(|window| window.navigator().clipboard()) else {
                                warn!("no clipboard");
                                return;
                            };
                            let _ = clipboard.write_text(&hex);
                        }
                    }),
                )),
                ..Default::default()
            },
        ))
        // Hue
        .with_child::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    margin: Edge::all(0.0).left(20.0).right(20.0),
                    width: 280.0.into(),
                    height: 32.0.into(),
                    ..Default::default()
                },
                children: WidgetChildren::default().with_child::<Element>((
                    ElementBundle {
                        styles: WoodpeckerStyle {
                            position: WidgetPosition::Absolute,
                            top: 7.0.into(),
                            left: (9.0 + ((state.current_color.hue / 365.0) * 245.0)).into(),
                            background_color: srgba_color,
                            width: 18.0.into(),
                            height: 18.0.into(),
                            border_radius: Corner::all(100.0),
                            border_color: Color::WHITE,
                            border: Edge::all(4.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    WidgetRender::Quad,
                )),
                ..Default::default()
            },
            get_hue_gradient(state.current_color),
            PickableBundle::default(),
            On::<Pointer<Drag>>::run(move |event: Listener<Pointer<Drag>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = true;

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let value = (event.pointer_location.position.x - layout.location.x)
                / layout.size.x;
                state.current_color.hue = value.clamp(0.0, 1.0) * 365.0;

                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
            On::<Pointer<DragEnd>>::run(move |mut query: Query<&mut ColorPickerState>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = false;
            }),
            On::<Pointer<Click>>::run(move |event: Listener<Pointer<Click>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                if state.is_dragging {
                    return;
                }

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let relative_x = event.pointer_location.position.x - (layout.location.x + 40.0);
                let value = relative_x / (layout.size.x - 80.0);
                state.current_color.hue = value.clamp(0.0, 1.0) * 365.0;
                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
        ))
        // Saturation
        .with_child::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    margin: Edge::all(0.0).left(20.0).right(20.0).top(20.0),
                    width: 280.0.into(),
                    height: 32.0.into(),
                    ..Default::default()
                },
                children: WidgetChildren::default().with_child::<Element>((
                    ElementBundle {
                        styles: WoodpeckerStyle {
                            position: WidgetPosition::Absolute,
                            top: 7.0.into(),
                            left: (9.0 + (state.current_color.saturation * 245.0)).into(),
                            background_color: srgba_color,
                            width: 18.0.into(),
                            height: 18.0.into(),
                            border_radius: Corner::all(100.0),
                            border_color: Color::WHITE,
                            border: Edge::all(4.0),
                            ..Default::default()
                        },
                        children: WidgetChildren::default().with_child::<Element>((
                            ElementBundle {
                                styles: WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    left: (-3.0).into(),
                                    top: (-3.0).into(),
                                    background_color: srgba_color,
                                    width: 16.0.into(),
                                    height: 16.0.into(),
                                    border_radius: Corner::all(100.0),
                                    border_color: colors::BACKGROUND_LIGHT,
                                    border: Edge::all(3.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            WidgetRender::Quad,
                        )),
                        ..Default::default()
                    },
                    WidgetRender::Quad,
                )),
                ..Default::default()
            },
            get_saturation_gradient(state.current_color),
            PickableBundle::default(),
            On::<Pointer<Drag>>::run(move |event: Listener<Pointer<Drag>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = true;

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let value = (event.pointer_location.position.x - layout.location.x)
                / layout.size.x;
                state.current_color.saturation = value.clamp(0.0, 1.0);
                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
            On::<Pointer<DragEnd>>::run(move |mut query: Query<&mut ColorPickerState>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = false;
            }),
            On::<Pointer<Click>>::run(move |event: Listener<Pointer<Click>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                if state.is_dragging {
                    return;
                }

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let relative_x = event.pointer_location.position.x - (layout.location.x + 40.0);
                let value = relative_x / (layout.size.x - 80.0);
                state.current_color.saturation = value.clamp(0.0, 1.0);
                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
        ))
        // Value
        .with_child::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    margin: Edge::all(0.0).left(20.0).right(20.0).top(20.0).bottom(20.0),
                    width: 280.0.into(),
                    height: 32.0.into(),
                    ..Default::default()
                },
                children: WidgetChildren::default().with_child::<Element>((
                    ElementBundle {
                        styles: WoodpeckerStyle {
                            position: WidgetPosition::Absolute,
                            top: 7.0.into(),
                            left: (9.0 + (state.current_color.value * 245.0)).into(),
                            background_color: srgba_color,
                            width: 18.0.into(),
                            height: 18.0.into(),
                            border_radius: Corner::all(100.0),
                            border_color: Color::WHITE,
                            border: Edge::all(4.0),
                            ..Default::default()
                        },
                        children: WidgetChildren::default().with_child::<Element>((
                            ElementBundle {
                                styles: WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    left: (-3.0).into(),
                                    top: (-3.0).into(),
                                    background_color: srgba_color,
                                    width: 16.0.into(),
                                    height: 16.0.into(),
                                    border_radius: Corner::all(100.0),
                                    border_color: colors::BACKGROUND_LIGHT,
                                    border: Edge::all(3.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            WidgetRender::Quad,
                        )),
                        ..Default::default()
                    },
                    WidgetRender::Quad,
                )),
                ..Default::default()
            },
            get_value_gradient(state.current_color),
            PickableBundle::default(),
            On::<Pointer<Drag>>::run(move |event: Listener<Pointer<Drag>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = true;

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let value = (event.pointer_location.position.x - layout.location.x)
                / layout.size.x;
                state.current_color.value = value.clamp(0.0, 1.0);
                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
            On::<Pointer<DragEnd>>::run(move |mut query: Query<&mut ColorPickerState>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                state.is_dragging = false;
            }),
            On::<Pointer<Click>>::run(move |event: Listener<Pointer<Click>>, mut query: Query<&mut ColorPickerState>, layout_query: Query<&WidgetLayout>, mut event_writer: EventWriter<Change<ColorPickerChanged>>| {
                let Ok(mut state) = query.get_mut(state_entity) else {
                    return;
                };

                if state.is_dragging {
                    return;
                }

                let Ok(layout) = layout_query.get(widget_entity) else {
                    return;
                };

                let relative_x = event.pointer_location.position.x - (layout.location.x + 40.0);
                let value = relative_x / (layout.size.x - 80.0);
                state.current_color.value = value.clamp(0.0, 1.0);
                let color: Color = state.current_color.into();
                event_writer.send(Change {
                    target: widget_entity,
                    data: ColorPickerChanged {
                        color: color.to_srgba().into(),
                    },
                });
            }),
        )),
        ..Default::default()
    });

    children.apply(current_widget.as_parent());
}

/// Renders a color picker color gradient.
fn get_hue_gradient(color: Hsva) -> WidgetRender {
    WidgetRender::Custom {
        render: WidgetRenderCustom::new(move |vello_scene, layout, _styles| {
            let location_x = layout.location.x as f64;
            let location_y = layout.location.y as f64;
            let size_x = layout.size.x as f64;
            let size_y = layout.size.y as f64;

            let rect = kurbo::RoundedRect::new(
                location_x - layout.border.left.value_or(0.0) as f64,
                location_y - layout.border.top.value_or(0.0) as f64,
                location_x + (size_x + layout.border.right.value_or(0.0) as f64),
                location_y + (size_y + layout.border.bottom.value_or(0.0) as f64),
                RoundedRectRadii::new(50.0, 50.0, 50.0, 50.0),
            );

            let mut lch_color =
                palette::Hsva::new(0.0, color.saturation as f64, color.value as f64, 1.0);

            let mut stops = vec![];
            let step = 360.0 / size_x;
            for _ in 0..size_x as u32 {
                lch_color.hue += step;
                let srgba = palette::Srgba::from_color(lch_color);
                stops.push(peniko::Color::rgba(
                    srgba.red,
                    srgba.green,
                    srgba.blue,
                    srgba.alpha,
                ));
            }

            let brush = peniko::Gradient::new_linear((location_x, 0.0), (location_x + size_x, 0.0))
                .with_stops(&*stops);

            vello_scene.fill(
                peniko::Fill::NonZero,
                kurbo::Affine::default(),
                &brush,
                None,
                &rect,
            );

            vello_scene.push_layer(peniko::Mix::Multiply, 0.75, kurbo::Affine::default(), &rect);

            let mut color = peniko::Color::parse("#818181").unwrap();
            color = color.with_alpha_factor(1.0);
            vello_scene.stroke(
                &kurbo::Stroke::new(5.0),
                kurbo::Affine::default(),
                color,
                None,
                &rect,
            );

            vello_scene.pop_layer();
        }),
    }
}

/// Renders a color picker color gradient.
fn get_saturation_gradient(color: Hsva) -> WidgetRender {
    WidgetRender::Custom {
        render: WidgetRenderCustom::new(move |vello_scene, layout, _styles| {
            let location_x = layout.location.x as f64;
            let location_y = layout.location.y as f64;
            let size_x = layout.size.x as f64;
            let size_y = layout.size.y as f64;

            let rect = kurbo::RoundedRect::new(
                location_x - layout.border.left.value_or(0.0) as f64,
                location_y - layout.border.top.value_or(0.0) as f64,
                location_x + (size_x + layout.border.right.value_or(0.0) as f64),
                location_y + (size_y + layout.border.bottom.value_or(0.0) as f64),
                RoundedRectRadii::new(50.0, 50.0, 50.0, 50.0),
            );

            let mut color = palette::Hsva::new(color.hue as f64, 0.0, color.value as f64, 1.0);

            let mut stops = vec![];
            let step = 1.0 / size_x;
            for _ in 0..size_x as u32 {
                color.saturation += step;
                let srgba: palette::Alpha<palette::rgb::Rgb<palette::encoding::Srgb, f64>, f64> =
                    palette::Srgba::from_color(color);
                stops.push(peniko::Color::rgba(
                    srgba.red,
                    srgba.green,
                    srgba.blue,
                    srgba.alpha,
                ));
            }

            let brush = peniko::Gradient::new_linear((location_x, 0.0), (location_x + size_x, 0.0))
                .with_stops(&*stops);

            vello_scene.fill(
                peniko::Fill::NonZero,
                kurbo::Affine::default(),
                &brush,
                None,
                &rect,
            );

            vello_scene.push_layer(peniko::Mix::Multiply, 0.75, kurbo::Affine::default(), &rect);

            let mut color = peniko::Color::parse("#818181").unwrap();
            color = color.with_alpha_factor(1.0);
            vello_scene.stroke(
                &kurbo::Stroke::new(5.0),
                kurbo::Affine::default(),
                color,
                None,
                &rect,
            );

            vello_scene.pop_layer();
        }),
    }
}

/// Renders a color picker color gradient.
fn get_value_gradient(color: Hsva) -> WidgetRender {
    WidgetRender::Custom {
        render: WidgetRenderCustom::new(move |vello_scene, layout, _styles| {
            let location_x = layout.location.x as f64;
            let location_y = layout.location.y as f64;
            let size_x = layout.size.x as f64;
            let size_y = layout.size.y as f64;

            let rect = kurbo::RoundedRect::new(
                location_x - layout.border.left.value_or(0.0) as f64,
                location_y - layout.border.top.value_or(0.0) as f64,
                location_x + (size_x + layout.border.right.value_or(0.0) as f64),
                location_y + (size_y + layout.border.bottom.value_or(0.0) as f64),
                RoundedRectRadii::new(50.0, 50.0, 50.0, 50.0),
            );
            let mut color = palette::Hsva::new(color.hue as f64, color.saturation as f64, 0.0, 1.0);

            let mut stops = vec![];
            let step = 1.0 / size_x;
            for _ in 0..size_x as u32 {
                color.value += step;
                let srgba: palette::Alpha<palette::rgb::Rgb<palette::encoding::Srgb, f64>, f64> =
                    palette::Srgba::from_color(color);
                stops.push(peniko::Color::rgba(
                    srgba.red,
                    srgba.green,
                    srgba.blue,
                    srgba.alpha,
                ));
            }

            let brush = peniko::Gradient::new_linear((location_x, 0.0), (location_x + size_x, 0.0))
                .with_stops(&*stops);

            vello_scene.fill(
                peniko::Fill::NonZero,
                kurbo::Affine::default(),
                &brush,
                None,
                &rect,
            );

            vello_scene.push_layer(peniko::Mix::Multiply, 0.75, kurbo::Affine::default(), &rect);

            let mut color = peniko::Color::parse("#818181").unwrap();
            color = color.with_alpha_factor(1.0);
            vello_scene.stroke(
                &kurbo::Stroke::new(5.0),
                kurbo::Affine::default(),
                color,
                None,
                &rect,
            );

            vello_scene.pop_layer();
        }),
    }
}
