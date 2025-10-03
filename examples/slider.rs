use bevy::prelude::*;
use woodpecker_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WoodpeckerUIPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, WoodpeckerView));

    let material_red = materials.add(Color::Srgba(Srgba::RED.with_alpha(0.5)));

    commands.spawn((
        Mesh2d(meshes.add(Circle { radius: 50.0 })),
        MeshMaterial2d(material_red),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let root = commands.spawn_empty().id();
    commands.entity(root).insert((
        WoodpeckerApp,
        WoodpeckerStyle {
            padding: Edge::all(10.0),
            ..default()
        },
        WidgetChildren::default()
            .with_child::<Slider>(Slider {
                start: 0.0,
                end: 1.0,
                value: 0.5,
            })
            .with_observe(
                CurrentWidget(root),
                |trigger: On<Change<SliderChanged>>,
                 mut material_assets: ResMut<Assets<ColorMaterial>>,
                 query: Query<&MeshMaterial2d<ColorMaterial>>| {
                    println!("SliderChanged");
                    for material in query.iter() {
                        material_assets
                            .get_mut(material)
                            .unwrap()
                            .color
                            .set_alpha(trigger.data.value)
                    }
                },
            ),
    ));
    ui_context.set_root_widget(root);
}
