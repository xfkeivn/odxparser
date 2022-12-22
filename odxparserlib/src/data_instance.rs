
use std::sync::Arc;
use std::cell::RefCell;
use bitvec::prelude::*;
use marcolib::Instance;
use crate::data_type::{ComputeMethod,InternalConstrain, DiagCodedType, DataObjectProp, Structure, DynamicLengthField,Mux,EndOfPDUField, EnvDataDesc, StaticField, Reversed};

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

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>;
    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>);

}


#[derive(Default)]
pub struct DataInstanceCore<T>
{
    
    pub byte_postiion:u32,
    pub parent:Option<Arc<RefCell<dyn TDataInstance>>>,
    pub bit_position:u32,
    pub datatype:Arc<RefCell<T>>,
    // for request data only
    pub pending_value:Option<Vec<u8>>,
    pub nominal_value:Option<Vec<u8>>,
    // for response data only 
    pub current_value:Option<Vec<u8>>,
}

#[derive(Default)]
pub struct CodedDataDataInstance
{
    pub instance_core:DataInstanceCore<DiagCodedType>,
}


impl TDataInstance for CodedDataDataInstance
{
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
   
}

#[derive(Default,Instance)]
pub struct DataObjectPropDataInstance
{
    pub instance_core:DataInstanceCore<DataObjectProp>
}


#[derive(Instance)]
pub struct StaticFieldInstance
{ 
    pub instance_core:DataInstanceCore<StaticField>,

}
#[derive(Default,Instance)]
pub struct ReversedInstance
{ 
    pub instance_core:DataInstanceCore<Reversed>,

}

impl StaticFieldInstance {
    fn get_element_name(&self)->String {
        
        return String::new();
    }
    
}

#[derive(Instance)]
pub struct DynamicLengthFieldInstance
{ 
    pub instance_core:DataInstanceCore<DynamicLengthField>,

}


#[derive(Instance)]
pub struct MuxInstance
{
    pub instance_core:DataInstanceCore<Mux>,
}



#[derive(Instance)]
pub struct EndOfPDUFieldInstance
{
    pub instance_core:DataInstanceCore<EndOfPDUField>,
}


#[derive(Instance)]
pub struct EnvDataDescInstance
{
    pub instance_core:DataInstanceCore<EnvDataDesc>,
}



#[derive(Default)]
pub struct ServiceMessageInstance{
    pub param_instances:Vec<Arc<RefCell<dyn TDataInstance>>>
}

#[derive(Default)]
pub struct StructureDataInstance
{
    pub instance_core:DataInstanceCore<Structure>,
    pub children_instances:Vec<Arc<RefCell<dyn TDataInstance>>>
    
}

impl StructureDataInstance
{


}

impl TDataInstance for StructureDataInstance
{
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.parent;
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       
       self.instance_core.parent = Some(parent);
    }
    
    fn get_full_name(&self)->String
    {
        let parent = self.instance_core.parent.as_ref();
        let full_name;
        match  parent
        {
        Some(p)=>{
          
            full_name = format!("{}.{}",p.try_borrow().unwrap().get_full_name(),self.instance_core.datatype.borrow().ident.short_name.as_str());
            
        },
        _=>{
         full_name = self.instance_core.datatype.borrow().ident.short_name.clone();
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