use config::Config;
use std::thread;
use std::time::{Duration, Instant};

pub fn main() {
    let settings = get_config_settings();
    loop {
        pomodoro(
            settings
                .get("minutes")
                .expect("Could not get default minute setting"),
        )
    }
}

fn get_config_settings() -> Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .expect("Could not find ./Settings.toml");

    settings
}

fn pomodoro(minutes: u64) {
    println!("Starting Pomodoro");
    let start = Instant::now();
    let duration = Duration::from_secs(minutes * 60);

    while start.elapsed() < duration {
        println!(
            "{:?}",
            (duration.as_secs() - start.elapsed().as_secs()) / 60
        );
        thread::sleep(Duration::from_secs(60));
    }

    println!("Finished Pomodoro");
    thread::sleep(Duration::from_secs(5 * 60));
}
