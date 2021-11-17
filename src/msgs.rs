pub mod msgs {
	use tungstenite::{WebSocket, client::AutoStream};

	pub fn get_msg(socket: &mut WebSocket<AutoStream>) -> String {
		let msg = (*socket).read_message().expect("Error reading message");
		let msg = match msg {
			tungstenite::Message::Text(s) => s,
			_ => {
				panic!("Error getting text");
			}
		};
		msg
	}
}