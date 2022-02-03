use hyper::{
	server::Server,
	service::{make_service_fn, service_fn},
	Body, Response, StatusCode,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
	// Read config from env
	let default_margins: usize = std::env::var("MARGINS")
		.unwrap_or("40".to_string())
		.parse()
		.unwrap();
	let pend_address = std::env::var("PEND_ADDRESS").unwrap_or("pend:23".to_string());
	let listen_address = std::env::var("LISTEN").unwrap_or("0.0.0.0:80".to_string());
	// Ensure font cache exists
	assert!(std::fs::read_dir("/fonts").is_ok());
	// Open TCP socket to pend
	let pend = Arc::new(Mutex::new(
		tokio::net::TcpStream::connect(pend_address).await.unwrap(),
	));
	// Setup HTTP server
	let make_service = make_service_fn(move |_| {
		let pend = pend.clone();
		async move {
			Ok::<_, hyper::Error>(service_fn(move |req| {
				let path = req.uri().path().to_string();
				async move {
					let path = path.as_str();
					match path {
						"/txt" => {
							let mut res = Response::new(Body::from("Not implemented"));
							*res.status_mut() = StatusCode::NOT_IMPLEMENTED;
							Ok::<_, hyper::Error>(res)
						}
						"/pdf" => {
							let mut res = Response::new(Body::from("Not implemented"));
							*res.status_mut() = StatusCode::NOT_IMPLEMENTED;
							Ok::<_, hyper::Error>(res)
						}
						"/home" => {
							let mut res = Response::new(Body::from("Not implemented"));
							*res.status_mut() = StatusCode::NOT_IMPLEMENTED;
							Ok::<_, hyper::Error>(res)
						}
						_ => {
							let mut res = Response::new(Body::from("Not found"));
							*res.status_mut() = StatusCode::NOT_FOUND;
							Ok::<_, hyper::Error>(res)
						}
					}
				}
			}))
		}
	});

	let server = Server::bind(&listen_address.parse().unwrap()).serve(make_service);

	eprintln!("Listening on http://{}", listen_address);

	if let Err(e) = server.await {
		eprintln!("server error: {}", e);
	}
}
