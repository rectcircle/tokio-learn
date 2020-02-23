use tokio::net::TcpStream;
use tokio::prelude::*;  // 引入预定义的实现

use tokio::net::TcpListener;
use futures::stream::StreamExt;


#[tokio::main] // 该宏创建了一个tokio运行时环境
async fn main() {
    // === tcp 客户端 ===
    // 创建一个Tcp连接
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    // 向Tcp连接中写入数据
    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    // 关闭Tcp连接
    if let Ok(()) = stream.shutdown(std::net::Shutdown::Write) {
        println!("close stream success");
    }

    // === tcp 服务端 Echo 服务 ===
    let addr = "127.0.0.1:6143";
    let mut listener:TcpListener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on {}", addr);

    // 以下实现通过 Stream 方式实现
    async move {
        // 将 TcpListener 转换为一个 Stream
        let mut incoming = listener.incoming();
        // 等待 stream 获取到数据（即客户端的连接）
        // 依赖 tokio 的 stream feature 和 futures = "0.3" 
        while let Some(conn) = incoming.next().await {
            match conn {
                Err(e) => eprintln!("accept failed = {:?}", e),
                Ok(mut sock) => {
                    // 当收到一个Tcp连接时，提交一个 Future 到 tokio 运行时
                    tokio::spawn(async move {
                        // 获取到读写句柄
                        let (mut reader, mut writer) = sock.split();

                        // 将接收到的数据写回，完成Echo
                        // 使用 tokio::io::copy 方法，同样该方法是异步的
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("wrote {} bytes", amt);
                            }
                            Err(err) => {
                                eprintln!("IO error {:?}", err);
                            }
                        }
                    });
                }
            }
        }
    }.await;

    // // 直观的实现实现
    // loop {
    //     // Asynchronously wait for an inbound socket.
    //     // 异步等待 客户端 连接
    //     let (mut socket, _): (TcpStream, _) = listener.accept().await.unwrap();

    //     // 获取到客户端连接后，提交一个异步任务到 tokio 运行时，用来处理运行客户端连接
    //     tokio::spawn(async move {
    //         // buffer
    //         let mut buf = [0; 1024];

    //         // In a loop, read data from the socket and write the data back.
    //         // 循环等待用户输入
    //         loop {
    //             // 异步等待用户输入数据到buffer中
    //             let n = socket
    //                 .read(&mut buf)
    //                 .await
    //                 .expect("failed to read data from socket");

    //             if n == 0 {
    //                 return;
    //             }

    //             // 写会到客户端
    //             socket
    //                 .write_all(&buf[0..n])
    //                 .await
    //                 .expect("failed to write data to socket");
    //         }
    //     });
    // }
}