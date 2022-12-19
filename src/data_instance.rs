
use std::{rc::{Rc, Weak}, any::Any, borrow::Borrow};
use std::cell::RefCell;
use bitvec::prelude::*;
use crate::data_type::{ComputeMethod,InternalConstrain, DiagCodedType, DataObjectProp, Structure, DynamicLengthField,Mux,EndOfPDUField, EnvDataDesc};

pub trait TDataInstance
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

    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>;
    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>);

}


#[derive(Default)]
pub struct DataInstanceCore<T>
{
    
    pub byte_postiion:u32,
    pub parent:Option<Rc<RefCell<dyn TDataInstance>>>,
    pub bit_position:u32,
    pub datatype:Rc<T>,
    // for request data only
    pub pending_value:Option<Vec<u8>>,
    pub nominal_value:Option<Vec<u8>>,
    // for response data only 
    pub current_value:Option<Vec<u8>>,
}

pub struct CodedDataDataInstance
{
    pub instance_core:DataInstanceCore<DiagCodedType>,
    pub coded_value:u64,
    pub bit_length:u32
}

#[derive(Default)]
pub struct DataObjectPropDataInstance
{
    pub instance_core:DataInstanceCore<DataObjectProp>
}

impl TDataInstance for DataObjectPropDataInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}


pub struct StaticFieldInstance
{ 
    pub instance_core:DataInstanceCore<DataObjectProp>,

}

impl TDataInstance for StaticFieldInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}



impl StaticFieldInstance {
    fn get_element_name(&self)->String {
        
        return String::new();
    }
    
}

pub struct DynamicLengthFieldInstance
{ 
    pub instance_core:DataInstanceCore<DynamicLengthField>,

}

impl TDataInstance for DynamicLengthFieldInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}
pub struct MuxInstance
{
    pub instance_core:DataInstanceCore<Mux>,
}

impl TDataInstance for MuxInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}

pub struct EndOfPDUFieldInstance
{
    pub instance_core:DataInstanceCore<EndOfPDUField>,
}

impl TDataInstance for EndOfPDUFieldInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}

pub struct EnvDataDescInstance
{
    pub instance_core:DataInstanceCore<EnvDataDesc>,
}


impl TDataInstance for EnvDataDescInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}








#[derive(Default)]
pub struct ServiceMessageInstance{
    pub param_instances:Vec<Rc<RefCell<dyn TDataInstance>>>
}







#[derive(Default)]
pub struct StructureDataInstance
{
    pub instance_core:DataInstanceCore<Structure>,
    pub children_instances:Vec<Rc<RefCell<dyn TDataInstance>>>
    
}

impl StructureDataInstance
{

}

impl TDataInstance for StructureDataInstance
{
    fn get_parent(&self)->&Option<Rc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Rc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
    fn get_full_name(&self)->String
    {
        let parent = self.instance_core.parent.as_ref();
        let full_name;
        let parent_full_name:String;
        match  parent
        {
        Some(p)=>{
          
            full_name = format!("{}.{}",p.try_borrow().unwrap().get_full_name(),self.instance_core.datatype.ident.short_name.as_str());
            
        },
        _=>{
         full_name = self.instance_core.datatype.ident.short_name.clone();
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

#[derive(Default)]
pub struct DiagServiceInstance
{
    pub request_instance:ServiceMessageInstance,
    pub positive_response_instance:ServiceMessageInstance,
    pub negative_response_instance:ServiceMessageInstance,
} 