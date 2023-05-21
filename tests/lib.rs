use assert_cmd::Command;
use std::thread;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_host_port_as_flags() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();
    cmd.arg("--host")
        .arg(addr.ip().to_string())
        .arg("--port")
        .arg(addr.port().to_string())
        .arg("--timeout")
        .arg("5");
    cmd.assert().success();
}

#[tokio::test]
async fn test_host_and_port_as_single_path() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
                                           //
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();

    let path = format!("{}:{}", addr.ip().to_string(), addr.port().to_string());

    cmd.arg(path).arg("--timeout").arg("5");

    cmd.assert().success();
}

#[tokio::test]
async fn test_host_conflicts_with_path_option() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();
    cmd.arg("localhost.com:3000")
        .arg("--host")
        .arg(addr.ip().to_string())
        .arg("--timeout")
        .arg("5");
    cmd.assert().failure();
}

#[tokio::test]
async fn test_port_conflicts_with_path_option() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();
    cmd.arg("localhost.com:3000")
        .arg("--port")
        .arg(addr.port().to_string())
        .arg("--timeout")
        .arg("5");
    cmd.assert().failure();
}

#[tokio::test]
async fn test_host_required() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();
    cmd.arg("localhost.com")
        .arg("--port")
        .arg(addr.port().to_string())
        .arg("--timeout")
        .arg("5");
    cmd.assert().failure();
}

#[tokio::test]
async fn test_port_required() {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();

    // Spawn a task to accept the connection
    tokio::spawn(async move {
        listener.accept().await.unwrap();
    });

    // Run the CLI tool to wait for the TCP server
    thread::sleep(Duration::from_secs(1)); // wait for the server to start
    let mut cmd = Command::cargo_bin("wait_for_it").unwrap();
    cmd.arg("--host")
        .arg(addr.ip().to_string())
        .arg("--timeout")
        .arg("5");
    cmd.assert().failure();
}
