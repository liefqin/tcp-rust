use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::time;

fn main() {
    //服务端线程
    let handler_server = thread::spawn(move || {
        println!("[Server] start ...");
        //服务端
        worker_server();
        println!("[Server] close ...");
    });

    //客户端线程
    let handler_client = thread::spawn(move || {
        //delay 200ms
        thread::sleep(time::Duration::from_millis(200));
        println!("[Client] start ...");
        //客户端
        worker_client();
        thread::sleep(time::Duration::from_millis(500));
        println!("[Client] close ...");
    });

    handler_server.join();
    handler_client.join();
}

fn worker_server() {
    //绑定 ip port
    let listener = TcpListener::bind("localhost:3000").unwrap();
    //获取网络流迭代器
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //处理消息
                process_stream(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn process_stream(mut stream: TcpStream) {
    //生成缓存单元
    let mut data = [0; 1024];
    //读取流
    while match stream.read(&mut data) {
        Ok(size) => {
            // 打印从客户端发送过来的消息
            let text = from_utf8(&data).unwrap();
            thread::sleep(time::Duration::from_millis(400));
            println!("[Server] Recv from client : {}", text);
            //服务器给客户端回复信息
            let text = stream.write(&data[0..size]).unwrap();
            //关闭流
            stream.shutdown(Shutdown::Both).unwrap();
            true

        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn worker_client() {
    match TcpStream::connect("localhost:3000") {
        Ok(mut stream) => {
            println!("[Client] Successfully connected to server in port 3333");
            let msg = b"Hello!";
            stream.write(msg).unwrap();
            println!("[Client] Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            //匹配服务端返回的消息
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("[Client] Reply From Server is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("[Client] Unexpected reply: {}", text);
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

}