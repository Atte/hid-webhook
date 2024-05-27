use std::sync::Arc;

use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    ClientConfig, Error, RootCertStore, SignatureScheme,
};

#[derive(Debug, Clone, Copy)]
struct NoopVerifier;

impl ServerCertVerifier for NoopVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy,
        ]
    }
}

pub fn safe() -> ClientConfig {
    log::trace!("Safe TLS configuration");
    let mut root_certs = RootCertStore::empty();
    root_certs.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    ClientConfig::builder()
        .with_root_certificates(root_certs)
        .with_no_client_auth()
}

pub fn noverify() -> ClientConfig {
    log::trace!("No-verify TLS configuration");
    ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoopVerifier))
        .with_no_client_auth()
}
