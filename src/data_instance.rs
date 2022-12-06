
pub use bv::BitVec;
use crate::data_type::DataType;

#[derive(Debug)]
pub enum DataInstanceValue<'a>
{
    Array(&'a [u8]),
    ValueInt(u64)
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

pub struct DataInstance<'a>
{
    name:&'a str,
    bytePostiion:u32,
    bitPosition:u32,
    parent:&'a DataInstance<'a>,
    datatype:&'a dyn DataType,


}

