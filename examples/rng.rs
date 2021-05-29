use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_rng::*;
use std::time::Duration;

fn main() {
    // Don't register the plugin (non-deterministic)...
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_system(random_number_1.system())
        .add_system(random_number_2.system())
        .run();

    // ...don't provide a seed (same as above)...
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::default())
        .add_system(random_number_1.system())
        .add_system(random_number_2.system())
        .run();

    // ...seed from u64 (deterministic)...
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from(42))
        .add_system(random_number_1.system())
        .add_system(random_number_2.system())
        .run();

    // ...or from a string (same as above).
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from("your seed here"))
        .add_system(random_number_1.system())
        .add_system(random_number_2.system())
        .run();
}

fn random_number_1(mut rng: Local<Rng>) {
    println!("1: {}", rng.gen::<f64>());
}

fn random_number_2(mut rng: Local<Rng>) {
    println!("2: {}", rng.gen::<f64>());
}
