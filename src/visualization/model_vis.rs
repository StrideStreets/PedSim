use crate::model::pedestrian::Pedestrian;
use crate::model::state::ModelState;
use crate::visualization::ped_vis::PedVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
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
        match state.ped_grid.get(&Pedestrian::new(
            agent_render.get_id(),
            Int2D { x: 0, y: 0 },
            Int2D { x: 0, y: 0 },
            None,
            1,
        )) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None,
        }
    }
}
