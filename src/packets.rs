pub mod packet_base {

	pub struct Packet {
		pub byte: u8,
	}
	// Server commands
	pub const PACKET_REGISTER_CONNECTION: u8 = 161;
	pub const PACKET_TEAM_LIST: u8 = 115;
	pub const PACKET_HEART_BEAT: u8 = 108;
	pub const PACKET_SEND_CHAT: u8 = 141;
	pub const PACKET_SERVER_INFO: u8 = 106;
	pub const PACKET_START_GAME: u8 = 120;
	pub const PACKET_QUESTION: u8 = 117;
	pub const PACKET_QUESTION_RESPONCE: u8 = 118;

	//Client Commands
	pub const PACKET_PREREGISTER_CONNECTION: u8 = 160;
	pub const PACKET_HEART_BEAT_RESPONSE: u8 = 109;
	pub const PACKET_ADD_CHAT: u8 = 140;
	pub const PACKET_PLAYER_INFO: u8 = 110;
	pub const PACKET_DISCONNECT: u8 = 111;
	pub const PACKET_RANDY: u8 = 112;

	//Game Commands
	pub const PACKET_ADD_GAMECOMMAND: u8 = 20;
	pub const PACKET_TICK: u8 = 10;
	pub const PACKET_SYNC_CHECKSUM: u8 = 30;
	pub const PACKET_SYNC_CHECKSUM_RESPONCE: u8 = 31;
	pub const PACKET_SYNC: u8 = 35;

	fn string_packet(string: String) -> Vec<u8> {
		string.into_bytes()
	}

	pub fn pre_register() -> Vec<u8> {
		let mut result: Vec<u8> = vec![1, 176, 176];
		result.append(&mut string_packet(String::from("dixe.rustyserver")));
		result.append(&mut string_packet(String::from("Dixxe")));
		result.push(255);
		result.push(176);
		result.push(PACKET_REGISTER_CONNECTION);
		result	
	}

	pub fn gen_packet(byte: u8) -> Packet {
		Packet {
			byte,
		}
	}
}
