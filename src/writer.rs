use std::io
use std::fs
use create::types::*;

fn save_database(table:&Table) -> Result <(), std::io::Error> { 
	let cant_rows = table.rows.len();
	let cant_columns = table.columns.len(); 
	let mut file = File::create("database.db");
	let b_c = cant_columns.to_le_bytes();
	let b_r = cant_rows.to_le_bytes();
	file.write_all(&b_c)?;
	file.write_all(&b_r)?;
		for c in &table.columns {
			let b_n = c.name.as_bytes();
			let n_len = c.name.len();
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
		for v in &r.values { 
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

fn load_table(file:&mut File) -> Result <Table, std::io::Error> {
	let mut file = file;
	let mut buffer = [0u8;4;]
	file.read_exact(&mut buffer)?;
	c_c = u32::from_le_bytes(buffer);
	file.read_exact(&mut buffer)?;
	c_r = u32::from_le_bytes(buffer);
		for i in 0..c_c {
			file.read_exact(&mut buffer)?;
			c_nlen = u32::from_le_bytes(buffer);
			n_buffer:u32 = !vec[0u8; c_nlen] as usize;
			file.read_exact(&mut n_buffer)?;
			c_n = String::from_utf8(n_buffer)?;
			file.read_exact(&mut buffer)?;
			c_t = u32::from_le_bytes(buffer);	
			let mut columns:Vec<Column> = vec::new(); 
			let column: Column = {name: c_n, col_type: c_t};
			columns.push(column);
				}
		for i in 0..c_r {
		let mut rows:Vec<Row> = Vec::new();
   		for i in 0..c_c {
			match c.col_type {
				Type::Int => {
					let mut buffer = [0u8;4];
					file.read_exact(&mut buffer);
					let n:u32 = u32::from_le_bytes(buffer?);
					rows.push(n);
						}
				Type::Text => {
					file.read_exact(&mut buffer)?;
					let t_len = u32::from_le_bytes(buffer);
					let mut t_buffer = [0u8;&t_len]; 
					let t: String = file.read_exact(t_buffer)?;
					rows.push(t);		
						}
					}
		


				}	 
			}
		let table = Table {
		 columns: columns, 
		 rows: row,
			};
		Ok(table)
	}

