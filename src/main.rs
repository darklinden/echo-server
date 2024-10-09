use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    http::HeaderValue,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut port = 3000;
    if args.len() == 1 {
        // no port provided
    } else if args.len() == 2 {
        // port provided
        let port_str = &args[1];
        // if -h / --help print help
        if port_str == "-h" || port_str == "--help" {
            println!("Usage: echo_server [port]");
            println!("port: port number to listen on [default: 3000]");
            return;
        }

        let parse_port = port_str.trim().parse::<u16>();
        match parse_port {
            Ok(p) => {
                port = p;
            }
            Err(_) => {
                println!("Invalid port number: {}", port_str);
                return;
            }
        }
    }

    let bind_server = TcpListener::bind(format!("0.0.0.0:{}", port));
    let server = match bind_server {
        Ok(s) => {
            println!("Server started on port {}", port);
            s
        }
        Err(e) => {
            println!("Failed to bind server on port {}: {}", port, e);
            return;
        }
    };

    let client_auto_id = std::sync::atomic::AtomicUsize::new(1);
    let arc_client_auto_id = std::sync::Arc::new(client_auto_id);

    for stream in server.incoming() {
        let auto_id = arc_client_auto_id.clone();
        spawn(move || {
            let mut u_client_id = 0;
            let mut client_ip: String = String::default();
            let callback = |req: &Request, response: Response| {
                println!("Received a new ws handshake path: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, value) in req.headers() {
                    println!("* {:?} {:?}", header, value);
                }
                u_client_id = auto_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                client_ip = req
                    .headers()
                    .get("x-real-ip")
                    .unwrap_or(&HeaderValue::from_static("unknown"))
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                println!("Client assigned id: {}", u_client_id);

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
            websocket
                .send(tungstenite::Message::Text(format!(
                    "Hello! Client [{}] from IP: [{}]",
                    u_client_id, client_ip
                )))
                .unwrap();

            loop {
                let msg = websocket.read();
                match msg {
                    Err(err) => {
                        println!("Error reading message: {:?}", err.to_string());
                        break;
                    }
                    Ok(msg) => match msg {
                        tungstenite::Message::Text(text) => {
                            println!("Received {} text message: {:?}", u_client_id, text);
                            websocket.send(tungstenite::Message::Text(text)).unwrap();
                        }
                        tungstenite::Message::Binary(bin) => {
                            println!("Received {} binary message: {:?}", u_client_id, bin);
                            websocket.send(tungstenite::Message::Binary(bin)).unwrap();
                        }
                        tungstenite::Message::Close(close) => {
                            println!("Received {} close message: {:#?}", u_client_id, close);
                            // wait for the client to close the connection
                            let _ = websocket.flush();
                            break;
                        }
                        tungstenite::Message::Ping(ping) => {
                            println!("Received {} message: {:?}", u_client_id, ping);
                            websocket.send(tungstenite::Message::Pong(ping)).unwrap();
                        }
                        tungstenite::Message::Pong(pong) => {
                            println!("Received {} message: {:?}", u_client_id, pong);
                        }
                        tungstenite::Message::Frame(f) => {
                            println!("Received {} frame message: {:?}", u_client_id, f);
                        }
                    },
                }
            }
        });
    }
}
