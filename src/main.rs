use evdev::{Device, InputEventKind};
use noop_verifier::NoopVerifier;
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use std::{
    collections::HashSet,
    env::VarError,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ureq::{json, Agent, AgentBuilder};

mod noop_verifier;

fn env_set(name: &str) -> Result<HashSet<String>, VarError> {
    Ok(std::env::var(name)?
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect())
}

fn root_certs() -> RootCertStore {
    let mut root_store = RootCertStore::empty();
    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    root_store
}

fn main() {
    let devices = env_set("HID_WEBHOOK_DEVICES").expect("HID_WEBHOOK_DEVICES invalid");
    let urls = env_set("HID_WEBHOOK_URLS").expect("HID_WEBHOOK_URLS invalid");

    let devices: HashSet<_> = devices.into_iter().map(PathBuf::from).collect();

    let tls_config = Arc::new(
        if std::env::var_os("HID_WEBHOOK_NOVERIFY") == Some("true".into()) {
            ClientConfig::builder()
                .with_safe_defaults()
                .with_custom_certificate_verifier(Arc::new(NoopVerifier))
                .with_no_client_auth()
        } else {
            ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_certs())
                .with_no_client_auth()
        },
    );

    let agent = AgentBuilder::new()
        .timeout(Duration::from_secs(3))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .tls_config(tls_config)
        .build();

    std::thread::scope(|scope| {
        for device in &devices {
            scope.spawn(|| loop {
                if let Err(err) = device_thread(device, &agent, &urls) {
                    eprintln!("{err}");
                }
                std::thread::sleep(Duration::from_secs(1));
            });
        }
    });
}

fn device_thread(device_path: &Path, agent: &Agent, urls: &HashSet<String>) -> std::io::Result<()> {
    let mut device = Device::open(device_path)?;
    device.grab()?;

    loop {
        for event in device.fetch_events()? {
            match event.kind() {
                InputEventKind::Key(key) => {
                    let down = event.value() == 1;
                    let code = key.code();
                    println!(
                        "{} {} {}",
                        device_path.display(),
                        code,
                        if down { "down" } else { "up" }
                    );

                    for url in urls {
                        if let Err(err) = agent
                            .post(&url)
                            .set("Content-Type", "application/json")
                            .send_json(json!({
                                "device_path": device_path,
                                "code": code,
                                "down": down,
                            }))
                        {
                            eprintln!("failed to post to {}: {}", url, err);
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
