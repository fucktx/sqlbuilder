// Copyright 2018 Huan Du. All rights reserved.
// Licensed under the MIT license that can be found in the LICENSE file.

pub mod sqlbuilder {
    const LPAREN: &str = "(";
    const RPAREN: &str = ")";
    const OP_OR: &str = " OR ";
    const OP_AND: &str = " AND ";
    const OP_NOT: &str = "NOT ";

    const MIN_INDEX_BASE: usize = 256;

    // Cond 提供了多个辅助方法来构建条件
    pub struct Cond {
        args: Args,
    }

    impl Cond {
        // NewCond 返回一个新的 Cond。
        pub fn new() -> Self {
            Cond {
                args: Args {
                    index_base: MIN_INDEX_BASE,
                },
            }
        }

        // Equal 用于构建 "field = value" 表达式。
        pub fn equal(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" = ");
                    ctx.write_value(value);
                }),
            })
        }

        // E 是 Equal 的别名。
        pub fn e(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.equal(field, value)
        }

        // EQ 是 Equal 的别名。
        pub fn eq(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.equal(field, value)
        }

        // NotEqual 用于构建 "field <> value" 表达式。
        pub fn not_equal(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" <> ");
                    ctx.write_value(value);
                }),
            })
        }

        // NE 是 NotEqual 的别名。
        pub fn ne(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.not_equal(field, value)
        }

        // NEQ 是 NotEqual 的别名。
        pub fn neq(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.not_equal(field, value)
        }

        // GreaterThan 用于构建 "field > value" 表达式。
        pub fn greater_than(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" > ");
                    ctx.write_value(value);
                }),
            })
        }

        // G 是 GreaterThan 的别名。
        pub fn g(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.greater_than(field, value)
        }

        // GT 是 GreaterThan 的别名。
        pub fn gt(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.greater_than(field, value)
        }

        // GreaterEqualThan 用于构建 "field >= value" 表达式。
        pub fn greater_equal_than(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" >= ");
                    ctx.write_value(value);
                }),
            })
        }

        // GE 是 GreaterEqualThan 的别名。
        pub fn ge(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.greater_equal_than(field, value)
        }

        // GTE 是 GreaterEqualThan 的别名。
        pub fn gte(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.greater_equal_than(field, value)
        }

        // LessThan 用于构建 "field < value" 表达式。
        pub fn less_than(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" < ");
                    ctx.write_value(value);
                }),
            })
        }

        // L 是 LessThan 的别名。
        pub fn l(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.less_than(field, value)
        }

        // LT 是 LessThan 的别名。
        pub fn lt(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.less_than(field, value)
        }

        // LessEqualThan 用于构建 "field <= value" 表达式。
        pub fn less_equal_than(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" <= ");
                    ctx.write_value(value);
                }),
            })
        }

        // LE 是 LessEqualThan 的别名。
        pub fn le(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.less_equal_than(field, value)
        }

        // LTE 是 LessEqualThan 的别名。
        pub fn lte(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            self.less_equal_than(field, value)
        }

        // In 用于构建 "field IN (value...)" 表达式。
        pub fn in_(&self, field: &str, values: &[&dyn std::fmt::Debug]) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" IN (");
                    ctx.write_values(values, ", ");
                    ctx.write_string(")");
                }),
            })
        }

        // NotIn 用于构建 "field NOT IN (value...)" 表达式。
        pub fn not_in(&self, field: &str, values: &[&dyn std::fmt::Debug]) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" NOT IN (");
                    ctx.write_values(values, ", ");
                    ctx.write_string(")");
                }),
            })
        }

        // Like 用于构建 "field LIKE value" 表达式。
        pub fn like(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" LIKE ");
                    ctx.write_value(value);
                }),
            })
        }

        // ILike 用于构建 "field ILIKE value" 表达式。
        pub fn i_like(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string("LOWER(");
                    ctx.write_string(field);
                    ctx.write_string(") LIKE LOWER(");
                    ctx.write_value(value);
                    ctx.write_string(")");
                }),
            })
        }

        // NotLike 用于构建 "field NOT LIKE value" 表达式。
        pub fn not_like(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" NOT LIKE ");
                    ctx.write_value(value);
                }),
            })
        }

        // NotILike 用于构建 "field NOT ILIKE value" 表达式。
        pub fn not_i_like(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string("LOWER(");
                    ctx.write_string(field);
                    ctx.write_string(") NOT LIKE LOWER(");
                    ctx.write_value(value);
                    ctx.write_string(")");
                }),
            })
        }

        // IsNull 用于构建 "field IS NULL" 表达式。
        pub fn is_null(&self, field: &str) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" IS NULL");
                }),
            })
        }

        // IsNotNull 用于构建 "field IS NOT NULL" 表达式。
        pub fn is_not_null(&self, field: &str) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" IS NOT NULL");
                }),
            })
        }

        // Between 用于构建 "field BETWEEN lower AND upper" 表达式。
        pub fn between(
            &self,
            field: &str,
            lower: &dyn std::fmt::Debug,
            upper: &dyn std::fmt::Debug,
        ) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" BETWEEN ");
                    ctx.write_value(lower);
                    ctx.write_string(" AND ");
                    ctx.write_value(upper);
                }),
            })
        }

        // NotBetween 用于构建 "field NOT BETWEEN lower AND upper" 表达式。
        pub fn not_between(
            &self,
            field: &str,
            lower: &dyn std::fmt::Debug,
            upper: &dyn std::fmt::Debug,
        ) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" NOT BETWEEN ");
                    ctx.write_value(lower);
                    ctx.write_string(" AND ");
                    ctx.write_value(upper);
                }),
            })
        }

        // Or 用于构建 "expr1 OR expr2 OR expr3" 的逻辑表达式。
        pub fn or(&self, or_expr: Vec<String>) -> String {
            if or_expr.is_empty() {
                return String::new();
            }

            let expr_byte_len = estimate_strings_bytes(&or_expr);
            if expr_byte_len == 0 {
                return String::new();
            }

            let mut buf = String::new();
            buf.push_str(LPAREN);
            buf.push_str(&or_expr.join(OP_OR));
            buf.push_str(RPAREN);
            buf
        }

        // And 用于构建 "expr1 AND expr2 AND expr3" 的逻辑表达式。
        pub fn and(&self, and_expr: Vec<String>) -> String {
            if and_expr.is_empty() {
                return String::new();
            }

            let expr_byte_len = estimate_strings_bytes(&and_expr);
            if expr_byte_len == 0 {
                return String::new();
            }

            let mut buf = String::new();
            buf.push_str(LPAREN);
            buf.push_str(&and_expr.join(OP_AND));
            buf.push_str(RPAREN);
            buf
        }

        // Not 用于构建 "NOT expr" 表达式。
        pub fn not(&self, not_expr: String) -> String {
            if not_expr.is_empty() {
                return String::new();
            }

            let mut buf = String::new();
            buf.push_str(OP_NOT);
            buf.push_str(&not_expr);
            buf
        }

        // Exists 用于构建 "EXISTS (subquery)" 表达式。
        pub fn exists(&self, subquery: &dyn std::fmt::Debug) -> String {
            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string("EXISTS (");
                    ctx.write_value(subquery);
                    ctx.write_string(")");
                }),
            })
        }

        // NotExists 用于构建 "NOT EXISTS (subquery)" 表达式。
        pub fn not_exists(&self, subquery: &dyn std::fmt::Debug) -> String {
            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string("NOT EXISTS (");
                    ctx.write_value(subquery);
                    ctx.write_string(")");
                }),
            })
        }

        // Any 用于构建 "field op ANY (value...)" 表达式。
        pub fn any(&self, field: &str, op: &str, values: &[&dyn std::fmt::Debug]) -> String {
            if field.is_empty() || op.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" ");
                    ctx.write_string(op);
                    ctx.write_string(" ANY (");
                    ctx.write_values(values, ", ");
                    ctx.write_string(")");
                }),
            })
        }

        // All 用于构建 "field op ALL (value...)" 表达式。
        pub fn all(&self, field: &str, op: &str, values: &[&dyn std::fmt::Debug]) -> String {
            if field.is_empty() || op.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" ");
                    ctx.write_string(op);
                    ctx.write_string(" ALL (");
                    ctx.write_values(values, ", ");
                    ctx.write_string(")");
                }),
            })
        }

        // Some 用于构建 "field op SOME (value...)" 表达式。
        pub fn some(&self, field: &str, op: &str, values: &[&dyn std::fmt::Debug]) -> String {
            if field.is_empty() || op.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" ");
                    ctx.write_string(op);
                    ctx.write_string(" SOME (");
                    ctx.write_values(values, ", ");
                    ctx.write_string(")");
                }),
            })
        }

        // IsDistinctFrom 用于构建 "field IS DISTINCT FROM value" 表达式。
        pub fn is_distinct_from(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" IS DISTINCT FROM ");
                    ctx.write_value(value);
                }),
            })
        }

        // IsNotDistinctFrom 用于构建 "field IS NOT DISTINCT FROM value" 表达式。
        pub fn is_not_distinct_from(&self, field: &str, value: &dyn std::fmt::Debug) -> String {
            if field.is_empty() {
                return String::new();
            }

            self.var(CondBuilder {
                builder: Box::new(move |ctx| {
                    ctx.write_string(field);
                    ctx.write_string(" IS NOT DISTINCT FROM ");
                    ctx.write_value(value);
                }),
            })
        }

        // Var 返回一个值的占位符。
        fn var(&self, builder: CondBuilder) -> String {
            self.args.add(builder)
        }
    }

    pub struct Args {
        index_base: usize,
    }

    impl Args {
        pub fn add(&self, builder: CondBuilder) -> String {
            // 这里是占位符代码
            "placeholder".to_string()
        }
    }

    pub struct CondBuilder {
        pub builder: Box<dyn Fn(&mut ArgsCompileContext)>,
    }

    pub struct ArgsCompileContext {
        flavor: String,
    }

    impl ArgsCompileContext {
        pub fn write_string(&mut self, s: &str) {
            // 写入字符串的实现
        }

        pub fn write_value(&mut self, value: &dyn std::fmt::Debug) {
            // 写入值的实现
        }

        pub fn write_values(&mut self, values: &[&dyn std::fmt::Debug], separator: &str) {
            // 写入多个值的实现
        }
    }

    fn estimate_strings_bytes(strs: &[String]) -> usize {
        strs.iter().map(|s| s.len()).sum()
    }
}
