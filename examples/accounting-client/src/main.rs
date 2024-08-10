use std::io::Write;

use test_domain::User;
use zrpc::{
    client::ZRpcClient,
    models::{dt::ZRpcDt, req::ZRpcReq},
    params, req,
};

fn main() {
    let mut client = ZRpcClient::new("127.0.0.1:3000").expect("Failed to initialize ZRpcClient");

    let mut buf = String::new();

    loop {
        print!("$");

        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim() {
                "show users" => match client.call(req!("list_users", params!())) {
                    Ok(res) => {
                        let users = res.deserialize::<Vec<User>>().unwrap();

                        for i in 0..users.len() {
                            let user = &users[i];

                            println!("{}: {}, {} y. o.", i + 1, user.name, user.age);
                        }
                    }
                    Err(_) => eprintln!("Failed to call 'list_users' procedure"),
                },
                "create doe" => {
                    match client.call(req!("create_user", params!("John Doe".to_string(), 18))) {
                        Ok(res) => {
                            let user = res.deserialize::<User>().unwrap();

                            println!("Created: {:#?}", user);
                        }
                        Err(_) => todo!(),
                    }
                }
                "exit" => std::process::exit(0),
                _ => {}
            },
            Err(e) => eprintln!("Failed to read line: {}", e),
        }

        buf.clear();
    }
}
