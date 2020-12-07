use config::Config;
use hueclient::bridge::{Bridge, CommandLight};
use hueclient::HueError;
use std::time::{Duration, Instant};
use std::{env, thread};

pub fn main() {
    let settings = get_config_settings();
    loop {
        pomodoro(&settings)
    }
}

fn pomodoro(settings: &Config) {
    let start = Instant::now();
    let duration = Duration::from_secs(
        (settings
            .get_int("minutes")
            .expect("Could not get default minute setting")
            * 60) as u64,
    );

    println!("Starting Pomodoro ${:?}", duration);
    set_all_lights(
        settings
            .get("api_key")
            .expect("Could not get default api_key setting"),
        get_start_command(),
    );

    while start.elapsed() < duration {
        println!(
            "{:?}",
            (duration.as_secs() - start.elapsed().as_secs()) / 60
        );
        thread::sleep(Duration::from_secs(60));
    }

    set_all_lights(
        settings
            .get("api_key")
            .expect("Could not get default api_key setting"),
        get_end_command(),
    );

    println!("Finished Pomodoro");
    thread::sleep(Duration::from_secs(
        (settings
            .get_int("break_duration")
            .expect("Could not get default api_key setting")
            * 60) as u64,
    ));
}

fn set_all_lights(api_key: String, command: CommandLight) {
    let bridge = ::hueclient::bridge::Bridge::discover_required().with_user(api_key);
    match bridge.get_all_lights() {
        Ok(lights) => {
            for ref l in lights.iter() {
                bridge.set_light_state(l.id, &command).unwrap();
                std::thread::sleep(::std::time::Duration::from_millis(50));
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            ::std::process::exit(2)
        }
    }
}

fn get_config_settings() -> Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .expect("Could not find ./Settings.toml");

    settings
}

fn get_start_command() -> CommandLight {
    let mut start_command = hueclient::bridge::CommandLight::default().on();
    let hsv = rgb_to_hsv(255, 255, 251);
    start_command.hue = Some((hsv.0 * 65535f64) as u16);
    start_command.sat = Some((hsv.1 * 255f64) as u8);
    start_command.bri = Some((hsv.2 * 255f64) as u8);
    start_command.transitiontime = Some(5);
    start_command
}

fn get_end_command() -> CommandLight {
    let mut start_command = hueclient::bridge::CommandLight::default().on();
    let hsv = rgb_to_hsv(255, 147, 41);
    start_command.hue = Some((hsv.0 * 65535f64) as u16);
    start_command.sat = Some((hsv.1 * 255f64) as u8);
    start_command.bri = Some((hsv.2 * 255f64) as u8);
    start_command.transitiontime = Some(5);
    start_command
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255f64;
    let g = g as f64 / 255f64;
    let b = b as f64 / 255f64;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    if max == min {
        (0f64, 0f64, max)
    } else {
        let d = max - min;
        let s = d / max;
        let h = if max == r {
            (g - b) / d + (if g < b { 6f64 } else { 0f64 })
        } else if max == g {
            (b - r) / d + 2f64
        } else {
            (r - g) / d + 4f64
        };
        (h / 6f64, s, max)
    }
}

#[allow(dead_code)]
fn get_bridge() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}

#[allow(dead_code)]
fn register_user() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let mut bridge = ::hueclient::bridge::Bridge::discover_required();
        println!("posting user {:?} in {:?}", args[1], bridge);
        loop {
            let r = bridge.register_user(&args[1]);
            match r {
                Ok(r) => {
                    eprint!("done: ");
                    println!("{}", r);
                    break;
                }
                Err(HueError::BridgeError { code, .. }) if code == 101 => {
                    println!("Push the bridge button");
                    std::thread::sleep(::std::time::Duration::from_secs(5));
                }
                Err(e) => panic!("error {:?}", e),
            }
        }
    }
}
