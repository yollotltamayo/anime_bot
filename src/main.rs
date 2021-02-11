#[macro_use] extern crate prettytable;
use prettytable::{Table};
use serde_json::{Value};
use std::env;
use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(),Error> {
    let args:Vec<String> = env::args().collect();
    let token = env::var("TELEGRAM_TOKEN").expect("No se encontro el token");
    let api = Api::new(token);

    if args.len() <= 1 {
        panic!("WOWOWO NO HAY ARGUMENTOS");
    }
    let anime = &args[1];
    let query = format!("https://api.jikan.moe/v3/search/anime?q={}",anime);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }
    let resp = reqwest::get(&query)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let copi = serde_json::from_str(&resp);
    let cop :Value=copi.unwrap();
    let mut table  = Table::new();
    table.add_row(row![FR=>"Tittle", "Episodes", "Airing"]);
    for x in 1..=3{
        table.add_row(row![FY => cop["results"][x]["title"],
            cop["results"][x]["episodes"],
            cop["results"][x]["airing"]
        ]);
    }
      table.printstd();
    Ok(())
}
