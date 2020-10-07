extern crate bitcoin;
extern crate cfd_rust;
extern crate clap;
extern crate hex;
extern crate secp256k1;
extern crate json;

use self::cfd_rust as cfd;

use cfd::{decode_raw_transaction, Network};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  let network_arg = Arg::with_name("network")
    .help("network option")
    .short("n")
    .long("network")
    .takes_value(true);
  let tx_arg = Arg::with_name("tx")
    .help("transaction hex")
    .short("t")
    .long("tx")
    .takes_value(true);
  let file_arg = Arg::with_name("file")
  .help("transaction hex file")
  .short("f")
  .long("file")
  .takes_value(true);

  let app = App::new("cfd_tester")
    .about("cfd test CLI")
    .version(VERSION)
    .subcommand(
      SubCommand::with_name("decoderawtransaction")
        .about("decde raw transaction data.")
        .arg(tx_arg.clone())
        .arg(file_arg.clone())
        .arg(network_arg.clone()),
    );

  let matches = app.get_matches();

  if let Some(ref matches) = matches.subcommand_matches("decoderawtransaction") {
    decode(matches);
    return;
  }
}

fn get_network(matches: &ArgMatches) -> Network {
  let mut network_type = Network::Mainnet;
  if let Some(network) = matches.value_of("network") {
    network_type = match network {
      "mainnet" => Network::Mainnet,
      "testnet" => Network::Testnet,
      "regtest" => Network::Regtest,
      "liquidv1" => Network::LiquidV1,
      "elementsregtest" => Network::ElementsRegtest,
      _ => Network::Mainnet,
    };
  }
  network_type
}

fn get_tx(matches: &ArgMatches) -> String {
  let tx;
  if let Some(hex) = matches.value_of("tx") {
    tx = hex.to_string();
  } else if let Some(file) = matches.value_of("file") {
    tx = fs::read_to_string(file).expect("Failed to read file.");
  } else {
    eprintln!("error: require tx or file.");
    panic!("error: require tx or file.");
  }
  tx
}

fn decode(matches: &ArgMatches) {
  let network_type = get_network(matches);
  let tx = get_tx(matches);
  let dec_tx_ret = decode_raw_transaction(&network_type, tx.trim());
  match dec_tx_ret {
    Ok(dec_tx) => {
      let json_obj = json::parse(&dec_tx).expect("Invalid json format.");
      let json_str = json::stringify_pretty(json_obj, 2);
      println!("tx: {}", json_str)
    },
    Err(e) => eprintln!("error: {}", e),
  }
}
