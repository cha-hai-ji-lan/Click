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

/// **等待指定时间（在后台运行）**
///
/// `duration_secs` : 需要计时的时间
///
/// `target_time` : 执行某操作的时间点 <可选>
///
/// `callback`: 某操作的回调函数 <可选>
///
/// `mode`:执行模式 <可选>
/// > None: 不执行任何操作
/// >
/// > 0: 关闭计算机
/// >
/// > 1: 阻止计算机进入休眠 -> 关闭计算机
/// >
/// > 2: 执行 `callback` 函数
/// >
/// > 3: 执行 `callback` 函数 -> 关闭计算机
/// >
/// > 4: 阻止计算机进入休眠 -> 执行 `callback` 函数 -> 关闭计算机
/// >
/// > 5: 检查有无到达`target_time` -> 执行 `callback` 函数
/// >
/// > 6: 检查有无到达`target_time` -> 执行 `callback` 函数 --结束计时--> 阻止计算机进入休眠 -> 关闭计算机
/// >
/// > 7: 阻止计算机进入休眠 -> 检查有无到达`target_time` -> 执行 `callback` 函数 --结束计时--> 关闭计算机
#[allow(dead_code)]
pub fn wait_with_timer(duration_secs: u64, target_time: Option<u64>, callback:  Option<fn()>, mode:Option<i32>) {
    // 启动计时器
    start_timer(duration_secs);

    // 在新线程中执行计时
    thread::spawn(move || {
        let total_duration = Duration::from_secs(duration_secs);
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
                        // 执行结束时函数
                        match mode {
                            //  未获得模式不执行任何操作
                            None => {
                                break;
                            }
                            Some(1) => {
                                break;
                            }
                            Some(2) => {
                                break;
                            }
                            Some(_) => {
                                break;
                            }
                        }
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
#[allow(dead_code)]
pub fn get_current_time() -> Duration {
    let state = TIMER_STATE.lock().unwrap();
    state.elapsed_time
}

/// 获取计时进度百分比
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn stop_timer() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.is_running = false;
}

/// 重置计时器
#[allow(dead_code)]
///
pub fn reset_timer() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.start_time = None;
    state.target_duration = None;
    state.elapsed_time = Duration::new(0, 0);
    state.is_running = false;
}

/// 阻止计算机进入休眠
#[allow(dead_code)]
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
