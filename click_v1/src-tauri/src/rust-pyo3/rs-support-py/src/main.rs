use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// 全局计时状态结构体
#[derive(Debug)]
pub struct TimerState {
    pub start_time: Option<Instant>,
    pub target_duration: Option<Duration>,
    pub elapsed_time: Duration,
    pub is_running: bool,
}

// 使用静态变量存储全局状态
use std::sync::LazyLock;

pub static TIMER_STATE: LazyLock<Arc<Mutex<TimerState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(TimerState {
        start_time: None,
        target_duration: None,
        elapsed_time: Duration::new(0, 0),
        is_running: false,
    }))
});

/// 启动计时器
pub fn start_timer(duration_secs: u64) {
    let mut state = TIMER_STATE.lock().unwrap();
    state.start_time = Some(Instant::now());
    state.target_duration = Some(Duration::from_secs(duration_secs));
    state.elapsed_time = Duration::new(0, 0);
    state.is_running = true;
}

/// 等待指定时间（在后台运行）
/// 运行一个后台线程，每秒检查一次计时进度，并更新全局状态。
///
pub fn wait_with_timer(duration_secs: u64) {
    // 启动计时器
    start_timer(duration_secs);

    // 在新线程中执行计时
    thread::spawn(move || {
        let total_duration = Duration::from_secs(duration_secs); // 目标时间
        let check_interval = Duration::from_secs(1); // 每秒更新一次

        loop {
            let state = TIMER_STATE.lock().unwrap();
            let start_time = state.start_time;
            let target_duration = state.target_duration;
            let is_running = state.is_running;
            drop(state); // 释放锁

            if !is_running {
                break;
            }

            if let (Some(start), Some(target)) = (start_time, target_duration) {
                let elapsed = start.elapsed();

                // 更新全局状态
                {
                    let mut state = TIMER_STATE.lock().unwrap();
                    state.elapsed_time = elapsed;

                    // 如果达到目标时间，停止计时
                    if elapsed >= target {
                        state.is_running = false;
                        break;
                    }
                }

                // 每秒检查一次，避免过于频繁的检查
                if elapsed < target {
                    thread::sleep(check_interval);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    });
}

/// 获取当前计时进度
pub fn get_current_time() -> Duration {
    let state = TIMER_STATE.lock().unwrap();
    state.elapsed_time
}

/// 获取计时进度百分比
pub fn get_timer_progress_percentage() -> f64 {
    let state = TIMER_STATE.lock().unwrap();

    if let (Some(_), Some(target)) = (state.start_time, state.target_duration) {
        if target.as_millis() == 0 {
            return 0.0;
        }

        let progress = state.elapsed_time.as_millis() as f64;
        let total = target.as_millis() as f64;
        (progress / total) * 100.0
    } else {
        0.0
    }
}

/// 检查计时是否完成
pub fn is_timer_finished() -> bool {
    let state = TIMER_STATE.lock().unwrap();
    let is_running = state.is_running;
    let elapsed = state.elapsed_time;
    let target = state.target_duration;

    if let Some(target_duration) = target {
        !is_running && elapsed >= target_duration
    } else {
        !is_running
    }
}

/// 停止计时器
pub fn stop_timer() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.is_running = false;
}

/// 重置计时器
pub fn reset_timer() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.start_time = None;
    state.target_duration = None;
    state.elapsed_time = Duration::new(0, 0);
    state.is_running = false;
}

fn get_work() {
    // 获取当前时间
    let now = Instant::now();

    // 获取当前时间戳
    let timestamp = now.elapsed().as_secs();

    // 获取当前时间戳的秒数部分
    let seconds = timestamp % 60;

    // 获取当前时间戳的分钟数部分
    let minutes = (timestamp / 60) % 60;

    // 获取当前时间戳的小时数部分
    let hours = (timestamp / 3600) % 24;

    // 获取当前时间戳的天数部分
    let days = timestamp / 86400;
    println!("{}天 {}小时 {}分 {}秒", days, hours, minutes, seconds);
}

fn end_work() {
    println!("结束工作--------------");
    // Command::new("shutdown")
    //     .args(&["-s", "-t", "0"])
    //     .output()
    //     .unwrap();
}

fn start_end_work(duration_secs: u64) {
    wait_with_timer(duration_secs);
    loop {
        if is_timer_finished() {
            end_work();
            break;
        }
    }
}

fn prevent_sleep() {
    // Windows
    #[cfg(windows)]
    Command::new("powercfg")
        .args(&["/SETACTIVE", "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"])
        .output()
        .unwrap();

    // Linux (需要特定的工具如 caffeine)
    #[cfg(unix)]
    Command::new("caffeine")
        .arg("-t")
        .arg("28800") // 8小时
        .output()
        .unwrap();
}

fn main() {
    start_end_work(12);
}
