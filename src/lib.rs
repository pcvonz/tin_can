

use nom::bits::{streaming::take};
use bitfield::bitfield;
use nom::IResult;


type BitInput<'a> = (&'a [u8], usize);

fn take_u32(i: BitInput) ->  IResult<BitInput, u32> {
    take(32usize)(i)
}

fn take_u16(i: BitInput) ->  IResult<BitInput, u16> {
    take(16usize)(i)
}

fn take_byte(i: BitInput) ->  IResult<BitInput, u8> {
    take(8usize)(i)
}

fn take_nibble(i: BitInput) ->  IResult<BitInput, u8> {
    take(4usize)(i)
}

fn take_two_bits(i: BitInput) ->  IResult<BitInput, u8> {
    take(2usize)(i)
}

fn take_bit(i: BitInput) ->  IResult<BitInput, u8> {
    take(1usize)(i)
}


// fn parse_system_time(input: &[u8]) -> IResult<SystemTime, ()> {

// }

pub trait Message<T> {
    fn get_data(data: &[u8]) -> Result<SystemTime, ()>;
}


pub fn parse_system_time(i: BitInput) -> IResult<BitInput, SystemTime> {
        // let (i, opcode) = map_res(take_nibble, Opcode::try_from)(i)?;
        let (i, sid) = take_byte(i)?;
        let (i, source) = take_nibble(i)?;
        let (i, _) = take_byte(i)?;
        let (i, date) = take_u16(i)?;
        let (i, time_since_midnight) = take_u32(i)?;
        let system_time = SystemTime {
            sid,
            source,
            date,
            time_since_midnight
            
        };
        Ok((i, system_time))
}

impl Message<SystemTime> for SystemTime {
    fn get_data(input: &[u8]) ->  Result<SystemTime, ()> {
        // let parsed = tuple((parse_system_time_sid, take(2usize), parse_system_time_source))(i)
        let parse_result: IResult<&[u8], SystemTime> = nom::bits::bits(parse_system_time)(input);
        match parse_result {
            Ok((_, system_time)) => {
                Ok(system_time)
            }
            Err(_) => {
                Err(())
            }
        }
    }
}

pub struct SystemTime {
    pub sid: u8,
    pub source: u8,
    pub date: u16,
    pub time_since_midnight: u32
}

impl SystemTime {
}

pub struct Navigation{
}

impl Navigation {
    pub fn get_time() -> u32 {
        20
    }
}


pub enum NmeaData {
    SystemTime(SystemTime),
    Navigation,
}

bitfield! {
    pub struct NMEA_ID(u32);
    impl Debug;
    pub priority_bit, _: 28, 26;
    pub reserved_data_page, _: 25, 24;
    pub pf, _: 23, 16;
    pub ps, _: 15, 8;
    pub sa, _: 7, 0;
}

#[derive(Debug)]
pub enum NmeaPgnFormat {
    PDU1,
    PDU2
}

impl NMEA_ID {
    pub fn get_format(&self) -> NmeaPgnFormat {
        if self.pf() < 240 {
            NmeaPgnFormat::PDU1
        } else {
            NmeaPgnFormat::PDU2
        }
    }
    pub fn get_pgn(&self) -> u32 {
        // PDU1 format
        match self.get_format() {
            NmeaPgnFormat::PDU1 => {
                ((self.reserved_data_page() as u32 ) << 16) | ((self.pf() as u32) << 8)
            }
            NmeaPgnFormat::PDU2 => {

                ((self.reserved_data_page() as u32) << 16 | (self.pf() as u32) << 8) | self.ps()
            }
        }
    }

    pub fn parse_data(&self, data: &[u8]) -> Result<NmeaData, ()> {
        let pgn = self.get_pgn();
        match pgn {
            0x1F010 => {
                let system_time = SystemTime::get_data(data)?;
                Ok(NmeaData::SystemTime(system_time))
            }
            0x129285 => {
                todo!("Not implemented!")
            }
            _ => {
                todo!("Not implemented!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn gets_corret_pgn() {
        let id = 0b01101111100000001000000000000;
        let nmea_id = NMEA_ID(id);
        assert_eq!(nmea_id.reserved_data_page(), 1);
        assert_eq!(nmea_id.pf(), 240);
        assert_eq!(nmea_id.ps(), 16);
        assert_eq!(nmea_id.get_pgn(), 0b011111000000010000);
    }
    
    #[test]
    fn parses_system_time_correctly() {
        let id = 0b01101111100000001000000000000;
        let nmea_id = NMEA_ID(id);
        assert_eq!(nmea_id.reserved_data_page(), 1);
        assert_eq!(nmea_id.pf(), 240);
        assert_eq!(nmea_id.ps(), 16);
        assert_eq!(nmea_id.get_pgn(), 0b011111000000010000);
    }

}
