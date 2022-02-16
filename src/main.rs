use serde_json::Value;

enum Indexers {
    /// For indexing into [Value::Object]
    String(String),

    /// Input values that parse to a number. For indexing into [Value::Array].
    /// This keeps the original string in case the value is an object with a numeric string key.
    Number(String, usize),
}

fn main() {
    let mut count = false;
    let mut print_keys = false;
    let mut keep_invalid = false;
    let mut remove_quotes = false;
    let mut indexers = Vec::new();

    for arg in std::env::args().skip(1) {
        if arg == "-c" || arg == "--count" {
            count = true;
        } else if arg == "-h" || arg == "--help" {
            println!("Usage: jsx -h or --help            Show this help message");
            println!("Usage: jsx -c or --count           Display the count of values in an array or object");
            println!(
                "Usage: jsx -k or --keys            Display the keys within a selected object"
            );
            println!(
                "Usage: jsx -i or --keep-invalid    Print empty row (instead of nothing) for 
                                                        invalid json or too deep querying"
            );
            println!("Usage: jsx -q or --remove-quotes   Removes quotes from selected strings");
            return;
        } else if arg == "-k" || arg == "--keys" {
            print_keys = true;
        } else if arg == "-i" || arg == "--keep-invalid" {
            keep_invalid = true;
        } else if arg == "-q" || arg == "--remove-quotes" {
            remove_quotes = true;
        } else {
            match arg.parse() {
                Ok(u) => indexers.push(Indexers::Number(arg, u)),
                Err(_) => indexers.push(Indexers::String(arg)),
            }
        }
    }

    let mut buffer = String::with_capacity(1024);
    let stdin = std::io::stdin();
    while stdin.read_line(&mut buffer).is_ok() && !buffer.is_empty() {
        let j: Result<Value, _> = serde_json::from_str(&buffer);
        if let Ok(j) = j {
            if let Some(val) = recurse(j, &indexers[..]) {
                match &val {
                    Value::Array(a) if count => println!("{}", a.len()),
                    Value::Array(a) if print_keys => {
                        let mut it = 0..a.len();
                        if let Some(i) = it.next() {
                            print!("{}", i);
                            for i in it {
                                print!(", {}", i);
                            }

                            println!();
                        }
                    }
                    Value::Array(_) => println!("{val}"),

                    Value::Object(o) if count => println!("{}", o.len()),
                    Value::Object(o) if print_keys => {
                        let mut it = o.keys();

                        if let Some(k) = it.next() {
                            print!("{}", k);
                            for k in it {
                                print!(", {}", k);
                            }
                            println!();
                        }
                    }
                    Value::Object(_) => println!("{val}"),

                    Value::String(s) if remove_quotes => println!("{}", s.trim_matches('"')),

                    v => println!("{v}"),
                }
            } else if keep_invalid {
                println!();
            }
        } else if keep_invalid {
            println!();
        }

        buffer.clear();
    }
}

fn recurse(val: Value, indexers: &[Indexers]) -> Option<Value> {
    let (i, rest) = match &indexers {
        // No indexing (ie, into arrays/objects), so we can return this `val`
        [] => return Some(val),
        [i] => (i, None),
        [i, rest @ ..] => (i, Some(rest)),
    };

    match (val, i, rest) {
        // can't index an array by string
        (Value::Array(_), Indexers::String(_), _) => None,

        (Value::Object(mut m), Indexers::String(key) | Indexers::Number(key, _), rest) => {
            // got a map - does it contain the key we want?
            if let Some(v) = m.get_mut(key) {
                let val = v.take();

                if let Some(r) = rest {
                    recurse(val, r)
                } else {
                    Some(val)
                }
            } else {
                // key not found
                None
            }
        }

        (Value::Array(a), Indexers::Number(_, idx), rest) => {
            if *idx < a.len() {
                let val = a.into_iter().nth(*idx).unwrap();
                if let Some(r) = rest {
                    recurse(val, r)
                } else {
                    Some(val)
                }
            } else {
                None
            }
        }

        // Trying to index into a scalar is invalid
        _ => None,
    }
}
