use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	eprintln!("Begin");
	let width: usize = std::env::var("WIDTH")
		.unwrap_or("80".to_string())
		.parse()
		.unwrap();
	let tab_width: usize = std::env::var("TAB_WIDTH")
		.unwrap_or("4".to_string())
		.parse()
		.unwrap();
	let listen_addr = std::env::var("LISTEN").unwrap_or("0.0.0.0:23".to_string());
	let post_url = std::env::var("POST_URL").unwrap_or("http://pland/txt".to_string());
	let listener = TcpListener::bind(listen_addr)
		.await
		.expect("Couldn't listen");
	let mut x = 0;
	let client = reqwest::Client::new();
	while let Ok(incoming) = listener.accept().await {
		async {
			let incoming = incoming.0;
			eprintln!("{} opened", incoming.peer_addr().unwrap());
			let mut buf = [0; 1024];
			while let Ok(n) = incoming.try_read(&mut buf) {
				if n == 0 {
					continue;
				}
				let to_send: String = buf[..n]
					.iter()
					// Convert to chars
					.map(|&x| std::char::from_u32(x as u32).unwrap())
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
				client.post(&post_url).body(to_send).send().await.unwrap();
			}
			eprintln!("{} closed", incoming.peer_addr().unwrap());
		}
		.await
	}
}
