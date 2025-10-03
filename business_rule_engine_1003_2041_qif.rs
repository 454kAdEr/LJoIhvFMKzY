use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

// 业务规则引擎
pub struct BusinessRuleEngine<T> {
    rules: Arc<Mutex<HashMap<String, Box<dyn Fn(&mut T) -> Result<(), String>>>>,
}

impl<T> BusinessRuleEngine<T> {
    // 创建一个新的业务规则引擎
    pub fn new() -> Self {
        BusinessRuleEngine {
            rules: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 添加规则到引擎中
    pub fn add_rule(&self, name: String, rule: Box<dyn Fn(&mut T) -> Result<(), String>>) {
        let mut guard = self.rules.lock().unwrap();
        guard.insert(name, rule);
    }

    // 执行规则
    pub async fn execute(&self, instance: &mut T, name: &str) -> Result<(), String> {
        let guard = self.rules.lock().await;
        if let Some(rule) = guard.get(name) {
            (rule(instance)).map_err(|e| e)?;
            Ok(())
        } else {
            Err(format!("Rule '{}' not found.", name))
        }
    }
}

// 示例规则
async fn example_rule(instance: &mut i32) -> Result<(), String> {
    if *instance < 0 {
        Err("Instance value cannot be negative.".to_string())
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut instance = 10;
    let engine = BusinessRuleEngine::<i32>::new();

    engine.add_rule("example_rule".to_string(), Box::new(example_rule));

    match engine.execute(&mut instance, "example_rule").await {
        Ok(_) => println!("Rule executed successfully."),
        Err(e) => println!("Error executing rule: {}", e),
    }
}
