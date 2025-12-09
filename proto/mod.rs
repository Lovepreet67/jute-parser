pub mod connect;
/*This is a auto generated code based on the jute file provided*/
	#[derive(Default,Clone)]
	pub struct RequestHeader {
		pub xid : i32,
		pub type_ : i32,
	}
	impl RequestHeader {
		pub fn get_xid(&self)->i32{
			self.xid.clone()
		}
		pub fn set_xid(&mut self,val: i32){
			self.xid = val
		}
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
	}
	impl JuteSerializable for RequestHeader {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.xid.serialize(out)?;
			self.type_.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 xid : i32::deserialize(bytes)?,
				 type_ : i32::deserialize(bytes)?,
			}
		}
	}
/*This is a auto generated code based on the jute file provided*/
	#[derive(Default,Clone)]
	pub struct MultiHeader {
		pub type_ : i32,
		pub done : bool,
		pub err : i32,
	}
	impl MultiHeader {
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
		pub fn get_done(&self)->bool{
			self.done.clone()
		}
		pub fn set_done(&mut self,val: bool){
			self.done = val
		}
		pub fn get_err(&self)->i32{
			self.err.clone()
		}
		pub fn set_err(&mut self,val: i32){
			self.err = val
		}
	}
	impl JuteSerializable for MultiHeader {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.type_.serialize(out)?;
			self.done.serialize(out)?;
			self.err.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 type_ : i32::deserialize(bytes)?,
				 done : bool::deserialize(bytes)?,
				 err : i32::deserialize(bytes)?,
			}
		}
	}
