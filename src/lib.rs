mod lib_office;
mod lib_key_words;

use pyo3::prelude::*;
use rand::Rng;
use std::process::Command;
use std::time::Duration;
use nvml_wrapper::Nvml;
use sysinfo::{ProcessesToUpdate, System};
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::PROCESS_TERMINATE;

/// 咔嗒主屏轮播句选择器
#[pyfunction]
fn info() -> PyResult<String> {
    let info: Vec<&str> = vec![
        "钟表可以回到起点，但早已不是昨天。", "生命的很多事，你错过一小时，就错过一生了",
        "也许我们该追随黄昏而去。", "总要热爱点什么不能被这无趣的生活吞没", "时光知味，岁月沉香",
        "一个人只有偏执地相信自已的选择，才能过好这一生。", "逝去的人躲在云里，下雨时再来看你。",
        "你觉得你现在的时间过的快吗？", "泛黄的笔记本记载着太多故事。",
        "微光散落在窗前，你可会想起往事？", "不知你是否见过我的过去", "原来那件事已经过去许久了。",
        "我喜欢雨天，天地融为一体", "留恋的过去有意义吗",
        "若不趁起风时扬帆，船是不会前进的。", "只有流过血的手指，才能弹出世间的绝响。", "抱怨身处黑暗，不如提灯前行。",
        "从南到北，春来秋往。", "我们哭着降临世界，却可以笑着走向永恒。", "坐听弦诵声，细嗅杏坛香。",
        "好戏存烟火，且慢且从容。", "岁月流转。流年偷换。", "人生最可贵的事情，便是少年的迷茫。",
        "安逸不进取，卒落井底蛙。", "见贤不思齐， 终成栋中雀。", "许多人都自诩有志之士却鲜见行动之时",
        "岁月悠悠，衰微只及肌肤", "热枕抛却，颓废必致灵魂", "艰难方显勇毅，磨砺始得玉成。", "旧物被时间刻下了记忆。",
        "你一生追求的东西其实一开始就在",
    ];
    let random_number = rand::rng().random_range(1..info.len());
    Ok(info[random_number].to_string())
}
/// 咔嗒首次开机轮播提示
#[pyfunction]
fn preload_txt(index: usize) -> &'static str {
    let text = vec![
        "您可以回复“关于”查看我具备的能力", "以胜任您的PC资源生命周期管理",
        "我会不断提升自己的能力", "你好我是咔嗒，很高兴见到你"
    ];
    text[index]
}


/// 咔嗒2.0.0版本更新介绍
#[pyfunction]
fn click_about(index: usize) -> PyResult<String> {
    let about_info: Vec<&str> = vec![
        r#"<h1>咔嗒 2.0.0</h1>"#,  // 0
        r#"<a>https:https://github.com/cha-hai-ji-lan/Click</a>"#,  // 1
        r#"相较于 1.0.0 版本，加入以下功能："#,  // 2
        r#"1. 添加了 GPU 资源占用信息获取功能"#,  // 3
        r#"2. 添加了 CPU 进程数获取功能"#,  // 4
        r#"3. 添加了 CPU 资源占用量获取功能"#,  // 5
        r#"4. 可以监控资源占比波动来控制计算机开关"#,  // 6
        r#"5. 为面向PC资源的生命周期管理提供了基础功能"#,  // 7
        r#"&nbsp;&nbsp;&nbsp;(1).添加了类HTML的Emmet语法来批量生成文件以及可嵌套文件夹"#,  // 8
        r#"&nbsp;&nbsp;&nbsp;(2).提供了Explorer.exe聚焦路径监控可以获取当前聚焦的资源路径"#,  // 9
        r#"&nbsp;&nbsp;&nbsp;(3).添加了自启动功能"#,  // 10
        r#"        {1}.添加了辅助自启动功能对于一些未提供自启动功能的辅助软件"#,  // 11
        r#"可以通过ClickSever来辅助开机自启动"#,  // 12
        r#"                                     ————茶海霁澜敬上"#]; // 14
    Ok(about_info[index].to_string())
}

/// GPU资源占用信息获取
#[pyfunction]
fn gpu_info(py: Python<'_>) -> PyResult<(String, String, String, String)> {
    py.allow_threads(move || {
        let output = Command::new("nvidia-smi")
            .arg("--query-gpu=index,name,utilization.gpu,utilization.memory")
            .arg("--format=csv")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let result = result.to_string();
            let nvidia_gpu_info = result.split("\r\n").collect::<Vec<_>>();
            let nvidia_gpu_info = &nvidia_gpu_info[1].split(",").collect::<Vec<_>>();
            Ok(
                (nvidia_gpu_info[0].to_string(),  //  GPU索引
                 nvidia_gpu_info[1][16..nvidia_gpu_info[1].len()].to_string(),  // GPU名称
                 nvidia_gpu_info[2][1..nvidia_gpu_info[2].len() - 2].to_string(),
                 nvidia_gpu_info[3][1..nvidia_gpu_info[3].len() - 2].to_string())
            )
        } else {
            Ok((String::from("0"), String::from(""),
                String::from("0.0"), String::from("0.0")))
        }
    })
}


/// CPU进程数获取
/// 返回当前全部进程数
#[pyfunction]
fn get_cpu_handle() -> PyResult<usize> {
    // 创建一个系统句柄
    let mut handle = System::new_all();
    // 刷新一次所有信息
    handle.refresh_all();
    Ok(handle.processes().len())
}

///  CPU资源占用量获取
///
#[pyfunction]
fn cpu_info(py: Python<'_>) -> PyResult<Vec<(String, String, String, String)>> {
    py.allow_threads(move || {
        // 手动释放 GIL 锁
        // 保证 线程运行时不会影响Python主线程GUI
        let mut system = System::new_all();
        system.refresh_all();
        std::thread::sleep(Duration::from_secs(1)); // 等待1秒以获取使用率变化
        system.refresh_all(); // 第二次刷新
        let mut cpu_data_tup: Vec<(String, String, String, String)> = vec![];

        for (pid, process) in system.processes() {
            cpu_data_tup.push((
                pid.to_string(),
                process.name().to_str().unwrap().to_string(),
                process.cpu_usage().to_string(),
                (process.memory() as f32 / 1024.0).to_string(),
            ));
        }
        cpu_data_tup.sort_by(|a, b| b
            .2
            .partial_cmp(&a.2)
            .unwrap_or(std::cmp::Ordering::Equal));

        Ok(cpu_data_tup)
    })
}

#[pyfunction]
fn cpu_memory_info(py: Python<'_>) -> PyResult<Vec<(String, String, String, String)>> {
    py.allow_threads(move || {  // 手动释放 GIL 锁
        let mut system = System::new_all();
        system.refresh_all();
        std::thread::sleep(Duration::from_secs(1)); // 等待1秒以获取使用率变化
        system.refresh_all(); // 第二次刷新
        let mut cpu_data: Vec<CpuInfo> = vec![];
        let mut cpu_data_tup: Vec<(String, String, String, String)> = vec![];

        for (pid, process) in system.processes() {
            cpu_data_tup.push((
                pid.to_string(),
                process.name().to_str().unwrap().to_string(),
                process.cpu_usage().to_string(),
                process.memory().to_string(),
            ));
        }
        cpu_data_tup.sort_by(|a, b| b
            .3
            .partial_cmp(&a.3)
            .unwrap_or(std::cmp::Ordering::Equal));

        Ok(cpu_data_tup)
    })
}

#[pyfunction]
fn kill_process(pid: u32) -> PyResult<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return Ok("无法打开进程".to_string());
        }

        if TerminateProcess(handle, 1) == 0 {
            return Ok("无法终止进程".to_string());
        }

        CloseHandle(handle);
        Ok(String::from(pid.to_string() + "进程已终止"))
    }
}


#[pyclass]
#[derive(Debug)]
struct CpuInfo {
    #[pyo3(get)]
    cpu_pid: u32,
    #[pyo3(get)]
    cpu_name: String,
    #[pyo3(get)]
    cpu_usage: f32,
    #[pyo3(get)]
    cpu_memory_usage: u64,
}

impl CpuInfo {
    fn new(cpu_pid: u32, cpu_name: String, cpu_usage: f32, cpu_memory_usage: u64) -> Self {
        Self {
            cpu_pid,
            cpu_name,
            cpu_usage,
            cpu_memory_usage,
        }
    }
}

fn nvidia_gpu_info() -> () {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap(); // 首张 GPU
    let processes = device.running_graphics_processes().unwrap();
    for proc in processes {
        println!("PID: {},进程名称:{}, GPU实例ID: {:?} 计算实例ID: {:?}  显存: {:?}",
                 proc.pid, get_process_name(proc.pid), proc.gpu_instance_id,
                 proc.compute_instance_id, proc.used_gpu_memory);
    };
}

#[test]
fn test_fn() {
     nvidia_gpu_info();
}

fn get_process_name(pid: u32) -> String {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, false); // 刷新进程信息

    // 将 PID 转换为 sysinfo 的 Pid 类型
    let sysinfo_pid = sysinfo::Pid::from(pid as usize);


    sys.process(sysinfo_pid)
        .map(|process| process
            .name()
            .to_str()
            .unwrap()
            .to_string())
        .unwrap_or_else(|| String::from("未知进程"))
}

#[test]
fn main() {
    nvidia_gpu_info()
}
/// 咔嗒主屏提示字模块
/// 咔嗒CPU, GPU性能监测模块
#[pymodule]
fn click_rust_depends(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CpuInfo>()?; // 注册 CpuInfo 类型
    m.add_function(wrap_pyfunction!(gpu_info, m)?)?;
    m.add_function(wrap_pyfunction!(preload_txt, m)?)?;
    m.add_function(wrap_pyfunction!(get_cpu_handle, m)?)?;
    m.add_function(wrap_pyfunction!(cpu_info, m)?)?;
    m.add_function(wrap_pyfunction!(cpu_memory_info, m)?)?;
    m.add_function(wrap_pyfunction!(kill_process, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(click_about, m)?)?;
    Ok(())
}
