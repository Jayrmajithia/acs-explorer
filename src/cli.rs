use structopt::StructOpt;
use std::cmp::Ordering;

#[derive(Debug, StructOpt)]
#[structopt(name="acs-explorer")]
pub struct CliCommand {
    #[structopt(short="y", long="year")]
    pub year: String,
    #[structopt(short="e", long="acs-estimate")]
    pub estimate: String,
    #[structopt(short="l", long="load")]
    pub load: bool,
    #[structopt(short="t", long="table-id")]
    pub table_id: String,
    #[structopt(short="p", long="pretty")]
    pub prettify: bool,
    #[structopt(short="c", long="config")]
    pub config: bool,
    #[structopt(short="u", long="username")]
    pub username: String,
    #[structopt(short="s", long="schema")]
    pub schema: String,
    #[structopt(short="d", long="database")]
    pub database: String,
}


fn parse_table_id(id:&str) -> (String, String, String) {
    let mut i = id.clone();
    let prefix = i.split_at(1).0;
    i = i.split_at(1).1;
    let table = i.split_at(5).0;
    i = i.split_at(5).1;
    if i.len() == 1 {
        (prefix.to_string(), table.to_string(), i.to_string())
    } else {
        (prefix.to_string(), table.to_string(), "".to_string())
    }
}

// ToDo: Add support to store the estimates in the table
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableVersion {
    pub table_id: String,
    pub min_year: i32,
    pub max_year: i32,
    pub record: Vec<TableRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table {
    pub table_id: String,
    pub year: String,
    pub acs_estimate: String,
    pub record: Vec<TableRecord>,
}

impl Ord for Table{
    fn cmp(&self, other: &Table) -> Ordering {
        let (p1, t1, s1) = parse_table_id(&self.table_id);
        let (p2, t2, s2) = parse_table_id(&other.table_id);
        if t1 != t2 {
            t1.cmp(&t2)
        } else if p1 != p2 {
            p1.cmp(&p2)
        } else if s1 == "" && s2 != "" {
            Ordering::Less
        } else if s1 != "" && s2 == "" {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Table {
    fn partial_cmp(&self, other: &Table) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_column_id(id: &str) -> (String, String) {
    let mut i = id.clone();
    let column = i.split_at(3).0;
    i = i.split_at(3).1;
    (column.to_string(), i.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableRecord {
    pub column_id: String,
    pub label: String,
}
impl Ord for TableRecord {
    fn cmp(&self, other: &TableRecord) -> Ordering {
        let (c1, s1) = parse_column_id(&self.column_id);
        let (c2, s2) = parse_column_id(&other.column_id);
        if c1 != c2 {
            c1.cmp(&c2)
        } else if s1 != s2 {
            s1.cmp(&s2)
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TableRecord {
    fn partial_cmp(&self, other: &TableRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
