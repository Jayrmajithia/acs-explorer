use reqwest::{Url, Client, Method};
use std::collections::HashMap;
use error::*;
use cli::{Table, TableRecord};
use load::fetch_details;

const CENSUS_URL:&str = "https://api.census.gov/data/";
const VARIABLE_BASE:&str = "variables.json";

pub fn explore(year:&str, acs_est:&str) -> Result<HashMap<String, Table>> {
    let handle = Client::new();
    let acs_est = format!("acs{}", acs_est);
    let url:String;
    url = format!("{}{}/acs/{}/{}", CENSUS_URL, year, acs_est, VARIABLE_BASE);
    let url = Url::parse(&url)?;
    let resp = Client::request(&handle, Method::GET, url).build()?;
    let mut data = Client::execute(&handle, resp)?;
    if data.status().is_server_error(){
        bail!("Server Error");
    } else if !data.status().is_success() {
        bail!("Something happened the status is {:?}", data.status());
    }
    let vars_data = data.text()?;
    let table_map = process_vars_data(&vars_data)?;
    let tables = format_table(year, table_map, &acs_est);
    Ok(tables)
}

fn process_vars_data(result:&str) -> Result<HashMap<String, Vec<Vec<String>>>> {
    let data = json::parse(result).chain_err(|| "error in parsing")?;
    let mut table_map = HashMap::new();
    for (acs_var, _acs_info) in data["variables"].entries() {
        let acs_var_str = acs_var.to_string();
        let json_id = acs_var_str.clone();
        let t_c = acs_var_str.split("_");
        let f = t_c.clone();
        if t_c.count() != 2 {
            continue;
        }
        let t_c:Vec<&str> = f.collect();
        table_map.entry(t_c[0].to_string()).or_insert(Vec::new()).push([t_c[1].to_string(), data["variables"][json_id]["label"].to_string()].to_vec());
    }
    Ok(table_map)
}

fn format_table(year:&str, table: HashMap<String, Vec<Vec<String>>>, acs_est:&str ) -> HashMap<String, Table> {
    let mut tables = HashMap::new();
    for (table_id, table_details) in table.iter() {
        let mut r = Vec::new();
        for col_details in table_details.iter() {
            let mut cid = col_details[0].to_string();
            cid.truncate(3);
            r.push(TableRecord{
                column_id: cid.clone(),
                label: col_details[1].to_string(),
            });
        }
        r.sort_by(|a, b| a.cmp(b));
        let t = Table{
            table_id: table_id.to_string(),
            year: year.to_string(),
            acs_estimate: acs_est.to_string(),
            record: r.clone()
        };
        tables.entry(table_id.to_string()).or_insert(t);
    }
    // tables.sort_by(|a, b| a.cmp(b));
    tables
}

pub fn format_label(schema: &str, database: &str, username: &str, load: &bool, records: &Table) -> Result<String> {
    let mut res: String = "".to_string();
    let indent = "    ";
    if !load {
        for column in records.record.iter() {
            let cid = &column.column_id;
            let label = &column.label;
            let pattern = "!!";
            let split_index = match label.rfind(pattern) {
                Some(i) => i + 2,
                None => 0
            };
            let (indents, label) = label.split_at(split_index);
            let indents: String = indents.split(pattern).skip(3).map(|_| indent).collect();
            let label = label.trim_end_matches(":");
            res.push_str(&format!("{}|{}{}\n", cid, indents, label)[..]);
        }
    } else {
        let result = fetch_details(database, username, schema, records)?;
        res.push_str(&format!("Table Id: {}\n", result.table_id));
        res.push_str(&format!("Min Year: {}\n", result.min_year));
        res.push_str(&format!("Max Year: {}\n", result.max_year));
        res.push_str("---------------------------------------------------------------------------------------------------------------------\n");
        for column in result.record.iter() {
            let cid = &column.column_id;
            let label = &column.label;
            let pattern = "!!";
            let split_index = match label.rfind(pattern) {
                Some(i) => i + 2,
                None => 0
            };
            let (indents, label) = label.split_at(split_index);
            let indents: String = indents.split(pattern).skip(3).map(|_| indent).collect();
            let label = label.trim_end_matches(":");
            res.push_str(&format!("{}|{}{}\n", cid, indents, label)[..]);
        }
    }
    Ok(res)
}


pub fn format_table_config(schema: &str, database: &str, username: &str, load: &bool, records: &Table) -> Result<String> {
    let mut res: String = "".to_string();
    if !load {
        for column in records.record.iter() {
            let cid = &column.column_id;
            let pattern = "!!";
            let mut label: String;
            let split_index = match &column.label.find(pattern) {
                Some(i) => i + 2,
                None => 0
            };
            let (_, good_label) = &column.label.split_at(split_index);
            if cid != "001" {
                label = good_label.to_owned().replace("Total!!", "");
            } else {
                label = good_label.to_owned().to_string();
            }
            label = label.replace(pattern, "_").replace("'", "");
            label = to_camelcase(&label);
            res.push_str(&format!("{}: {:?}\n", cid, label)[..]);
        }
    } else {
        let result = fetch_details(database, username, schema, records)?;
        res.push_str(&format!("Table Id: {}\n", result.table_id));
        res.push_str(&format!("Min Year: {}\n", result.min_year));
        res.push_str(&format!("Max Year: {}\n", result.max_year));
        res.push_str("---------------------------------------------------------------------------------------------------------------------\n");
        for column in result.record.iter() {
            let cid = &column.column_id;
            let pattern = "!!";
            let mut label: String;
            let split_index = match &column.label.find(pattern) {
                Some(i) => i + 2,
                None => 0
            };
            let (_, good_label) = &column.label.split_at(split_index);
            if cid != "001" {
                label = good_label.to_owned().replace("Total!!", "");
            } else {
                label = good_label.to_owned().to_string();
            }
            label = label.replace(pattern, "_").replace("'", "");
            label = to_camelcase(&label);
            res.push_str(&format!("{}: {:?}\n", cid, label)[..]);
        }
    }
    Ok(res)
}

fn to_camelcase(s: &str) -> String {
    s.split_whitespace().map(|words| {
        let mut c = words.chars();
        match c.next() {
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }).collect()
}
