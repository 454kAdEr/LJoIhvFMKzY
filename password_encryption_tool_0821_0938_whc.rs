use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use crypto_common::CryptoError;
use hex_literal::hex;
use rand::rngs::OsRng;
use rand::RngCore;
use std::io::{self, Write};

// 使用AES-256-CBC模式的密码加密解密工具
pub struct PasswordEncryptionTool;

impl PasswordEncryptionTool {
    // 加密密码
    pub fn encrypt_password(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Self::create_cipher(key)?;
        let iv = Self::generate_iv();
        let ciphertext = cipher.encrypt_vec(plaintext, iv.as_ref())?;
        Ok(ciphertext)
    }

    // 解密密码
    pub fn decrypt_password(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Self::create_cipher(key)?;
        let iv = Self::get_iv(ciphertext)?;
        let plaintext = cipher.decrypt_vec(ciphertext, iv)?;
        Ok(plaintext)
    }

    // 创建AES-256-CBC密码器
    fn create_cipher(key: &[u8]) -> Result<Aes256Cbc<Pkcs7>, CryptoError> {
        let cipher = Aes256::new_from_slices(key, None).map_err(|_| CryptoError::InvalidKey)?;
        Ok(cipher.into())
    }

    // 生成随机IV
    fn generate_iv() -> [u8; 16] {
        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut iv);
        iv
    }

    // 从密文中提取IV
    fn get_iv(ciphertext: &[u8]) -> Result<[u8; 16], CryptoError> {
        if ciphertext.len() < 16 {
            return Err(CryptoError::InvalidCiphertext);
        }
        let iv = ciphertext[..16].try_into().map_err(|_| CryptoError::InvalidCiphertext)?;
        Ok(iv)
    }
}

// 主函数
fn main() -> Result<(), CryptoError> {
    // 生成随机密钥
    let key = OsRng.gen();
    let key = key.as_bytes()[..32].try_into().expect("Failed to create key");

    // 要加密的密码
    let password = b"my_secret_password";

    // 加密密码
    let ciphertext = PasswordEncryptionTool::encrypt_password(&key, password)?;
    println!("Ciphertext: {}", hex::encode(&ciphertext));

    // 解密密码
    let plaintext = PasswordEncryptionTool::decrypt_password(&key, &ciphertext)?;
    println!("Plaintext: {}", String::from_utf8_lossy(&plaintext));

    Ok(())
}

// 定义加密错误类型
#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    InvalidCiphertext,
    Other(String),
}

impl From<block_modes::BlockModeError> for CryptoError {
    fn from(_: block_modes::BlockModeError) -> Self {
        CryptoError::Other("Block mode error".to_string())
    }
}

impl From<aes::AesError> for CryptoError {
    fn from(_: aes::AesError) -> Self {
        CryptoError::Other("AES error".to_string())
    }
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CryptoError::InvalidKey => write!(f, "Invalid key"),
            CryptoError::InvalidCiphertext => write!(f, "Invalid ciphertext"),
            CryptoError::Other(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for CryptoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
