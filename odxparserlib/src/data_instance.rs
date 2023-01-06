use std::{sync::Arc};
use std::cell::RefCell;
use bitvec::prelude::*;
use crate::data_type::{DiagCodedType, DataObjectProp, Structure, DynamicLengthField,Mux,EndOfPDUField, EnvDataDesc, StaticField, Reversed, MuxSwitch, DataTypeEnum};
pub type BitVecU8 = BitVec<u8,Lsb0>; 
pub trait TDataInstance
{
    fn is_high_low_byte_order(&self)->bool{return false;}
    fn update_data_instance(&mut self,bit_array:&BitVecU8);
    fn get_bit_length(&self)->usize;
    fn get_bit_position(&self)->usize;
    fn get_byte_position(&self)->usize;
    fn get_full_name(&self)->String;
    fn get_name(&self)->String;
    fn reset(&mut self);
    fn get_pending(&self)->Option<BitVecU8>;
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8);
    fn get_current(&mut self,param_name:&str)->BitVecU8;
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>;
    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>);
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>>;
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
    pub pending_value:Option<BitVecU8>,
    // for the default value of the instance
    pub nominal_value:Option<BitVecU8>,
    // for response data only, update_data_instance will update the current value 
    pub current_value:Option<BitVecU8>,
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
    fn get_current(&mut self,param_name:&str)->BitVecU8
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
    fn update_data_instance(&mut self,bit_array:&BitVecU8)
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
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8)
    {
        self.pending_value = Some(pending_value.clone());
    }
    fn get_pending(&self)->Option<BitVecU8> {
        return self.pending_value.clone();
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None;
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

    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8)
    {
       // don't do anything
    }
    fn get_pending(&self)->Option<BitVecU8> {

        let byte_len = self.get_bit_length()/8;
        let coded_value = self.coded_values.first().unwrap();
        let bytevec = coded_value.to_le_bytes().to_vec()[0..byte_len].to_vec();
        return Some(BitVecU8::from_vec(bytevec));
    }
    fn get_name(&self)->String {
        return self.instance_core.get_name();
    }
    fn reset(&mut self) {
        self.instance_core.reset();
        let v = self.get_bit_length()/8;
        let zero_vec = vec![0u8; v];
        self.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
        self.instance_core.current_value = Some(BitVecU8::from_vec(zero_vec.clone()));
           
        
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_length()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        // the coded value cannot be updated 
        //self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None;
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_bit_position(&self)->usize {
        return self.instance_core.get_bit_position()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset();
        let v = self.get_bit_length()/8;
        let zero_vec = vec![0u8; v];
        self.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
        self.instance_core.current_value = Some(BitVecU8::from_vec(zero_vec.clone()));
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None;
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn reset(&mut self) {
        self.instance_core.reset();
        let v = self.get_bit_length()/8;
        let zero_vec = vec![0u8; v];
        self.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
        self.instance_core.current_value = Some(BitVecU8::from_vec(zero_vec.clone()));
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array);
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
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)

    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.children_instances.clone())
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_bit_length(&self)->usize {
        return self.instance_core.get_bit_length()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset();
        let v = self.get_bit_length()/8;
        let zero_vec = vec![0u8; v];
        self.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
        self.instance_core.current_value = Some(BitVecU8::from_vec(zero_vec.clone()));
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None
    }
}




#[derive(Default)]
pub struct ListLengthInstance
{
    pub instance_core:DataInstanceCore<u32>,
}



impl TDataInstance for  ListLengthInstance {
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_bit_length(&self)->usize {
        return self.instance_core.get_bit_length()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None
    }
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.children_instances.clone())
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.children_case_instances.clone())
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
    fn update_data_instance(&mut self,bit_array:&BitVecU8)
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
           let ci = child_datatype.as_ref().unwrap().create_data_instance(name, 0, current_pos as u32,Option::None);
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
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }

    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.children_instances.clone())
    }

}



#[derive(Default)]
pub struct EnvDataDescInstance
{
    pub instance_core:DataInstanceCore<EnvDataDesc>,
   
}


impl TDataInstance for EnvDataDescInstance {
    fn get_pending(&self)->Option<BitVecU8> {
        return self.instance_core.get_pending();
    }
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

    fn get_bit_length(&self)->usize {
        return self.instance_core.get_bit_length()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Option::None
    }
}



pub trait StructInstance {
    fn as_struct(&self)->&StructureDataInstance;
    fn as_mut_struct(&mut self)->&mut StructureDataInstance;
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

    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        //self.children_instances.clear();
        self.instance_core.current_value = Some(bit_array.clone());
        for instance in self.children_instances.iter()
        {
         let mut child_instance = instance.as_ref().borrow_mut();
         let byte_position = child_instance.get_byte_position();
         let bit_position = child_instance.get_bit_position();
         let bit_length = child_instance.get_bit_length();
         let bit_vec_slice = &bit_array[bit_position+byte_position*8..bit_position+bit_length+byte_position*8];
         let child_bitvec = BitVec::from_bitslice(bit_vec_slice);
         child_instance.update_data_instance(&child_bitvec);
        }
    }

    fn get_current(&mut self,param_name:&str)->BitVecU8
    {
        if param_name == ""
        { 
            let mut current_bit_pos = 0;
            let mut bit_array = BitVecU8::new();
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
        let v = self.instance_core.datatype.borrow().bytesize;
        match v
        {
            Some(p)=>{
                let zero_vec = vec![0u8; p as usize];
                self.instance_core.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
                self.set_pending("", &BitVecU8::from_vec(zero_vec.clone()));
                self.instance_core.current_value = Some(BitVecU8::from_vec(zero_vec.clone()));
            },
            _=>{
                panic!("The struct {} has no byte-size",self.get_name());

            }
        }
    }
    fn set_pending(&mut self,param:&str,pending_value:&BitVecU8)
    {
        
        if param == ""
        {
            //update the all instance pending value

            if pending_value.len() != self.get_bit_length()
            {
                panic!("The set pending data length is not the size of my size");
            }

            self.instance_core.set_pending("", pending_value);
            //let bit_vec:BitVecU8= pending_value;
           for instance in self.children_instances.iter()
           {
            let mut child_instance = instance.as_ref().borrow_mut();
            let byte_position = child_instance.get_byte_position();
            let bit_position = child_instance.get_bit_position();
            let bit_length = child_instance.get_bit_length();
            let bit_start = bit_position+byte_position*8;
            let bit_end = bit_position+byte_position*8+bit_length;
            let bit_vec_slice = &pending_value.as_bitslice()[bit_start..bit_end];
            //println!("{}",bit_vec_slice);
            let child_bitvec = BitVecU8::from_bitslice(bit_vec_slice);
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
                paths.remove(0);
                let remainder_string = paths.join(".");
                child_instance.as_ref().borrow_mut().set_pending(remainder_string.as_str(), pending_value);
                // updated the parents pending data as well
               // let byte_position = child_instance.as_ref().borrow().get_byte_position();
             //   let bit_position = child_instance.as_ref().borrow().get_bit_position();
              //  let bit_length = child_instance.as_ref().borrow().get_bit_length();
              //  let bit_start = bit_position+byte_position*8;
               
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
    fn get_pending(&self)->Option<BitVecU8> {
       //
        let mut bitvect = BitVecU8::new();
       for instance in self.children_instances.iter()
       {
        let mut child_instance = instance.as_ref().borrow_mut();
        let byte_position = child_instance.get_byte_position();
        let bit_position = child_instance.get_bit_position();
        let bit_length = child_instance.get_bit_length();
        let bit_start = bit_position+byte_position*8;
        let bit_end = bit_position+byte_position*8+bit_length;
        let child_pending_data = child_instance.get_pending();
        let size = bitvect.as_bitslice().len();
        if (bit_start == bitvect.as_bitslice().len())
        {
            bitvect.extend_from_bitslice(child_pending_data.unwrap().as_bitslice());
        }
        else {
            let padding_count = bit_start - bitvect.as_bitslice().len();
            for _ in (0..padding_count)
            {
                bitvect.push(false);

            }
            
        }

       }

       return Some(bitvect);

       // return self.instance_core.get_pending();
    }

    fn get_bit_length(&self)->usize {

        let bytesize = self.instance_core.datatype.borrow().bytesize;
        match bytesize
        {
            Some(size)=>{return 8*size as usize;}
            
            _=>{
            return self.instance_core.get_bit_length()
            }
        }

       
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }

    fn get_name(&self)->String {
        self.instance_core.get_name()
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.children_instances.clone())
    }

}


#[derive(Default)]

pub struct ServiceMessageInstance {
    pub short_name:String,
    pub long_name:Option<String>,
    pub id:String,
    //pub param_instances:Vec<Arc<RefCell<dyn TDataInstance>>>,
    pub struct_data_instance:StructureDataInstance,
    pub instance_core:DataInstanceCore<Structure>,
}


impl TDataInstance for ServiceMessageInstance {
    fn get_pending(&self)->Option<BitVecU8> {
        return self.struct_data_instance.get_pending();
    }
    fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
    {
        return &self.instance_core.get_parent();
    }

    fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
    {
       self.instance_core.set_parent(parent)
    }

   
    fn get_full_name(&self)->String {
        self.get_name()
    }

    fn get_bit_length(&self)->usize {
        return self.instance_core.get_bit_length()
    }
    fn get_byte_position(&self)->usize {
        self.instance_core.get_byte_position()
    }
    fn get_bit_position(&self)->usize {
        self.instance_core.get_bit_position()
    }
    fn get_current(&mut self,param_name:&str)->BitVecU8 {
        self.instance_core.get_current(param_name)
    }
    fn get_name(&self)->String {
        self.short_name.clone()
    }
    fn reset(&mut self) {
        self.instance_core.reset()
    }
    fn set_pending(&mut self,paramname:&str,pending_value:&BitVecU8) {
        self.instance_core.set_pending(paramname, pending_value)
    }
    fn update_data_instance(&mut self,bit_array:&BitVecU8) {
        self.instance_core.update_data_instance(bit_array)
    }
    fn get_children(&self)->Option<Vec<Arc<RefCell<dyn TDataInstance>>>> {
        return Some(self.struct_data_instance.children_instances.clone())
    }
    
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
    pub request_instance:Arc<RefCell<ServiceMessageInstance>>,
    pub positive_response_instance:Option<Arc<RefCell<ServiceMessageInstance>>>,
    pub negative_response_instance:Option<Arc<RefCell<ServiceMessageInstance>>>,
} 