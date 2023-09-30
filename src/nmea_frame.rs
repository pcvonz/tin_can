pub struct NavigationDataFrame {
    pub data: [u8; 8],
}
pub struct COGSOGRapidUpdateFrame {
    pub data: [u8; 8],
}
pub struct SystemTimeFrame {
    pub data: [u8; 8],
}
pub struct VesselHeadingFrame {
    pub data: [u8; 8],
}

pub enum NmeaFrame {
    SystemTime(SystemTimeFrame),
    NavigationData(NavigationDataFrame),
    COGSOGRapidUpdate(COGSOGRapidUpdateFrame),
    VesselHeading(VesselHeadingFrame),
}
