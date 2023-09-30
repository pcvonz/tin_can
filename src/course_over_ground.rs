use crate::{
    nmea_frame::COGSOGRapidUpdateFrame,
    parse::{take_byte, take_nibble, take_two_bits, take_u16, BitInput},
    rad::Rad,
    vessel_heading::DirectionReference,
    Message, NmeaError,
};
use nom::IResult;
use num_traits::FromPrimitive;

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
            sog: 0,
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
        sog,
    };
    Ok((i, system_time))
}

impl Message<CourseOverGround, COGSOGRapidUpdateFrame> for CourseOverGround {
    fn get_data(frame: COGSOGRapidUpdateFrame) -> Result<CourseOverGround, NmeaError> {
        let data = frame.data;
        let parse_result: IResult<&[u8], CourseOverGround> =
            nom::bits::bits(parse_course_over_ground)(&data);
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
    fn parses_cog() {
        let cog_data = [0b11100010, 255, 255, 255, 255, 255, 255, 255];
        let parsed_data = CourseOverGround::get_data(COGSOGRapidUpdateFrame { data: cog_data });
        let data = parsed_data.unwrap();
        // TODO: Why is it error?
        assert_eq!(DirectionReference::Error, data.cog_reference.unwrap());
        assert_eq!(6.5534997, data.cog.get_radians());
        // TODO: This should be a float (unit is m/s)
        assert_eq!(65535, data.sog);
    }
}
