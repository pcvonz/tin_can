// #![cfg_attr(not(test), no_std)]

use crate::nmea_frame::NmeaFrame;
use bitfield::bitfield;
pub mod bearing_type;
pub mod course_over_ground;
pub mod date;
pub mod navigation_data;
pub mod nmea_frame;
mod parse;
pub mod pgn;
pub mod rad;
mod system_time;
pub mod vessel_heading;
use nmea_frame::{
    COGSOGRapidUpdateFrame, NavigationDataFrame, SystemTimeFrame, VesselHeadingFrame,
};
use num_traits::FromPrimitive;
use pgn::PGN;

pub trait FastPacketMessage<T, S> {
    fn get_data(&mut self) -> Result<(), NmeaError>;
    fn parse_frame(&mut self, data: T) -> Result<(), ()>;
}

pub trait Message<T, S> {
    fn get_data(data: S) -> Result<T, NmeaError>;
}

bitfield! {
    pub struct NmeaId(u32);
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
    PDU2,
}

#[derive(Debug)]
pub enum NmeaError {
    ParseError,
    NotImplemented,
    NotFullyParsed,
}

impl NmeaId {
    pub fn get_format(&self) -> NmeaPgnFormat {
        if self.pf() < 240 {
            NmeaPgnFormat::PDU1
        } else {
            NmeaPgnFormat::PDU2
        }
    }

    pub fn get_raw_pgn(&self) -> u32 {
        match self.get_format() {
            NmeaPgnFormat::PDU1 => {
                ((self.reserved_data_page() as u32) << 16) | ((self.pf() as u32) << 8)
            }
            NmeaPgnFormat::PDU2 => {
                ((self.reserved_data_page() as u32) << 16 | (self.pf() as u32) << 8) | self.ps()
            }
        }
    }

    pub fn get_pgn(&self) -> Option<PGN> {
        // PDU1 format
        let pgn = self.get_raw_pgn();
        FromPrimitive::from_u32(pgn)
    }

    pub fn parse_data(&self, data: &[u8]) -> Result<NmeaFrame, NmeaError> {
        let pgn = self.get_pgn();

        match pgn {
            Some(PGN::SystemTime) => {
                let data: [u8; 8] = data[0..8].try_into().unwrap();
                Ok(NmeaFrame::SystemTime(SystemTimeFrame { data }))
            }
            Some(PGN::VesselHeading) => {
                let data: [u8; 8] = data[0..8].try_into().unwrap();
                Ok(NmeaFrame::VesselHeading(VesselHeadingFrame { data }))
            }
            Some(PGN::NavigationData) => {
                let data: [u8; 8] = data[0..8].try_into().unwrap();
                Ok(NmeaFrame::NavigationData(NavigationDataFrame { data }))
            }
            Some(PGN::COGSOGRapidUpdate) => {
                let data: [u8; 8] = data[0..8].try_into().unwrap();
                Ok(NmeaFrame::COGSOGRapidUpdate(COGSOGRapidUpdateFrame {
                    data,
                }))
            }
            _ => Err(NmeaError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn gets_corret_pgn_for_system_time() {
        let id = 0b01101111100000001000000000000;
        let nmea_id = NmeaId(id);
        assert_eq!(nmea_id.reserved_data_page(), 1);
        assert_eq!(nmea_id.pf(), 240);
        assert_eq!(nmea_id.ps(), 16);
        assert_eq!(nmea_id.get_pgn(), Some(PGN::SystemTime));
    }

    #[test]
    fn gets_corret_pgn_for_vessel_heading() {
        let id = 0b01001111100010001001000000000;
        let nmea_id = NmeaId(id);
        assert_eq!(nmea_id.get_pgn(), Some(PGN::VesselHeading));
    }
}
