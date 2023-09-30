use nom::{IResult, AsBytes};
use num_traits::FromPrimitive;
use crate::{NmeaError, parse::{self, take_two_bits, take_three_bits, take_five_bits}, vessel_heading::DirectionReference, rad::Rad, bearing_type::BearingType, FastPacketMessage, nmea_frame::NavigationDataFrame};
use parse::{BitInput, take_byte, take_u32, take_u16};


fn parse_frame_and_sequence(i: BitInput) -> IResult<BitInput, (u8, u8)> {
        let (i, sequence) = take_three_bits(i)?;
        let (i, frame_count) = take_five_bits(i)?;
        Ok((i, (sequence, frame_count)))
}

fn parse_navigation_data(i: BitInput) -> IResult<BitInput,  (
        u8,
        u32,
        Option<DirectionReference>,
        bool,
        bool,
        Option<BearingType>,
        u32,
        u16,
        u16,
        u16,
        u32,
        u32,
        u32,
        u16,
        u32
    )> {
        let (i, sid) = take_byte(i)?;
        let ( i, distance_to_waypoint ) = take_u32(i)?;
        let ( i, course_bearing_reference_raw ) = take_two_bits(i)?;
        let ( i, perpindicular_crossed ) = take_two_bits(i)?;
        let ( i, arrival_circle_entered ) = take_two_bits(i)?;
        let ( i, calculation_type) = take_two_bits(i)?;
        let ( i, eta_time ) = take_u32(i)?;
        let ( i, eta_date ) = take_u16(i)?;
        let ( i, bearing_origin_to_destination_waypoint ) = take_u16(i)?;
        let ( i, bearing_origin_to_waypoint ) = take_u16(i)?;
        let ( i, origin_waypoint_number ) = take_u32(i)?;
        let ( i, destination_waypoint_number ) = take_u32(i)?;
        let ( i, destination_latitude ) = take_u32(i)?;
        let ( i, destination_longitude )= take_u32(i)?;
        let ( i, waypoint_closing_velocity ) = take_u16(i)?;

        let course_bearing_reference: Option<DirectionReference> = FromPrimitive::from_u8(course_bearing_reference_raw);
        let calculation_type: Option<BearingType> = FromPrimitive::from_u8(calculation_type);



        Ok((i, (
        sid,
        distance_to_waypoint,
        course_bearing_reference,
        perpindicular_crossed == 1,
        arrival_circle_entered == 1,
        calculation_type,
        eta_time,
        eta_date,
        bearing_origin_to_waypoint,
        bearing_origin_to_destination_waypoint,
        origin_waypoint_number,
        destination_latitude,
        destination_longitude,
        waypoint_closing_velocity,
        destination_waypoint_number
                    )))
}


impl FastPacketMessage<NavigationDataFrame, [u8; 64]> for NavigationData {
    fn parse_frame(&mut self, frame: NavigationDataFrame) -> Result<(), ()> {
        let data = frame.data;
        let parse_result: IResult<&[u8], (u8, u8)>  = nom::bits::bits(parse_frame_and_sequence)(&data);
        let (i, ( sequence, frame_count )) = parse_result.unwrap();

        match frame_count {
            0 => {
                println!("start frame");
                self.start_sequence = sequence;
                for index in 0..6 {
                    self.data[index] = i[index + 1];
                }
            }
            _ => {
                if self.start_sequence == sequence {
                    for index in 0..7 {
                        let byte_to_set: usize = index + (6 + (( frame_count - 1 ) * 7)) as usize;
                        self.data[byte_to_set] = i[index];
                    }
                }
            }
        }
        Ok(())
    }

    fn get_data(&mut self) ->  Result<(), NmeaError> {
        if self.end_sequence == self.start_sequence {
            self.parse_navigation_data()?;
        }
        Ok(())
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
pub struct NavigationData {
    data: [u8; 64],
    start_sequence: u8,
    end_sequence: u8,
    pub frame: u8,
    pub length: Option<u8>,
    pub sid: Option<u8>,
    pub distance_to_waypoint: Option<u32>,
    pub course_bearing_reference: Option<DirectionReference>,
    pub perpindicular_crossed: Option<bool>,
    pub arrival_circle_entered: Option<bool>,
    pub calculation_type: Option<BearingType>,
    pub eta_time: Option<Time>,
    pub eta_date: Option<u16>,
    pub bearing_origin_to_destination_waypoint: Option<Rad>,
    pub bearing_origin_to_waypoint: Option<Rad>,
    pub origin_waypoint_number: Option<u32>,
    pub destination_waypoint_number: Option<u32>,
    pub destination_latitude: Option<u32>,
    pub destination_longitude: Option<u32>,
    pub waypoint_closing_velocity: Option<f32>,
}

impl NavigationData {
    pub fn parse_navigation_data(&mut self) -> Result<(), NmeaError> {
        let parse_result: IResult<&[u8], (
            u8,
            u32,
            Option<DirectionReference>,
            bool,
            bool,
            Option<BearingType>,
            u32,
            u16,
            u16,
            u16,
            u32,
            u32,
            u32,
            u16,
            u32
        )> = nom::bits::bits(parse_navigation_data)(self.data.as_bytes());

        if let Err(_) = parse_result {
            return Err(NmeaError::ParseError);
        }
        
        let (_, (
        sid,
        distance_to_waypoint,
        course_bearing_reference,
        perpindicular_crossed,
        arrival_circle_entered,
        calculation_type,
        eta_time,
        eta_date,
        bearing_origin_to_waypoint,
        bearing_origin_to_destination_waypoint,
        origin_waypoint_number,
        destination_latitude,
        destination_longitude,
        waypoint_closing_velocity,
        destination_waypoint_number
                    )
        ) = parse_result.unwrap();

        self.sid = Some(sid);
        self.distance_to_waypoint = Some(distance_to_waypoint);
        self.course_bearing_reference = course_bearing_reference;
        self.perpindicular_crossed = Some(perpindicular_crossed);
        self.arrival_circle_entered = Some(arrival_circle_entered);
        self.calculation_type = calculation_type;
        self.eta_time = Some(Time::new(eta_time));
        self.eta_date = Some(eta_date);
        self.bearing_origin_to_waypoint = Some(Rad::new(bearing_origin_to_waypoint));
        self.bearing_origin_to_destination_waypoint = Some(Rad::new(bearing_origin_to_destination_waypoint));
        self.origin_waypoint_number = Some(origin_waypoint_number);
        self.destination_latitude = Some(destination_latitude);
        self.destination_longitude = Some(destination_longitude);
        self.waypoint_closing_velocity = Some(waypoint_closing_velocity as f32 * 0.01);
        self.destination_waypoint_number = Some(destination_waypoint_number);
        Ok(())
    }
    pub fn new() -> Self {
        NavigationData {
            data: [0; 64],
            length: None,
            start_sequence: 0,
            end_sequence: 0,
            frame: 0,
            sid: None,
            distance_to_waypoint: None,
            perpindicular_crossed: None,
            arrival_circle_entered: None,
            calculation_type: None,
            course_bearing_reference: None,
            eta_time: None,
            eta_date: None,
            bearing_origin_to_waypoint: None,
            bearing_origin_to_destination_waypoint: None,
            origin_waypoint_number: None,
            destination_latitude: None,
            destination_longitude: None,
            waypoint_closing_velocity: None,
            destination_waypoint_number: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_navigation_data_correctly_2() {
        let nav_data: Vec<[u8; 8]> = vec!(
[64, 34, 204, 55, 29, 16, 0, 0],
[65, 48, 87, 21, 39, 170, 76, 255],
[66, 255, 255, 255, 255, 255, 255, 255],
[67, 0, 0, 0, 0, 255, 255, 255],
[68, 127, 255, 255, 255, 127, 141, 0],
[128, 34, 206, 232, 117, 25, 0, 0],
);

        let mut navigation_data  = NavigationData::new();
        for data in nav_data {
            let bytes = data.as_bytes();
            let data: [u8; 8] = bytes[0..8].try_into().unwrap();
            let _ = navigation_data.parse_frame(NavigationDataFrame { data }).unwrap();
        }
        let _ = navigation_data.parse_navigation_data();
        assert_eq!(1.41, navigation_data.waypoint_closing_velocity.unwrap());
        assert_eq!(6.5534997, navigation_data.bearing_origin_to_waypoint.unwrap().get_radians());
    }

}
