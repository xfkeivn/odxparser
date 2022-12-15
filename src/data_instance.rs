
use std::{rc::Rc, fmt::format};
use bitvec::prelude::*;
use std::cell::*;
use crate::data_type::{InstanceType, ComputeMethod,InternalConstrain, DiagCodedType, DataObjectProp, Structure, DataTypeEnum};

#[derive(Debug)]
pub enum DataInstanceValue
{
    Bytes(Vec<u8>),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8)

}

pub trait TDataInstance<'a> 
{
    fn is_high_low_byte_order(&self)->bool
    {
        return false;
    }
    fn update_data_instance(&self,bit_array:&BitVec)
    {

    }
  
    fn get_bit_length(&self)->usize
    {
        return usize::default();
    }
    fn get_bit_position(&self)->usize
    {
        return usize::default();
    }
    fn get_full_name(&self)->String;
    fn get_type(&self)->DataTypeEnum;
    fn get_parameter_key(&self)->String;
    fn reset(&self){}
    fn set_pending(&self,paramname:&str,pending_value:Vec<u8>)
    {}

    fn get_parent(&self)->&Option<&'a dyn TDataInstance>;
    fn set_parent(&self,parent:Option<&'a dyn TDataInstance>);

}



#[derive(Default)]
pub struct DataInstanceCore<'a,T>
{
    pub name:String,
    pub full_name:String,
    pub long_name:String,
    pub byte_postiion:u32,
    pub bit_position:u32,
    pub parent:Option<&'a dyn TDataInstance<'a>>,
    pub datatype:Rc<T>,
    // for request data only
    pub pending_value:Option<Vec<u8>>,
    pub nominal_value:Option<Vec<u8>>,
    // for response data only 
    pub current_value:Option<Vec<u8>>,
}

pub struct CodedDataDataInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,DiagCodedType>,
    pub coded_value:u64,
    pub bit_length:u32
}

pub struct DataObjectPropDataInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,DataObjectProp>,
    pub compute_method:&'a dyn ComputeMethod,
    pub interal_constraint:InternalConstrain,
    pub unit_ref_id:String
}
#[derive(Default)]
pub struct StructureDataInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,Structure>,
    pub internal_data_instances:Vec<&'a dyn TDataInstance<'a>>
}

impl<'a> StructureDataInstance<'a>
{
    pub fn get_interal_dataInstance(&self)->&Vec<&'a dyn TDataInstance>
    {

        return &self.internal_data_instances
    }
}

impl<'a> TDataInstance<'a> for StructureDataInstance<'a>
{
    fn get_type(&self)->DataTypeEnum {
        return DataTypeEnum::Structure(&self.instance_core.datatype);
    }
    fn get_parent(&self)->&Option<&'a dyn TDataInstance>
    {
        return &self.instance_core.parent
    }
    fn set_parent(&self,parent:Option<&'a dyn TDataInstance>)
    {

    }
    
    fn get_full_name(&self)->String
    {
        let parent = self.instance_core.parent;
        let full_name;
        match  parent
        {
        Some(p)=>{
            match p.get_type()
            {
               DataTypeEnum::StaticField(s)=>{
                   
                },
                _=>{

                }
            }

            let str = p.get_full_name();
            full_name = format!("{}.{}",str,self.instance_core.name.as_str());
            

        },
        _=>{
         full_name = self.instance_core.name.clone();
        }
        }
        return full_name;

                
            
        }
    
    fn get_parameter_key(&self)->String
    {
        return self.get_full_name();
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