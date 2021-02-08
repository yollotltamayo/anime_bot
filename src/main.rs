#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use serde_json::{ Result, Value};
use std::env;
#[tokio::main]
async fn main() -> Result<()> {
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() <= 1 {
        panic!("WOWOWO NO HAY ARGUMENTOS");
    }
    let anime = &args[1];
    let query = format!("https://api.jikan.moe/v3/search/anime?q={}",anime);
    let resp = reqwest::get(&query)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let cop : Value = serde_json::from_str(&resp)?;
    let mut table  = Table::new();
    table.add_row(row![FR=>"Tittle", "Episodes", "Airing"]);
    //println!("{}", cop["results"][2]["title"]);
    for x in 1..=3{
        table.add_row(row![FY => cop["results"][x]["title"],
            cop["results"][x]["episodes"],
            cop["results"][x]["airing"]
        ]);
    }
      table.printstd();
   Ok(())
}
