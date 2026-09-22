use std::io
use std::fs

fn save_database(table:&Table) -> Result <(), std::io::Error> { 
	let mut output = String::new();
	let mut cant_rows = table.rows.len();
	let mut cant_columns = table.columns.len(); 
	let mut file = File::create("database.db")
	file.write_all(&cant_rows);
	file.write_all(&cant_columns);
	let b_c = cant_columns.to_le_bytes()?;
	let b_r = cant_rows.to_le_bytes()?;
	file.write_all(&b_c);
	file.write_all(&b_r);
		for c in &table.columns {
			let b_n = c.name.as_bytes();
			let n_len = n.len();
			let nb_len = n_len.to_le_bytes();
			let t =	match c.col_type {
				Type::Int => 0,
				Type::Text => 1,
			  	}
			
			file.write_all(&nb_len)?;
			file.write_all(&b_n)?;
			file.write_all(&[t])?;
		}
		for r in &table.rows { 	 
		for v in &r.Values { 
			match v { 
				Value::Int(n) => { 
					let bytes = n.to_le_bytes(); 
					file.write_all(&bytes)?; 
					} 	
				Value::Text(t) => { 
					let bytes = t.as_bytes();
					let t_len = t.len();
					let bt_len = t_len.to_le_bytes();		
					file.write_all(&bt_len)?;
					file.write_all(&bytes)?; 
					} 
		} 
	} 
Ok(())
}

fn load_table(file:&file) -> Result <(), std::io::Error> {
	
}
