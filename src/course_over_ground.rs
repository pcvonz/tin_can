use nom::IResult;
    
use num_traits::FromPrimitive;

use crate::{Message, NmeaError, parse::{take_byte, take_u16, BitInput, take_two_bits, take_nibble}, rad::Rad, vessel_heading::DirectionReference};

#[derive(Debug)]
pub struct CourseOverGround {
    pub sid: u8,
    pub cog_reference: Option<DirectionReference>,
    pub cog: Rad,
    pub sog: u16,
}

impl CourseOverGround {
    pub fn new() -> Self {
        CourseOverGround {
            sid: 0,
            cog_reference: Some(DirectionReference::True),
            cog: Rad::new(0),
            sog: 0
        }

    }
}

fn parse_course_over_ground(i: BitInput) -> IResult<BitInput, CourseOverGround> {
        let (i, sid) = take_byte(i)?;
        let (i, cog_reference) = take_two_bits(i)?;
        let (i, _) = take_nibble(i)?;
        let (i, _) = take_two_bits(i)?;
        let (i, cog) = take_u16(i)?;
        let (i, sog) = take_u16(i)?;

        let system_time = CourseOverGround {
            sid,
            cog_reference: FromPrimitive::from_u8(cog_reference),
            cog: Rad::new(cog),
            sog
            
        };
        Ok((i, system_time))
}

impl Message<CourseOverGround> for CourseOverGround {
    fn get_data(input: &[u8]) ->  Result<CourseOverGround, NmeaError> {
        let parse_result: IResult<&[u8], CourseOverGround> = nom::bits::bits(parse_course_over_ground)(input);
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



// [142, 255, 255, 255, 127, 187, 6, 253]

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parses_cog() {
        let cog_data = [0b11100010, 255, 255, 255, 255, 255, 255, 255];
        // 10011111 sid
        // 11111111 h
        // 11111111 h
        // 11111111 d
        // 01111111 d
        // 10111001 v
        // 00000110 v
        // 11 111101

        let parsed_data = CourseOverGround::get_data(&cog_data);
        let data = parsed_data.unwrap();
        // TODO: Why is it error?
        assert_eq!(DirectionReference::Error, data.cog_reference.unwrap());
        assert_eq!(6.5534997, data.cog.get_radians());
        // TODO: This should be a float (unit is m/s)
        assert_eq!(65535, data.sog);
    }
}

