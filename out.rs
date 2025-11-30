/*This is a auto generated code based on the jute file provided*/
pub mod proto {
	#[derive(Default,Clone)]
	pub struct ConnectRequest {
		pub protocol_version : i32,
		pub last_zxid_seen : i64,
		pub time_out : i32,
		pub session_id : i64,
		pub passwd : std::vec::Vec<u8>,
		pub read_only : bool,
	}
	impl ConnectRequest {
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
	impl JuteSerializable for ConnectRequest {
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
	#[derive(Default,Clone)]
	pub struct ConnectResponse {
		pub protocol_version : i32,
		pub time_out : i32,
		pub session_id : i64,
		pub passwd : std::vec::Vec<u8>,
		pub read_only : bool,
	}
	impl ConnectResponse {
		pub fn get_protocol_version(&self)->i32{
			self.protocol_version.clone()
		}
		pub fn set_protocol_version(&mut self,val: i32){
			self.protocol_version = val
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
	impl JuteSerializable for ConnectResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.protocol_version.serialize(out)?;
			self.time_out.serialize(out)?;
			self.session_id.serialize(out)?;
			self.passwd.serialize(out)?;
			self.read_only.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 protocol_version : i32::deserialize(bytes)?,
				 time_out : i32::deserialize(bytes)?,
				 session_id : i64::deserialize(bytes)?,
				 passwd : std::vec::Vec<u8>::deserialize(bytes)?,
				 read_only : bool::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetWatches {
		pub relative_zxid : i64,
		pub data_watches : std::vec::Vec<String>,
		pub exist_watches : std::vec::Vec<String>,
		pub child_watches : std::vec::Vec<String>,
	}
	impl SetWatches {
		pub fn get_relative_zxid(&self)->i64{
			self.relative_zxid.clone()
		}
		pub fn set_relative_zxid(&mut self,val: i64){
			self.relative_zxid = val
		}
		pub fn get_data_watches(&self)->std::vec::Vec<String>{
			self.data_watches.clone()
		}
		pub fn set_data_watches(&mut self,val: std::vec::Vec<String>){
			self.data_watches = val
		}
		pub fn get_exist_watches(&self)->std::vec::Vec<String>{
			self.exist_watches.clone()
		}
		pub fn set_exist_watches(&mut self,val: std::vec::Vec<String>){
			self.exist_watches = val
		}
		pub fn get_child_watches(&self)->std::vec::Vec<String>{
			self.child_watches.clone()
		}
		pub fn set_child_watches(&mut self,val: std::vec::Vec<String>){
			self.child_watches = val
		}
	}
	impl JuteSerializable for SetWatches {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.relative_zxid.serialize(out)?;
			self.data_watches.serialize(out)?;
			self.exist_watches.serialize(out)?;
			self.child_watches.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 relative_zxid : i64::deserialize(bytes)?,
				 data_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 exist_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 child_watches : std::vec::Vec<String>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetWatches2 {
		pub relative_zxid : i64,
		pub data_watches : std::vec::Vec<String>,
		pub exist_watches : std::vec::Vec<String>,
		pub child_watches : std::vec::Vec<String>,
		pub persistent_watches : std::vec::Vec<String>,
		pub persistent_recursive_watches : std::vec::Vec<String>,
	}
	impl SetWatches2 {
		pub fn get_relative_zxid(&self)->i64{
			self.relative_zxid.clone()
		}
		pub fn set_relative_zxid(&mut self,val: i64){
			self.relative_zxid = val
		}
		pub fn get_data_watches(&self)->std::vec::Vec<String>{
			self.data_watches.clone()
		}
		pub fn set_data_watches(&mut self,val: std::vec::Vec<String>){
			self.data_watches = val
		}
		pub fn get_exist_watches(&self)->std::vec::Vec<String>{
			self.exist_watches.clone()
		}
		pub fn set_exist_watches(&mut self,val: std::vec::Vec<String>){
			self.exist_watches = val
		}
		pub fn get_child_watches(&self)->std::vec::Vec<String>{
			self.child_watches.clone()
		}
		pub fn set_child_watches(&mut self,val: std::vec::Vec<String>){
			self.child_watches = val
		}
		pub fn get_persistent_watches(&self)->std::vec::Vec<String>{
			self.persistent_watches.clone()
		}
		pub fn set_persistent_watches(&mut self,val: std::vec::Vec<String>){
			self.persistent_watches = val
		}
		pub fn get_persistent_recursive_watches(&self)->std::vec::Vec<String>{
			self.persistent_recursive_watches.clone()
		}
		pub fn set_persistent_recursive_watches(&mut self,val: std::vec::Vec<String>){
			self.persistent_recursive_watches = val
		}
	}
	impl JuteSerializable for SetWatches2 {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.relative_zxid.serialize(out)?;
			self.data_watches.serialize(out)?;
			self.exist_watches.serialize(out)?;
			self.child_watches.serialize(out)?;
			self.persistent_watches.serialize(out)?;
			self.persistent_recursive_watches.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 relative_zxid : i64::deserialize(bytes)?,
				 data_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 exist_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 child_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 persistent_watches : std::vec::Vec<String>::deserialize(bytes)?,
				 persistent_recursive_watches : std::vec::Vec<String>::deserialize(bytes)?,
			}
		}
	}
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
	#[derive(Default,Clone)]
	pub struct AuthPacket {
		pub type_ : i32,
		pub scheme : String,
		pub auth : std::vec::Vec<u8>,
	}
	impl AuthPacket {
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
		pub fn get_scheme(&self)->String{
			self.scheme.clone()
		}
		pub fn set_scheme(&mut self,val: String){
			self.scheme = val
		}
		pub fn get_auth(&self)->std::vec::Vec<u8>{
			self.auth.clone()
		}
		pub fn set_auth(&mut self,val: std::vec::Vec<u8>){
			self.auth = val
		}
	}
	impl JuteSerializable for AuthPacket {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.type_.serialize(out)?;
			self.scheme.serialize(out)?;
			self.auth.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 type_ : i32::deserialize(bytes)?,
				 scheme : String::deserialize(bytes)?,
				 auth : std::vec::Vec<u8>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct ReplyHeader {
		pub xid : i32,
		pub zxid : i64,
		pub err : i32,
	}
	impl ReplyHeader {
		pub fn get_xid(&self)->i32{
			self.xid.clone()
		}
		pub fn set_xid(&mut self,val: i32){
			self.xid = val
		}
		pub fn get_zxid(&self)->i64{
			self.zxid.clone()
		}
		pub fn set_zxid(&mut self,val: i64){
			self.zxid = val
		}
		pub fn get_err(&self)->i32{
			self.err.clone()
		}
		pub fn set_err(&mut self,val: i32){
			self.err = val
		}
	}
	impl JuteSerializable for ReplyHeader {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.xid.serialize(out)?;
			self.zxid.serialize(out)?;
			self.err.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 xid : i32::deserialize(bytes)?,
				 zxid : i64::deserialize(bytes)?,
				 err : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetDataRequest {
		pub path : String,
		pub watch : bool,
	}
	impl GetDataRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_watch(&self)->bool{
			self.watch.clone()
		}
		pub fn set_watch(&mut self,val: bool){
			self.watch = val
		}
	}
	impl JuteSerializable for GetDataRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.watch.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 watch : bool::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetDataRequest {
		pub path : String,
		pub data : std::vec::Vec<u8>,
		pub version : i32,
	}
	impl SetDataRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_data(&self)->std::vec::Vec<u8>{
			self.data.clone()
		}
		pub fn set_data(&mut self,val: std::vec::Vec<u8>){
			self.data = val
		}
		pub fn get_version(&self)->i32{
			self.version.clone()
		}
		pub fn set_version(&mut self,val: i32){
			self.version = val
		}
	}
	impl JuteSerializable for SetDataRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.data.serialize(out)?;
			self.version.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 data : std::vec::Vec<u8>::deserialize(bytes)?,
				 version : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct ReconfigRequest {
		pub joining_servers : String,
		pub leaving_servers : String,
		pub new_members : String,
		pub cur_config_id : i64,
	}
	impl ReconfigRequest {
		pub fn get_joining_servers(&self)->String{
			self.joining_servers.clone()
		}
		pub fn set_joining_servers(&mut self,val: String){
			self.joining_servers = val
		}
		pub fn get_leaving_servers(&self)->String{
			self.leaving_servers.clone()
		}
		pub fn set_leaving_servers(&mut self,val: String){
			self.leaving_servers = val
		}
		pub fn get_new_members(&self)->String{
			self.new_members.clone()
		}
		pub fn set_new_members(&mut self,val: String){
			self.new_members = val
		}
		pub fn get_cur_config_id(&self)->i64{
			self.cur_config_id.clone()
		}
		pub fn set_cur_config_id(&mut self,val: i64){
			self.cur_config_id = val
		}
	}
	impl JuteSerializable for ReconfigRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.joining_servers.serialize(out)?;
			self.leaving_servers.serialize(out)?;
			self.new_members.serialize(out)?;
			self.cur_config_id.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 joining_servers : String::deserialize(bytes)?,
				 leaving_servers : String::deserialize(bytes)?,
				 new_members : String::deserialize(bytes)?,
				 cur_config_id : i64::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetDataResponse {
		pub stat : Stat,
	}
	impl SetDataResponse {
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for SetDataResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetSASLRequest {
		pub token : std::vec::Vec<u8>,
	}
	impl GetSASLRequest {
		pub fn get_token(&self)->std::vec::Vec<u8>{
			self.token.clone()
		}
		pub fn set_token(&mut self,val: std::vec::Vec<u8>){
			self.token = val
		}
	}
	impl JuteSerializable for GetSASLRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.token.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 token : std::vec::Vec<u8>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetSASLRequest {
		pub token : std::vec::Vec<u8>,
	}
	impl SetSASLRequest {
		pub fn get_token(&self)->std::vec::Vec<u8>{
			self.token.clone()
		}
		pub fn set_token(&mut self,val: std::vec::Vec<u8>){
			self.token = val
		}
	}
	impl JuteSerializable for SetSASLRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.token.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 token : std::vec::Vec<u8>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetSASLResponse {
		pub token : std::vec::Vec<u8>,
	}
	impl SetSASLResponse {
		pub fn get_token(&self)->std::vec::Vec<u8>{
			self.token.clone()
		}
		pub fn set_token(&mut self,val: std::vec::Vec<u8>){
			self.token = val
		}
	}
	impl JuteSerializable for SetSASLResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.token.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 token : std::vec::Vec<u8>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct CreateRequest {
		pub path : String,
		pub data : std::vec::Vec<u8>,
		pub acl : std::vec::Vec<ACL>,
		pub flags : i32,
	}
	impl CreateRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_data(&self)->std::vec::Vec<u8>{
			self.data.clone()
		}
		pub fn set_data(&mut self,val: std::vec::Vec<u8>){
			self.data = val
		}
		pub fn get_acl(&self)->std::vec::Vec<ACL>{
			self.acl.clone()
		}
		pub fn set_acl(&mut self,val: std::vec::Vec<ACL>){
			self.acl = val
		}
		pub fn get_flags(&self)->i32{
			self.flags.clone()
		}
		pub fn set_flags(&mut self,val: i32){
			self.flags = val
		}
	}
	impl JuteSerializable for CreateRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.data.serialize(out)?;
			self.acl.serialize(out)?;
			self.flags.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 data : std::vec::Vec<u8>::deserialize(bytes)?,
				 acl : std::vec::Vec<ACL>::deserialize(bytes)?,
				 flags : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct CreateTTLRequest {
		pub path : String,
		pub data : std::vec::Vec<u8>,
		pub acl : std::vec::Vec<ACL>,
		pub flags : i32,
		pub ttl : i64,
	}
	impl CreateTTLRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_data(&self)->std::vec::Vec<u8>{
			self.data.clone()
		}
		pub fn set_data(&mut self,val: std::vec::Vec<u8>){
			self.data = val
		}
		pub fn get_acl(&self)->std::vec::Vec<ACL>{
			self.acl.clone()
		}
		pub fn set_acl(&mut self,val: std::vec::Vec<ACL>){
			self.acl = val
		}
		pub fn get_flags(&self)->i32{
			self.flags.clone()
		}
		pub fn set_flags(&mut self,val: i32){
			self.flags = val
		}
		pub fn get_ttl(&self)->i64{
			self.ttl.clone()
		}
		pub fn set_ttl(&mut self,val: i64){
			self.ttl = val
		}
	}
	impl JuteSerializable for CreateTTLRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.data.serialize(out)?;
			self.acl.serialize(out)?;
			self.flags.serialize(out)?;
			self.ttl.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 data : std::vec::Vec<u8>::deserialize(bytes)?,
				 acl : std::vec::Vec<ACL>::deserialize(bytes)?,
				 flags : i32::deserialize(bytes)?,
				 ttl : i64::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct DeleteRequest {
		pub path : String,
		pub version : i32,
	}
	impl DeleteRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_version(&self)->i32{
			self.version.clone()
		}
		pub fn set_version(&mut self,val: i32){
			self.version = val
		}
	}
	impl JuteSerializable for DeleteRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.version.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 version : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetChildrenRequest {
		pub path : String,
		pub watch : bool,
	}
	impl GetChildrenRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_watch(&self)->bool{
			self.watch.clone()
		}
		pub fn set_watch(&mut self,val: bool){
			self.watch = val
		}
	}
	impl JuteSerializable for GetChildrenRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.watch.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 watch : bool::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetAllChildrenNumberRequest {
		pub path : String,
	}
	impl GetAllChildrenNumberRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for GetAllChildrenNumberRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetChildren2Request {
		pub path : String,
		pub watch : bool,
	}
	impl GetChildren2Request {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_watch(&self)->bool{
			self.watch.clone()
		}
		pub fn set_watch(&mut self,val: bool){
			self.watch = val
		}
	}
	impl JuteSerializable for GetChildren2Request {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.watch.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 watch : bool::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct CheckVersionRequest {
		pub path : String,
		pub version : i32,
	}
	impl CheckVersionRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_version(&self)->i32{
			self.version.clone()
		}
		pub fn set_version(&mut self,val: i32){
			self.version = val
		}
	}
	impl JuteSerializable for CheckVersionRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.version.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 version : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetMaxChildrenRequest {
		pub path : String,
	}
	impl GetMaxChildrenRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for GetMaxChildrenRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetMaxChildrenResponse {
		pub max : i32,
	}
	impl GetMaxChildrenResponse {
		pub fn get_max(&self)->i32{
			self.max.clone()
		}
		pub fn set_max(&mut self,val: i32){
			self.max = val
		}
	}
	impl JuteSerializable for GetMaxChildrenResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.max.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 max : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetMaxChildrenRequest {
		pub path : String,
		pub max : i32,
	}
	impl SetMaxChildrenRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_max(&self)->i32{
			self.max.clone()
		}
		pub fn set_max(&mut self,val: i32){
			self.max = val
		}
	}
	impl JuteSerializable for SetMaxChildrenRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.max.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 max : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SyncRequest {
		pub path : String,
	}
	impl SyncRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for SyncRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SyncResponse {
		pub path : String,
	}
	impl SyncResponse {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for SyncResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetACLRequest {
		pub path : String,
	}
	impl GetACLRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for GetACLRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetACLRequest {
		pub path : String,
		pub acl : std::vec::Vec<ACL>,
		pub version : i32,
	}
	impl SetACLRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_acl(&self)->std::vec::Vec<ACL>{
			self.acl.clone()
		}
		pub fn set_acl(&mut self,val: std::vec::Vec<ACL>){
			self.acl = val
		}
		pub fn get_version(&self)->i32{
			self.version.clone()
		}
		pub fn set_version(&mut self,val: i32){
			self.version = val
		}
	}
	impl JuteSerializable for SetACLRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.acl.serialize(out)?;
			self.version.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 acl : std::vec::Vec<ACL>::deserialize(bytes)?,
				 version : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct SetACLResponse {
		pub stat : Stat,
	}
	impl SetACLResponse {
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for SetACLResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct AddWatchRequest {
		pub path : String,
		pub mode : i32,
	}
	impl AddWatchRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_mode(&self)->i32{
			self.mode.clone()
		}
		pub fn set_mode(&mut self,val: i32){
			self.mode = val
		}
	}
	impl JuteSerializable for AddWatchRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.mode.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 mode : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct WatcherEvent {
		pub type_ : i32,
		pub state : i32,
		pub path : String,
	}
	impl WatcherEvent {
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
		pub fn get_state(&self)->i32{
			self.state.clone()
		}
		pub fn set_state(&mut self,val: i32){
			self.state = val
		}
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for WatcherEvent {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.type_.serialize(out)?;
			self.state.serialize(out)?;
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 type_ : i32::deserialize(bytes)?,
				 state : i32::deserialize(bytes)?,
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct ErrorResponse {
		pub err : i32,
	}
	impl ErrorResponse {
		pub fn get_err(&self)->i32{
			self.err.clone()
		}
		pub fn set_err(&mut self,val: i32){
			self.err = val
		}
	}
	impl JuteSerializable for ErrorResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.err.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 err : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct CreateResponse {
		pub path : String,
	}
	impl CreateResponse {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
	}
	impl JuteSerializable for CreateResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct Create2Response {
		pub path : String,
		pub stat : Stat,
	}
	impl Create2Response {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for Create2Response {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct ExistsRequest {
		pub path : String,
		pub watch : bool,
	}
	impl ExistsRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_watch(&self)->bool{
			self.watch.clone()
		}
		pub fn set_watch(&mut self,val: bool){
			self.watch = val
		}
	}
	impl JuteSerializable for ExistsRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.watch.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 watch : bool::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct ExistsResponse {
		pub stat : Stat,
	}
	impl ExistsResponse {
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for ExistsResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetDataResponse {
		pub data : std::vec::Vec<u8>,
		pub stat : Stat,
	}
	impl GetDataResponse {
		pub fn get_data(&self)->std::vec::Vec<u8>{
			self.data.clone()
		}
		pub fn set_data(&mut self,val: std::vec::Vec<u8>){
			self.data = val
		}
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for GetDataResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.data.serialize(out)?;
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 data : std::vec::Vec<u8>::deserialize(bytes)?,
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetChildrenResponse {
		pub children : std::vec::Vec<String>,
	}
	impl GetChildrenResponse {
		pub fn get_children(&self)->std::vec::Vec<String>{
			self.children.clone()
		}
		pub fn set_children(&mut self,val: std::vec::Vec<String>){
			self.children = val
		}
	}
	impl JuteSerializable for GetChildrenResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.children.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 children : std::vec::Vec<String>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetAllChildrenNumberResponse {
		pub total_number : i32,
	}
	impl GetAllChildrenNumberResponse {
		pub fn get_total_number(&self)->i32{
			self.total_number.clone()
		}
		pub fn set_total_number(&mut self,val: i32){
			self.total_number = val
		}
	}
	impl JuteSerializable for GetAllChildrenNumberResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.total_number.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 total_number : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetChildren2Response {
		pub children : std::vec::Vec<String>,
		pub stat : Stat,
	}
	impl GetChildren2Response {
		pub fn get_children(&self)->std::vec::Vec<String>{
			self.children.clone()
		}
		pub fn set_children(&mut self,val: std::vec::Vec<String>){
			self.children = val
		}
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for GetChildren2Response {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.children.serialize(out)?;
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 children : std::vec::Vec<String>::deserialize(bytes)?,
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetACLResponse {
		pub acl : std::vec::Vec<ACL>,
		pub stat : Stat,
	}
	impl GetACLResponse {
		pub fn get_acl(&self)->std::vec::Vec<ACL>{
			self.acl.clone()
		}
		pub fn set_acl(&mut self,val: std::vec::Vec<ACL>){
			self.acl = val
		}
		pub fn get_stat(&self)->Stat{
			self.stat.clone()
		}
		pub fn set_stat(&mut self,val: Stat){
			self.stat = val
		}
	}
	impl JuteSerializable for GetACLResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.acl.serialize(out)?;
			self.stat.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 acl : std::vec::Vec<ACL>::deserialize(bytes)?,
				 stat : Stat::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct CheckWatchesRequest {
		pub path : String,
		pub type_ : i32,
	}
	impl CheckWatchesRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
	}
	impl JuteSerializable for CheckWatchesRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.type_.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 type_ : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct RemoveWatchesRequest {
		pub path : String,
		pub type_ : i32,
	}
	impl RemoveWatchesRequest {
		pub fn get_path(&self)->String{
			self.path.clone()
		}
		pub fn set_path(&mut self,val: String){
			self.path = val
		}
		pub fn get_type_(&self)->i32{
			self.type_.clone()
		}
		pub fn set_type_(&mut self,val: i32){
			self.type_ = val
		}
	}
	impl JuteSerializable for RemoveWatchesRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.path.serialize(out)?;
			self.type_.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 path : String::deserialize(bytes)?,
				 type_ : i32::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetEphemeralsRequest {
		pub prefix_path : String,
	}
	impl GetEphemeralsRequest {
		pub fn get_prefix_path(&self)->String{
			self.prefix_path.clone()
		}
		pub fn set_prefix_path(&mut self,val: String){
			self.prefix_path = val
		}
	}
	impl JuteSerializable for GetEphemeralsRequest {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.prefix_path.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 prefix_path : String::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct GetEphemeralsResponse {
		pub ephemerals : std::vec::Vec<String>,
	}
	impl GetEphemeralsResponse {
		pub fn get_ephemerals(&self)->std::vec::Vec<String>{
			self.ephemerals.clone()
		}
		pub fn set_ephemerals(&mut self,val: std::vec::Vec<String>){
			self.ephemerals = val
		}
	}
	impl JuteSerializable for GetEphemeralsResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.ephemerals.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 ephemerals : std::vec::Vec<String>::deserialize(bytes)?,
			}
		}
	}
	#[derive(Default,Clone)]
	pub struct WhoAmIResponse {
		pub client_info : std::vec::Vec<ClientInfo>,
	}
	impl WhoAmIResponse {
		pub fn get_client_info(&self)->std::vec::Vec<ClientInfo>{
			self.client_info.clone()
		}
		pub fn set_client_info(&mut self,val: std::vec::Vec<ClientInfo>){
			self.client_info = val
		}
	}
	impl JuteSerializable for WhoAmIResponse {
		fn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {
			self.client_info.serialize(out)?;
		}
		fn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {
			Self {
				 client_info : std::vec::Vec<ClientInfo>::deserialize(bytes)?,
			}
		}
	}
}
