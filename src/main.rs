use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::collections::HashMap;

const SERVER_ADDRESS:&str = "localhost:8888";
const HEAD_SEPARATOR: &str ="\r\n";


//Simple function to remove whitespaces from a string slice
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

//consume header request map to make the response HEAD str
fn response_headers(request_headers:HashMap<String, String>)->String {
    let mut head_response = String::from("HTTP/1.1 200 OK\r\n");
    for (header_name,header_value) in &request_headers {
        let header_entry = format!("{}:{}{}",header_name,header_value,HEAD_SEPARATOR);
        head_response+= &header_entry;
    }
    return head_response + HEAD_SEPARATOR;
}


fn parse_http_request(request:&String)->Result <(Vec<String>,HashMap<String,String>), String> {
    
    // given an HTTP GET request,
    // returns the start-line and a HashMap<name,value> for headers
    
    let req_vec: Vec<&str> = request.split(HEAD_SEPARATOR).collect();
    let startline:&Vec<&str> = &req_vec[0].split(" ").collect();
    let mut head:HashMap<String, String> = HashMap::new();
    if * &startline.len() == 3{
        //let method = startline[0];
        //let target = startline[1];

        for header in &req_vec[1..&req_vec.len()-2] {
            let header_field:Vec<&str> = header.split(":").collect();
            let header_name = String::from(header_field[0]);
            let mut header_value = String::new();
            if header_name == String::from("Host") && remove_whitespace(header_field[1])== "localhost".to_owned() {
                header_value = remove_whitespace(header_field[1])+":"+header_field[2];
            } else {
                header_value = remove_whitespace(header_field[1]);
            }
            //in the case there are repeated headers
            head.entry(header_name).or_insert(header_value);
        }
        let mut owned_startline = Vec::new();
        for item in startline {
            owned_startline.push(String::from(*item));
        }
        Ok((owned_startline, head))
        
    } else {
        Err("Failed to parse start-line".to_owned())
    }
}

fn is_homepage(headers:&HashMap<String, String>) -> bool {
    headers.get(&String::from("Host")).unwrap().to_string() == SERVER_ADDRESS
}

fn handle_connection(mut conn:TcpStream) {
    let mut buffer = [0;1024];
    println!("reading message...");
    let len = conn.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..len]).to_string();
    let (_, head) = parse_http_request(&request).unwrap();
    if is_homepage(&head) {
        println!("received: {}", request);
        let resp_head = response_headers(head);
        let resp_body = fs::read_to_string("index.html").expect("couldn´t read file");
    
        let response = resp_head + &resp_body;
        let _ = conn.write_all(response.as_bytes());
        println!("sent: {}", response);

    } else {
        //here goes the proxy request
    }
}


fn main() {
    println!("creating server in {}", SERVER_ADDRESS);

    if let Ok(server) = TcpListener::bind(SERVER_ADDRESS) {
        println!("proxy server created in {}:{}",
            server.local_addr().unwrap().ip(),
            server.local_addr().unwrap().port());

        for connection in server.incoming() {
            let connection = connection.unwrap();
            println!("connection stablished");
            handle_connection(connection);
        }
    } else {
        println!("failed to create proxy server in {}", SERVER_ADDRESS)
    }
}
