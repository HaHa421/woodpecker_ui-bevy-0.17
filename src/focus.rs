use bevy::{
    picking::{hover::PickingInteraction, pointer::PointerPress},
    prelude::*,
};

/// Marks an entity as focusable
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Focusable;

/// A resource used to keep track of the currently focused entity.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CurrentFocus(Entity);

impl CurrentFocus {
    /// Create a new CurrentFocus.
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }

    /// Gets the entity that has focus.
    pub fn get(&self) -> Entity {
        self.0
    }

    /// Sets the entity that has focus.
    pub fn set(&mut self, entity: Entity) {
        self.0 = entity;
    }

    #[allow(dead_code)]
    pub(crate) fn find_next_focus() {
        // TODO: Write system to find next focused object
        todo!();
    }

    #[allow(dead_code)]
    pub(crate) fn find_prev_focus() {
        // TODO: Write system to find prev focused object
        todo!();
    }

    pub(crate) fn click_focus(
        mut commands: Commands,
        mut current_focus: ResMut<CurrentFocus>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        query: Query<
            (Entity, Option<&PickingInteraction>),
            (With<Focusable>, Changed<PickingInteraction>),
        >,
        pointer_query: Query<&PointerPress>,
    ) {
        let mut none_selected = true;
        for (entity, picking_interaction) in query.iter() {
            if let Some(picking_interaction) = picking_interaction {
                // Check if pressed
                if mouse_input.just_pressed(MouseButton::Left) {
                    if matches!(picking_interaction, PickingInteraction::Pressed) {
                        // Blur previously focused entity.
                        if current_focus.get() != entity {
                            commands.trigger(
                                WidgetBlur {
                                    target: current_focus.get(),
                                },
                                //current_focus.get(),
                            );
                        }
                        // Focus new entity
                        *current_focus = CurrentFocus::new(entity);
                        commands
                            .trigger(WidgetFocus { target: entity }/*, current_focus.get()*/);
                        none_selected = false;
                    }
                }
            }
        }

        if mouse_input.just_pressed(MouseButton::Left) {
            if none_selected && pointer_query.iter().any(|press| press.is_primary_pressed()) {
                // Blur if we have a focused entity because we had no "hits" this frame.
                if current_focus.get() != Entity::PLACEHOLDER {
                    commands.trigger(
                        WidgetBlur {
                            target: current_focus.get(),
                        },
                        //current_focus.get(),
                    );
                }
                // Remove current focus.
                *current_focus = CurrentFocus::new(Entity::PLACEHOLDER);
            }
        }
    }
}

/// A bevy_eventlistener Event that triggers when a widget has focus.
/// Note: The widget must have the Focusable component tag.
#[derive(Clone, PartialEq, Debug, Reflect,  EntityEvent)]
pub struct WidgetFocus {
    /// The target of this event
    #[event_target]
    pub target: Entity,
}

/// A bevy_eventlistener Event that triggers when a widget has lost focus.
/// Note: The widget must have the Focusable component tag.
#[derive(Clone, PartialEq, Debug, Reflect,  EntityEvent)]
pub struct WidgetBlur {
    /// The target of this event
    #[event_target]
    pub target: Entity,
}
