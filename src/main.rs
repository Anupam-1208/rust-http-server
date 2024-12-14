use std::{fs, io::{BufRead, BufReader, Write}, net::TcpListener,};
use http_server::ThreadPool;

fn main() {
    // create a TCP server
    let addr = "127.0.0.1:8989";
    let socket = TcpListener::bind(addr).unwrap();

    // creating a thread pool, to pickup new connection for a request from the request  queue
    let pool = ThreadPool::new(1);

    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
                // std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("connection failed {}",e);
            }
        }

    }

}

// used to get the tcp stream data and print it
fn handle_connection(mut stream:std::net::TcpStream){
    let buff_reader = BufReader::new(&stream);
    // let http_request:Vec<_> = buff_reader.lines().map(|res| res.unwrap()).take_while(|line| !line.is_empty()).collect();
    // println!("Request: {http_request:#?}");

    let request_line = buff_reader.lines().next().unwrap().unwrap();
    // write a wrapper for Http request 
    let (status_line, path) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(10));
            ("HTTP/1.1 200 OK","test.html")},
        _ => ("HTTP/1.1 404 NOT FOUND","error.html"),
    };
    let content = fs::read_to_string(path).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Lenth: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
// build a HTTP server
// steps involved
// 1 create a TCP connection on a port

