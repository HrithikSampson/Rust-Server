use std::io::BufRead;
use std::io::Write;
use std::net::TcpListener;
//use std::io::prelude;
use std::io::BufReader;
use std::net::TcpStream;
use std::format;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // listener is a iterator to a array of streams
    for stream in listener.incoming(){
        let stream = stream.unwrap();// overshadowing stream
        
        println!("Connection established");
        // stream is a connection attempt and when os limit reaches and throws error
        handle_connection(stream);
    }
}
fn handle_connection(mut stream:TcpStream){
    let http_request: Vec<_> = BufReader::new(&mut stream)
                                       .lines()
                                       .map(|el|el.unwrap())
                                       .take_while(|el| !el.is_empty())
                                       .collect();
    println!("{:#?}",http_request);
    
    let iter = http_request.get(0).unwrap().split(' ').map(|el|el.to_string());
    let bind = iter.collect::<Vec<String>>();
    let path = bind.get(1).unwrap();
    
    let mut response:String = String::new();
    if path.as_str() == "/"{
        response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    }
    else if path.starts_with("/echo") == true {
        let x:Vec<&str> = path.split('/').collect();
        let p = x.get(1).unwrap().len().to_string();
        let q = x.get(1).unwrap();
        response = format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}\r\n",(p.as_str()),q);
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }
    println!("{:#?}",response);
    stream.write_all(response.as_bytes()).unwrap();
}
