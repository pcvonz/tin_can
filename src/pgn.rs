use num_derive::FromPrimitive;

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum PGN {
    SystemTime = 126992,
    VesselHeading = 127250,
    NavigationData = 129284,
    GNSSPositionData = 129029,
    COGSOGRapidUpdate = 129026,
    PositionRapidUpdate = 129025,
    GNSSSatsInView = 129540,
    WindData = 130306,
    ISOAddressClaim = 60928,
}
