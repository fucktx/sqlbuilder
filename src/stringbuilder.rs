use std::io::{self, Write};

// stringBuilder 结构体，用于构建字符串
pub struct StringBuilder {
    buffer: String,
}

impl StringBuilder {
    // 创建一个新的 StringBuilder 实例
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    // 写入前导字符串
    // 如果缓冲区不为空，则在写入前添加一个空格
    pub(crate) fn write_leading_string(&mut self, s: &str) {
        if !self.buffer.is_empty() {
            self.buffer.push(' ');
        }
        self.buffer.push_str(s);
    }

    // 写入字符串
    fn write_string(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    // 写入多个字符串，用分隔符连接
    pub(crate) fn write_strings(&mut self, ss: &[String], sep: &str) {
        if ss.is_empty() {
            return;
        }

        let mut first_added = false;

        // 写入第一个非空字符串
        if !ss[0].is_empty() {
            self.write_string(&ss[0]);
            first_added = true;
        }

        // 写入剩余字符串
        for s in &ss[1..] {
            if !s.is_empty() {
                if first_added {
                    self.write_string(sep);
                }
                self.write_string(s);
                first_added = true;
            }
        }
    }

    // 写入一个字符
    fn write_rune(&mut self, r: char) {
        self.buffer.push(r);
    }

    // 实现 io::Write trait，用于写入字节数据
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(data);
        self.buffer.push_str(&s);
        Ok(data.len())
    }

    // 返回缓冲区内容
    fn to_string(&self) -> &str {
        &self.buffer
    }

    // 重置缓冲区
    fn reset(&mut self) {
        self.buffer.clear();
    }

    // 预分配缓冲区容量
    // 扩展容量
    fn grow(&mut self, n: usize) {
        if n > self.buffer.capacity() - self.buffer.len() {
            let new_capacity = 2 * self.buffer.capacity() + n;
            self.buffer.reserve(new_capacity - self.buffer.len());
        }
    }
}
