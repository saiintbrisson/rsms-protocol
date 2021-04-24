use misc::prelude::{
    BlockPosition, ChatComponent, ChatMode, ChatPosition, ChunkPosition, Difficulty, Dimension,
    EntityLocation, GameMode, Property, Vec2D, Vec3D,
};
use protocol_internal::{ProtocolSupportDeserializer, ProtocolSupportSerializer, VarNum};
use uuid::Uuid;

use super::macros::packet_enum;

packet_enum!(client_bound, ClientBound =>
    0x00 => KeepAlive {
        #[protocol_field(varnum)]
        keep_alive_id: i32
    },
    0x01 => JoinGame {
        entity_id: i32,
        game_mode: GameMode,
        dimension: Dimension,
        difficulty: Difficulty,
        max_players: u8,
        level_type: String,
        reduced_debug_info: bool
    },
    0x02 => ChatMessage {
        json_data: ChatComponent<'static>,
        position: ChatPosition
    },
    0x05 => SpawnPosition {
        #[protocol_field(position)]
        location: EntityLocation
    },
    0x08 => PlayerPositionAndLook {
        entity_location: EntityLocation,
        flags: u8
    },
    0x21 => ChunkData {
        position: ChunkPosition,
        ground_up_continuous: bool,
        primary_bit_mask: u16,
        data: Vec<u8>
    },
    0x22 => MultiBlockChange {
        chunk: ChunkPosition,
        records: Vec<MultiBlockChangeRecord>;
        items {
            #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
            pub struct MultiBlockChangeRecord {
                pub horizontal_position: u8,
                pub y_coordinate: u8,
                #[protocol_field(varnum)]
                pub block_id: i32,
            }
        }
    },
    0x23 => BlockChange {
        #[protocol_field(position)]
        location: BlockPosition,
        #[protocol_field(varnum)]
        block_id: i32
    },
    0x24 => BlockAction {
        #[protocol_field(position)]
        location: BlockPosition,
        extra: u16,
        #[protocol_field(varnum)]
        block_type: i32
    },
    0x25 => BlockBreakAnimation {
        #[protocol_field(varnum)]
        entity_id: i32,
        #[protocol_field(position)]
        location: BlockPosition,
        destroy_stage: i8
    },
    0x26 => MapChunkBulk {
        sky_light_sent: bool,
        meta: Vec<ChunkMeta>,
        data: Vec<Vec<u8>>;
        items {
            #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
            pub struct ChunkMeta {
                pub position: ChunkPosition,
                pub primary_bit_mask: u16,
            }
        };
        support {
            fn calculate_len(&self) -> usize {
                1
                    + protocol_internal::VarNum::<i32>::calculate_len(&(self.meta.len() as i32))
                    + protocol_internal::DynArray::calculate_len(&self.meta)
                    + protocol_internal::DynArray::calculate_len(&self.data)
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                protocol_internal::ProtocolSupportSerializer::serialize(&self.sky_light_sent, dst)?;
                protocol_internal::VarNum::<i32>::serialize(&(self.meta.len() as i32), dst)?;
                protocol_internal::DynArray::serialize(&self.meta, dst)?;
                protocol_internal::DynArray::serialize(&self.data, dst)
            }

            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                let sky_light_sent = protocol_internal::ProtocolSupportDeserializer::deserialize(src)?;
                let len = protocol_internal::VarNum::<i32>::deserialize(src)? as usize;

                let meta = protocol_internal::FixedVec::deserialize(src, len)?;
                let data = protocol_internal::FixedVec::deserialize(src, len)?;

                Ok(Self {
                    sky_light_sent,
                    meta,
                    data
                })
            }
        }
    },
    0x38 => PlayerListItem {
        players: Vec<(Uuid, PlayerListItemAction)>;
        items {
            #[derive(Clone, Debug)]
            pub enum PlayerListItemAction {
                AddPlayer {
                    name: String,
                    properties: Vec<Property>,
                    game_mode: GameMode,
                    ping: i32,
                    display_name: Option<ChatComponent<'static>>,
                },
                UpdateGameMode(GameMode),
                UpdateLatency(i32),
                UpdateDisplayName(Option<ChatComponent<'static>>),
                RemovePlayer,
            }
            impl Default for PlayerListItemAction {
                fn default() -> Self { Self::RemovePlayer }
            }
        };
        support {
            fn calculate_len(&self) -> usize {
                self
                    .players
                    .iter()
                    .fold(0, |acc, (id, action)| acc + id.calculate_len() + action.calculate_len())
            }

            fn serialize<W: std::io::Write>(&self, _dst: &mut W) -> std::io::Result<()> {
                todo!()
            }

            fn deserialize<R: std::io::Read>(_src: &mut R) -> std::io::Result<Self> {
                todo!()
            }
        }
    },
    0x3B => ScoreboardObjective {
        objective_name: String,
        mode: ScoreboardObjectiveMode;
        items {
            #[derive(Clone, Debug)]
            pub enum ScoreboardObjectiveMode {
                Create(ScoreboardObjectiveInfo),
                Remove,
                Update(ScoreboardObjectiveInfo),
            }
            impl ScoreboardObjectiveMode {
                fn get_id(&self) -> u8 {
                    match self {
                        Self::Create(_) => 0,
                        Self::Remove => 1,
                        Self::Update(_) => 2,
                    }
                }
            }
            impl ProtocolSupportSerializer for ScoreboardObjectiveMode {
                fn calculate_len(&self) -> usize {
                    1 + match self {
                        Self::Create(info) => info.calculate_len(),
                        Self::Remove => 0,
                        Self::Update(info) => info.calculate_len(),
                    }
                }
                fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                    self.get_id().serialize(dst)?;
                    match self {
                        Self::Create(info) => info.serialize(dst),
                        Self::Remove => Ok(()),
                        Self::Update(info) => info.serialize(dst),
                    }
                }
            }
            impl ProtocolSupportDeserializer for ScoreboardObjectiveMode {
                fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                    Ok(match u8::deserialize(src)? {
                        0 => Self::Create(ScoreboardObjectiveInfo::deserialize(src)?),
                        1 => Self::Remove,
                        2 => Self::Update(ScoreboardObjectiveInfo::deserialize(src)?),
                        _ => panic!(),
                    })
                }
            }
            impl Default for ScoreboardObjectiveMode {
                fn default() -> Self { Self::Remove }
            }
            #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
            pub struct ScoreboardObjectiveInfo {
                objective_value: String,
                objective_type: String,
            }
        }
    },
    0x3C => UpdateScore {
        score_name: String,
        action: UpdateScoreAction;
        items {
            #[derive(Clone, Debug)]
            pub enum UpdateScoreAction {
                Create_Update {
                    objective_name: String,
                    value: i32,
                },
                Remove(String),
            }
            impl UpdateScoreAction {
                fn get_id(&self) -> u8 {
                    match self {
                        Self::Create_Update { .. } => 0,
                        Self::Remove(_) => 1,
                    }
                }
            }
            impl Default for UpdateScoreAction {
                fn default() -> Self { Self::Remove(String::new()) }
            }
            impl ProtocolSupportSerializer for UpdateScoreAction {
                fn calculate_len(&self) -> usize {
                    1 + match self {
                        Self::Create_Update { objective_name, value } => objective_name.calculate_len() + protocol_internal::VarNum::<i32>::calculate_len(value),
                        Self::Remove(objective_name) => objective_name.calculate_len(),
                    }
                }
                fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                    self.get_id().serialize(dst)?;
                    match self {
                        Self::Create_Update { objective_name, value } => {
                            objective_name.serialize(dst)?;
                            protocol_internal::VarNum::<i32>::serialize(value, dst)
                        },
                        Self::Remove(objective_name) => objective_name.serialize(dst),
                    }
                }
            }
            impl ProtocolSupportDeserializer for UpdateScoreAction {
                fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                    Ok(match u8::deserialize(src)? {
                        0 => Self::Create_Update {
                            objective_name: String::deserialize(src)?,
                            value: protocol_internal::VarNum::<i32>::deserialize(src)?,
                        },
                        1 => Self::Remove(String::deserialize(src)?),
                        _ => panic!(),
                    })
                }
            }
        }
    },
    0x3D => DisplayScoreboard {
        position: DisplayScoreboardPosition,
        score_name: String;
        items {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
            pub enum DisplayScoreboardPosition {
                List = 0,
                Sidebar = 1,
                BelowName = 2,
            }
            impl Default for DisplayScoreboardPosition {
                fn default() -> Self { Self::Sidebar }
            }
        }
    },
    0x3E => Teams {
        team_name: String,
        mode: TeamsMode;
        items {
            #[repr(u8)]
            #[derive(Clone, Debug)]
            pub enum TeamsMode {
                Create {
                    info: TeamInfo,
                    players: Vec<String>,
                },
                Remove,
                InfoUpdate(TeamInfo),
                AddPlayers(Vec<String>),
                RemovePlayers(Vec<String>),
            }
            impl TeamsMode {
                fn get_id(&self) -> u8 {
                    match self {
                        Self::Create { .. } => 0,
                        Self::Remove => 1,
                        Self::InfoUpdate(_) => 2,
                        Self::AddPlayers(_) => 3,
                        Self::RemovePlayers(_) => 4,
                    }
                }
            }
            impl Default for TeamsMode {
                fn default() -> Self { Self::Remove }
            }
            impl ProtocolSupportSerializer for TeamsMode {
                fn calculate_len(&self) -> usize {
                    1 + match self {
                        Self::Create { info, players } => info.calculate_len() + players.calculate_len(),
                        Self::Remove => 0,
                        Self::InfoUpdate(info) => info.calculate_len(),
                        Self::AddPlayers(players) => players.calculate_len(),
                        Self::RemovePlayers(players) => players.calculate_len(),
                    }
                }
                fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                    self.get_id().serialize(dst)?;
                    match self {
                        Self::Create { info, players } => {
                            info.serialize(dst)?;
                            players.serialize(dst)
                        },
                        Self::Remove => Ok(()),
                        Self::InfoUpdate(info) => info.serialize(dst),
                        Self::AddPlayers(players) => players.serialize(dst),
                        Self::RemovePlayers(players) => players.serialize(dst),
                    }
                }
            }
            impl ProtocolSupportDeserializer for TeamsMode {
                fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                    Ok(match u8::deserialize(src)? {
                        0 => Self::Create {
                            info: TeamInfo::deserialize(src)?,
                            players: Vec::<String>::deserialize(src)?,
                        },
                        1 => Self::Remove,
                        2 => Self::InfoUpdate(TeamInfo::deserialize(src)?),
                        3 => Self::AddPlayers(Vec::<String>::deserialize(src)?),
                        4 => Self::RemovePlayers(Vec::<String>::deserialize(src)?),
                        _ => panic!(),
                    })
                }
            }
            #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
            pub struct TeamInfo {
                team_display_name: String,
                team_prefix: String,
                team_suffix: String,
                friendly_fire: FriendlyFire,
                name_tag_visibility: String,
                color: misc::prelude::ChatColor,
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
            pub enum FriendlyFire {
                Off = 0,
                On = 1,
                ShowInvisible = 3,
            }
        }
    },
    0x3F => PluginMessage {
        channel: String,
        #[protocol_field(dynarray)]
        data: Vec<u8>
    },
    0x40 => Disconnect {
        reason: ChatComponent<'static>
    },
    0x45 => Title {
        action: TitleAction;
        items {
            #[derive(Clone, Debug)]
            pub enum TitleAction {
                SetTitle(ChatComponent<'static>),
                SetSubTitle(ChatComponent<'static>),
                SetTimesAndDisplay {
                    fade_in: i32,
                    stay: i32,
                    fade_out: i32,
                },
                Hide,
                Reset
            }
            impl TitleAction {
                pub fn get_id(&self) -> i32 {
                    match self {
                        TitleAction::SetTitle(_) => 0,
                        TitleAction::SetSubTitle(_) => 1,
                        TitleAction::SetTimesAndDisplay { .. } => 2,
                        TitleAction::Hide => 3,
                        TitleAction::Reset => 4,
                    }
                }
            }
            impl Default for TitleAction {
                fn default() -> Self { Self::Reset }
            }
        };
        support {
            fn calculate_len(&self) -> usize {
                match &self.action {
                    TitleAction::SetTitle(chat) => { 1 + chat.calculate_len() },
                    TitleAction::SetSubTitle(chat) => { 1 + chat.calculate_len() },
                    TitleAction::SetTimesAndDisplay { .. } => 13,
                    TitleAction::Hide => 1,
                    TitleAction::Reset => 1,
                }
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                let id = &self.action.get_id();

                protocol_internal::VarNum::<i32>::serialize(id, dst)?;
                match &self.action {
                    TitleAction::SetTitle(chat) => { chat.serialize(dst) },
                    TitleAction::SetSubTitle(chat) => { chat.serialize(dst) },
                    TitleAction::SetTimesAndDisplay { fade_in, stay, fade_out } => {
                        fade_in.serialize(dst)?;
                        stay.serialize(dst)?;
                        fade_out.serialize(dst)
                    },
                    TitleAction::Hide => Ok(()),
                    TitleAction::Reset => Ok(()),
                }
            }

            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                Ok(Self {
                    action: match protocol_internal::VarNum::<i32>::deserialize(src)? {
                        0 => TitleAction::SetTitle(ChatComponent::deserialize(src)?),
                        1 => TitleAction::SetSubTitle(ChatComponent::deserialize(src)?),
                        2 => TitleAction::SetTimesAndDisplay {
                            fade_in: i32::deserialize(src)?,
                            stay: i32::deserialize(src)?,
                            fade_out: i32::deserialize(src)?,
                        },
                        3 => TitleAction::Hide,
                        4 => TitleAction::Reset,
                        _ => panic!(),
                    }
                })
            }
        }
    },
    0x47 => PlayerListHeaderAndFooter {
        header: ChatComponent<'static>,
        footer: ChatComponent<'static>
    }
);

impl ProtocolSupportSerializer for client_bound::PlayerListItemAction {
    fn calculate_len(&self) -> usize {
        use client_bound::PlayerListItemAction;
        match self {
            PlayerListItemAction::AddPlayer {
                name,
                properties,
                game_mode,
                ping,
                display_name,
            } => {
                name.calculate_len()
                    + properties.calculate_len()
                    + game_mode.calculate_len()
                    + VarNum::<i32>::calculate_len(ping)
                    + display_name.calculate_len()
            }
            PlayerListItemAction::UpdateGameMode(game_mode) => game_mode.calculate_len(),
            PlayerListItemAction::UpdateLatency(ping) => VarNum::<i32>::calculate_len(ping),
            PlayerListItemAction::UpdateDisplayName(display_name) => display_name.calculate_len(),
            PlayerListItemAction::RemovePlayer => 0,
        }
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        use client_bound::PlayerListItemAction;
        match self {
            PlayerListItemAction::AddPlayer {
                name,
                properties,
                game_mode,
                ping,
                display_name,
            } => {
                name.serialize(dst)?;
                properties.serialize(dst)?;
                game_mode.serialize(dst)?;
                VarNum::<i32>::serialize(ping, dst)?;
                display_name.serialize(dst)
            }
            PlayerListItemAction::UpdateGameMode(game_mode) => game_mode.serialize(dst),
            PlayerListItemAction::UpdateLatency(ping) => VarNum::<i32>::serialize(ping, dst),
            PlayerListItemAction::UpdateDisplayName(display_name) => display_name.serialize(dst),
            PlayerListItemAction::RemovePlayer => Ok(()),
        }
    }
}

packet_enum!(server_bound, ServerBound =>
    0x00 => KeepAlive {
        #[protocol_field(varnum)]
        keep_alive_id: i32
    },
    0x01 => ChatMessage {
        #[protocol_field(range(min = 1, max = 100))]
        message: String
    },
    0x03 => Player {
        on_ground: bool
    },
    0x04 => PlayerPosition {
        position: Vec3D<f64>,
        on_ground: bool
    },
    0x05 => PlayerLook {
        look: Vec2D<f32>,
        on_ground: bool
    },
    0x06 => PlayerPositionAndLook {
        entity_location: EntityLocation,
        on_ground: bool
    },
    0x07 => PlayerDigging {
        status: PlayerDiggingStatus,
        #[protocol_field(position)]
        location: Vec3D<i32>,
        face: i8;
        items {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
            pub enum PlayerDiggingStatus {
                StartedDigging = 0,
                CancelledDigging = 1,
                FinishedDigging = 2,
                DropItemStack = 3,
                DropItem = 4,
                ShootArrow_FinishEating = 5,
            }
            impl Default for PlayerDiggingStatus {
                fn default() -> Self {
                    Self::StartedDigging
                }
            }
        }
    },
    0x09 => HeldItemChange {
        slot: i16
    },
    0x0A => Animation {},
    0x0B => EntityAction {
        #[protocol_field(varnum)]
        entity_id: i32,
        action: EntityActionType,
        #[protocol_field(varnum)]
        action_parameter: i32;
        items {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
            pub enum EntityActionType {
                StartSneaking = 0,
                StopSneaking = 1,
                LeaveBed = 2,
                StartSprinting = 3,
                StopSprinting = 4,
                JumpWithHorse = 5,
                OpenRiddenHorseInventory = 6,
            }
            impl Default for EntityActionType {
                fn default() -> Self {
                    Self::StartSneaking
                }
            }
        }
    },
    0x0D => CloseWindow {
        window_id: u8
    },
    0x10 => CreativeInventoryAction {
        slot: i16,
        #[protocol_field(dynarray)]
        clicked_item: Vec<u8>
    },
    0x15 => ClientSettings {
        locale: String,
        view_distance: i8,
        chat_mode: ChatMode,
        chat_colors: bool,
        displayed_skin_parts: u8
    },
    0x16 => ClientStatus {
        action: ClientStatusAction;
        items {
            #[repr(u8)]
            #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
            pub enum ClientStatusAction {
                PerformRespawn = 0,
                RequestStats = 1,
                TakingInventoryAchievement = 2,
            }
            impl Default for ClientStatusAction {
                fn default() -> Self {
                    Self::PerformRespawn
                }
            }
        }
    },
    0x17 => PluginMessage {
        channel: String,
        #[protocol_field(dynarray)]
        data: Vec<u8>
    }
);

#[cfg(test)]
mod test {
    use protocol_internal::{PacketSerializer, ProtocolSupportSerializer};

    #[test]
    fn test_join_game() {
        let join_game = super::ClientBound::JoinGame(super::client_bound::JoinGame::default());
        assert_eq!(ProtocolSupportSerializer::calculate_len(&join_game), 10);
        assert_eq!(PacketSerializer::calculate_len(&join_game), 11);
    }
}
