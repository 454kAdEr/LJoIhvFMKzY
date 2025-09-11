use tokio::time::{self, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

/// 定义任务接口
trait Task: Send + Sync + 'static + Fn() {
    // 执行任务
    fn run(&self);
}

/// 定时任务调度器
struct Scheduler {
    tasks: Arc<Mutex<HashMap<String, (Box<dyn Task>, Duration)>>>,
}

impl Scheduler {
    /// 创建一个新的调度器
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 添加任务到调度器
    pub async fn add_task<F>(&self, task_name: String, task: F, interval: Duration)
    where
        F: Task + 'static,
    {
        let mut tasks = self.tasks.lock().await;
        tasks.insert(task_name, (Box::new(task), interval));
    }

    /// 运行所有添加的任务
    pub async fn run(&self) {
        let mut tasks = self.tasks.lock().await;
        for (_name, (task, interval)) in tasks.iter().cloned() {
            let scheduler = self.clone();
            tokio::spawn(async move {
                loop {
                    task.run();
                    time::sleep(interval).await;
                }
            });
        }
    }
}

#[tokio::main]
async fn main() {
    let scheduler = Scheduler::new();

    // 添加任务到调度器
    scheduler.add_task("example_task".to_string(), example_task, time::Duration::from_secs(5)).await;

    scheduler.run().await;
}

/// 示例任务函数
fn example_task() {
    println!("Task executed at: {}","{}");
}