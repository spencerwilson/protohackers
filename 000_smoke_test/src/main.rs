use std::io::copy;
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

// The problem specification states that the server must support at least
// 5 clients simultaneously.
const THREAD_COUNT: u8 = 8;

fn handle_stream(mut stream: TcpStream) {
    println!("accepted connection");

    // Creating a second handle to the socket enables the use of
    // `std::io::copy`, which manages the complexity of piping
    // a reader (the socket) into a writer (the same socket).
    let mut write_stream = match stream.try_clone() {
        Ok(write_stream) => write_stream,
        Err(e) => {
            println!("error while cloning TcpStream: {}", e);
            return;
        }
    };

    let echoed_byte_count = match copy(&mut stream, &mut write_stream) {
        Err(e) => {
            println!("error while echoing bytes on TcpStream: {}", e);
            return;
        }
        Ok(count) => count,
    };

    println!("socket closed, bytes echoed: {}", echoed_byte_count);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("listening on {}", listener.local_addr().unwrap());

    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..THREAD_COUNT {
        let cloned_listener = match listener.try_clone() {
            Ok(listener) => listener,
            Err(e) => {
                println!("error cloning TcpListener for thread {}", i);
                return Err(e);
            }
        };

        join_handles.push(thread::spawn(move || {
            println!("thread {} started", i);

            for stream in cloned_listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_stream(stream);
                    }
                    Err(e) => {
                        println!("error on Incoming iterator: {}", e);
                    }
                }
            }
        }));
    }

    for (i, handle) in join_handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => println!("thread {} finished", i),
            Err(_) => {}
        }
    }

    Ok(())
}
