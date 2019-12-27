extern crate reqwest;
extern crate json;
extern crate structopt;
#[macro_use]
extern crate error_chain;

mod cli;
mod explore;
mod error;
mod load;

use cli::CliCommand;
use structopt::StructOpt;
use explore::*;
use error::*;
use load::*;

fn main() {
    if let Err(err) = run() {
        println!("{:?}", err)
    }
}

fn run() -> Result<()> {
    let cli_command = CliCommand::from_args();
    let year = cli_command.year;
    let acs_est = cli_command.estimate;
    let table_id = cli_command.table_id;
    let username = cli_command.username;
    let schema = cli_command.schema;
    let database = cli_command.database;
    let table = explore(&year, &acs_est)?;
    if cli_command.load {
        create_table(&schema, &database, &username, &table, &table_id)?;
    }
    if cli_command.config {
        println!("\nConfig File:\n{}", format_table_config(&schema, &database,&username, &cli_command.load, &table[&table_id])?);
    }
    if cli_command.prettify {
        println!("\nPretty Table:\n{}", format_label(&schema, &database,&username, &cli_command.load, &table[&table_id])?);
    }
    Ok(())
}
