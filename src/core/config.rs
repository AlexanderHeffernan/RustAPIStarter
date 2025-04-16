use rustls::{pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject}, ServerConfig};
use std::path::Path;

/*
    Load TLS configuration for HTTPS.
    This function reads the certificate and private key from the specified paths.
    It returns a ServerConfig object that can be used to configure the Actix web server.
*/
pub fn load_rustls_config(cert_path: impl AsRef<Path>, key_path: impl AsRef<Path>) -> Option<ServerConfig> {
    let cert_path = cert_path.as_ref();
    let key_path = key_path.as_ref();

    // Load the certificate chain from the provided file
    let cert_chain: Vec<CertificateDer> = match CertificateDer::pem_file_iter(cert_path)
        .map(|res| res.flatten().collect::<Vec<_>>())
    {
        Ok(chain) if !chain.is_empty() => chain,
        Ok(_) => {
            println!("Error: No certificates found in {}", cert_path.display());
            return None;
        }
        Err(e) => {
            println!("Error: Failed to parse PEM file at {}: {}", cert_path.display(), e);
            return None;
        }
    };

    // Load the private key from the provided file
    let key_der = match PrivateKeyDer::from_pem_file(key_path) {
        Ok(key) => key,
        Err(_) => {
            println!("Error: No private key found in {}", key_path.display());
            return None;
        }
    };

    // Build and return the Rustls server configuration
    match ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
    {
        Ok(config) => Some(config),
        Err(e) => {
            println!("Error: Failed to build TLS configuration: {}", e);
            None
        }
    }
}