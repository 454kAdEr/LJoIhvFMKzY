//! Rust程序，使用TOKIO框架实现学习效果评估

#[macro_use]
extern crate log; // 日志宏
extern crate env_logger; // 环境日志

use tokio::sync::Mutex; // 异步互斥锁
use std::sync::Arc; // 互斥访问
use std::collections::HashMap; // 哈希映射

/// 学习效果评估程序
#[derive(Debug, Clone)]
struct LearningAssessment {
    student_id: String,
    scores: HashMap<String, f64>, // 学生的成绩
}

impl LearningAssessment {
    /// 创建一个新的学习效果评估实例
    fn new(student_id: &str) -> Self {
        LearningAssessment {
            student_id: student_id.to_string(),
            scores: HashMap::new(),
        }
    }

    /// 添加或更新学生的成绩
    fn add_or_update_score(&mut self, subject: &str, score: f64) {
        self.scores.insert(subject.to_string(), score);
    }

    /// 计算学生的平均成绩
    async fn calculate_average_score(&self) -> Result<f64, String> {
        if self.scores.is_empty() {
            return Err("No scores available".to_string());
        }
        let total: f64 = self.scores.values().sum();
        let count = self.scores.len() as f64;
        Ok(total / count)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init(); // 初始化环境日志
    let assessment = Arc::new(Mutex::new(LearningAssessment::new("12345")));

    // 添加成绩
    let mut assessment_write = assessment.lock().await;
    assessment_write.add_or_update_score("Math", 85.0);
    assessment_write.add_or_update_score("Science", 90.0);
    assessment_write.add_or_update_score("English", 78.0);
    drop(assessment_write); // 显式释放锁

    // 计算平均成绩
    let assessment_read = assessment.lock().await;
    match assessment_read.calculate_average_score().await {
        Ok(average) => {
            println!("Average score for student {}: {}", assessment_read.student_id, average);
        },
        Err(e) => {
            eprintln!("Error calculating average score: {}", e);
        },
    }
}
