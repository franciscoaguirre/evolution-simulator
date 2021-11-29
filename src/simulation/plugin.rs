use bevy::prelude::*;

use super::muscle::MusclePlugin;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(MusclePlugin)
            .add_system(simulate.system());
    }
}

fn simulate() {

}
