use std::error::Error;
use tokio;
use serde::Deserialize;
use serde_json::Value;
use async_trait::async_trait;

// 定义一个表单数据结构体
#[derive(Deserialize, Debug)]
struct FormData {
    field1: String,
    field2: u32,
    // 可以添加更多字段
}

// 定义一个表单验证器的特质
#[async_trait]
trait FormValidator {
    async fn validate(&self, data: &FormData) -> Result<(), Box<dyn Error>>;
}

// 实现一个简单的表单验证器
struct SimpleFormValidator;

#[async_trait]
impl FormValidator for SimpleFormValidator {
    async fn validate(&self, data: &FormData) -> Result<(), Box<dyn Error>> {
        if data.field1.is_empty() {
            return Err(From::from("Field1 cannot be empty"));
        }
        if data.field2 == 0 {
            return Err(From::from("Field2 must not be zero"));
        }
        // 可以添加更多验证规则
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 模拟从客户端接收的表单数据
    let data_json = r#"{"field1":"Hello", "field2":10}"#;
    let data: Value = serde_json::from_str(data_json)?;
    let form_data: FormData = serde_json::from_value(data)?;

    // 创建一个验证器实例
    let validator = SimpleFormValidator;

    // 执行验证
    validator.validate(&form_data).await?;

    println!("Form data is valid");
    Ok(())
}
