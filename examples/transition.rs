use bevy::prelude::*;
use woodpecker_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WoodpeckerUIPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, mut ui_context: ResMut<WoodpeckerContext>) {
    commands.spawn((Camera2d, WoodpeckerView));

    // Some default styles for our transition examples
    let quad_styles = WoodpeckerStyle {
        position: WidgetPosition::Absolute,
        background_color: Srgba::new(1.0, 0.0, 0.0, 1.0).into(),
        left: 50.0.into(),
        top: 50.0.into(),
        width: 100.0.into(),
        height: 100.0.into(),
        ..Default::default()
    };

    let root = commands
        .spawn((
            WoodpeckerApp,
            WoodpeckerStyle {
                font_size: 50.0,
                color: Srgba::RED.into(),
                ..Default::default()
            },
            WidgetChildren::default()
                .with_child::<Element>((
                    Element,
                    WidgetRender::Quad,
                    Transition {
                        easing: TransitionEasing::QuadraticInOut,
                        timeout: 500.0,
                        looping: true,
                        style_a: WoodpeckerStyle {
                            ..quad_styles.clone()
                        },
                        style_b: WoodpeckerStyle {
                            left: Units::Pixels(500.0).into(),
                            ..quad_styles.clone()
                        },
                        ..Default::default()
                    },
                ))
                .with_child::<Element>((
                    Element,
                    WidgetRender::Quad,
                    Transition {
                        easing: TransitionEasing::CubicInOut,
                        timeout: 500.0,
                        looping: true,
                        style_a: WoodpeckerStyle {
                            top: 175.0.into(),
                            ..quad_styles.clone()
                        },
                        style_b: WoodpeckerStyle {
                            top: 175.0.into(),
                            left: Units::Pixels(500.0).into(),
                            ..quad_styles.clone()
                        },
                        ..Default::default()
                    },
                ))
                .with_child::<Element>((
                    Element,
                    WidgetRender::Quad,
                    Transition {
                        easing: TransitionEasing::CircularInOut,
                        timeout: 500.0,
                        looping: true,
                        style_a: WoodpeckerStyle {
                            top: 300.0.into(),
                            ..quad_styles.clone()
                        },
                        style_b: WoodpeckerStyle {
                            top: 300.0.into(),
                            left: Units::Pixels(500.0).into(),
                            background_color: Srgba::new(0.0, 0.0, 1.0, 1.0).into(),
                            ..quad_styles.clone()
                        },
                        ..Default::default()
                    },
                ))
                // With clipping!
                .with_child::<Clip>((
                    Clip,
                    WidgetChildren::default().with_child::<Element>((
                        Element,
                        WidgetRender::Text {
                            content: "Hello, I am some random long text that gets clipped by a transition! :D".into(),
                        },
                    )),
                    Transition {
                        easing: TransitionEasing::CubicInOut,
                        timeout: 500.0,
                        looping: true,
                        style_a: WoodpeckerStyle {
                            width: 0.0.into(),
                            top: 425.0.into(),
                            ..quad_styles.clone()
                        },
                        style_b: WoodpeckerStyle {
                            top: 425.0.into(),
                            width: 1000.0.into(),
                            ..quad_styles.clone()
                        },
                        ..Default::default()
                    },
                ))
                // With no clipping!
                .with_child::<Element>((
                    Element,
                    WidgetChildren::default().with_child::<Element>((
                            Element,
                            WoodpeckerStyle {
                                width: Units::Percentage(100.0),
                                ..Default::default()
                            },
                            WidgetRender::Text {
                                content: "Hello, I am some random long text that gets wrapped by a transition! :D".into(),
                            },
                        )),
                    Transition {
                        easing: TransitionEasing::CubicInOut,
                        timeout: 2500.0,
                        looping: true,
                        style_a: WoodpeckerStyle {
                            width: 0.0.into(),
                            top: 450.0.into(),
                            ..quad_styles.clone()
                        },
                        style_b: WoodpeckerStyle {
                            top: 450.0.into(),
                            width: 500.0.into(),
                            ..quad_styles.clone()
                        },
                        ..Default::default()
                    },
                )),
        ))
        .id();
    ui_context.set_root_widget(root);
}
