use std::{fs, io::{prelude::*}, net::{TcpListener, TcpStream}, thread};
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let lister = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in lister.incoming().take(2){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }
    println!("Shutting Down");
}


fn  handle_connection(mut stream:TcpStream){
    let mut buffer = [0; 2014];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let response = format!("{}{}", status_line, fs::read_to_string(filename).unwrap());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}