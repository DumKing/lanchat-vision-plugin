use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

#[derive(Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
enum Request {
    Ping,
    RuntimeStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response<'a> {
    ok: bool,
    result: serde_json::Value,
    error: Option<&'a str>,
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines().map_while(Result::ok) {
        let response = match serde_json::from_str::<Request>(&line) {
            Ok(Request::Ping) => Response { ok: true, result: serde_json::json!({"version":"0.8.0"}), error: None },
            Ok(Request::RuntimeStatus) => Response { ok: true, result: serde_json::json!({"state":"idle"}), error: None },
            Err(_) => Response { ok: false, result: serde_json::Value::Null, error: Some("INVALID_REQUEST") },
        };
        if serde_json::to_writer(&mut stdout, &response).is_err() { break; }
        if writeln!(stdout).and_then(|_| stdout.flush()).is_err() { break; }
    }
}
