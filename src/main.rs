use std::{collections::HashMap, net::TcpListener, sync::{mpsc::{Sender, channel}}, thread::{sleep, spawn}, time::Duration};

use sysdata::SysData;
use sysinfo::{System, SystemExt};
use tungstenite::{Message, accept};

mod sysdata;

macro_rules! unwrap_or_continue {
    ($expr:expr) => {
        match $expr { Ok(v) => v, Err(_) => continue }
    };
}


struct TxData {
    id: usize,
    tx: Sender<Message>,
}

fn main() {
    let (send_tick, receive_tick) = channel();
    spawn(move || {
        loop {
            send_tick.send(()).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    let (send_tx_base, receive_tx) = channel::<TxData>();
    let (send_tx_remove_base, receive_tx_remove) = channel::<usize>();

    

    let mut system = System::new();

    spawn(move || {
        let mut txs = HashMap::new();

        loop {
            receive_tick.recv().unwrap();
            for tx_data in receive_tx.try_iter()  {
                txs.insert(tx_data.id, tx_data.tx);
            }
            for remove_id in receive_tx_remove.try_iter() {
                txs.remove(&remove_id);
            }
            system.refresh_all();
            let sys_data = Message::Text(unwrap_or_continue!(serde_json::to_string(&SysData::from_system(&mut system))));
            for (_, tx) in txs.iter() {
                tx.send(sys_data.clone()).unwrap_or(());
            }
        }
    });

    let server = TcpListener::bind("127.0.0.1:54345").unwrap();

    for (id, stream) in server.incoming().enumerate() {
        let send_tx = send_tx_base.clone();
        let send_tx_remove = send_tx_remove_base.clone();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            let (tx, rx) = channel();

            send_tx.send(TxData { id, tx }).unwrap();

            loop {
                let msg = unwrap_or_continue!(rx.recv());
                
                websocket.write_message(msg).unwrap_or_else(|err| {
                    match err {
                        tungstenite::Error::ConnectionClosed => {
                            send_tx_remove.send(id).unwrap();
                            panic!();
                        }
                        tungstenite::Error::AlreadyClosed => {
                            send_tx_remove.send(id).unwrap();
                            panic!();
                        }
                        _ => (),
                    }
                });
            }
        });
    }
}
