use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use win_native_command::shell::win_shutdown;

// 全局计时状态结构体
#[derive(Debug)]
pub struct TimerState {
    pub start_time: Option<Instant>,       // 计时开始时间
    pub target_duration: Option<Duration>, // 计时目标时间
    pub elapsed_time: Duration,            // 当前已使用的时间
    pub is_running: bool,                  // 计时是否正在运行
    pub exit_normally: bool,               // 是否正常退出
}

// 使用静态变量存储全局状态
use crate::utils::win_native_command;
use std::sync::LazyLock;

pub static TIMER_STATE: LazyLock<Arc<Mutex<TimerState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(TimerState {
        start_time: None,
        target_duration: None,
        elapsed_time: Duration::new(0, 0),
        is_running: false,
        exit_normally: true, // 默认正常退出
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
/// 计时分配器 用于计时线程 替换计数时间
pub fn timing_allocation(time: u64, target_time: u64, fn_mode: i32, mode: i32) {
    let state = TIMER_STATE.lock().unwrap();
    if state.is_running {
        drop(state);  // 用完运行检查 锁就没用了 可以释放锁
        match fn_mode {
            // 无执行函数， 执行函数作用范围大于 指定时间 所以指定时间也为 None
            0 => { // 允许 mode in [None, 0, 1]
                start_timer(time);
            }
            1 => {}
            _ => {}
        }
    } else {
        drop(state);  // 注意两个分支都需要释放锁
        match fn_mode {
            // 无执行函数， 执行函数作用范围大于 指定时间 所以指定时间也为 None
            0 => { // 允许 mode in [None, 0, 1]
                wait_with_timer(time, None, None, Option::from(mode));
            }
            1 => {}
            _ => {}
        }
    }
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
pub fn wait_with_timer(
    duration_secs: u64,
    target_time: Option<u64>,
    callback: Option<fn()>,
    mode: Option<i32>,
) {
    start_timer(duration_secs);

    // 在新线程中执行计时
    thread::spawn(move || {
        // let total_duration = Duration::from_secs(duration_secs);
        let check_interval = Duration::from_secs(1); // 每秒更新一次

        loop {
            let state = TIMER_STATE.lock().unwrap();
            let start_time = state.start_time;
            let target_duration = state.target_duration;
            let is_running = state.is_running;
            drop(state);

            if !is_running {
                break;
            }

            if let (Some(start), Some(target)) = (start_time, target_duration) {
                let elapsed = start.elapsed();

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
        // 计时结束
        execute_callback(mode);
    });
}
#[allow(dead_code)]
pub fn get_current_time() -> u64 {
    let state = TIMER_STATE.lock().unwrap();
    if let (Some(start), Some(target)) = (state.start_time, state.target_duration) {
        drop(state);
        let elapsed = start.elapsed();
        let progress = elapsed.as_secs_f64();
        let total = target.as_secs_f64();
        (total - progress) as u64
    } else {
        0.0 as u64
    }
}
#[allow(dead_code)]
pub fn get_fmt_time() -> String {
    let time = get_current_time();
    let hours = (time / 3600) as u64;
    let minutes = ((time % 3600) / 60) as u64;
    let seconds = (time % 60) as u64;
    // 格式化为 "时:分:秒"
    format!("{}:{}:{}", hours, minutes, seconds)
}

/// 获取计时进度百分比
#[allow(dead_code)]
pub fn get_timer_progress_percentage() -> f64 {
    let state = TIMER_STATE.lock().unwrap();
    if let (Some(start), Some(target)) = (state.start_time, state.target_duration) {
        drop(state);
        let elapsed = start.elapsed();
        let progress = elapsed.as_secs_f64();
        let total = target.as_secs_f64();
        progress / total
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
    drop(state);

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
pub fn reset_timer() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.start_time = None;
    state.target_duration = None;
    state.elapsed_time = Duration::new(0, 0);
    state.is_running = false;
}
/// 非正常退出重置计时器
#[allow(dead_code)]
pub fn reset_timer_unnormal() {
    let mut state = TIMER_STATE.lock().unwrap();
    state.start_time = None;
    state.target_duration = None;
    state.elapsed_time = Duration::new(0, 0);
    state.is_running = false;
    state.exit_normally = false;
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

/// 计时结束后执行函数
#[allow(dead_code)]
fn execute_callback(mode: Option<i32>) {
    let mut state = TIMER_STATE.lock().unwrap();
    if state.exit_normally {
        // 执行结束时函数
        match mode {
            //  未获得模式不执行任何操作
            None => {}
            Some(0) => {
                reset_timer();
                win_shutdown();
            }
            Some(1) => {}
            Some(2) => {}
            Some(_) => {}
        }
    } else {
        // 执行异常退出函数
        match mode {
            //  未获得模式不执行任何操作
            None => {}
            Some(0) => {}
            Some(1) => {}
            Some(2) => {}
            Some(_) => {}
        }
        state.exit_normally = true; // 恢复为正常退出状态
    }
}
