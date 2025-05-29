use crate::game::dev::inspector_ui::UiState;
use bevy::asset::{Assets, Handle};
use bevy::color::{Alpha, Color, Luminance};
use bevy::input::ButtonInput;
use bevy::pbr::{Material, MeshMaterial3d, StandardMaterial};
use bevy::picking::pointer::PointerInteraction;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use std::fmt::Debug;

#[auto_register_type]
#[derive(Component, Debug, Copy, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DebugSelected;

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Copy, Clone, Reflect)]
pub enum DebugSelect {
    Clear,
    Extend(Entity),
    Remove(Entity),
}

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Debug, Copy, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct DebugSelectEnabled(pub bool);

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Copy, Clone, Reflect)]
pub struct DebugHighlightTriggerClear;

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Copy, Clone, Reflect)]
pub struct DebugHighlightTriggerHighlight;

pub fn on_debug_select_event(
    mut evr: EventReader<DebugSelect>,
    mut commands: Commands,
    selected_entities: Query<Entity, With<DebugSelected>>,
    mut ui_state: ResMut<UiState>,
    nodes: Query<(), With<Node>>,
) {
    for select in evr.read() {
        log::debug!("DebugSelect: {:?}", select);
        match *select {
            DebugSelect::Clear => {
                ui_state.selected_entities_mut().clear();
                for entity in selected_entities.iter() {
                    let Ok(mut entity_commands) = commands.get_entity(entity) else {
                        continue;
                    };
                    entity_commands.remove::<DebugSelected>();
                }
            }
            DebugSelect::Extend(entity) => {
                if nodes.contains(entity) {
                    log::debug!("skipping node {entity}");
                    continue;
                }
                ui_state
                    .selected_entities_mut()
                    .select_maybe_add(entity, true);
                let Ok(mut entity_commands) = commands.get_entity(entity) else {
                    continue;
                };
                entity_commands.insert(DebugSelected);
            }
            DebugSelect::Remove(entity) => {
                if nodes.contains(entity) {
                    log::debug!("skipping node {entity}");
                    continue;
                }
                ui_state.selected_entities_mut().remove(entity);
                let Ok(mut entity_commands) = commands.get_entity(entity) else {
                    continue;
                };
                entity_commands.remove::<DebugSelected>();
            }
        }
    }
}

pub fn global_debug_picking(
    previously_selected: Query<Entity, With<DebugSelected>>,
    button_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    pointer_hits: Query<&PointerInteraction>,
    mut debug_select_evw: EventWriter<DebugSelect>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        let is_shift = button_input.pressed(KeyCode::ShiftLeft);
        let is_ctrl = button_input.pressed(KeyCode::ControlLeft);
        enum DebugSelectionMode {
            Add,
            Toggle,
            Select,
        }
        let select_mod = match (is_shift, is_ctrl) {
            (true, _) => DebugSelectionMode::Add,
            (false, true) => DebugSelectionMode::Toggle,
            (false, false) => DebugSelectionMode::Select,
        };
        let mut cleared = false;
        for hit in pointer_hits.iter() {
            let Some((entity, _hit)) = hit.get_nearest_hit() else {
                continue;
            };
            if let DebugSelectionMode::Select = select_mod {
                if !cleared {
                    cleared = true;
                    // clear previous
                    log::trace!("clear");
                    debug_select_evw.write(DebugSelect::Clear);
                }
            }
            match select_mod {
                DebugSelectionMode::Select | DebugSelectionMode::Add => {
                    // select
                    if previously_selected.contains(*entity) {
                        continue;
                    }
                    log::trace!("extending a {entity}");
                    debug_select_evw.write(DebugSelect::Extend(*entity));
                }
                DebugSelectionMode::Toggle => {
                    if previously_selected.contains(*entity) {
                        // deselect
                        log::trace!("removing {entity}");
                        debug_select_evw.write(DebugSelect::Remove(*entity));
                    } else {
                        // select
                        log::trace!("extending b {entity}");
                        debug_select_evw.write(DebugSelect::Extend(*entity));
                    }
                }
            }
        }
    }
}

pub fn on_debug_selected_added(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    debug_selected_added: Query<(Entity, Ref<DebugSelected>), Added<DebugSelected>>,
) {
    for (entity, selected) in debug_selected_added.iter() {
        if !selected.is_added() {
            continue;
        }
        log::debug!("debug selected added {entity}");
        commands
            .entity(entity)
            .trigger(DebugHighlightTriggerHighlight);
        if ui_state.selected_entities().contains(entity) {
            continue;
        }
        ui_state
            .selected_entities_mut()
            .select_maybe_add(entity, true);
    }
}

pub fn on_debug_selected_removed(
    mut commands: Commands,
    mut debug_selected_removed: RemovedComponents<DebugSelected>,
) {
    for entity in debug_selected_removed.read() {
        log::debug!("debug selected removed {entity}");
        let Ok(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };
        entity_commands.trigger(DebugHighlightTriggerClear);
    }
}

pub fn add_highlight_observers(
    mut commands: Commands,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    standard_material_q: Query<
        (Entity, Ref<MeshMaterial3d<StandardMaterial>>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
) {
    for (mesh_entity, initial_material) in standard_material_q.iter() {
        if !initial_material.is_added() {
            continue;
        }
        let initial_material_handle = &initial_material.0;
        let Some(initial_material) = standard_materials.get(initial_material_handle) else {
            log::warn!("highlight material not found for {mesh_entity}");
            continue;
        };

        let mut highlight_material = initial_material.clone();
        let original_linear_rgba = highlight_material.base_color.to_linear();
        let linear_rgba = original_linear_rgba.with_alpha(1.0);
        let linear_rgba = linear_rgba.lighter(0.15);
        let linear_rgba = if linear_rgba != original_linear_rgba {
            linear_rgba
        } else {
            linear_rgba.darker(0.15)
        };
        if linear_rgba == original_linear_rgba {
            log::warn!("failed to create highlight material for {mesh_entity}");
        }
        highlight_material.base_color = Color::from(linear_rgba);
        highlight_material.unlit = true;

        let mut pressed_material = highlight_material.clone();
        pressed_material.base_color = pressed_material.base_color.darker(0.1);
        pressed_material.unlit = true;

        commands
            .entity(mesh_entity)
            // over
            .observe(on_mouse_event_update_material::<
                Pointer<Over>,
                StandardMaterial,
            >(standard_materials.add(highlight_material)))
            // out
            .observe(on_mouse_event_update_material::<
                Pointer<Out>,
                StandardMaterial,
            >(initial_material_handle.clone()))
            .observe(on_debug_highlight_trigger_update_material::<
                DebugHighlightTriggerClear,
            >(initial_material_handle.clone()))
            // down
            .observe(on_mouse_event_update_material::<
                Pointer<Pressed>,
                StandardMaterial,
            >(standard_materials.add(pressed_material)));
    }
}

fn on_mouse_event_update_material<E, M>(
    material_handle: Handle<M>,
) -> impl FnMut(Trigger<E>, Commands, Query<Option<&DebugSelected>>)
where
    E: Event + Debug,
    M: Material,
{
    move |trigger, mut commands, debug_selected| {
        if let Ok(debug_selected_opt) = debug_selected.get(trigger.target()) {
            if debug_selected_opt.is_some() {
                return;
            }
        }
        commands
            .entity(trigger.target())
            .insert(MeshMaterial3d(material_handle.clone()));
    }
}

fn on_debug_highlight_trigger_update_material<E>(
    material_handle: Handle<StandardMaterial>,
) -> impl FnMut(Trigger<E>, Commands, Query<Option<&DebugSelected>>)
where
    E: Event + Debug,
{
    move |trigger, mut commands, debug_selected| {
        if let Ok(debug_selected_opt) = debug_selected.get(trigger.target()) {
            if debug_selected_opt.is_some() {
                return;
            }
        }
        commands
            .entity(trigger.target())
            .insert(MeshMaterial3d(material_handle.clone()));
    }
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            global_debug_picking.run_if(|res: Res<DebugSelectEnabled>| res.0),
            on_debug_select_event,
            on_debug_selected_removed,
            on_debug_selected_added,
        )
            .chain(),
    );
    app.add_systems(PostStartup, add_highlight_observers);
    app.add_systems(Update, add_highlight_observers);
}
