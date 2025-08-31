# std::net 模块教程

Rust 的 `std::net` 模块是标准库中处理网络通信的核心部分，提供 TCP、UDP 和 IP 相关的类型和函数，用于构建客户端/服务器应用、网络工具和低级 socket 操作。它抽象了底层 OS 网络 API（如 BSD sockets），确保跨平台兼容性（Windows、Unix、macOS），并通过 `io::Result` 显式处理错误如连接失败或超时。`std::net` 强调安全：无缓冲区溢出风险，集成 Rust 的借用检查器。模块支持 IPv4/IPv6、流式（TCP）和数据报（UDP）通信，但不包括高级协议如 HTTP（用 hyper 等 crate）。

## 1. std::net 简介

- **导入和基本结构**：通常用 `use std::net;` 或指定如 `use std::net::{TcpListener, TcpStream};`。模块分为地址类型、TCP、UDP 和实用工具四大类。
    - **地址类型**：
        - `IpAddr`：枚举 IPv4/IPv6 地址，支持解析和比较。
        - `Ipv4Addr`/`Ipv6Addr`：具体 IP 类型，支持 octet/segment 操作。
        - `SocketAddr`：Socket 地址（IP + 端口），枚举 V4/V6。
        - `ToSocketAddrs`：trait，用于将字符串/元组转为 SocketAddr 迭代器。
    - **TCP 类型**：`TcpListener`（服务器监听）、`TcpStream`（连接流，实现 Read/Write/Seek）。
    - **UDP 类型**：`UdpSocket`（数据报 socket，支持 send_to/recv_from）。
    - **函数和 trait**：`ToSocketAddrs` trait、`shutdown` 方法等。
- **设计哲学**：`std::net` 是阻塞同步的（非异步，用 tokio 替代）；错误通过 `io::ErrorKind` 分类（如 AddrInUse、ConnectionRefused）；支持非阻塞模式（set_nonblocking）。IPv6 优先，但兼容 v4。
- **跨平台注意**：Windows 用 Winsock，Unix 用 POSIX；地址解析处理 localhost 差异；测试多 OS 以验证绑定/连接。
- **性能基础**：TCP/UDP O(1) 操作，但网络延迟主导；用缓冲 Read/Write 优化；多连接用线程池。
- **常见用例**：简单 HTTP 服务器、聊天客户端、端口扫描、DNS 查询（低级）。
- **扩展概念**：与 std::io 集成（TcpStream 实现 Read/Write）；与 std::thread 结合多客户端；错误重试机制；socket 选项如 set_read_timeout。相比 crate 如 mio（事件循环），std::net 适合简单同步应用。

## 2. 地址类型：IpAddr 和 SocketAddr

地址类型是网络的基础，支持解析、格式化和比较。

### 示例：基本地址创建和解析（Ipv4 示例）
```rust
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let ipv4 = Ipv4Addr::new(127, 0, 1, 1);  // 本地回环
    let ip: IpAddr = ipv4.into();
    println!("IP: {}", ip);  // 127.0.0.1

    let socket = SocketAddr::new(ip, 8080);
    println!("Socket: {}", socket);  // 127.0.0.1:8080
}
```

- **解释**：`Ipv4Addr::new` 从 octet 创建。`into()` 转为 IpAddr 枚举。`SocketAddr::new` 组合 IP 和端口。性能：栈分配，常量时间。

### 示例：地址解析和迭代（ToSocketAddrs 扩展）
```rust
use std::net::ToSocketAddrs;

fn main() {
    let addrs: Vec<SocketAddr> = "localhost:80".to_socket_addrs().unwrap().collect();
    println!("解析地址: {:?}", addrs);  // [127.0.0.1:80, [::1]:80] (IPv4 和 IPv6)

    // 扩展：处理多个地址（故障转移）
    for addr in "example.com:443".to_socket_addrs().unwrap() {
        println!("地址: {}", addr);
    }
}
```

- **解释**：`to_socket_addrs` 返回迭代器，解析 DNS（阻塞）。unwrap 处理错误如 InvalidInput。陷阱：无网络返回 Err(AddrNotAvailable)。扩展：用 loop 尝试连接每个地址以故障转移。

### 示例：Ipv6 地址和比较（扩展变体）
```rust
use std::net::{IpAddr, Ipv6Addr};

fn main() {
    let ipv6 = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);  // ::1 等价
    let ip: IpAddr = ipv6.into();
    println!("IPv6: {}", ip);  // 2001:db8::1

    let loopback = IpAddr::V6(Ipv6Addr::LOCALHOST);
    println!("是本地？{}", loopback.is_loopback());  // true

    // 扩展：范围检查和多播
    println!("是多播？{}", ip.is_multicast());  // false
}
```

- **解释**：`Ipv6Addr::new` 从 segment 创建。方法如 `is_loopback`、`is_global` 检查属性。性能：Ipv6 更大，但操作常数时间。扩展：用 `is_ipv4_mapped` 处理 v4 兼容 v6。

## 3. TCP 通信：TcpListener 和 TcpStream

TCP 提供可靠流式连接。

### 示例：简单 TCP 服务器（基本监听）
```rust
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("监听于 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0; 512];
        let bytes = stream.read(&mut buf)?;
        stream.write_all(&buf[..bytes])?;  // 回显
    }
    Ok(())
}
```

- **解释**：`bind` 绑定地址，返回 TcpListener。`incoming()` 迭代新连接，返回 TcpStream。Read/Write 处理数据。陷阱：端口占用返回 AddrInUse。

### 示例：TCP 客户端连接（扩展重试）
```rust
use std::net::TcpStream;
use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

fn connect_with_retry(addr: &str, retries: u32) -> io::Result<TcpStream> {
    let mut attempts = 0;
    loop {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(e) if e.kind() == io::ErrorKind::ConnectionRefused && attempts < retries => {
                attempts += 1;
                sleep(Duration::from_secs(1));
            }
            Err(e) => return Err(e),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stream = connect_with_retry("127.0.0.1:8080", 3)?;
    stream.write_all(b"hello")?;
    Ok(())
}
```

- **解释**：`connect` 建立连接。重试 ConnectionRefused。性能：超时默认 OS 设置，用 set_read_timeout 自定义。扩展：用 peek 检查数据可用。

### 示例：多客户端服务器（线程扩展）
```rust
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 512];
    let bytes = stream.read(&mut buf)?;
    stream.write_all(&buf[..bytes])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;  // 监听所有接口
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle_client(stream) {
                eprintln!("客户端错误: {}", e);
            }
        });
    }
    Ok(())
}
```

- **解释**：每个连接 spawn 线程。`0.0.0.0` 监听所有 IP。性能：线程开销高，大并发用线程池（如 threadpool crate）。陷阱：未处理线程 panic，用 join 监控。

### 示例：TCP 选项和 shutdown（扩展控制）
```rust
use std::net::TcpStream;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;  // 读取超时
    stream.set_nodelay(true)?;  // 禁用 Nagle 算法，提高实时性
    stream.shutdown(std::net::Shutdown::Write)?;  // 关闭写入端
    Ok(())
}
```

- **解释**：`set_read_timeout` 设置超时。`set_nodelay` 减少延迟。`shutdown` 半关闭连接（Write/Read/Both）。扩展：用 `ttl` 设置 IP TTL。

## 4. UDP 通信：UdpSocket

UDP 是无连接数据报，适合低延迟但可能丢失。

### 示例：UDP 发送和接收（基本）
```rust
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;  // 随机端口
    socket.send_to(b"hello", "127.0.0.1:34254")?;

    let mut buf = [0; 1024];
    let (bytes, src) = socket.recv_from(&mut buf)?;
    println!("从 {} 接收 {} 字节: {:?}", src, bytes, &buf[..bytes]);
    Ok(())
}
```

- **解释**：`bind` 绑定本地地址。`send_to` 发送到目标。`recv_from` 接收并返回来源。性能：无连接，快于 TCP。

### 示例：UDP 多播（组播扩展）
```rust
use std::net::{UdpSocket, Ipv4Addr};

fn main() -> std::io::Result<()> {
    let multicast_addr = Ipv4Addr::new(224, 0, 0, 251);
    let socket = UdpSocket::bind("0.0.0.0:5353")?;
    socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::UNSPECIFIED)?;

    socket.send_to(b"multicast msg", (multicast_addr, 5353))?;

    let mut buf = [0; 1024];
    let (bytes, src) = socket.recv_from(&mut buf)?;
    println!("多播从 {}: {:?}", src, &buf[..bytes]);

    socket.leave_multicast_v4(&multicast_addr, &Ipv4Addr::UNSPECIFIED)?;
    Ok(())
}
```

- **解释**：`join_multicast_v4` 加入组。`leave_multicast_v4` 离开。扩展：用 loopback_mode 控制本地循环。

### 示例：UDP 广播（扩展广播）
```rust
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    socket.send_to(b"broadcast", "255.255.255.255:12345")?;
    Ok(())
}
```

- **解释**：`set_broadcast` 启用广播。目标 255.255.255.255 是广播地址。陷阱：防火墙可能阻挡。

## 5. 高级主题：Socket 选项、错误处理和集成

- 选项：set_ttl、set_reuse_address 等。
- 错误：分类处理。

### 示例：高级 socket 选项（TCP/UDP 扩展）
```rust
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_ttl(10)?;  // 时间生存
    socket.set_reuse_address(true)?;  // 复用地址
    println!("TTL: {:?}", socket.ttl()?);
    Ok(())
}
```

- **解释**：`set_ttl` 设置包生存跳数。`set_reuse_address` 允许复用端口。扩展：TCP 用 set_linger 控制关闭。

### 示例：详细错误处理（连接重试扩展）
```rust
use std::net::TcpStream;
use std::io;
use std::time::Duration;
use std::thread::sleep;

fn connect_retry(addr: &str, retries: u32, delay: Duration) -> io::Result<TcpStream> {
    let mut last_err = None;
    for _ in 0..retries {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                last_err = Some(e);
                match e.kind() {
                    io::ErrorKind::ConnectionRefused | io::ErrorKind::TimedOut => sleep(delay),
                    _ => return Err(e),
                }
            }
        }
    }
    Err(last_err.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "未知错误")))
}

fn main() {
    match connect_retry("127.0.0.1:8080", 5, Duration::from_secs(2)) {
        Ok(_) => println!("连接成功"),
        Err(e) => println!("失败: {} ({:?})", e, e.kind()),
    }
}
```

- **解释**：重试特定错误。`kind()` 分类。扩展：日志 raw_os_error() 的 OS 码。

### 示例：与 std::io 和 thread 集成（多路服务器扩展）
```rust
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let mut writer = stream;
            writer.write_all(b"响应\n").unwrap();
        });
    }
    Ok(())
}
```

- **解释**：集成 BufReader 读取行。线程处理并发。性能：线程池代替 spawn 以限线程数。

## 6. 最佳实践和常见陷阱

- **地址最佳实践**：用 "0.0.0.0" 监听所有；解析用 to_socket_addrs 处理多 IP。
- **性能陷阱**：阻塞 connect/accept 慢，用 set_nonblocking 和 select（用 mio crate）；小包用 nodelay 减延迟。
- **错误最佳实践**：分类 kind()；重试 Transient 如 TimedOut；日志完整 e（包括 os_error）。
- **安全性**：验证地址避免注入；用 shutdown 优雅关闭；防火墙考虑端口。
- **跨平台扩展**：Windows IPv6 需要启用；Unix socket 路径用 UnixStream（std::os::unix::net）。
- **测试扩展**：用 localhost 测试；mock socket 用 Cursor 测试 Read/Write。
- **资源管理**：drop 时关闭，但显式 shutdown 好；用 try_clone 复制 stream。
- **常见错误扩展**：
    - AddrInUse：检查端口占用，用 reuse_address。
    - ConnectionReset：对端关闭，重连。
    - InvalidInput：地址格式错，用 parse 检查。
    - NotConnected：未 connect 前 read/write。

## 7. 练习建议

1. 编写 echo 服务器：用 TcpListener 处理多客户端，用 thread 池。
2. 实现 UDP 聊天：用 UdpSocket 发送/接收，处理来源。
3. 创建端口扫描器：用 connect_timeout 检查端口开放。
4. 处理 IPv6 服务器：用 to_socket_addrs 绑定 v4/v6 双栈。
5. 基准测试：比较 TCP vs UDP 传输大数据时间，用 Instant。
6. 与 io 集成：用 BufReader 解析 HTTP 头从 TcpStream。
7. 错误模拟：用 mock 错误测试重试逻辑。
