// Raftノードを立ち上げるサーバー
use std::{net::SocketAddr};

use raftpico::{Server, FileStorage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr: SocketAddr = format!("IPアドレス:TCPポート ".parse()?;
    let storage_path = Some(FileStorage::new(format!("ファイルパス"))?);

    // Raftノードとして動作するサーバーを構築
    let mut server = Server::<type_>::start(listen_addr, storage)?;

    // サーバーの処理を実行する
    loop {
        server.poll(None)?;
    }
}


// ステートマシン
use std::{collections::HashMap};

use raftpico::Machine;
use serde::{Deserialize, Serialize};

struct KvsMachine {
    entries: HashMap<String, serde_json::Value>,
}

// KVSサーバー(JSON-RPC)
impl Machine for KvsMachine {

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

// KvsInputに対応するKVSサーバーの動作
// self -> KvsMachine
// ApplyContext : from raftpico
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
};

