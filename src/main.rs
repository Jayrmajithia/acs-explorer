extern crate reqwest;
extern crate json;
extern crate structopt;
#[macro_use]
extern crate error_chain;

mod cli;
mod explore;
mod error;
mod load;

use cli::{Clicommand, Command};
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
    let cli_command = Clicommand::from_args();
    let year = cli_command.year;
    let acs_est = cli_command.estimate;
    let table = explore(&year, &acs_est)?;
//    create_table(&table)?;
    match cli_command.command{
        Command::PrettyTable {table_id} => {
            print!("{}", format_label(&table[&table_id]));
        },
        Command::ConfigTable {table_id} => {
            print!("{}", format_table_config(&table[&table_id]));
        }
    }
    Ok(())
}
