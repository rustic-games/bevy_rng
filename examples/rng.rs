use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_rng::*;
use std::time::Duration;

fn main() {
    App::build()
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::seeded("my seed here"))
        .add_system(random_number.system())
        .run();
}

fn random_number(mut rng: Local<Rng>) {
    println!("{}", rng.gen::<f64>());
}
