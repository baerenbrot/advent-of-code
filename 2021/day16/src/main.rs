use std::env::args;
use bitreader::{BitReader,BitReaderError};
use hex::FromHex;

#[derive(Clone,Debug)]
enum Error {
    ArgumentMissing,
    ReadError,
    InvalidHexEncoding,
    RuntimeError,
    ParsingFailure(BitReaderError),
}

#[derive(Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum TypeId {
    Sum,
    Mul,
    Min,
    Max,
    CheckGT,
    CheckLT,
    CheckEQ,
}

#[derive(Clone,Copy,Hash,PartialEq,Eq)]
enum LengthType {
    TotalLength = 0,
    PacketCount = 1,
}

enum PacketBody {
    Literal(u64),
    Operator {
        method: TypeId,
        encoding: LengthType,
        packets: Vec<Packet>
    }
}

struct Packet {
    version: u8,
    body: PacketBody
}

impl LengthType {
    fn new(reader: &mut BitReader) -> bitreader::Result<Self> {
        match reader.read_u8(1)? {
            0 => Ok(LengthType::TotalLength),
            1 => Ok(LengthType::PacketCount),
            _ => panic!()
        }
    }
}

impl PacketBody {
    fn new(reader: &mut BitReader) -> bitreader::Result<Self> {
        let id = reader.read_u8(3)?;
        if id == 4 {
            let mut value: u64 = 0;
            loop {
                let done = reader.read_u8(1)? == 0;
                value *= 0b10000;
                value += reader.read_u64(4)?;
                if done { break; }
            }
            Ok(PacketBody::Literal(value))
        } else {
            let method = match id {
                0 => TypeId::Sum,
                1 => TypeId::Mul,
                2 => TypeId::Min,
                3 => TypeId::Max,
                5 => TypeId::CheckGT,
                6 => TypeId::CheckLT,
                7 => TypeId::CheckEQ,
                _ => panic!()
            };
            let encoding = LengthType::new(reader)?;
            let packets: Vec<_> = match encoding {
                LengthType::PacketCount => {
                    let t = reader.read_u16(11)?;
                    (0..t).map(|_| Packet::new(reader)).collect::<bitreader::Result<_>>()?
                },
                LengthType::TotalLength => {
                    let mut remaining = reader.read_u16(15)? as usize;
                    let mut packets: Vec<Packet> = Vec::new();
                    while remaining > 0 {
                        let packet = Packet::new(reader)?;
                        let length = packet.len();
                        packets.push(packet);
                        if remaining < length {
                            return Err(BitReaderError::NotEnoughData{
                                position: reader.position(),
                                length: 0,
                                requested: (length * 8) as u64,
                            });
                        } else {
                            remaining -= length;
                        }
                    }
                    packets
                }
            };
            Ok(PacketBody::Operator{
                packets,
                method,
                encoding,
            })
        }
    }

    fn len(&self) -> usize {
        3 + match self {
            PacketBody::Literal(value) => {
                let mut result: usize = 0;
                let mut remaining = *value;
                while remaining > 0 {
                    result += 5;
                    remaining /= 0b10000;
                }
                result
            },
            PacketBody::Operator{encoding,method:_,packets} => {
                let result: usize = packets.iter().map(|p| p.len()).sum();
                result + match encoding {
                    LengthType::PacketCount => 12,
                    LengthType::TotalLength => 16,
                }
            }
        }
    }

    fn chk(&self) -> usize {
        if let PacketBody::Operator{encoding:_,method:_,packets} = self {
            packets.iter().map(|p| p.check()).sum()
        } else { 0 }
    }

    fn value(&self) -> Option<u64> {
        match self {
            PacketBody::Literal(value) => Some(*value),
            PacketBody::Operator{encoding:_,method,packets} => {
                let values: Option<Vec<u64>> = packets.iter().map(|p| p.value()).collect();
                let mut it = values?.into_iter();
                Some(match method {
                    TypeId::Sum => it.sum(),
                    TypeId::Mul => it.product(),
                    TypeId::Min => it.min().unwrap(),
                    TypeId::Max => it.max().unwrap(),
                    _ => {
                        let a = it.next()?;
                        let b = it.next()?;
                        if match method {
                            TypeId::CheckGT => a > b,
                            TypeId::CheckLT => a < b,
                            TypeId::CheckEQ => a == b,
                            _ => false
                        } {1} else {0}
                    },
                })
            }
        }
    }
}

impl Packet {
    fn len(&self) -> usize { self.body.len() + 3}
    fn check(&self) -> usize { self.body.chk() + self.version as usize }
    fn new(reader: &mut BitReader) -> bitreader::Result<Self> {
        let version = reader.read_u8(3)?;
        let body = PacketBody::new(reader)?;
        Ok(Packet{version,body})
    }
    fn value(&self) -> Option<u64> {
        self.body.value()
    }
}

fn main_or_error() -> Result<(),Error> {
    let path = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let data = std::fs::read(path).or(Err(Error::ReadError))?;
    let data: Vec<u8> = Vec::from_hex(data).or(Err(Error::InvalidHexEncoding))?;
    let mut reader = BitReader::new(&data);
    let packet = Packet::new(&mut reader).map_err(|e| Error::ParsingFailure(e))?;
    println!("Check: {}", packet.check());
    println!("Value: {}", packet.value().ok_or(Error::RuntimeError)?);
    Ok(())
}

fn main() {
    match main_or_error() {
        Ok(()) => {},
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
