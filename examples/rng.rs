use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_rng::*;
use std::time::Duration;

fn main() {
    // don't provide a seed...
    App::build()
        .add_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::default())
        .add_system(random_number.system())
        .run();

    // ...seed from u64...
    App::build()
        .add_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from(42))
        .add_system(random_number.system())
        .run();

    // ...or from a string
    App::build()
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from("your seed here"))
        .add_system(random_number.system())
        .run();
}

fn random_number(mut rng: Local<Rng>) {
    println!("{}", rng.gen::<f64>());
}
