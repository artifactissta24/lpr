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
Self    {payload}
 }
     fn push_header(&mut self, header: &Header)
 {
  self.payload.reserve(header.data.capacity());
  for b in header.data.iter()       {
   self.payload.push_front(!b);
  }
 }
     fn push_back_bytes(&mut self, data: &[u8])
 {
  self.payload.extend(data       );
 }
}
impl Header
{
     fn with_capacity(data    : usize) -> Self
 {
  let data = Vec::with_capacity(data    );
Self    {data}
 }
     fn push8(&mut self, data: u8)
 {
  self.data.push(data);
 }
}
fn push_mac(packet: &mut Packet)
{
 let mut header = Header::with_capacity(30);
 let addr = [   2            ];
 for &b in addr.iter() {
  header.push8(b);
  header.push8(b);
 }
 packet.push_header(& header);
 println!{}
 packet.push_back_bytes(&addr);
}
fn main()
{
 let mut packet = Packet::new();
 push_mac (&mut packet);
 push_mac(&mut packet);
}
