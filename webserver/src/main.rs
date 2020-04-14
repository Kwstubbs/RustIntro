use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use web_server::ThreadPool;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:7868").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        pool.execute(|| {handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
let(status_line, filename) = if buffer.starts_with(get){
    ("HTTP/1.1 200 OK \r\n\r\n", "404.html")
}else{
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
};
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}