use std::io
use std::fs

fn save_database(table:&table) -> Result <(), std::io::Error> { 
	let mut output = String::new();
	let mut cant_rows = table.rows.len();  //i32 osea 4bytes
	let mut cant_columns = table.columns.len(); //mismo
	file.write_all(&cant_rows);
	file.write_all(&cant_columns);
	let mut file = File::create("database.db")
		b_c = cant_columns.to_le_bytes();
		b_r = cant_rows.to_le_bytes();
		file.write_all(&b_c);
		file.write_all(&b_r);
		for c in table.columns {
			n=c.name;
			t=c.type;
			file.write_all(&n);
			file.write_all(&t);
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
					file.write_all(&bytes)?; 
					} 
		} 
	} 
Ok(())
}

fn load_table(file:&file) -> Result <(), std::io::Error> {
	
}
