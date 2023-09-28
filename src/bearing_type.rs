use num_derive::FromPrimitive;    

#[derive(FromPrimitive, Debug, PartialEq)]
pub enum BearingType {
    GreatCircle,
    Rhumbline
}
