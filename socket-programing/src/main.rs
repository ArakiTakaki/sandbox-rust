use std::env;

#[macro_use]
extern crate log;

//mod tcp_client;
mod tcp_server;
//mod udp_client;
//mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect(); // argsを代入

    // validation argsが少ない場合エラー
    if args.len() != 4 {
        error!("引数が足りません [tcp|udp] [server|client] [addr:port]");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    println!("{}", address);
    match protocol {
        "tcp" => match role {
            "server" => {
                println!("started tcp server");
                tcp_server::server(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                println!("started tcp client");
                // TODO TCP client
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                println!("started udp server");
                // TODO UDP server
            }
            "client" => {
                println!("started udp client");
                // TODO UDP client
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Pleasespecifytcporudponthe1stargument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Pleasespecifytcporudponthe1stargument.");
    std::process::exit(1);
}
