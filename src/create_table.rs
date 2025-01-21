use std::collections::VecDeque;

// 定义常量枚举，用于标记 SQL 构建的不同阶段
#[derive(Clone, Copy, PartialEq, Eq)]
enum InjectionMarker {
    Init,
    AfterCreate,
    AfterDefine,
    AfterOption,
}

// CreateTableBuilder 是用于构建 CREATE TABLE 语句的结构体
struct CreateTableBuilder {
    verb: String,
    if_not_exists: bool,
    table: String,
    defs: Vec<Vec<String>>,
    options: Vec<Vec<String>>,
    args: Args,
    injection: Injection,
    marker: InjectionMarker,
}

impl CreateTableBuilder {
    // 创建一个新的 CreateTableBuilder 实例
    fn new() -> Self {
        Self {
            verb: "CREATE TABLE".to_string(),
            if_not_exists: false,
            table: String::new(),
            defs: Vec::new(),
            options: Vec::new(),
            args: Args::new(),
            injection: Injection::new(),
            marker: InjectionMarker::Init,
        }
    }

    // 设置表名
    fn create_table(&mut self, table: &str) -> &mut Self {
        self.table = escape(table);
        self.marker = InjectionMarker::AfterCreate;
        self
    }

    // 设置临时表名
    fn create_temp_table(&mut self, table: &str) -> &mut Self {
        self.verb = "CREATE TEMPORARY TABLE".to_string();
        self.table = escape(table);
        self.marker = InjectionMarker::AfterCreate;
        self
    }

    // 添加 IF NOT EXISTS 子句
    fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    // 添加列或索引定义
    fn define(&mut self, def: Vec<String>) -> &mut Self {
        self.defs.push(def);
        self.marker = InjectionMarker::AfterDefine;
        self
    }

    // 添加表选项
    fn option(&mut self, opt: Vec<String>) -> &mut Self {
        self.options.push(opt);
        self.marker = InjectionMarker::AfterOption;
        self
    }

    // 返回定义的数量
    fn num_define(&self) -> usize {
        self.defs.len()
    }

    // 返回编译后的 SQL 字符串
    fn string(&self) -> String {
        self.build().0
    }

    // 编译并返回 SQL 字符串和参数
    fn build(&self) -> (String, Vec<Box<dyn std::any::Any>>) {
        self.build_with_flavor(self.args.flavor.clone(), Vec::new())
    }

    // 使用指定的 Flavor 编译 SQL 字符串和参数
    fn build_with_flavor(
        &self,
        flavor: Flavor,
        initial_arg: Vec<Box<dyn std::any::Any>>,
    ) -> (String, Vec<Box<dyn std::any::Any>>) {
        let mut buf = StringBuilder::new();
        self.injection.write_to(&mut buf, InjectionMarker::Init);

        if !self.verb.is_empty() {
            buf.write_leading_string(&self.verb);
        }

        if self.if_not_exists {
            buf.write_leading_string("IF NOT EXISTS");
        }

        if !self.table.is_empty() {
            buf.write_leading_string(&self.table);
        }

        self.injection.write_to(&mut buf, InjectionMarker::AfterCreate);

        if !self.defs.is_empty() {
            buf.write_leading_string("(");

            let defs: Vec<String> = self.defs.iter().map(|def| def.join(" ")).collect();
            buf.write_strings(&defs, ", ");
            buf.write_char(')');

            self.injection.write_to(&mut buf, InjectionMarker::AfterDefine);
        }

        if !self.options.is_empty() {
            let opts: Vec<String> = self.options.iter().map(|opt| opt.join(" ")).collect();
            buf.write_leading_string(&opts.join(", "));
            self.injection.write_to(&mut buf, InjectionMarker::AfterOption);
        }

        self.args.compile_with_flavor(&buf.to_string(), flavor, initial_arg)
    }

    // 设置 Flavor
    fn set_flavor(&mut self, flavor: Flavor) -> Flavor {
        let old = self.args.flavor.clone();
        self.args.flavor = flavor;
        old
    }

    // 返回当前的 Flavor
    fn flavor(&self) -> &Flavor {
        &self.args.flavor
    }

    // 添加参数并返回占位符
    fn var(&mut self, arg: Box<dyn std::any::Any>) -> String {
        self.args.add(arg)
    }

    // 添加任意 SQL 片段
    fn sql(&mut self, sql: &str) -> &mut Self {
        self.injection.sql(self.marker, sql);
        self
    }
}

// Args 结构体，用于管理 SQL 参数
struct Args {
    flavor: Flavor,
    // 其他字段和方法
}

impl Args {
    fn new() -> Self {
        Self {
            flavor: Flavor::Default,
        }
    }

    // 使用指定的 Flavor 编译 SQL
    fn compile_with_flavor(
        &self,
        sql: &str,
        flavor: Flavor,
        initial_arg: Vec<Box<dyn std::any::Any>>,
    ) -> (String, Vec<Box<dyn std::any::Any>>) {
        // 这里实现具体的编译逻辑
        (sql.to_string(), initial_arg)
    }

    // 添加参数并返回占位符
    fn add(&mut self, arg: Box<dyn std::any::Any>) -> String {
        // 这里实现占位符逻辑
        "?".to_string()
    }
}

// Injection 结构体，用于管理 SQL 注入
struct Injection {
    // 字段和方法
}

impl Injection {
    fn new() -> Self {
        Self {
            // 初始化
        }
    }

    // 将注入内容写入 StringBuilder
    fn write_to(&self, buf: &mut StringBuilder, marker: InjectionMarker) {
        // 实现注入逻辑
    }

    // 添加 SQL 片段
    fn sql(&mut self, marker: InjectionMarker, sql: &str) {
        // 实现 SQL 注入逻辑
    }
}

// Flavor 枚举，用于表示 SQL 方言
#[derive(Clone)]
enum Flavor {
    Default,
    // 其他方言
}

// StringBuilder 结构体，用于构建 SQL 字符串
struct StringBuilder {
    buffer: VecDeque<String>,
}

impl StringBuilder {
    fn new() -> Self {
        StringBuilder {
            buffer: VecDeque::new(),
        }
    }

    // 添加前导字符串
    fn write_leading_string(&mut self, s: &str) {
        self.buffer.push_back(s.to_string());
    }

    // 添加多个字符串，用分隔符连接
    fn write_strings(&mut self, strings: &[String], sep: &str) {
        let joined = strings.join(sep);
        self.buffer.push_back(joined);
    }

    // 添加字符
    fn write_char(&mut self, c: char) {
        self.buffer.push_back(c.to_string());
    }

    // 将缓冲区内容转换为字符串
    fn to_string(&self) -> String {
        self.buffer.iter().map(|s| s.as_str()).collect()
    }
}

// 辅助函数：转义字符串
fn escape(s: &str) -> String {
    // 实现转义逻辑
    s.to_string()
}