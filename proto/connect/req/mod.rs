/*This is a auto generated code based on the jute file provided*/
	#[derive(Default,Clone)]
	pub struct Request {
		pub protocol_version : i32,
		pub last_zxid_seen : i64,
		pub time_out : i32,
		pub session_id : i64,
		pub passwd : std::vec::Vec<u8>,
		pub read_only : bool,
	}
	impl Request {
		pub fn get_protocol_version(&self)->i32{
			self.protocol_version.clone()
		}
		pub fn set_protocol_version(&mut self,val: i32){
			self.protocol_version = val
		}
		pub fn get_last_zxid_seen(&self)->i64{
			self.last_zxid_seen.clone()
		}
		pub fn set_last_zxid_seen(&mut self,val: i64){
			self.last_zxid_seen = val
		}
		pub fn get_time_out(&self)->i32{
			self.time_out.clone()
		}
		pub fn set_time_out(&mut self,val: i32){
			self.time_out = val
		}
		pub fn get_session_id(&self)->i64{
			self.session_id.clone()
		}
		pub fn set_session_id(&mut self,val: i64){
			self.session_id = val
		}
		pub fn get_passwd(&self)->std::vec::Vec<u8>{
			self.passwd.clone()
		}
		pub fn set_passwd(&mut self,val: std::vec::Vec<u8>){
			self.passwd = val
		}
		pub fn get_read_only(&self)->bool{
			self.read_only.clone()
		}
		pub fn set_read_only(&mut self,val: bool){
			self.read_only = val
		}
	}
	impl JuteSerializable for Request {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.protocol_version.serialize(out)?;
			self.last_zxid_seen.serialize(out)?;
			self.time_out.serialize(out)?;
			self.session_id.serialize(out)?;
			self.passwd.serialize(out)?;
			self.read_only.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 protocol_version : i32::deserialize(bytes)?,
				 last_zxid_seen : i64::deserialize(bytes)?,
				 time_out : i32::deserialize(bytes)?,
				 session_id : i64::deserialize(bytes)?,
				 passwd : std::vec::Vec<u8>::deserialize(bytes)?,
				 read_only : bool::deserialize(bytes)?,
			}
		}
	}
