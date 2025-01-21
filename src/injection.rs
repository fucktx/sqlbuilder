use std::collections::HashMap;
use crate::stringbuilder::StringBuilder;

// 定义 injectionMarker 类型
type InjectionMarker = i32;

// injection 结构体，用于管理 SQL 注入片段
struct Injection {
    marker_sql: HashMap<InjectionMarker, Vec<String>>,
}

impl Injection {
    // 创建一个新的 Injection 实例
    fn new() -> Self {
        Self {
            marker_sql: HashMap::new(),
        }
    }

    // 添加 SQL 片段到指定标记的位置
    fn sql(&mut self, marker: InjectionMarker, sql: String) {
        let sqls = self.marker_sql.entry(marker).or_insert_with(Vec::new);
        sqls.push(sql);
    }

    // 将指定标记的 SQL 片段写入 StringBuilder
    fn write_to(&self, buf: &mut StringBuilder, marker: InjectionMarker) {
        if let Some(sql) = self.marker_sql.get(&marker) {
            if !sql.is_empty() {
                buf.write_leading_string("");
                buf.write_strings(sql, " ");
            }
        }
    }
}

// StringBuilder 结构体，用于构建 SQL 字符串
