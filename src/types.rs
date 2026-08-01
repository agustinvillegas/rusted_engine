use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Int(i64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{}", s),
            Value::Int(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub col_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Text,
    Int,
}

impl std::str::FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TEXT" | "STRING" => Ok(Type::Text),
            "INT" | "INTEGER" => Ok(Type::Int),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Eq, // igual
    Neq, // no igual
    Gt, //  mayor que
    Lt, //menor que
    Gte,// mayor o igual 
    Lte, //mayor o igual        
}

#[derive(Debug, Clone)]
pub enum Command {
    CreateTable { name: String, columns: Vec<Column> },
    Insert { table: String, values: Vec<Value> },
    Select { table: String, columns: Option<Vec<String>>, where_clause: Option<(String, Operator, Value)> },
    Delete { table: String, where_clause: Option<(String, Operator, Value)> },
    Exit,
}

pub struct Row {
    pub values: Vec<Value>,
}
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>, // tablas, con filas y columnas
}

pub struct Database {
    pub tables: HashMap<String, Table>, //la base de datos en si, representada en un hash map, el nombre de la tablaa es la key, la tabla  el value.
}

impl Database {
   pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }
}