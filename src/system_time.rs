use nom::IResult;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use crate::{Message, NmeaError, parse, nmea_frame::SystemTimeFrame};
use parse::{BitInput, take_byte, take_nibble, take_u32, take_u16};

#[derive(FromPrimitive, Debug)]
pub enum SystemTimeSource {
    GPS,
    GLONASS,
    RadioStation,
    LocalCesiumClock,
    LocalRubidiumClock,
    LocalCrystalClock
}

pub fn parse_system_time(i: BitInput) -> IResult<BitInput, SystemTime> {
        let (i, sid) = take_byte(i)?;
        let (i, source_raw) = take_nibble(i)?;
        let (i, _) = take_nibble(i)?;
        let (i, date) = take_u16(i)?;
        let (i, time) = take_u32(i)?;

        let source = FromPrimitive::from_u8(source_raw);
        
        let system_time = SystemTime {
            sid,
            source,
            date,
            time: Time::new(time)
            
        };
        Ok((i, system_time))
}

impl Message<SystemTime, SystemTimeFrame,> for SystemTime {
    fn get_data(frame: SystemTimeFrame) ->  Result<SystemTime, NmeaError> {
        let data = frame.data;
        let parse_result: IResult<&[u8], SystemTime> = nom::bits::bits(parse_system_time)(&data);
        match parse_result {
            Ok((_, system_time)) => {
                Ok(system_time)
            }
            Err(_e) => {
                Err(NmeaError::ParseError)
            }
        }
    }
}

#[derive(Debug)]
pub struct Time {
    time: u32
}

impl Time {
    pub fn new(time: u32) -> Time {
        Time {
            time
        }
    }
    pub fn get_seconds(&self) -> u32 {
        self.time / 10000
    }
}

#[derive(Debug)]
pub struct SystemTime {
    pub sid: u8,
    pub source: Option<SystemTimeSource>,
    pub date: u16,
    pub time: Time
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parses_system_time_correctly() {
        // Data is 8 bytes long (64 bits)
        // Length = 8
        let system_time_data = [46, 255, 169, 76, 48, 180, 171, 1];
        // 0010 1110 1111 1111 10101001 01001100 00110 0001 0110 1001 0101 0110 0000 0001
        // This is the expected date in binary:
        // 01001100 10101001
        let parsed_data = SystemTime::get_data( SystemTimeFrame { data: system_time_data });
        let data = parsed_data.unwrap();
        assert_eq!(19625, data.date);
        assert_eq!(2803, data.time.get_seconds());
    }

}
