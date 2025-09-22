use anyhow::Result;
use tokio;
use secrecy::SecretString;
use ring::agreement::{EphemeralPrivateKey, X25519};
use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
use ring::error::Unspecified;
use std::io::{self, Write};

// 密码加密解密工具
#[tokio::main]
async fn main() -> Result<(), Unspecified> {
    let keypair = generate_keypair();
    let encrypted_password = encrypt_password("password".to_string(), &keypair).await?;
    let decrypted_password = decrypt_password(&encrypted_password, &keypair).await?;
    println!("Encrypted Password: {:?}", encrypted_password);
    println!("Decrypted Password: {:?}