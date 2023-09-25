use num_derive::FromPrimitive;    

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum PGN {
    SystemTime =  126992,
    VesselHeading = 127250,
    NavigationData = 129284,
}
