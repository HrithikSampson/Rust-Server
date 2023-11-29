// use std::io::BufRead;
// use std::io::Read;
// use std::io::Write;
// use std::net::TcpListener;
// use std::fs::File;
// // use std::io::Result;
// //use std::io::prelude;
// use std::io::BufReader;
// use std::net::TcpStream;
// use std::format;
// use std::thread;
// pub fn get_dir() -> Option<String> {
//     let arguments: Vec<String> = std::env::args().collect();
//     let index = arguments
//         .iter()
//         .position(|arg| arg == "--directory");
        
//     if let Some(_)=index {
//         return Some(arguments[index.unwrap() + 1].to_string())
//     }
//     return None;
// }
// fn main() {
    
//     let arguments: Vec<String> = std::env::args().collect();
//     println!("{:?}",arguments);
//     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
//     // listener is a iterator to a array of streams
    
//     for stream in listener.incoming(){
//         let stream = stream.unwrap();// overshadowing stream
//         let directory = get_dir();
//         println!("Connection established");
//         // stream is a connection attempt and when os limit reaches and throws error
//         thread::spawn(||handle_connection(stream,directory));
//     }
// }
// fn handle_connection(mut stream:TcpStream,directory: Option<String>){
//     let http_request: Vec<_> = BufReader::new(&mut stream)
//                                        .lines()
//                                        .map(|el|el.unwrap())
//                                        .take_while(|el| !el.is_empty())
//                                        .collect();
//     println!("{:#?}",http_request); 
    
//     let mut content_buffer = [0u8; 1024].to_vec();
//     let _sz=stream.read(&mut content_buffer).unwrap();
//     let iter = http_request.get(0).unwrap().split(' ').map(|el|el.to_string());
//     let bind = iter.collect::<Vec<String>>();
//     let path = bind.get(1).unwrap();
//     let method: &String = bind.get(0).unwrap();
//     let mut response= String::new();
//     if path.as_str() == "/"{
//         response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
//     } else if path.starts_with("/echo") == true {
//         let q = path[1..].split_once('/').unwrap();
//         let p = q.1.len().to_string();
//         response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",(p.as_str()),q.1);
//     } else if path.starts_with("/user-agent") == true {
//         let q = http_request.get(2).unwrap().split_once(" ").unwrap().1;
//         let p = q.len().to_string();
//         response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",(p.as_str()),q);
//     } else if path.starts_with("/files") == true {
//         println!("{}",method);
//         if method == "GET" {
//             if let Some(_) = directory {
//                 let filename = path[1..].split_once("/").unwrap().1;
//                 println!("{:?}",directory);
//                 println!("{}",filename);

//                 let file_result = File::open(directory.unwrap()+"/"+filename);

//                 response = match file_result {
//                     Ok(mut file) => {
//                         let mut contents = String::new();
//                         file.read_to_string(&mut contents).unwrap();
//                         format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length:{}\r\n\r\n{}",contents.len(),contents)
//                     },
//                     Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
//                 };
//             }
//             else if let None = directory {
//                 response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
//             }
//         }
//         else {
//             let filename = path[1..].split_once("/").unwrap().1;
//             println!("{:?}",directory);
//             println!("{}",filename);
//             let filepath = format!("{}{}", directory.unwrap(), filename);
//             println!("{}",filepath);
//             //let file_result = File::open(&filepath);
//             // let _sz = match stream.read(&mut content_buffer) {
//             //     Ok(n) => n,
//             //     Err(_) => 0
//             // };
//             //let content_buffer = http_request.get(http_request.len()-1).unwrap().to_string();
//             //println!("{}",_sz);
//             println!("{}",filepath); 
//             println!("{:?}",content_buffer);
//             // let parts: Vec<String> = String::from_utf8_lossy(&content_buffer).lines().map(|line| line.to_string()).collect();
//             // println!("{:?}",parts);
//             //let contents:Vec<String> = String::from_utf8_lossy(&content_buffer).lines().map(|line| line.to_string()).collect();//parts.get(parts.len()-1).unwrap();
//             //println!("{:?}",file_result);
//             let cont = String::from_utf8_lossy(&content_buffer).to_string();
//             // response = match file_result {
//             //     Ok(mut file) => {

//             //         file.write_all(&contents.as_bytes()).unwrap();
//             //         "HTTP/1.1 201 Created\r\n\r\n".to_string()
//             //     }, 
//             //     Err(_) => {
//                 println!("{}",filepath);
//     let mut file = File::create(&filepath).unwrap_or_else(|err| panic!("error: {}",err));
//     println!("{}",filepath);
//     file.write_all(cont.as_bytes()).unwrap_or_else(|err| panic!("error1: {}",err));
//     println!("{}",filepath); 

//     response = "HTTP/1.1 201 Created\r\n\r\n".to_string();
//             //     }
//             // };
//             // println!("{}",response);
//         }
//     } else {
//         response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
//     }
//     println!("{:#?}",response); 
//     stream.write_all(response.as_bytes()).unwrap();
// }
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;

pub fn get_dir() -> Option<String> {
    let arguments: Vec<String> = std::env::args().collect();
    let index = arguments.iter().position(|arg| arg == "--directory");

    index.map(|i| arguments[i + 1].clone())
}

fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
    let http_request: Vec<String> = BufReader::new(&mut stream)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let mut content_buffer = [0u8; 1024].to_vec();
    let _sz = stream.read(&mut content_buffer).unwrap();

    let bind: Vec<&str> = http_request[0].split(' ').collect();
    let path = bind.get(1).unwrap_or(&"");
    let method = bind.get(0).unwrap_or(&"");

    let mut response = String::new();

    if path.is_empty() || path == &"/" {
        response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    } else if path.starts_with("/echo") {
        let q = path[1..].split_once('/').unwrap_or(("", ""));
        let p = q.1.len().to_string();
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            p,
            q.1
        );
    } else if path.starts_with("/user-agent") {
        let q = http_request.get(2).map_or("", |line| line.split_once(" ").map_or("", |(_, s)| s));
        let p = q.len().to_string();
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
            p, q
        );
    } else if path.starts_with("/files") {
        println!("{}", method);
        if method == &"GET" {
            if let Some(dir) = &directory {
                let filename = path[1..].split_once("/").map_or("", |(_, f)| f);
                println!("{:?}", dir);
                println!("{}", filename);

                let file_result = File::open(format!("{}/{}", dir, filename));

                response = match file_result {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap_or_else(|err| {
                            panic!("Error reading file: {}", err);
                        });
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:{}\r\n\r\n{}",
                            contents.len(),
                            contents
                        )
                    }
                    Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                };
            } else {
                response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
            }
        } else {
            let filename = path[1..].split_once("/").map_or("", |(_, f)| f);
            println!("{:?}", directory);
            println!("{}", filename);
            let filepath = format!("{}{}", directory.unwrap_or_default(), filename);
            println!("{}", filepath);

            let cont = String::from_utf8_lossy(&content_buffer).to_string();

            let mut file = File::create(&filepath).unwrap_or_else(|err| {
                panic!("Error creating file: {}", err);
            });
            file.write_all(cont.as_bytes()).unwrap_or_else(|err| {
                panic!("Error writing to file: {}", err);
            });

            response = "HTTP/1.1 201 Created\r\n\r\n".to_string();
        }
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }

    println!("{:#?}", response);
    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        panic!("Error writing response to stream: {}", err);
    });
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let directory = get_dir();
        println!("Connection established");
        thread::spawn(|| handle_connection(stream, directory));
    }
}
