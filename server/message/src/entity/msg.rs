use byteorder::ByteOrder;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
    pub head: Head,
    pub payload: Vec<u8>,
}

pub const HEAD_LEN: usize = 37;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Head {
    pub length: u16,
    pub typ: Type,
    pub sender: u64,
    pub receiver: u64,
    pub timestamp: u64,
    pub seq_num: u64,
    pub version: u16,
}

impl From<&[u8]> for Head {
    fn from(buf: &[u8]) -> Self {
        Self {
            length: byteorder::BigEndian::read_u16(&buf[0..2]),
            typ: Type::from_i8(buf[2] as i8),
            sender: byteorder::BigEndian::read_u64(&buf[3..11]),
            receiver: byteorder::BigEndian::read_u64(&buf[11..19]),
            timestamp: byteorder::BigEndian::read_u64(&buf[19..27]),
            seq_num: byteorder::BigEndian::read_u64(&buf[27..35]),
            version: byteorder::BigEndian::read_u16(&buf[35..37]),
        }
    }
}

impl Head {
    pub fn as_bytes(&self) -> Box<[u8]> {
        let mut array: [u8;HEAD_LEN] = [0;HEAD_LEN];
        let mut buf = &mut array[..];
        // 网络传输选择大端序，大端序符合人类阅读，小端序地位低地址，符合计算机计算
        byteorder::BigEndian::write_u16(&mut buf[0..2], self.length);
        buf[2] = self.typ.value() as u8;
        byteorder::BigEndian::write_u64(&mut buf[3..11], self.sender);
        byteorder::BigEndian::write_u64(&mut buf[11..19], self.receiver);
        byteorder::BigEndian::write_u64(&mut buf[19..27], self.timestamp);
        byteorder::BigEndian::write_u64(&mut buf[27..35], self.seq_num);
        byteorder::BigEndian::write_u16(&mut buf[35..37], self.version);
        Box::new(array)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    NA,
    // 消息部分
    Text,
    Meme,
    Image,
    Video,
    Audio,
    File,
    // 逻辑部分
    Ack,
    Sync,
    Offline,
    Heartbeat,
    Auth
}

impl Type {
    pub fn from_i8(value: i8) -> Self {
        match value {
            1 => Type::Text,
            2 => Type::Meme,
            3 => Type::Image,
            4 => Type::Video,
            5 => Type::Audio,
            6 => Type::File,
            7 => Type::Ack,
            8 => Type::Sync,
            9 => Type::Offline,
            10 => Type::Heartbeat,
            11 => Type::Auth,
            _ => Type::NA
        }
    }

    pub fn value(&self) -> i8 {
        match *self {
            Type::Text => 1,
            Type::Meme => 2,
            Type::Image => 3,
            Type::Video => 4,
            Type::Audio => 5,
            Type::File => 6,
            Type::Ack => 7,
            Type::Sync => 8,
            Type::Offline => 9,
            Type::Heartbeat => 10,
            Type::Auth => 11,
            _ => 0
        }
    }
}

impl Default for Msg {
    fn default() -> Self {
        Msg {
            head: Head {
                length: 12,
                typ: Type::Text,
                sender: 1234,
                receiver: 4321,
                timestamp: 0,
                seq_num: 0,
                version: 1,
            },
            payload: Vec::from("codewithbuff"),
        }
    }
}

impl Msg {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.head.length as usize + HEAD_LEN);
        buf.extend_from_slice(&self.head.as_bytes()[0..HEAD_LEN]);
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn is_ping(&self) -> bool {
        todo!()
    }

    pub fn pong() -> Self {
        todo!()
    }
}

impl From<&[u8]> for Msg {
    fn from(buf: &[u8]) -> Self {
        Self {
            head: Head::from(buf),
            payload: Vec::from(&buf[HEAD_LEN..]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Msg;

    #[test]
    fn test() {
        let msg = Msg::default();
        println!("{:?}", msg);
        let bytes = msg.as_bytes();
        let buf = bytes.as_slice();
        let msg1 = Msg::from(buf);
        println!("{:?}", msg1);
    }
}