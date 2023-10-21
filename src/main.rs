use config::Config;
use evdev::{Device, InputEventKind, Key};
use serde::Serialize;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ureq::{Agent, AgentBuilder};

mod config;
mod tls_config;

fn main() {
    env_logger::init();

    let config: Config = match envy::prefixed("HID_WEBHOOK_").from_env() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to load configuration: {}", err);
            return;
        }
    };
    log::trace!("Configuration: {:#?}", config);

    let agent = AgentBuilder::new()
        .timeout(config.timeout)
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .tls_config(Arc::new(if config.no_verify {
            tls_config::noverify()
        } else {
            tls_config::safe()
        }))
        .build();

    let device_paths = config
        .devices
        .iter()
        .map(PathBuf::from)
        .collect::<HashSet<_>>();

    std::thread::scope(|scope| {
        for device_path in &device_paths {
            log::debug!("Spawning thread for {}", device_path.display());
            scope.spawn(|| loop {
                if let Err(err) = device_thread(device_path, &config, &agent) {
                    log::error!("{}: {err}", device_path.display());
                }
                std::thread::sleep(Duration::from_secs(1));
            });
        }
    });
}

#[derive(Debug, Clone, Serialize)]
struct PostBody<'a> {
    device_path: &'a Path,
    key: Key,
    code: u16,
    down: bool,
}

fn device_thread(device_path: &Path, config: &Config, agent: &Agent) -> std::io::Result<()> {
    let mut device = Device::open(device_path)?;
    device.grab()?;

    loop {
        for event in device.fetch_events()? {
            match event.kind() {
                InputEventKind::Key(key) => {
                    let down = event.value() == 1;
                    if config.down && !down {
                        log::trace!("{}: Ignoring up {}", device_path.display(), key.code());
                        continue;
                    }
                    if config.up && down {
                        log::trace!("{}: Ignoring down {}", device_path.display(), key.code());
                        continue;
                    }

                    let code = key.code();
                    if config.ignore_keys.contains(&code) {
                        log::trace!("{}: Ignoring key {}", device_path.display(), code);
                        continue;
                    }

                    log::info!(
                        "{}: {} {}",
                        device_path.display(),
                        code,
                        if down { "down" } else { "up" }
                    );

                    for url in &config.urls {
                        log::debug!("{}: Posting to {}", device_path.display(), url);
                        if let Err(err) = agent
                            .post(url)
                            .set("Content-Type", "application/json")
                            .send_json(PostBody {
                                device_path,
                                key,
                                code,
                                down,
                            })
                        {
                            log::error!(
                                "{}: Failed to post to {}: {}",
                                device_path.display(),
                                url,
                                err
                            );
                        }
                    }
                }
                _ => {
                    // noop
                }
            }
        }
    }
}
