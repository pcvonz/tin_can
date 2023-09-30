use crate::nmea_frame::VesselHeadingFrame;
use nom::IResult;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    parse::{take_byte, take_i16, take_two_bits, take_u16, BitInput},
    rad::Rad,
    Message, NmeaError,
};

#[derive(FromPrimitive, Debug, PartialEq)]
pub enum DirectionReference {
    True = 1,
    Magnetic = 2,
    Error = 3,
}

#[derive(Debug)]
pub struct VesselHeading {
    pub sid: u8,
    pub heading: Rad,
    pub deviation: Rad,
    pub variation: Rad,
    pub reference: Option<DirectionReference>,
}

impl VesselHeading {
    pub fn new() -> Self {
        VesselHeading {
            sid: 0,
            reference: Some(DirectionReference::True),
            heading: Rad::new(0),
            deviation: Rad::from_i16(0),
            variation: Rad::from_i16(0),
        }
    }
}

fn parse_vessel_heading(i: BitInput) -> IResult<BitInput, VesselHeading> {
    let (i, sid) = take_byte(i)?;
    let (i, heading) = take_u16(i)?;
    let (i, deviation) = take_i16(i)?;
    let (i, variation) = take_i16(i)?;
    let (i, reference) = take_two_bits(i)?;

    let system_time = VesselHeading {
        sid,
        heading: Rad::new(heading),
        deviation: Rad::from_i16(deviation),
        variation: Rad::from_i16(variation),
        reference: FromPrimitive::from_u8(reference),
    };
    Ok((i, system_time))
}

impl Message<VesselHeading, VesselHeadingFrame> for VesselHeading {
    fn get_data(frame: VesselHeadingFrame) -> Result<VesselHeading, NmeaError> {
        let data = frame.data;
        let parse_result: IResult<&[u8], VesselHeading> =
            nom::bits::bits(parse_vessel_heading)(&data);
        match parse_result {
            Ok((_, system_time)) => Ok(system_time),
            Err(_e) => Err(NmeaError::ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_vessel_heading_correctly() {
        let vessel_heading_data = [159, 255, 255, 255, 127, 185, 6, 253];
        let parsed_data = VesselHeading::get_data(VesselHeadingFrame {
            data: vessel_heading_data,
        });
        let data = parsed_data.unwrap();
        assert_eq!(6.5534997, data.heading.get_radians());
        assert_eq!(3.2767, data.deviation.get_radians());
        assert_eq!(0.1721, data.variation.get_radians());
    }
}
