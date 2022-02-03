use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	eprintln!("Begin");
	let width: usize = std::env::var("WIDTH")
		.unwrap_or_else(|_| "80".to_string())
		.parse()?;
	let tab_width: usize = std::env::var("TAB_WIDTH")
		.unwrap_or_else(|_| "4".to_string())
		.parse()?;
	let listen_addr = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:23".to_string());
	let post_url = std::env::var("POST_URL").unwrap_or_else(|_| "http://pland/txt".to_string());
	let listener = TcpListener::bind(listen_addr).await?;
	let mut x = 0;
	let client = reqwest::Client::new();
	while let Ok(incoming) = listener.accept().await {
		let _result: Result<(), Box<dyn std::error::Error>> = async {
			let incoming = incoming.0;
			eprintln!("{} opened", incoming.peer_addr()?);
			let mut buf = [0u8; 1024];
			loop {
				incoming.readable().await?;
				let n = match incoming.try_read(&mut buf) {
					Ok(0) => break,
					Ok(n) => n,
					Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
						continue;
					}
					Err(_) => break,
				};
				let to_send: String = buf[..n]
					.iter()
					// Convert to chars
					.flat_map(|&x| std::char::from_u32(x as u32))
					// Sanitize
					.flat_map(|c| match c {
						c if c.is_ascii_alphanumeric() => Some(c.to_string()),
						c if matches!(c, '\r' | '\n') => Some(c.to_string()),
						'\t' => Some(" ".repeat(tab_width)),
						_ => None,
					})
					// Wrap text
					.map(|s| {
						if x + s.len() > width {
							let x_ = x;
							x = 0;
							s.chars().take(width - x_).collect::<String>()
								+ "\n" + &s.chars().skip(width - x_).collect::<String>()
						} else {
							x += s.len();
							s
						}
					})
					.collect();
				print!("{}", to_send);
				client.post(&post_url).body(to_send).send().await?;
			}
			eprintln!("{} closed", incoming.peer_addr()?);
			Ok(())
		}
		.await;
		_result?
	}
	Ok(())
}
