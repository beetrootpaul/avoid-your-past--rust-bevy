use std::time::Duration;

use bevy::prelude::{debug, Res};

const FIXED_FPS: u64 = 30;

pub struct FixedTimestep;

impl FixedTimestep {
    pub fn duration() -> Duration {
        Duration::from_nanos(1_000_000_000 / FIXED_FPS)
    }
    pub fn label() -> iyes_loopless::fixedtimestep::TimestepName {
        "fixed_timestep_label"
    }
    pub fn log_measurements_system(timesteps: Res<iyes_loopless::fixedtimestep::FixedTimesteps>) {
        let info = timesteps
            .get_current()
            .expect("should get current fixed timestep");
        debug!(
            "Fixed timestep: expected = {:?} | overstepped by = {:?}",
            info.timestep(),
            info.remaining(),
        );
    }
}
