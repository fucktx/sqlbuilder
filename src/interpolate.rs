use std::fmt;
use std::str;
use crate::error::Error;

// 定义 Flavor 枚举，表示不同的 SQL 方言
#[derive(Clone, Copy, PartialEq, Eq)]
enum Flavor {
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



// 实现 std::error::Error 以便错误可以集成到 Rust 的错误处理系统

// 定义 mysql_interpolate 函数
pub fn mysql_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    mysql_like_interpolate(Flavor::MySQL, query, args)
}

// 定义 mysql_like_interpolate 函数
pub fn mysql_like_interpolate(flavor: Flavor, query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    let mut buf = String::with_capacity(query.len() + args.len() * 20);
    let mut cnt = 0;
    let max = args.len();
    let mut escaping = false;
    let mut quote = None;
    let mut chars = query.chars().peekable();

    while let Some(r) = chars.next() {
        if escaping {
            escaping = false;
            buf.push(r);
            continue;
        }

        match r {
            '?' => {
                if quote.is_none() {
                    if cnt >= max {
                        return Err(Error::MissingArgs);
                    }
                    encode_value(&mut buf, args[cnt], flavor)?;
                    cnt += 1;
                } else {
                    buf.push(r);
                }
            }
            '\'' | '"' | '`' => {
                if quote == Some(r) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(r);
                }
                buf.push(r);
            }
            '\\' => {
                if quote.is_some() {
                    escaping = true;
                }
                buf.push(r);
            }
            _ => buf.push(r),
        }
    }

    Ok(buf)
}

// 定义 encode_value 函数
fn encode_value(buf: &mut String, arg: &dyn fmt::Display, flavor: Flavor) -> Result<(), Error> {
    match flavor {
        Flavor::MySQL | Flavor::SQLite | Flavor::CQL | Flavor::ClickHouse | Flavor::Presto | Flavor::Informix => {
            write!(buf, "{}", arg).map_err(|_| Error::InvalidUtf8)
        }
        Flavor::PostgreSQL => {
            write!(buf, "${}", arg).map_err(|_| Error::InvalidUtf8)
        }
        Flavor::SQLServer => {
            write!(buf, "@p{}", arg).map_err(|_| Error::InvalidUtf8)
        }
        Flavor::Oracle => {
            write!(buf, ":{}", arg).map_err(|_| Error::InvalidUtf8)
        }
    }
}

// 定义 postgresql_interpolate 函数
pub fn postgresql_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    let mut buf = String::with_capacity(query.len() + args.len() * 20);
    let mut cnt = 0;
    let max = args.len();
    let mut escaping = false;
    let mut quote = None;
    let mut dollar_quote = None;
    let mut chars = query.chars().peekable();

    while let Some(r) = chars.next() {
        if escaping {
            escaping = false;
            buf.push(r);
            continue;
        }

        match r {
            '$' => {
                if quote.is_none() {
                    let mut placeholder = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            placeholder.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if !placeholder.is_empty() {
                        let idx: usize = placeholder.parse().map_err(|_| Error::UnsupportedArgs)?;
                        if idx > max {
                            return Err(Error::MissingArgs);
                        }
                        encode_value(&mut buf, args[idx - 1], Flavor::PostgreSQL)?;
                    } else {
                        buf.push(r);
                    }
                } else {
                    buf.push(r);
                }
            }
            '\'' | '"' => {
                if quote == Some(r) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(r);
                }
                buf.push(r);
            }
            '\\' => {
                if quote.is_some() {
                    escaping = true;
                }
                buf.push(r);
            }
            _ => buf.push(r),
        }
    }

    Ok(buf)
}

// 定义 sqlserver_interpolate 函数
pub fn sqlserver_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    let mut buf = String::with_capacity(query.len() + args.len() * 20);
    let mut cnt = 0;
    let max = args.len();
    let mut escaping = false;
    let mut quote = None;
    let mut chars = query.chars().peekable();

    while let Some(r) = chars.next() {
        if escaping {
            escaping = false;
            buf.push(r);
            continue;
        }

        match r {
            '@' => {
                if quote.is_none() {
                    let mut placeholder = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            placeholder.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if !placeholder.is_empty() {
                        let idx: usize = placeholder.parse().map_err(|_| Error::UnsupportedArgs)?;
                        if idx > max {
                            return Err(Error::MissingArgs);
                        }
                        encode_value(&mut buf, args[idx - 1], Flavor::SQLServer)?;
                    } else {
                        buf.push(r);
                    }
                } else {
                    buf.push(r);
                }
            }
            '\'' | '"' => {
                if quote == Some(r) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(r);
                }
                buf.push(r);
            }
            '\\' => {
                if quote.is_some() {
                    escaping = true;
                }
                buf.push(r);
            }
            _ => buf.push(r),
        }
    }

    Ok(buf)
}

// 定义 oracle_interpolate 函数
pub fn oracle_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    let mut buf = String::with_capacity(query.len() + args.len() * 20);
    let mut cnt = 0;
    let max = args.len();
    let mut escaping = false;
    let mut quote = None;
    let mut dollar_quote = None;
    let mut chars = query.chars().peekable();

    while let Some(r) = chars.next() {
        if escaping {
            escaping = false;
            buf.push(r);
            continue;
        }

        match r {
            ':' => {
                if quote.is_none() {
                    let mut placeholder = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            placeholder.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if !placeholder.is_empty() {
                        let idx: usize = placeholder.parse().map_err(|_| Error::UnsupportedArgs)?;
                        if idx > max {
                            return Err(Error::MissingArgs);
                        }
                        encode_value(&mut buf, args[idx - 1], Flavor::Oracle)?;
                    } else {
                        buf.push(r);
                    }
                } else {
                    buf.push(r);
                }
            }
            '\'' | '"' => {
                if quote == Some(r) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(r);
                }
                buf.push(r);
            }
            '\\' => {
                if quote.is_some() {
                    escaping = true;
                }
                buf.push(r);
            }
            _ => buf.push(r),
        }
    }

    Ok(buf)
}


// 定义 sqlite_interpolate 函数
pub fn sqlite_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    mysql_like_interpolate(Flavor::SQLite, query, args)
}

// 定义 cql_interpolate 函数
pub fn cql_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    mysql_like_interpolate(Flavor::CQL, query, args)
}

// 定义 clickhouse_interpolate 函数
pub fn clickhouse_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Error> {
    mysql_like_interpolate(Flavor::ClickHouse, query, args)
}

// 定义 presto_interpolate 函数
pub fn presto_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Err> {
    mysql_like_interpolate(Flavor::Presto, query, args)
}

// 定义 informix_interpolate 函数
pub fn informix_interpolate(query: &str, args: &[&dyn fmt::Display]) -> Result<String, Err> {
    mysql_like_interpolate(Flavor::Informix, query, args)
}

// // 示例用法
// fn main() {
//     let query = "SELECT * FROM users WHERE id = ? AND name = ?";
//     let args: Vec<&dyn fmt::Display> = vec![&1, &"Alice"];
//
//     match sqlite_interpolate(query, &args) {
//         Ok(sql) => println!("{}", sql),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }