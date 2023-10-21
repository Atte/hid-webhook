use std::sync::Arc;

use rustls::{
    client::{ServerCertVerified, ServerCertVerifier},
    Certificate, ClientConfig, Error, OwnedTrustAnchor, RootCertStore, ServerName,
};

struct NoopVerifier;

impl ServerCertVerifier for NoopVerifier {
    #[inline]
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }
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

pub fn safe() -> ClientConfig {
    log::trace!("Safe TLS configuration");
    ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certs())
        .with_no_client_auth()
}

pub fn noverify() -> ClientConfig {
    log::trace!("No-verify TLS configuration");
    ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(NoopVerifier))
        .with_no_client_auth()
}
