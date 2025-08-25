use serde::Deserialize;
use tokio;
use validator::{ValidationError, validate, ValidationErrors};
use validator::validators::{is_email, is_url};

// 定义表单数据结构
#[derive(Debug, Deserialize)]
pub struct FormData {
    // 邮箱字段
    email: String,
    // 网址字段
    url: String,
    // 数字字段
    number: u32,
}

// 实现表单数据验证器
pub async fn validate_form_data(data: FormData) -> Result<FormData, ValidationError> {
    // 使用validator库进行验证
    let mut errors = ValidationErrors::new();
    
    // 验证邮箱格式
    if !is_email(&data.email) {
        errors.add("email", ValidationError::new("invalid"));
    }
    
    // 验证网址格式
    if !is_url(&data.url) {
        errors.add("url", ValidationError::new("invalid"));
    }
    
    // 验证数字是否为正数
    if data.number == 0 || data.number < 0 {
        errors.add("number", ValidationError::new("must be a positive number"));
    }
    
    // 如果存在错误，返回错误
    if !errors.is_empty() {
        return Err(ValidationError::from(errors));
    }
    
    // 如果验证通过，返回原始数据
    Ok(data)
}

#[tokio::main]
async fn main() {
    // 示例表单数据
    let form_data = FormData {
        email: "example@example.com".to_string(),
        url: "https://www.example.com".to_string(),
        number: 42,
    };
    
    // 调用验证器
    match validate_form_data(form_data).await {
        Ok(valid_data) => println!("Validation passed: {:?}