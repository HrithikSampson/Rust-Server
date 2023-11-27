use std::io::BufRead;
use std::io::Write;
use std::net::TcpListener;
//use std::io::prelude;
use std::io::BufReader;
use std::net::TcpStream;
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
    
    let response = match path.as_str(){
        "/"=> "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    stream.write_all(response.as_bytes()).unwrap();
}
