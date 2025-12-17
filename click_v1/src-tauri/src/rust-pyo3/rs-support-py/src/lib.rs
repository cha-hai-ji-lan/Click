/// 暂时搁置未使用
use lazy_static::lazy_static;
use pyo3::prelude::*;
use std::collections::HashMap;


type InnerMap = HashMap<String, String>; // 内部值类型

enum HashValue {
    String(String),
    InnerMap(InnerMap),
}

// 定义嵌套 HashMap 结构 只读不修改，无需线程锁
lazy_static! {
    static ref NESTED_MAP: HashMap<String, HashValue> = {
        let mut map:HashMap<String, HashValue> = HashMap::new();
        map.insert(r"::{645FF040-5081-101B-9F08-00AA002F954E}".to_string(), HashValue::String(r"回收站:\".to_string()));
        map
    };
}
/// 将两个数字的和格式化为字符串。
#[pyfunction]
fn sum_as_string(guid: &str) -> PyResult<String> {

    Ok(format!("{}", guid))
}

/// 一个用Rust实现的Python模块。
#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}


