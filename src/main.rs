use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use ffi::Event;

use crate::poll::Poll;

mod ffi;
mod poll;

fn delayserver_request(delay: usize, message: &str) -> String {
    format!(
        "GET /{delay}/{message} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
}

fn main() -> Result<(), io::Error> {
    const NUMBER_OF_STREAMS: usize = 5;
    const DELAYSERVER_ADDR: &str = "localhost:8080";

    let mut poll = Poll::new()?;
    let mut streams = vec![];
    for stream_id in 0..NUMBER_OF_STREAMS {
        let mut stream = TcpStream::connect(DELAYSERVER_ADDR)?;
        stream.set_nonblocking(true)?;

        stream.write_all(
            delayserver_request(
                (NUMBER_OF_STREAMS - stream_id) * 1000,
                format!("request-{stream_id}").as_str(),
            )
            .as_bytes(),
        )?;

        poll.registry()
            .register(&stream, stream_id, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    eprintln!("Created connections... processing events");
    let expected_events = NUMBER_OF_STREAMS;
    let mut handled_events = 0;
    while handled_events < expected_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        eprintln!("Processing: {} events", events.len());
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }
        
        handled_events += handle_events(&events, &mut streams)?;
    }

    Ok(())
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize, io::Error> {
    let mut handled_events: usize = 0;

    let mut buf = [0; 4096];
    for event in events {
        let stream_id = event.token();
        match streams[stream_id].read(&mut buf) {
            Ok(n) if n == 0 => {
                handled_events += 1;
                break;
            }
            Ok(_) => {
                let txt = String::from_utf8_lossy(&buf);
                println!("Received: {:?}", event);
                println!("{txt}\n------\n");
            },
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                break;
            },
            Err(err) => return Err(err),
        }
    }

    Ok(handled_events)
}
