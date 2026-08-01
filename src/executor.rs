use crate::types::Command;
use crate::types::Database;
use crate::types::*;

pub fn execute(db: &mut Database, cmd: Command) -> Result<String, String> {
    match cmd {
        Command::CreateTable { name, columns } => {
            if db.tables.contains_key(&name) {
                return Err(format!("Table '{}' already exists", name));
            }
            db.tables.insert(
                name.clone(),
                Table {
                    columns,
                    rows: Vec::new(),
                },
            );
            Ok(format!("Created table '{}'", name))
        }
        Command::Insert { table, values } => {
            let tbl = db
                .tables
                .get_mut(&table)
                .ok_or_else(|| format!("Table '{}' not found", table))?;

            if values.len() != tbl.columns.len() {
                return Err(format!(
                    "Expected {} values, got {}",
                    tbl.columns.len(),
                    values.len()
                ));
            }

            for (i, val) in values.iter().enumerate() {
                match (&tbl.columns[i].col_type, val) {
                    (Type::Text, Value::Text(_)) => {}
                    (Type::Int, Value::Int(_)) => {}
                    (Type::Text, _) => {
                        return Err(format!(
                            "Column '{}' expects TEXT",
                            tbl.columns[i].name
                        ))
                    }
                    (Type::Int, _) => {
                        return Err(format!(
                            "Column '{}' expects INT",
                            tbl.columns[i].name
                        ))
                    }
                }
            }
            let row = Row { values };
            tbl.rows.push(row);
            Ok(format!("Inserted 1 row into '{}'", table))
        }
        Command::Select {
            table,
            columns,
            where_clause,
        } => {
            let tbl = db
                .tables
                .get(&table)
                .ok_or_else(|| format!("Table '{}' not found", table))?;

            let col_indices: Vec<usize> = match columns {
                Some(cols) => cols
                    .iter()
                    .map(|c| {
                        tbl.columns
                            .iter()
                            .position(|col| col.name == *c)
                            .ok_or_else(|| format!("Column '{}' not found", c))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                None => (0..tbl.columns.len()).collect(),
            };
            
            let rows: Vec<&Row> = match where_clause {  // rows contiene referencias a todas las filas que cumplan con las caracterizticas del select por ende alberga &row.
                Some((col, op, val)) => {
                    let col_idx = tbl
                        .columns
                        .iter()
                        .position(|c| c.name == col)
                        .ok_or_else(|| format!("Column '{}' not found", col))?;
                    tbl.rows
                        .iter()
                        .filter(|row| matches_op(&row.values[col_idx], &op, &val))
                        .collect()
                }
                None => tbl.rows.iter().collect(),
            };

            let mut output = String::new();

            for (i, &idx) in col_indices.iter().enumerate() {
                if i > 0 {
                    output.push_str(" | ");
                }
                output.push_str(&tbl.columns[idx].name);
            }
            output.push('\n');

            for (i, &idx) in col_indices.iter().enumerate() {
                if i > 0 {
                    output.push_str("-|-");
                }
                output.push_str(&"-".repeat(tbl.columns[idx].name.len()));
            }
            output.push('\n');

            for row in &rows {
                for (i, &idx) in col_indices.iter().enumerate() {
                    if i > 0 {
                        output.push_str(" | ");
                    }
                    output.push_str(&format!("{}", row.values[idx]));
                }
                output.push('\n');
            }

            output.push_str(&format!("({} rows)", rows.len()));
            Ok(output)
        }
        Command::Delete { table, where_clause } => {
            let tbl = db
                .tables
                .get_mut(&table)
                .ok_or_else(|| format!("Table '{}' not found", table))?;

            let before = tbl.rows.len();
            match where_clause {
                Some((col, op, val)) => {
                    let col_idx = tbl
                        .columns
                        .iter()
                        .position(|c| c.name == col)
                        .ok_or_else(|| format!("Column '{}' not found", col))?;
                    tbl.rows.retain(|row| !matches_op(&row.values[col_idx], &op, &val));
                }
                None => {
                    tbl.rows.clear();
                }
            }
            let deleted = before - tbl.rows.len();
            Ok(format!("Deleted {} rows from '{}'", deleted, table))
        }
        Command::Exit => unreachable!(),
    }
}


fn matches_op(a: &Value, op: &Operator, b: &Value) -> bool {
    match (a, b) {
        (Value::Text(a), Value::Text(b)) => match op {
            Operator::Eq => a == b,
            Operator::Neq => a != b,
            Operator::Gt => a > b,
            Operator::Lt => a < b,
            Operator::Gte => a >= b,
            Operator::Lte => a <= b,
        },
        (Value::Int(a), Value::Int(b)) => match op {
            Operator::Eq => a == b,
            Operator::Neq => a != b,
            Operator::Gt => a > b,
            Operator::Lt => a < b,
            Operator::Gte => a >= b,
            Operator::Lte => a <= b,
        },
        _ => false,
    }
}