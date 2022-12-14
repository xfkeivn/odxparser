
use std::rc::Rc;

use bitvec::prelude::*;

use crate::data_type::{DataType, ComputeMethod,InternalConstrain};

#[derive(Debug)]
pub enum DataInstanceValue
{
    Bytes(Vec<u8>),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8)

}

pub trait TDataInstance< P> 
{
    fn is_high_low_byte_order(&self)->bool
    {
        return false;
    }
    fn update_data_instance(&self,bit_array:&BitVec)
    {

    }
    fn get_name(&self)->&str;
    fn get_bit_length(&self)->usize
    {
        return usize::default();
    }
    fn get_bit_position(&self)->usize
    {
        return usize::default();
    }
    fn get_full_name(&self)->&str
    {
        return self.get_name();
    }
    fn get_parameter_key(&self)->&str
    {
        return self.get_name();
    }
   
    fn reset(&self)
    {

    }
    fn get_parent(&self)->&P;
    fn set_pending(&self,paramname:&str,pending_value:Vec<u8>);


}


#[derive(Default)]
pub struct DataInstance<P,T> where P:TDataInstance<P>,T:DataType<Self>
{
    name:String,
    longname:String,
    bytePostiion:u32,
    bitPosition:u32,
    parent:Box<P>,
    datatype:Rc<T>,
    // for request data only
    pub pending_value:Option<Vec<u8>>,
    pub nominal_value:Option<Vec<u8>>,
    // for response data only 
    pub current_value:Option<Vec<u8>>,
}


impl<P,T>  TDataInstance<P> for DataInstance<P,T> where P:TDataInstance<P>+Default,T:DataType<Self>
{
    fn get_name(&self)->&str {
        return &self.name.as_str()
    }

    fn get_parent(&self)->&P
    {
        return self.parent.as_ref();
    }
    
    fn set_pending(&self,paramname:&str,pending_value:Vec<u8>)
    {
        if paramname.contains('.') 
        {panic!("This is the leaf data instance")}
        else {
            
        }
    }

   
} 

pub struct CodedDataDataInstance<P,T> where P:TDataInstance<P>+Default,T:DataType<Self>
{
    pub data_instance:DataInstance<P,T>,
    pub coded_value:u64,
    pub bit_length:u32
}

pub struct SimpleDataDataInstance<P:TDataInstance<P>+Default,T:DataType<Self>>
{
    pub data_instance:DataInstance<P,T>,
}

pub struct DataObjectPropDataInstance<P:TDataInstance<P>+Default,T:DataType<Self>>
{
    pub data_instance:DataInstance<P,T>,
    pub compute_method:&'a dyn ComputeMethod,
    pub interal_constraint:InternalConstrain,
    pub unit_ref_id:String
}

pub struct StructureDataInstance<P,T> where P:TDataInstance<P>+Default,T:DataType<Self>
{
    pub data_instance:DataInstance<P,T>,
    pub internal_data_instances:Vec<& dyn TDataInstance<Self>>
}

impl<P,T> StructureDataInstance<P,T> where P:TDataInstance<P>+Default,T:DataType<Self>
{
    pub fn get_interal_dataInstance(&self)->&Vec<& dyn TDataInstance<Self>>
    {

        return &self.internal_data_instances
    }
}

impl<P,T> TDataInstance<P> for StructureDataInstance<P,T> where P:TDataInstance<P>+Default,T:TDataInstance<Self>
{

    fn get_name(&self)->&str {
        return &&self.data_instance.name.as_str()
    }

    fn get_parent(&self)->&P
    {
        return &self.data_instance.parent
    }

    fn set_pending(&self,param:&str,pending_value:Vec<u8>)
    {
        if param == ""
        {
            for instance in &self.internal_data_instances
            {
               let bit_length =  instance.get_bit_length();
               let bit_position = instance.get_bit_position();

               let bits= pending_value.view_bits::<Lsb0>();
               let bytes = bits.chunks(8);
            }
            
            
        }

    }

}