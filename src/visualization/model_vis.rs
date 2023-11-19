use crate::model::state::ModelState;
use crate::model::{
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
};
use crate::visualization::ped_vis::PedVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
pub struct ModelVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<ModelState> for ModelVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut ModelState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        SparseGrid2D::<Object>::init_graphics_grid(_sprite_render_factory, _commands, _state);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &ModelState,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(PedVis {
            id: agent.downcast_ref::<Pedestrian>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<ModelState>().unwrap();
        match state.field.get(&Pedestrian::new(
            agent_render.get_id(),
            Real2D { x: 0., y: 0. },
            Real2D { x: 0., y: 0. },
            None,
            1.0,
        )) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        }
    }
}

impl Component for Object {
    type Storage = TableStorage;
}

impl RenderObjectGrid2D<ModelState, Object> for SparseGrid2D<Object> {
    fn fetch_sparse_grid(state: &ModelState) -> Option<&SparseGrid2D<Object>> {
        Some(&state.obj_grid)
    }

    fn fetch_dense_grid(_state: &ModelState) -> Option<&DenseGrid2D<Object>> {
        None
    }

    fn fetch_emoji(_state: &ModelState, obj: &Object) -> String {
        match obj.value {
            ObjectType::Path => "house".to_string(),
            ObjectType::Obstacle => "no_entry_sign".to_string(),
            //_ => panic!("Object not recognized."),
        }
    }

    fn fetch_loc(state: &ModelState, obj: &Object) -> Option<Int2D> {
        state.obj_grid.get_location(*obj)
    }

    fn fetch_rotation(_state: &ModelState, _obj: &Object) -> f32 {
        0.0
    }

    fn scale(obj: &Object) -> (f32, f32) {
        match obj.value {
            ObjectType::Path => (0.1, 0.1),
            ObjectType::Obstacle => (0.05, 0.05),
            //_ => panic!("Object not recognized."),
        }
    }
}
