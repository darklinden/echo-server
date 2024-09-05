use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut port = 3012;
    if args.len() == 1 {
        // no port provided
    } else if args.len() == 2 {
        // port provided
        let port_str = &args[1];
        // if -h / --help print help
        if port_str == "-h" || port_str == "--help" {
            println!("Usage: echo_server [port]");
            println!("port: port number to listen on [default: 3012]");
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
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind server on port {}: {}", port, e);
            return;
        }
    };
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {header}");
                }

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read().unwrap();
                match msg {
                    tungstenite::Message::Text(text) => {
                        println!("Received a text message: {:?}", text);
                        websocket.send(tungstenite::Message::Text(text)).unwrap();
                    }
                    tungstenite::Message::Binary(bin) => {
                        println!("Received a binary message: {:?}", bin);
                        websocket.send(tungstenite::Message::Binary(bin)).unwrap();
                    }
                    tungstenite::Message::Close(close) => {
                        println!("Received a close message: {:?}", close);
                        break;
                    }
                    tungstenite::Message::Ping(ping) => {
                        println!("Received a ping message: {:?}", ping);
                        websocket.send(tungstenite::Message::Pong(ping)).unwrap();
                    }
                    tungstenite::Message::Pong(pong) => {
                        println!("Received a pong message: {:?}", pong);
                    }
                    tungstenite::Message::Frame(f) => {
                        println!("Received a Frame message: {:?}", f);
                    }
                }
            }
        });
    }
}
