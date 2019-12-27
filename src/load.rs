use cli::{Table, TableRecord, TableVersion};
use std::process::Command;
use std::collections::HashMap;
use std::io::Error;

pub fn create_table(schema: &str, database: &str, username: &str, table_map: &HashMap<String, Table>, table_id: &str) -> Result<(), Error> {
//    let acs_schema = "acs_data"; // get from param
//    let database = "datawheel"; // get from param
//    let username = "postgres"; // get from param
    let schema_str = format!("CREATE SCHEMA IF NOT EXISTS {}", schema);
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &schema_str]).status()?;
    let table = &table_map[table_id];
    let table_query = format!("CREATE TABLE {}.\"{}\" (column_id TEXT, column_name TEXT)", schema, table.table_id);
    let status = Command::new("psql").args(&["-d", database, "-U", username, "-c", &table_query]).status()?;
    if status.success(){
        insert_data(&schema, &table, &database, &username)?;
    } else {
        compare(&database, &username, &schema, &table)?;
    }
    Ok(())
}

pub fn fetch_details(database: &str, username: &str, schema: &str, table: &Table) -> Result<TableVersion, Error> {
    let select_query = format!("SELECT column_id, column_name FROM {}.\"{}\"", schema, table.table_id);
    let output = Command::new("psql").args(&["-d", database, "-U", username, "-c", &select_query]).output()?;
    let output = match String::from_utf8(output.stdout){
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    let rows = output.split("\r\n");
    let mut records:Vec<TableRecord> = Vec::new();
    let mut count = 0;
    let mut min_year:i32 = 0;
    let mut max_year:i32 = 0;
    for row in rows {
        let r: Vec<&str> = row.split("|").collect();
        if count == 0 || count == 1 || r.len() == 1{
            count += 1;
            continue
        }
        if r[0].trim() == "min_year"{
            min_year = r[1].to_string().replace(" ","").parse::<i32>().unwrap();
        }
        else if r[0].trim() == "max_year"{
            max_year = r[1].to_string().replace(" ","").parse::<i32>().unwrap();
        } else {
            let cid = r[0].to_string().trim().to_string();
            let cname = r[1].to_string().trim().to_string();
            records.push(TableRecord {
                column_id: cid.clone(),
                label: cname.clone(),
            });
        }
        count += 1;
    }
    // Performing sort to arrange the table data in order to get the sorted by column id data
    records.sort_by(|a, b| a.cmp(b));
    Ok(TableVersion{
        table_id: table.table_id.to_string(),
        min_year,
        max_year,
        record: records.clone()
    })
}

pub fn insert_data(schema: &str, table: &Table, database: &str, username: &str) -> Result<(), Error> {
    let insert_min_year = format!("INSERT INTO {}.\"{}\" (column_id, column_name) VALUES(\'{}\', \'{}\')", schema, table.table_id, "min_year", table.year);
    let insert_max_year = format!("INSERT INTO {}.\"{}\" (column_id, column_name) VALUES(\'{}\', \'{}\')", schema, table.table_id, "max_year", table.year);
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_min_year]).status()?;
    Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_max_year]).status()?;
    for record in table.record.iter() {
        let insert_query = format!("INSERT INTO {}.\"{}\" (column_id, column_name) VALUES('{}', '{}')", schema, table.table_id, record.column_id, record.label);
        Command::new("psql").args(&["-d", database, "-U", username, "-c", &insert_query]).status()?;
    }
    Ok(())
}

// ToDo: Modify this compare method for the use of comparison and to create new table with_year
// Currently it prints saying that the data is changed
fn compare(database: &str, username: &str, schema: &str, table: &Table) -> Result<(), Error> {
    let results = fetch_details(database, username, schema, table)?;
    let year = table.year.parse::<i32>().unwrap();
    if results.record.len()!= table.record.len() {
        println!("the table {} is different as length is different", table.table_id);
        return Ok(());
//        panic!("The Table are different");
    }
    for i in 0..results.record.len() {
        if results.record[i].label.len() != table.record[i].label.len() {
//            return Err("The tables are different".into())
            println!("the table {} is different", table.table_id);
            return Ok(());
        }
    }
    if results.min_year > year {
        let update_min_year = format!("UPDATE {}.\"{}\" set column_name={} where column_id='min_year'", schema, table.table_id, table.year);
        Command::new("psql").args(&["-d", database, "-U", username, "-c", &update_min_year]).status()?;
    } else if results.max_year < year {
        let update_max_year = format!("UPDATE {}.\"{}\" set column_name={} where column_id='max_year'", schema, table.table_id, table.year);
        Command::new("psql").args(&["-d", database, "-U", username, "-c", &update_max_year]).status()?;
    }
    Ok(())
}