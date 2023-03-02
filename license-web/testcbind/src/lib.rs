#[no_mangle]
pub extern "C" fn add(uno: String, due: u8) -> &u8{
    println!(uno);
    &due

}
#[no_mangle]
pub extern "C" fn deu(mut asd: Point) -> CodecResponse{
    asd.ciao2 = 2;
    CodecResponse::Ok

}

#[repr(C)]
pub struct Point{
    pub ciao: u8,
    pub ciao2: &u8,
    pub ciao3: i32,

}

#[repr(C)]
pub enum CodecResponse {
    Ok = 0,
    EncodeError = -1,
    DecodeError = -2,
}