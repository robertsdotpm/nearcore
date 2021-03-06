extern crate beacon;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate network;
extern crate node_cli;
extern crate parking_lot;
extern crate primitives;
extern crate storage;
extern crate tokio;

//use beacon::types::BeaconBlockHeader;
//use clap::{App, Arg};
//use client::Client;
//use env_logger::Builder;
//use futures::{Future, Stream};
//use futures::sync::mpsc::channel;
//use network::service::generate_service_task;
//use network::{protocol::ProtocolConfig, service::Service, test_utils::*};
//use node_cli::chain_spec::get_default_chain_spec;
//use parking_lot::RwLock;
//use primitives::hash::hash;
//use primitives::signature::DEFAULT_SIGNATURE;
//use primitives::signer::InMemorySigner;
//use primitives::types::{SignedTransaction, TransactionBody};
//use network::network_handler::ChannelNetworkHandler;
//use std::sync::Arc;
//use std::time::Duration;
//use tokio::runtime::current_thread::Runtime;
//use tokio::timer::Interval;
//
//fn create_addr(host: &str, port: &str) -> String {
//    format!("/ip4/{}/tcp/{}", host, port)
//}

pub fn main() {
//    // TODO: This might be broken due to https://github.com/nearprotocol/nearcore/issues/121
//    let mut builder = Builder::new();
//    builder.filter(Some("sub-libp2p"), log::LevelFilter::Debug);
//    builder.filter(Some("sync"), log::LevelFilter::Debug);
//    builder.filter(Some("main"), log::LevelFilter::Debug);
//    builder.filter(None, log::LevelFilter::Info);
//    builder.init();
//
//    // parse command line arguments for now. Will need to switch to use config file in the future.
//    let matches = App::new("Client")
//        .arg(Arg::with_name("host").long("host").takes_value(true))
//        .arg(Arg::with_name("port").long("port").takes_value(true))
//        .arg(Arg::with_name("is_root").long("is_root").required(true).takes_value(true))
//        .arg(Arg::with_name("root_port").long("root_port").required(true).takes_value(true))
//        .get_matches();
//    let host = matches.value_of("host").unwrap_or("127.0.0.1");
//    let port = matches.value_of("port").unwrap_or("30000");
//    let is_root = value_t!(matches, "is_root", bool).unwrap();
//    let root_port = matches.value_of("root_port").unwrap();
//
//    // start network service
//    let addr = create_addr(host, port);
//    let root_addr = create_addr(host, root_port);
//    let net_config = if is_root {
//        test_config_with_secret(&addr, vec![], special_secret())
//    } else {
//        let boot_node = root_addr + "/p2p/" + &raw_key_to_peer_id_str(special_secret());
//        println!("boot node: {}", boot_node);
//        test_config(&addr, vec![boot_node])
//    };
//    let chain_spec = get_default_chain_spec();
//    let storage = Arc::new(storage::test_utils::create_memory_db());
//    let signer = Arc::new(InMemorySigner::new());
//    let client = Arc::new(RwLock::new(Client::new(&chain_spec, storage, signer)));
//    let protocol_config = if is_root {
//        ProtocolConfig::new_with_default_id(special_secret())
//    } else {
//        ProtocolConfig::default()
//    };
//    let (submit_txn_tx, _submit_txn_rx) = channel(1024);
//    let (submit_receipt_tx, _submit_receipt_rx) = channel(1024);
//    let network_handler = ChannelNetworkHandler::new(submit_txn_tx.clone(), submit_receipt_tx.clone());
//    let service =
//        Service::new(protocol_config, net_config, network_handler, client.clone()).unwrap();
//    let task = generate_service_task::<_, _, BeaconBlockHeader>(
//        &service.network,
//        &service.protocol,
//    );
//    // produce some fake transactions once in a while
//    let tx_period = Duration::from_millis(1000);
//    let fake_tx_task = Interval::new_interval(tx_period)
//        .for_each({
//            let client = client.clone();
//            move |_| {
//                let tx_body = TransactionBody {
//                    nonce: 1,
//                    amount: 1,
//                    sender: hash(b"bob"),
//                    receiver: hash(b"alice"),
//                    method_name: vec![],
//                    args: vec![],
//                };
//                let tx = SignedTransaction::new(DEFAULT_SIGNATURE, tx_body);
//                client.write().receive_transaction(tx);
//                Ok(())
//            }
//        }).map_err(|_| ());
//    let mut runtime = Runtime::new().unwrap();
//    runtime.spawn(fake_tx_task);
//    runtime.block_on(task).unwrap();
}
