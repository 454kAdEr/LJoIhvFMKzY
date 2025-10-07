use tokio::fs;
use tokio::io::{self, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct Certificate {
    id: String,
    subject: String,
    issuer: String,
    validity: String,
}

struct CertificateManager {
    certificates: Arc<Mutex<HashMap<String, Certificate>>>,
    storage_path: PathBuf,
}

impl CertificateManager {
    /**
     * 创建一个新的证书管理器实例。
     *
     * @param storage_path 证书存储路径
     */
    async fn new(storage_path: PathBuf) -> io::Result<Self> {
        let certificates = Self::load_certificates(&storage_path).await?;
        Ok(Self {
            certificates: Arc::new(Mutex::new(certificates)),
            storage_path,
        })
    }

    /**
     * 加载存储的证书。
     */
    async fn load_certificates(storage_path: &PathBuf) -> io::Result<HashMap<String, Certificate>> {
        let mut certificates = HashMap::new();
        if let Ok(contents) = fs::read_to_string(storage_path).await {
            for line in contents.lines() {
                let cert: Certificate = serde_json::from_str(line)?;
                certificates.insert(cert.id.clone(), cert);
            }
        }
        Ok(certificates)
    }

    /**
     * 保存证书到存储。
     */
    async fn save_certificates(&self) -> io::Result<()> {
        let mut cert_str = String::new();
        for (_id, cert) in self.certificates.lock().await.iter() {
            cert_str.push_str(&serde_json::to_string(cert)?);
            cert_str.push('
');
        }
        fs::write(&self.storage_path, cert_str).await?;
        Ok(())
    }

    /**
     * 添加一个新的证书。
     */
    async fn add_certificate(&self, certificate: Certificate) -> io::Result<()> {
        let mut cert_map = self.certificates.lock().await;
        cert_map.insert(certificate.id.clone(), certificate);
        self.save_certificates().await?;
        Ok(())
    }

    /**
     * 获取一个证书。
     */
    async fn get_certificate(&self, id: &str) -> Option<Certificate> {
        let cert_map = self.certificates.lock().await;
        cert_map.get(id).cloned()
    }

    /**
     * 删除一个证书。
     */
    async fn remove_certificate(&self, id: &str) -> io::Result<()> {
        let mut cert_map = self.certificates.lock().await;
        if cert_map.remove(id).is_some() {
            self.save_certificates().await?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Certificate not found"))
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let storage_path = PathBuf::from("./certificates.txt");
    let manager = CertificateManager::new(storage_path).await?;

    // 添加证书
    let new_cert = Certificate {
        id: "cert1".to_string(),
        subject: "CN=example.com".to_string(),
        issuer: "CN=CA".to_string(),
        validity: "2023-01-01 to 2024-01-01".to_string(),
    };
    manager.add_certificate(new_cert).await?;

    // 获取证书
    if let Some(cert) = manager.get_certificate("cert1").await {
        println!("Certificate: {:?}