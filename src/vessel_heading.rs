use nom::IResult;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use crate::{Message, NmeaError, parse::{take_byte, take_u16, BitInput, take_i16, take_two_bits}, rad::Rad};

#[derive(FromPrimitive, Debug, PartialEq)]
pub enum DirectionReference {
    True = 1,
    Magnetic = 2,
    Error = 3
}

#[derive(Debug)]
pub struct VesselHeading {
    pub sid: u8,
    pub heading: Rad<u16>,
    pub deviation: Rad<i16>,
    pub variation: Rad<i16>,
    pub reference: Option<DirectionReference>,
}

pub fn parse_vessel_heading(i: BitInput) -> IResult<BitInput, VesselHeading> {
        let (i, sid) = take_byte(i)?;
        let (i, heading) = take_u16(i)?;
        let (i, deviation) = take_i16(i)?;
        let (i, variation) = take_i16(i)?;
        let (i, reference) = take_two_bits(i)?;
        println!("ref bit: {}", reference);

        let system_time = VesselHeading {
            sid,
            heading: Rad::new(heading),
            deviation: Rad::from_i16(deviation),
            variation: Rad::from_i16(variation),
            reference: FromPrimitive::from_u8(reference)
            
        };
        Ok((i, system_time))
}

impl Message<VesselHeading> for VesselHeading {
    fn get_data(input: &[u8]) ->  Result<VesselHeading, NmeaError> {
        let parse_result: IResult<&[u8], VesselHeading> = nom::bits::bits(parse_vessel_heading)(input);
        match parse_result {
            Ok((_, system_time)) => {
                Ok(system_time)
            }
            Err(e) => {
                println!("{:?}", e);
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
    fn parses_vessel_heading_correctly() {
        let vessel_heading_data = [159, 255, 255, 255, 127, 185, 6, 253];
        // 10011111 sid
        // 11111111 h
        // 11111111 h
        // 11111111 d
        // 01111111 d
        // 10111001 v
        // 00000110 v
        // 11 111101

        for i in vessel_heading_data {
            println!("{:08b}", i);
        }
        let parsed_data = VesselHeading::get_data(&vessel_heading_data);
        let data = parsed_data.unwrap();
        assert_eq!(6.5534997, data.heading.get_radians());
        assert_eq!(3.2767, data.deviation.get_radians());
        assert_eq!(0.1721, data.variation.get_radians());
        // Something weird here
        // assert_eq!(Some(DirectionReference::Magnetic),  data.reference);
    }
}