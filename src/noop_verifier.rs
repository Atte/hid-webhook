use rustls::{
    client::{ServerCertVerified, ServerCertVerifier},
    Certificate, Error, ServerName,
};

pub struct NoopVerifier;

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
