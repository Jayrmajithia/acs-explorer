## ACS explorer, cli

BREAKING CHANGE: v0.2.0 no longer inserts the `E` value after table ids in etl config.
FIX BUT: v0.3.0 properly formats 2016+ BUT for etl purposes there is no information on whether label is terminal hierarchy or not.
BREAKING CHANGE: v0.5.0 has been refactored and restructured to support the changes in the ACS api from the year 2010-2018.

A small utility to examine the metadata of ACS tables and vars.

The ACS is a survey of United States demographics run by the Census Bureau. [American Community Survey](https://www.census.gov/programs-surveys/acs/)

## Installation

Binaries for linux and osx are available on the releases page.

For building from source, you need Rust. I suggest [Rustup](https://rustup.rs). Then:

```
$ git clone https://github.com/Jayrmajithia/acs-explorer.git
$ cd acs-explorer && cargo install
```

## Usage
```
USAGE:
    acs-explorer [FLAGS] --database <database> --acs-estimate <estimate> --schema <schema> --table-id <table-id> --username <username> --year <year>

FLAGS:
    -c, --config     
    -h, --help       Prints help information
    -l, --load       
    -p, --pretty     
    -V, --version    Prints version information

OPTIONS:
    -d, --database <database>        
    -e, --acs-estimate <estimate>    
    -s, --schema <schema>            
    -t, --table-id <table-id>        
    -u, --username <username>        
    -y, --year <year> 
```

Note that `search` and `describe` have aliases `s` and `d`.

## Examples

```
$ acs-explorer -d datausa_test -u datawheel -e 1 -y 2015 -s acs_data -t B28002 -p

Pretty Table:
001|Total
002|With an Internet subscription
003|    Dial-up alone
004|    DSL
005|        With mobile broadband
006|        Without mobile broadband
007|    Cable modem
008|        With mobile broadband
009|        Without mobile broadband
010|    Fiber-optic
011|        With mobile broadband
012|        Without mobile broadband
013|    Satellite Internet service
014|        With mobile broadband
015|        Without mobile broadband
016|    Two or more fixed broadband types, or other
017|        With mobile broadband
018|        Without mobile broadband
019|    Mobile broadband alone or with dialup
020|Internet access without a subscription
021|No Internet access
```

```
$ acs-explorer % acs-explorer -d database -u username -e 1 -y 2015 -s acs_data -t B28002 -c

Config File:
001: "Total"
002: "WithAnInternetSubscription"
003: "WithAnInternetSubscription_Dial-upAlone"
004: "WithAnInternetSubscription_DSL"
005: "WithAnInternetSubscription_DSL_WithMobileBroadband"
006: "WithAnInternetSubscription_DSL_WithoutMobileBroadband"
007: "WithAnInternetSubscription_CableModem"
008: "WithAnInternetSubscription_CableModem_WithMobileBroadband"
009: "WithAnInternetSubscription_CableModem_WithoutMobileBroadband"
010: "WithAnInternetSubscription_Fiber-optic"
011: "WithAnInternetSubscription_Fiber-optic_WithMobileBroadband"
012: "WithAnInternetSubscription_Fiber-optic_WithoutMobileBroadband"
013: "WithAnInternetSubscription_SatelliteInternetService"
014: "WithAnInternetSubscription_SatelliteInternetService_WithMobileBroadband"
015: "WithAnInternetSubscription_SatelliteInternetService_WithoutMobileBroadband"
016: "WithAnInternetSubscription_TwoOrMoreFixedBroadbandTypes,OrOther"
017: "WithAnInternetSubscription_TwoOrMoreFixedBroadbandTypes,OrOther_WithMobileBroadband"
018: "WithAnInternetSubscription_TwoOrMoreFixedBroadbandTypes,OrOther_WithoutMobileBroadband"
019: "WithAnInternetSubscription_MobileBroadbandAloneOrWithDialup"
020: "InternetAccessWithoutASubscription"
021: "NoInternetAccess"

```
Todo:
1) Bring it more closer to the previous versions functionalites
2) Store the Estimates to seprate it out
3) What to do next in case of changes to the table
4) Provide an accurate number grouping and repitative name to the group in order to make the same number of group across all of the table.
5) Add support to other databases. Currently only supports the postgres database.
