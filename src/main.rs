use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    transaction_type: String,
    contract_name: String,
    contract_address: String,
}

#[derive(Deserialize, Debug)]
struct ScriptRun {
    transactions: Vec<Transaction>,
}

impl Serialize for ScriptRun {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(
            self.transactions
                .iter()
                .filter(|t| t.transaction_type == "CREATE")
                .map(|t| (&t.contract_name, &t.contract_address)),
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let json_input = match std::fs::read_to_string(file_path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let script_run: ScriptRun = match serde_json::from_str(&json_input) {
        Ok(script_run) => script_run,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    };

    let json_output = serde_json::to_string_pretty(&script_run).unwrap();
    print!("{}", json_output);
}
