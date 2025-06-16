// Raftノードを立ち上げるサーバー
use std::{net::SocketAddr, collections::HashMap};

use raftpico::{Server, FileStorage, Machine, ApplyContext};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = std::env::args().nth(1).and_then(|a| a.parse().ok()).expect("invalid command line arg");

    let listen_addr: SocketAddr = format!("127.0.0.1:{port}").parse()?;
    let storage = Some(FileStorage::new(format!("raftkvs-{port}.jsonl"))?);

    let mut server = Server::<KvsMachine>::start(listen_addr, storage)?;

    loop {
        server.poll(None)?;
    }
}

// ステートマシン
#[derive(Debug, Default, Serialize, Deserialize)]
struct KvsMachine {
    entries: HashMap<String, serde_json::Value>,
}

// KVSサーバー(JSON-RPC)
impl Machine for KvsMachine {
    // KvsInputに対応するKVSサーバーの動作
    type Input = KvsInput;
    fn apply(&mut self, ctx: &mut ApplyContext, input: Self::Input) {
        match input {
            KvsInput::Put { key, value } => {
                let old_value = self.entries.insert(key, value);
                ctx.output(&old_value);
            }
            KvsInput::Get { key } => {
                let value = self.entries.get(&key);
                ctx.output(&value);
            }
            KvsInput::Delete { key } => {
                let value = self.entries.remove(&key);
                ctx.output(&value);
            }
        }
    }

}

// KVSサーバーのinput定義
#[derive(serde::Serialize, serde::Deserialize)]
enum KvsInput {
    Put {
        key: String,
        value: serde_json::Value,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    }
}

