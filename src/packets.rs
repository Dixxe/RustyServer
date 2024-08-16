pub mod packet_base {
	enum PacketKind {
		RegisterConnection,
		TeamList,
		HeartBeat,
		SendChat,
		ServerInfo,
		StartGame,
		Question,
		QuestionResponce,

		PreregisterConnection,
		HeartBeatResponse,
		AddChat,
		PlayerInfo,
		Disconnect,
		Randy,

		AddGamecommand,
		Tick,
		SyncChecksum,
		SyncChecksumResponce,
		Sync,
	}
	struct Packet {
		kind: PacketKind,
		byte: u8,
	}
	pub const PACKET_REGISTER_CONNECTION: Packet = Packet{kind: PacketKind::RegisterConnection, byte: 161};
	pub const PACKET_TEAM_LIST: Packet = Packet{kind: PacketKind::TeamList, byte: 115};
	pub const PACKET_HEART_BEAT: Packet = Packet{kind: PacketKind::HeartBeat, byte: 108};
	pub const PACKET_SEND_CHAT: Packet  = Packet{kind: PacketKind::SendChat, byte: 141};
	pub const PACKET_SERVER_INFO: Packet  = Packet{kind: PacketKind::ServerInfo, byte: 106};
	pub const PACKET_START_GAME: Packet  = Packet{kind: PacketKind::StartGame, byte: 120};
	pub const PACKET_QUESTION: Packet  = Packet{kind: PacketKind::Question, byte: 117};
	pub const PACKET_QUESTION_RESPONCE: Packet  = Packet{kind: PacketKind::QuestionResponce, byte: 118};

	//Client Commands
	pub const PACKET_PREREGISTER_CONNECTION: Packet  = Packet{kind: PacketKind::PreregisterConnection, byte: 160};
	pub const PACKET_HEART_BEAT_RESPONSE: Packet  = Packet{kind: PacketKind::HeartBeatResponse, byte: 109};
	pub const PACKET_ADD_CHAT: Packet  = Packet{kind: PacketKind::AddChat, byte: 140};
	pub const PACKET_PLAYER_INFO: Packet  = Packet{kind: PacketKind::PlayerInfo, byte: 110};
	pub const PACKET_DISCONNECT: Packet  = Packet{kind: PacketKind::Disconnect, byte: 111};
	pub const PACKET_RANDY: Packet  = Packet{kind: PacketKind::Randy, byte: 112};

	//Game Commands
	pub const PACKET_ADD_GAMECOMMAND: Packet  = Packet{kind: PacketKind::AddGamecommand, byte: 20};
	pub const PACKET_TICK: Packet  = Packet{kind: PacketKind::Tick, byte: 10};
	pub const PACKET_SYNC_CHECKSUM: Packet  = Packet{kind: PacketKind::Sync, byte: 30};
	pub const PACKET_SYNC_CHECKSUM_RESPONCE: Packet  = Packet{kind: PacketKind::SyncChecksumResponce, byte: 31};
	pub const PACKET_SYNC: Packet  = Packet{kind: PacketKind::Sync, byte: 35};

	pub fn gen_packet(kind: PacketKind, byte: u8) -> Packet {
		Packet {
			kind,
			byte,
		}
	}
}
