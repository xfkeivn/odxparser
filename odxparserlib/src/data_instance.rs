use std::{sync::Arc};
use std::cell::RefCell;
use bitvec::prelude::*;
use crate::data_type::{DiagCodedType, DataObjectProp, Structure, DynamicLengthField,Mux,EndOfPDUField, EnvDataDesc, StaticField, Reversed, MuxSwitch, DataTypeEnum};
use marcolib::Instance;
pub trait TDataInstance
{
    fn is_high_low_byte_order(&self)->bool{return false;}
    fn update_data_instance(&mut self,bit_array:&BitVec){unimplemented!()}
    fn get_bit_length(&self)->usize{unimplemented!();}
    fn get_bit_position(&self)->usize{unimplemented!();}
    fn get_byte_position(&self)->usize{unimplemented!();}
    fn get_full_name(&self)->String{unimplemented!();}
    fn get_name(&self)->String{unimplemented!();}
    fn reset(&mut self){unimplemented!();}
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVec){unimplemented!();}
    fn get_current(&mut self,param_name:&str)->BitVec{unimplemented!();}
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>{unimplemented!();}
    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>){unimplemented!();}
}


#[derive(Default)]
pub struct DataInstanceCore<T>
{  
    // the name of the data instance 
    pub name:String,
    // the parent of this instance 
    pub parent:Option<Arc<RefCell<dyn TDataInstance>>>,
    // the byte position in its parent structure 
    pub byte_position:u32,
    // the bit position in the byte position
    pub bit_position:u32,
    // the bit length 
    pub bit_length:u32,
    // the data type reference
    pub datatype:Arc<RefCell<T>>,
    // for request data only
    pub pending_value:Option<BitVec<usize>>,
    // for the default value of the instance
    pub nominal_value:Option<BitVec<usize>>,
    // for response data only, update_data_instance will update the current value 
    pub current_value:Option<BitVec<usize>>,
}



impl <T> TDataInstance for DataInstanceCore<T> {

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.parent;
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.parent = Some(parent);
    }
    fn get_current(&mut self,param_name:&str)->BitVec
        {
        if param_name!=""
        {
            panic!("The name of should be empty");
        }
            
        return self.current_value.as_ref().unwrap().clone();
    }


    fn is_high_low_byte_order(&self)->bool
    {
        return false;
    }
    fn update_data_instance(&mut self,bit_array:&BitVec)
    {
        self.current_value = Some(bit_array.clone());
    }
  
    fn get_bit_length(&self)->usize
    {
        return self.bit_length as usize;
    }
    fn get_bit_position(&self)->usize
    {
        return self.bit_position as usize;
    }    
    fn get_byte_position(&self)->usize
    {
        return self.byte_position as usize;
    }
    fn get_full_name(&self)->String
    {
        return self.get_parent().as_ref().unwrap().as_ref().borrow().get_full_name() + "." + self.name.as_str()
    }

    fn get_name(&self)->String
    {
        return self.name.clone();
    }

    fn reset(&mut self){
       self.current_value = Option::None;
       self.pending_value = Option::None;
       self.nominal_value = Option::None;

    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVec)
    {
        self.pending_value = Some(pending_value.clone());
    }


}

#[derive(Default)]
pub struct CodedDataDataInstance
{
    pub instance_core:DataInstanceCore<DiagCodedType>,
    pub coded_values:Vec<u32>

}



impl TDataInstance for CodedDataDataInstance
{
    fn get_bit_length(&self)->usize
    {
        return self.instance_core.datatype.as_ref().borrow().bit_length.unwrap() as usize;
    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn set_pending(&mut self,paramname:&str,pending_value:&BitVec)
    {
       // don't do anything
    }
}




impl CodedDataDataInstance {
    fn get_coded_value(&self)
    {
        self.coded_values.first().unwrap();
    }
    fn set_pending()
    {
        panic!("coded instance cannot be called set pending")
    }
    fn get_name(&self)->String
    {
        return self.instance_core.name.clone();
    }

}

#[derive(Default)]
pub struct DataObjectPropDataInstance
{
    pub instance_core:DataInstanceCore<DataObjectProp>
}




impl TDataInstance for DataObjectPropDataInstance{

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn get_bit_length(&self)->usize
    {
        return self.instance_core.datatype.as_ref().borrow().diag_coded_type.as_ref().unwrap().borrow().bit_length.unwrap() as usize;
    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}


#[derive(Default)]
pub struct StaticFieldInstance
{ 
    pub instance_core:DataInstanceCore<StaticField>,
    pub children_instances:Vec<Arc<RefCell<dyn TDataInstance>>>
}



impl StaticFieldInstance {
    fn get_element_name(&self)->String {
        
        return String::new();
    }
    
}

impl TDataInstance for StaticFieldInstance{

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn get_bit_length(&self)->usize
    {
        let s = self.instance_core.datatype.as_ref().borrow();
        return s.item_size.unwrap() as usize * s.size.unwrap() as usize;
    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}


#[derive(Default)]
pub struct ReversedInstance
{ 
    pub instance_core:DataInstanceCore<Reversed>,

}




impl TDataInstance for ReversedInstance{

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

   
    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}




#[derive(Default)]
pub struct ListLengthInstance
{
    pub instance_core:DataInstanceCore<u32>,
}



impl TDataInstance for  ListLengthInstance {
    
}

#[derive(Default)]
pub struct DynamicLengthFieldInstance
{ 
    pub instance_core:DataInstanceCore<DynamicLengthField>,
    pub children_instances:Vec<Arc<RefCell<dyn TDataInstance>>>,    
}





impl TDataInstance for DynamicLengthFieldInstance{

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn get_bit_length(&self)->usize
    {
        let mut bit_length = 0;
        for child in self.children_instances.iter()
        {
            bit_length+=child.as_ref().borrow().get_bit_length();
        }
       return bit_length;
    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}






#[derive(Default)]
pub struct MuxInstance
{
    pub instance_core:DataInstanceCore<Mux>,
    pub children_case_instances:Vec<Arc<RefCell<dyn TDataInstance>>>,
    pub default_case_instance:Option<Arc<RefCell<dyn TDataInstance>>>,
    pub mux_switch_case_instance:Option<Arc<RefCell<dyn TDataInstance>>>,
    pub current_case:Option<Arc<RefCell<dyn TDataInstance>>>
}


impl TDataInstance for MuxInstance{

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn get_bit_length(&self)->usize
    {
       match &self.current_case
       {
        Some(current_case)=>{
            return current_case.as_ref().borrow().get_bit_length();
        }
        _=>{
            return 0;
        }
       }

    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}



#[derive(Default)]
pub struct EndOfPDUFieldInstance
{
    pub instance_core:DataInstanceCore<EndOfPDUField>,
    pub children_instances:Vec<Arc<RefCell<dyn TDataInstance>>>,
}



impl TDataInstance for EndOfPDUFieldInstance
{
    fn update_data_instance(&mut self,bit_array:&BitVec)
    {
        let mut current_pos = 0;
        let index = 0;
        self.children_instances.clear();
        let datatyperef = self.instance_core.datatype.as_ref().borrow();
        let bsr = datatyperef.basic_struct_ref.as_ref().unwrap();
        let variant_ref = datatyperef.ident.variant.as_ref().unwrap().as_ref().borrow();
        let child_datatype = variant_ref.get_data_type_by_ref_id(bsr);
        let index = 0;
        while current_pos < bit_array.len()
        {
           let name = format!("[{}]",index);
           let name =  self.instance_core.name.as_str();
           let ci = child_datatype.as_ref().unwrap().create_data_instance(name, 0, current_pos as u32);
           let childbitlen = ci.as_ref().borrow().get_bit_length();
           self.children_instances.push(ci);
           current_pos+=childbitlen;


        }
        
       
    }
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

    fn get_bit_length(&self)->usize
    {
        let mut bit_length = 0;
        for child in self.children_instances.iter()
        {
            bit_length+=child.as_ref().borrow().get_bit_length();
        }
       return bit_length;

    }

    fn get_full_name(&self)->String {
        return self.instance_core.get_full_name();
    }
}



#[derive(Instance)]
pub struct EnvDataDescInstance
{
    pub instance_core:DataInstanceCore<EnvDataDesc>,
   
}



pub trait StructInstance {
    fn as_struct(&self)->&StructureDataInstance;
    fn as_mut_struct(& mut self)->&mut StructureDataInstance;
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

    fn update_data_instance(&mut self,bit_array:&BitVec) {
        //self.children_instances.clear();
        for instance in self.children_instances.iter()
        {
         let mut child_instance = instance.as_ref().borrow_mut();
         let bit_position = child_instance.get_bit_position();
         let bit_length = child_instance.get_bit_length();
         let bit_vec_slice = &bit_array[bit_position..=bit_position+bit_length];
         let child_bitvec = BitVec::from_bitslice(bit_vec_slice);
         child_instance.update_data_instance(&child_bitvec);
        }
    }

    fn get_current(&mut self,param_name:&str)->BitVec
    {
        if param_name == ""
        { 
            let mut current_bit_pos = 0;
            let mut bit_array = BitVec::<usize,Lsb0>::new();
            for instance in self.children_instances.iter()
            {
             let mut child_instance = instance.as_ref().borrow_mut();
             let bit_position = child_instance.get_bit_position();
             let bit_length = child_instance.get_bit_length();
             let child_data = child_instance.get_current("");
             if bit_position == current_bit_pos
             {
                if child_data.len() <bit_length
                {   let v = format!("The bit array of data instance does not match it bit_length, It is impossibl {}",child_instance.get_name());
                    
                }
              

             }
             else {
                let padding_num = bit_position - current_bit_pos;
                for _ in (0..padding_num)
                {
                    bit_array.push(false);
                }


             }
             bit_array.extend(child_data);
             current_bit_pos= bit_position + bit_length;
            }
            self.instance_core.current_value = Some(bit_array.clone());
            return bit_array;
        }
        else
        {
            let mut paths:Vec<&str> =  param_name.split(".").collect();
            let header = paths[0];
            let child_instance_name;
            match  header.find("[")
            {
                Some(index)=>{
                    child_instance_name = &header[0..index];


                },
                _=>{
                    child_instance_name = header;}
            }
            for child_instance in self.children_instances.iter()
            {
                let mut childref = child_instance.as_ref().borrow_mut();
                if child_instance_name ==  childref.get_name()
                {   paths.remove(0);
                    let remainder_string = paths.join(".");
                    return childref.get_current(remainder_string.as_str());
                    
                }

            }
            panic!("There is no child instance found {}",child_instance_name);    
        }
            
        

    }
    fn reset(& mut self){
        
        for instance in self.children_instances.iter()
        {
            instance.as_ref().borrow_mut().reset();
        }
        self.instance_core.reset();
    }
    fn set_pending(&mut self,param:&str,pending_value:&BitVec)
    {
        if param == ""
        {
            //let bit_vec:BitVec<u8,Lsb0>= pending_value;
           for instance in self.children_instances.iter()
           {
            let mut child_instance = instance.as_ref().borrow_mut();
            let bit_position = child_instance.get_bit_position();
            let bit_length = child_instance.get_bit_length();
            let bit_vec_slice = &pending_value[bit_position..=bit_position+bit_length];
            let child_bitvec = BitVec::from_bitslice(bit_vec_slice);
            child_instance.set_pending("", &child_bitvec);
           }
        }
        else {
           let mut paths:Vec<&str> =  param.split(".").collect();
           let header = paths[0];
           for child_instance in self.children_instances.iter_mut()
           {
            if header ==  child_instance.as_ref().borrow().get_name()
            {
                let remainder_string = paths.join(".");
                paths.remove(0);
                child_instance.as_ref().borrow_mut().set_pending(remainder_string.as_str(), pending_value);
                break;
            }
           
           }
           

        
        }

    }

    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }
   


}


#[derive(Default)]

pub struct ServiceMessageInstance {
    pub short_name:String,
    pub long_name:Option<String>,
    pub id:String,
    //pub param_instances:Vec<Arc<RefCell<dyn TDataInstance>>>,
    pub struct_data_instance:StructureDataInstance

}

impl StructInstance for ServiceMessageInstance {
    fn as_struct(&self)->&StructureDataInstance
    {
        return &self.struct_data_instance;
    }
    fn as_mut_struct(& mut self)->&mut StructureDataInstance
    {
        return &mut self.struct_data_instance;

    }
    
}




#[derive(Default)]
pub struct DiagServiceInstance
{
    pub diag_service_name:String,
    pub request_instance:ServiceMessageInstance,
    pub positive_response_instance:Option<ServiceMessageInstance>,
    pub negative_response_instance:Option<ServiceMessageInstance>,
} 