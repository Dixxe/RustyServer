pub mod packet_base {

	struct Packet {
		pub byte: u8,
	}
	impl Packet {
		fn preRegister() {
			
		}
	}
	const PACKET_REGISTER_CONNECTION: u8 = 161;
	const PACKET_TEAM_LIST: u8 = 115;
	const PACKET_HEART_BEAT: u8 = 108;
	const PACKET_SEND_CHAT: u8 = 141;
	const PACKET_SERVER_INFO: u8 = 106;
	const PACKET_START_GAME: u8 = 120;
	const PACKET_QUESTION: u8 = 117;
	const PACKET_QUESTION_RESPONCE: u8 = 118;

	//Client Commands
	const PACKET_PREREGISTER_CONNECTION: u8 = 160;
	const PACKET_HEART_BEAT_RESPONSE: u8 = 109;
	const PACKET_ADD_CHAT: u8 = 140;
	const PACKET_PLAYER_INFO: u8 = 110;
	const PACKET_DISCONNECT: u8 = 111;
	const PACKET_RANDY: u8 = 112;

	//Game Commands
	const PACKET_ADD_GAMECOMMAND: u8 = 20;
	const PACKET_TICK: u8 = 10;
	const PACKET_SYNC_CHECKSUM: u8 = 30;
	const PACKET_SYNC_CHECKSUM_RESPONCE: u8 = 31;
	const PACKET_SYNC: u8 = 35;

	pub fn gen_packet(byte: u8) -> Packet {
		Packet {
			byte,
		}
	}
}
