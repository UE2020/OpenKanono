use crate::types;
use std::io::Result;

#[derive(Debug)]
pub enum ClientboundPacket<'entities> {
    RoomInfo {
        width: u16,
        height: u16,
        mode: String,
        accounts_enabled: bool,
        border_style: u16,
    },
    Identifier(u32),
    Census {
        entities: Vec<&'entities dyn crate::simulation::entity::Entity>,
    },
    Joining,
    CameraUpdate {
        x: i32,
        y: i32,
        fov: f32,
    },
    CmdOutput(String),
    EntityTypes(String),
    Death(u16),
    Message {
        message: String,
        color: types::Color,
    },
    TankUpgrade(u16),
    UpgradeReset,
    LeaderBoard {
        leaderboard: Vec<LeaderboardEntry>,
    },
    Kill,
    Skill(u8),
    Account,
    DominationColors([types::Color; 4]),
    Audio(u8),
    GameEvent(u8),
}

#[derive(Debug)]
pub struct LeaderboardEntry {
    pub id: u32,
    pub score: u32,
    pub name: String,
    pub class: u16,
    pub color: types::Color,
}

impl<'entities> ClientboundPacket<'entities> {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ClientboundPacket::RoomInfo {
                width,
                height,
                mode,
                accounts_enabled,
                border_style,
            } => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x0);
                buf.put_u16(*width);
                buf.put_u16(*height);
                buf.put_utf8(&mode);
                buf.put_u8(*accounts_enabled as u8);
                buf.put_u16(*border_style);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Identifier(id) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x1);
                buf.put_u32(*id);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Census { entities } => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x2);
                buf.put_u32(entities.len() as u32);
                dbg!(entities.len());
                for entity in entities {
                    buf.put_u32(entity.get_id() as u32);
                    buf.put_32(entity.get_x() as i32);
                    buf.put_32(entity.get_y() as i32);
                    buf.put_utf8(entity.get_name());
                    buf.put_float(entity.get_angle());
                    buf.put_float(entity.get_radius());
                    buf.put_u32(entity.get_level());
                    buf.put_u32(entity.get_score());
                    buf.put_u16(entity.get_class());
                    buf.put_u8(entity.get_color() as u8);
                    buf.put_u8(entity.show_name() as u8);
                    buf.put_u8(entity.show_health() as u8);
                    buf.put_u16(0); // no barrels for now
                    buf.put_u8((entity.get_alpha() * 100.0) as u8);
                    buf.put_float(entity.get_velocity().x);
                    buf.put_float(entity.get_velocity().y);
                    buf.put_float(entity.get_health());
                    buf.put_u8(entity.barrel_flash() as u8);
                    buf.put_u8(entity.shield_flash() as u8);
                    buf.put_u8(entity.can_move_through_border() as u8);
                }
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Joining => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x3);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::CameraUpdate { x, y, fov } => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x4);
                buf.put_32(*x);
                buf.put_32(*y);
                buf.put_float(*fov);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::CmdOutput(output) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x5);
                buf.put_utf8(&output);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::EntityTypes(types) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x6);
                buf.put_utf8(&types);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Death(id) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x7);
                buf.put_u16(*id);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Message { message, color } => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x8);
                buf.put_utf8(&message);
                buf.put_u8(*color as u8);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::TankUpgrade(id) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x9);
                buf.put_u16(*id);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::UpgradeReset => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0xA);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::LeaderBoard { leaderboard } => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0xB);
                buf.put_u8(leaderboard.len() as u8);
                for entry in leaderboard {
                    buf.put_u32(entry.id);
                    buf.put_u32(entry.score);
                    buf.put_utf8(&entry.name);
                    buf.put_u16(entry.class);
                    buf.put_u8(entry.color as u8);
                }
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Kill => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0xC);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Skill(skill) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0xD);
                buf.put_u8(*skill);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Account => todo!("Add accounts once they're on mainline kanono"),
            ClientboundPacket::DominationColors(colors) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0xF);
                for color in colors {
                    buf.put_u8(*color as u8);
                }
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::Audio(id) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x10);
                buf.put_u8(*id);
                buf.cursor.get_ref().to_vec()
            }
            ClientboundPacket::GameEvent(event) => {
                let mut buf = crate::binary::StreamPeerBuffer::new();
                buf.put_u8(0x11);
                buf.put_u8(*event);
                buf.cursor.get_ref().to_vec()
            }
        }
    }
}

#[derive(Debug)]
pub enum ServerboundPacket {
    Input {
        left: bool,
        right: bool,
        up: bool,
        down: bool,
        angle: f32,
        lmb: bool,
        mx: i16,
        my: i16,
        rmb: bool,
    },

    Spawn(String),
    Cmd(String),
    LevelUp,
    Ping,
    SkillUpgrade(u8),
    TankUpgrade(u8),
    Login {
        typ: LoginType,
        name: String,
        password: String,
    },
    Version(u16),
}

#[derive(Debug)]
pub enum LoginType {
    Login,
    Register,
}

impl TryFrom<u8> for LoginType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Login),
            0 => Ok(Self::Register),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid logintype",
            )),
        }
    }
}

impl ServerboundPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut buf = crate::binary::StreamPeerBuffer::new();
        buf.set_data_array(bytes.to_vec());
        let packet_id = buf.get_u8()?;
        match packet_id {
            0x0 => Ok(ServerboundPacket::Input {
                left: buf.get_u8()? != 0,
                right: buf.get_u8()? != 0,
                up: buf.get_u8()? != 0,
                down: buf.get_u8()? != 0,
                angle: buf.get_float()?,
                lmb: buf.get_u8()? != 0,
                mx: buf.get_16()?,
                my: buf.get_16()?,
                rmb: buf.get_u8()? != 0,
            }),
            0x1 => Ok(ServerboundPacket::Spawn(buf.get_utf8()?)),
            0x2 => Ok(ServerboundPacket::Cmd(buf.get_utf8()?)),
            0x3 => Ok(ServerboundPacket::LevelUp),
            0x4 => Ok(ServerboundPacket::Ping),
            0x5 => Ok(ServerboundPacket::SkillUpgrade(buf.get_u8()?)),
            0x6 => Ok(ServerboundPacket::TankUpgrade(buf.get_u8()?)),
            0x7 => Ok(ServerboundPacket::Login {
                typ: LoginType::try_from(buf.get_u8()?)?,
                name: buf.get_utf8()?,
                password: buf.get_utf8()?,
            }),
            0x8 => Ok(ServerboundPacket::Version(buf.get_u16()?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid packet id",
            )),
        }
    }
}
