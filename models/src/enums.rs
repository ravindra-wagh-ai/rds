use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cop {
    Eq,
    Nq,
    Gt,
    Lt,
    Ge,
    Le,
    In,
    Ni,
    Like,
}

impl Cop {
    // Convert Enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            Cop::Eq => "=",
            Cop::Nq => "!=",
            Cop::Gt => ">",
            Cop::Lt => "<",
            Cop::Ge => ">=",
            Cop::Le => "<=",
            Cop::In => "IN",
            Cop::Ni => "NOT IN",
            Cop::Like => "ILIKE",
        }
    }
}

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Lop {
    And,
    Or,
}

impl Lop {
    // Convert Enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            Lop::And => "AND",
            Lop::Or => "OR",
        }
    }
}

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum JoinType {
    Cross,
    Inner,
    Left,
    Right,
    LeftOuter,
    RightOuter,
    FullOuter,
}

impl JoinType {
    // Convert Enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            JoinType::Cross => "CROSS JOIN",
            JoinType::Inner => "INNER JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
            JoinType::LeftOuter => "LEFT OUTER JOIN",
            JoinType::RightOuter => "RIGHT OUTER JOIN",
            JoinType::FullOuter => "FULL OUTER JOIN",
        }
    }
}

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Function {
    AVG,
    COUNT,
    MIN,
    MAX,
    SUM,
}

impl Function {
    // Convert Enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            Function::AVG => "AVG",
            Function::COUNT => "COUNT",
            Function::MIN => "MIN",
            Function::MAX => "MAX",
            Function::SUM => "SUM",
        }
    }
}
