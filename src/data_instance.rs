
pub use bv::BitVec;
use crate::data_type::DataType;

#[derive(Debug)]
pub enum DataInstanceValue
{
    Array([u8]),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8)

}

pub trait TDataInstance
{
    fn is_high_low_byte_order(self)->bool;
    fn get_current(&self,param:&str)->DataInstanceValue;
    fn get_nominal(self,param:&str)->DataInstanceValue;
    fn get_pending(self,param:&str)->DataInstanceValue;
    fn set_pending(self,param:&str,value:DataInstanceValue);
    fn update_data_instance(self,bit_array:&BitVec);
    fn get_bit_length(&self)->usize;
    fn get_bit_position(&self)->usize;
    fn get_full_name(&self)->&str;
    fn get_parameter_key(&self)->&str;
    fn get_name(&self)->&str;
    fn reset(&self);
    fn get_internal_data_instance(&self)->&Vec<& dyn TDataInstance>;
}

fn value_to_bit_array(intvalue:DataInstanceValue,bitlength:u32,is_highlow_byte:bool)
{

}

pub struct DataInstance<'a>
{
    name:String,
    longname:String,
    bytePostiion:u32,
    bitPosition:u32,
    parent:&'a DataInstance<'a>,
    datatype:&'a dyn DataType,
    // for request data only
    pub pending_value:Option<Vec<u8>>,
    pub nominal_value:Option<Vec<u8>>,
    // for response data only 
    pub current_value:Option<Vec<u8>>,
}
impl<'a>  TDataInstance for DataInstance<'a>
{
    fn is_high_low_byte_order(&self)->bool
    {
        return self.datatype.is_high_low_byte_order();
    }
    
    fn set_pending(self,paramname:&str,value:DataInstanceValue)
    {
        if paramname.contains('.') 
        {panic!("This is the leaf data instance")}
        else {
            
        }
    }
    fn update_data_instance(self,bit_array:&BitVec);
    fn get_bit_length(&self)->usize;
    fn get_bit_position(&self)->usize;
    fn get_full_name(&self)->&str;
    fn get_parameter_key(&self)->&str;
    fn get_name(&self)->&str;
    fn reset(&self);
    fn get_internal_data_instance(&self)->&Vec<& dyn TDataInstance>;
} 
