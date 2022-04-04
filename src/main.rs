use std::net::{TcpListener, TcpStream};
// Tcp Stream 实现了读写 traits, 故包含此库
use std::io::{Read, Write};
use std::fs;

fn main() {
    // 创建TCP连接监听本地7000端口,unwrap隐含panic处理
    let connection_listener = TcpListener::bind("127.0.0.1:7000").unwrap();
    println!("Running on port 7000!");
    // 监听TCP连接
    for stream in connection_listener.incoming() {
        // 使stream可变，以便对其进行读写
        let mut stream = stream.unwrap();
        // 打印建立连接信息
        println!("Connection Established");
        // 打印请求信息
        let req = print_request(&stream);
        // 服务端处理请求，返回一个Result类型数据
        let result = handle_http_method(req.as_str());
        // 用Result进行match,若为<OK, "XXX">则返回正确响应信息，若为<Err, "XXX"> 则返回错误响应信息
        let resp = match result {
            Ok(m) => {
                print!("request method: {}", m.as_str());
                return_success_response()
            },
            Err(_) => return_error_response()
        };
        // 将响应信息写入到流中
        stream.write(resp.as_bytes()).unwrap();
        // flush强制直接返回响应，把缓存中的内容直接写入
        stream.flush().unwrap();
    }
}


fn print_request(mut stream: &TcpStream) -> String {
    // 在栈上声明一个 buffer 来存放读取到的数据，创建缓冲区的大小为1024字节
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // 将缓冲区的字节转换为字符串
    let content = String::from_utf8_lossy(&buffer);
    // 打印请求内容
    print!("{}", content);
    // 返回请求信息
    content.to_string()
}

fn return_success_response() -> String {
    // 读取正常响应文件html res_html 变量隐藏
    let res_html = fs::read_to_string("hello.html").unwrap();
    // 生成正常返回信息字符串
    let success_resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    success_resp
}

fn return_error_response() -> String {
    // 读取异常响应文件html
    let res_html = fs::read_to_string("error.html").unwrap();
    // 生成异常响应信息字符串
    let err_resp = format!(
        "HTTP/1.1 404 NOTFOUND\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    err_resp
}

fn handle_http_method (content: &str) -> Result<HttpMethod, &str> {
    // 根据请求内容判断HTTP请求method 正确则返回包含HTTPMethod的OK枚举,非GET POST则返回包含错误信息ERR枚举
    if content.starts_with("GET") {
        Result::Ok(HttpMethod::GET)
    } else if content.starts_with("POST") {
        Result::Ok(HttpMethod::POST)
    } else {
        Result::Err("http method not support")
    }
}

enum HttpMethod {
    GET,
    POST
}

impl HttpMethod {
    // 实现as_str方法便于打印
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST"
        }
    }
}