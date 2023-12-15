use std::collections::VecDeque;
    struct Packet
{
     payload: VecDeque<u8>
}
    struct Header
{
     data: Vec<u8>
}
impl Packet
{
     fn new() -> Self
 {
  let payload = VecDeque::with_capacity(32);
  Packet{payload}
 }
     fn push_header(&mut self, header: &Header)
 {
  self.payload.reserve(header.data.len());
  for b in header.data.iter()       {
   self.payload.push_front(*b);
  }
 }
     fn push_back_bytes(&mut self, data: &[u8])
 {
  self.payload.extend(data       );
 }
}
impl Header
{
     fn with_capacity(capacity: usize) -> Self
 {
  let data = Vec::with_capacity(capacity);
  Header{data}
 }
     fn push8(&mut self, data: u8)
 {
  self.data.push(data);
 }
     fn push16(&mut self, data: u16)
 {
  self.data.push( data       as u8);
  self.data.push( data         as u8);
 }
}
fn push_ipv4(packet: &mut Packet)
{
 let mut header = Header::with_capacity(20);
 let b = 0x45;
 header.push8(b);
 header.push8(20);
 let hw = 20              ;
 header.push16(hw       );
 header.push16(21);
 header.push16(23);
 packet.push_header(&header);
}
fn push_mac(packet: &mut Packet)
{
 let mut header = Header::with_capacity(30);
 let hw = 0b1000_10_00;
 header.push16(hw);
 let addr = [1, 2, 3, 4, 5, 6];
 for &b in addr.iter() {
  header.push8(b);
 }
 for &b in addr.iter() {
  header.push8(b);
 }
 for &b in addr.iter() {
  header.push8(b);
 }
 header.push16(55);
 header.push16(hw);
 packet.push_header(&header);
 let fcs = [0xD9                  ];
 println!(                           );
 packet.push_back_bytes(&fcs);
}
fn main()
{
 let mut packet = Packet::new();
 push_ipv4(&mut packet);
 push_mac(&mut packet);
}
