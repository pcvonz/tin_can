use nom::IResult;
use nom::bits::streaming::take;

pub type BitInput<'a> = (&'a [u8], usize);

pub fn take_i32(i: BitInput) ->  IResult<BitInput, i32> {
    let (i, data): (BitInput, i32) = take(32usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_i16(i: BitInput) ->  IResult<BitInput, i16> {
    let (i, data): (BitInput, i16) = take(16usize)(i)?;
    Ok((i, data.to_be()))
}


pub fn take_u32(i: BitInput) ->  IResult<BitInput, u32> {
    let (i, data): (BitInput, u32) = take(32usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_u16(i: BitInput) ->  IResult<BitInput, u16> {
    let (i, data): (BitInput, u16) = take(16usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_byte(i: BitInput) ->  IResult<BitInput, u8> {
    let (i, data): (BitInput, u8) = take(8usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_nibble(i: BitInput) ->  IResult<BitInput, u8> {
    let (i, data): (BitInput, u8) = take(4usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_two_bits(i: BitInput) ->  IResult<BitInput, u8> {
    let (i, data): (BitInput, u8) = take(2usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_five_bits(i: BitInput) ->  IResult<BitInput, u8> {
    let (i, data): (BitInput, u8) = take(5usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_three_bits(i: BitInput) ->  IResult<BitInput, u8> {
    println!("{:?}", i);
    let (i, data): (BitInput, u8) = take(3usize)(i)?;
    Ok((i, data.to_be()))
}

pub fn take_bit(i: BitInput) ->  IResult<BitInput, bool> {
    let (i, data): (BitInput, u8) = take(1usize)(i)?;
    Ok((i, data.to_be() == 1))
}
