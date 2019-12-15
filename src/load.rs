use cli::{Table, TableRecord, TableVersion};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::io::Error;

pub fn create_table(table_map: &HashMap<String, Table>) -> Result<(), Error>{
    let acs_schema = "acs_data"; // get from param
    let database = "datawheel"; // get from param
    let username = "postgres"; // get from param
    let schema_str = format!("CREATE SCHEMA IF NOT EXISTS {}", acs_schema);
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &schema_str]).status()?;
    for (table_id, table) in table_map.iter(){
        let table_query = format!("CREATE TABLE {}.{} (column_id TEXT, column_name TEXT)", acs_schema, table.table_id);
        let status = Command::new("psql").args(&["-d", database, "-U", username, "-c", &table_query]).status()?;
        if status.success(){
            insert_data(&acs_schema, &table, &database, &username)?;
        } else {
            fetch_details(&database, &username, &acs_schema, &table)?;
        }
    }
    Ok(())
}

pub fn fetch_details(database: &str, username: &str, schema: &str, table: &Table) -> Result<TableVersion, Error> {
    let select_query = format!("SELECT column_id, column_name FROM {}.{}", schema, table.table_id);
    let output = Command::new("psql").args(&["-d", database, "-U", username, "-c", &select_query]).output()?;
    let output = match String::from_utf8(output.stdout){
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    let rows = output.split("\r\n");
    let mut records:Vec<TableRecord> = Vec::new();
    let mut count = 0;
    let mut min_year:i32;
    let mut max_year:i32;
    for row in rows {
        println!("dfgdfgdf");
        let r: Vec<&str> = row.split("|").collect();
        println!("{}, {:?}", r.len(), r);
        println!("blaaaaaa");
        if count == 2{
            min_year = r[1].to_string().replace(" ","").parse::<i32>().unwrap();
        }
        if count == 3{
            min_year = r[1].to_string().replace(" ","").parse::<i32>().unwrap();
        }
        let cid = r[0].to_string().replace(" ","");
        let cname = r[1].to_string().replace(" ","");
        println!("{}", cid);
        println!("{}", cname);
        records.push(TableRecord {
            column_id: cid.clone(),
            label: cname.clone(),
        });
        count += 1;
    }
    Ok(TableVersion{
        table_id: "B24010".to_string(),
        min_year: 2020,
        max_year: 2020,
        estimate: Vec::new(),
        record: records.clone()
    })
}

pub fn insert_data(acs_schema: &str, table: &Table, database: &str, username: &str) -> Result<(), Error> {
    let insert_min_year = format!("INSERT INTO {}.{} (column_id, column_name) VALUES(\'{}\', \'{}\')", acs_schema, table.table_id, "min_year", table.year);
    let insert_max_year = format!("INSERT INTO {}.{} (column_id, column_name) VALUES(\'{}\', \'{}\')", acs_schema, table.table_id, "max_year", table.year);
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_min_year]).status()?;
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_max_year]).status()?;
    for record in table.record.iter() {
        let insert_query = format!("INSERT INTO {}.{} (column_id, column_name) VALUES('{}', '{}')", acs_schema, table.table_id, record.column_id, record.label);
        Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_query]).status()?;
    }
    Ok(())
}

//fn compare(table)