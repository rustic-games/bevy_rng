#[cfg(all(feature = "bevy-nightly", not(feature = "bevy-stable")))]
use bevy_nightly as bevy;
#[cfg(all(feature = "bevy-stable", not(feature = "bevy-nightly")))]
use bevy_stable as bevy;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_rng::*;
use std::time::Duration;

fn main() {
    // Don't register the plugin (non-deterministic)...
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_system(random_number_1)
        .add_system(random_number_2)
        .run();

    // ...don't provide a seed (same as above)...
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::default())
        .add_system(random_number_1)
        .add_system(random_number_2)
        .run();

    // ...seed from u64 (deterministic)...
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from(42))
        .add_system(random_number_1)
        .add_system(random_number_2)
        .run();

    // ...or from a string (same as above).
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugins(MinimalPlugins)
        .add_plugin(RngPlugin::from("your seed here"))
        .add_system(random_number_1)
        .add_system(random_number_2)
        .run();
}

fn random_number_1(mut rng: Local<Rng>) {
    println!("1: {}", rng.gen::<f64>());
}

fn random_number_2(mut rng: Local<Rng>) {
    println!("2: {}", rng.gen::<f64>());
}
