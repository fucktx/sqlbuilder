use crate::error::Error;
use crate::interpolate::{
    clickhouse_interpolate, cql_interpolate, informix_interpolate, mysql_interpolate,
    oracle_interpolate, postgresql_interpolate, presto_interpolate, sqlite_interpolate,
    sqlserver_interpolate,
};
use std::fmt;

// 定义错误类型

// 定义 Flavor 枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flavor {
    InvalidFlavor,
    MySQL,
    PostgreSQL,
    SQLite,
    SQLServer,
    CQL,
    ClickHouse,
    Presto,
    Oracle,
    Informix,
}

impl Flavor {
    // 返回 Flavor 的字符串表示
    pub fn to_string(&self) -> &str {
        match self {
            Flavor::MySQL => "MySQL",
            Flavor::PostgreSQL => "PostgreSQL",
            Flavor::SQLite => "SQLite",
            Flavor::SQLServer => "SQLServer",
            Flavor::CQL => "CQL",
            Flavor::ClickHouse => "ClickHouse",
            Flavor::Presto => "Presto",
            Flavor::Oracle => "Oracle",
            Flavor::Informix => "Informix",
            _ => "<invalid>",
        }
    }

    // 对于不同的数据库类型，返回其相应的插值方法
    pub fn interpolate(&self, sql: &str, args: &[Box<dyn std::fmt::Debug>]) -> Result<String, Err> {
        match self {
            Flavor::MySQL => mysql_interpolate(sql, args),
            Flavor::PostgreSQL => postgresql_interpolate(sql, args),
            Flavor::SQLite => sqlite_interpolate(sql, args),
            Flavor::SQLServer => sqlserver_interpolate(sql, args),
            Flavor::CQL => cql_interpolate(sql, args),
            Flavor::ClickHouse => clickhouse_interpolate(sql, args),
            Flavor::Presto => presto_interpolate(sql, args),
            Flavor::Oracle => oracle_interpolate(sql, args),
            Flavor::Informix => informix_interpolate(sql, args),
            _ => Err(Error::NotImplemented),
        }
    }

    // 创建 CREATE TABLE 构造器
    pub fn new_create_table_builder(&self) -> CreateTableBuilder {
        let mut builder = CreateTableBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 DELETE 构造器
    pub fn new_delete_builder(&self) -> DeleteBuilder {
        let mut builder = DeleteBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 INSERT 构造器
    pub fn new_insert_builder(&self) -> InsertBuilder {
        let mut builder = InsertBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 SELECT 构造器
    pub fn new_select_builder(&self) -> SelectBuilder {
        let mut builder = SelectBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 UPDATE 构造器
    pub fn new_update_builder(&self) -> UpdateBuilder {
        let mut builder = UpdateBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 UNION 构造器
    pub fn new_union_builder(&self) -> UnionBuilder {
        let mut builder = UnionBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 CTE 构造器
    pub fn new_cte_builder(&self) -> CTEBuilder {
        let mut builder = CTEBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 创建 CTE 查询构造器
    pub fn new_cte_query_builder(&self) -> CTEQueryBuilder {
        let mut builder = CTEQueryBuilder::new();
        builder.set_flavor(self.clone());
        builder
    }

    // 为数据库名称添加引号
    pub fn quote(&self, name: &str) -> String {
        match self {
            Flavor::MySQL | Flavor::ClickHouse => format!("`{}`", name),
            Flavor::PostgreSQL
            | Flavor::SQLServer
            | Flavor::SQLite
            | Flavor::Presto
            | Flavor::Oracle
            | Flavor::Informix => format!("\"{}\"", name),
            Flavor::CQL => format!("'{}'", name),
            _ => name.to_string(),
        }
    }

    // 为 INSERT IGNORE 语句准备构造器
    pub fn prepare_insert_ignore(&self, table: &str, ib: &mut InsertBuilder) {
        match self {
            Flavor::MySQL | Flavor::Oracle => {
                ib.set_verb("INSERT IGNORE");
            }
            Flavor::PostgreSQL => {
                ib.set_verb("INSERT");
                ib.add_sql("ON CONFLICT DO NOTHING");
            }
            Flavor::SQLite => {
                ib.set_verb("INSERT OR IGNORE");
            }
            _ => {
                ib.set_verb("INSERT");
            }
        }

        ib.set_table(table);
        ib.reset_marker();
    }
}

// 假设我们有一个创建表的构造器（这个构造器是一个简单的示例）
pub struct CreateTableBuilder {
    flavor: Flavor,
}

impl CreateTableBuilder {
    pub fn new() -> Self {
        CreateTableBuilder {
            flavor: Flavor::InvalidFlavor,
        }
    }

    pub fn set_flavor(&mut self, flavor: Flavor) {
        self.flavor = flavor;
    }
}

// 示例的插入构造器
pub struct InsertBuilder {
    verb: String,
    table: String,
    marker: Option<String>,
}

impl InsertBuilder {
    pub fn new() -> Self {
        InsertBuilder {
            verb: String::from("INSERT"),
            table: String::new(),
            marker: None,
        }
    }

    pub fn set_verb(&mut self, verb: &str) {
        self.verb = verb.to_string();
    }

    pub fn set_table(&mut self, table: &str) {
        self.table = table.to_string();
    }

    pub fn add_sql(&mut self, sql: &str) {
        self.marker = Some(sql.to_string());
    }

    pub fn reset_marker(&mut self) {
        self.marker = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flavor_to_string() {
        assert_eq!(Flavor::MySQL.to_string(), "MySQL");
        assert_eq!(Flavor::PostgreSQL.to_string(), "PostgreSQL");
        assert_eq!(Flavor::SQLite.to_string(), "SQLite");
        assert_eq!(Flavor::InvalidFlavor.to_string(), "<invalid>");
    }

    #[test]
    fn test_quote() {
        let flavor = Flavor::MySQL;
        assert_eq!(flavor.quote("table_name"), "`table_name`");

        let flavor = Flavor::PostgreSQL;
        assert_eq!(flavor.quote("table_name"), "\"table_name\"");
    }

    #[test]
    fn test_interpolate_error_handling() {
        let flavor = Flavor::MySQL;
        let result = flavor.interpolate("SELECT * FROM table WHERE id = ?", &[]);
        assert_eq!(result, Err(InterpolateError::MissingArgs));
    }
}
