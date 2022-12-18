
use std::{rc::{Rc, Weak}, any::Any, borrow::Borrow};
use std::cell::RefCell;
use bitvec::prelude::*;
use crate::data_type::{ComputeMethod,InternalConstrain, DiagCodedType, DataObjectProp, Structure};

#[derive(Debug)]
pub enum DataInstanceValue
{
    Bytes(Vec<u8>),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8)

}

pub trait TDataInstance<'a,T>
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
    fn get_full_name(&self)->String
    {
        return String::new();
    }
    fn get_parameter_key(&self)->String
    {
        
         return String::new();
        
    }
    fn reset(&self){}
    fn set_pending(&self,paramname:&str,pending_value:Vec<u8>)
    {

    }

    fn get_parent(&self)->&Option<&'a RefCell<&'a dyn TDataInstance<'a,T>>>;


    fn set_parent(&mut self,parent:&'a  RefCell<&'a dyn TDataInstance<'a,T> >);

}


#[derive(Default)]
pub struct DataInstanceCore<'a,T>
{
    pub name:String,
    pub full_name:String,
    pub long_name:String,
    pub byte_postiion:u32,
    pub parent:Option<&'a RefCell<&'a dyn TDataInstance<'a,T> >>,
    pub bit_position:u32,
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

pub struct StaticFieldInstance<'a>
{ 
    pub instance_core:DataInstanceCore<'a,DataObjectProp>,

}


impl<'a> StaticFieldInstance<'a> {
    fn get_element_name(&self)->String {
        
        return String::new();
    }
    
}

pub struct DynamicLengthFieldInstance<'a>
{ 
    pub instance_core:DataInstanceCore<'a,DataObjectProp>,

}
pub struct MuxInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,DataObjectProp>,
}

pub struct EndOfPDUFieldInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,DataObjectProp>,
}

#[derive(Default)]
pub struct StructureDataInstance<'a>
{
    pub instance_core:DataInstanceCore<'a,Structure>,
    pub children_instances:Vec<&'a dyn Any>
    
}

impl<'a> StructureDataInstance<'a>
{

}

impl<'a> TDataInstance<'a,Structure> for StructureDataInstance<'a>  
{

    fn get_parent(&self)->&Option<&'a RefCell<&'a dyn TDataInstance<'a,Structure>>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:&'a RefCell<&'a dyn TDataInstance<'a,Structure>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
    fn get_full_name(&self)->String
    {
        let parent = self.instance_core.parent;
        let full_name;
        let parent_full_name:String;
        match  parent
        {
        Some(p)=>{
          
            full_name = format!("{}.{}",p.borrow().get_full_name(),self.instance_core.name.as_str());
            
        },
        _=>{
         full_name = self.instance_core.name.clone();
        }
        }
        return full_name;

                
            
        }

   

    fn set_pending(&self,param:&str,pending_value:Vec<u8>)
    {
        if param == ""
        {
           
            
            
        }

    }

}