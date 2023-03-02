use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

const FIXED_FPS: u64 = 10;

const FIXED_TIMESTEP_LABEL: &str = "fixed_timestep_label";

pub struct FixedFpsPlugin;

impl Plugin for FixedFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep(
            Duration::from_nanos(1_000_000_000 / FIXED_FPS),
            FIXED_TIMESTEP_LABEL,
        );
        app.insert_resource(LastSubstageIndex(0));
    }
}

#[derive(Resource)]
struct LastSubstageIndex(usize);

pub trait FixedFpsBevyAppExtension {
    fn log_fixed_fps_measurements(&mut self) -> &mut App;
    fn add_fixed_fps_stage(&mut self, system_sets: Vec<SystemSet>) -> &mut App;
}

impl FixedFpsBevyAppExtension for App {
    fn log_fixed_fps_measurements(&mut self) -> &mut App {
        self.add_fixed_timestep_system(FIXED_TIMESTEP_LABEL, 0, log_measurements)
    }

    fn add_fixed_fps_stage(&mut self, system_sets: Vec<SystemSet>) -> &mut App {
        self.add_fixed_timestep_child_stage(FIXED_TIMESTEP_LABEL);

        let mut res_last_substage_index: Mut<LastSubstageIndex> = self
            .world
            .get_resource_mut::<LastSubstageIndex>()
            .expect("should retrieve LastSubstageIndex");
        res_last_substage_index.0 += 1;
        let last_substage_index = res_last_substage_index.0;

        for system_set in system_sets {
            self.add_fixed_timestep_system_set(
                FIXED_TIMESTEP_LABEL,
                last_substage_index,
                system_set,
            );
        }

        self
    }
}

fn log_measurements(timesteps: Res<iyes_loopless::fixedtimestep::FixedTimesteps>) {
    let info = timesteps
        .get_current()
        .expect("should get current fixed timestep");
    debug!(
        "Fixed timestep: expected = {:?} | overstepped by = {:?}",
        info.timestep(),
        info.remaining(),
    );
}
