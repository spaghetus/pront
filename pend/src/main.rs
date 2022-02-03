use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	eprintln!("This is a dummy pend server and does nothing.");

	let listen_addr = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:23".to_string());
	let listener = TcpListener::bind(listen_addr).await?;
	while let Ok(incoming) = listener.accept().await {
		let incoming = incoming.0;
		loop {
			incoming.readable().await?;
			let mut buf = Vec::with_capacity(4096);
			match incoming.try_read(&mut buf) {
				Ok(0) => break,
				Ok(n) => {
					println!("read {} bytes", n);
				}
				Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
					continue;
				}
				_ => {}
			};
		}
	}
	Ok(())
}
