use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::vec;
#[derive(Debug,PartialEq)]
pub struct Identity
{
    pub short_name:String,
    pub long_name:Option<String>,
    pub id:String
}

pub trait ComputeMethod {
    fn get_physical_value(&self,rawvalue:&[u8])->f64
    {
        return 0.0;
    }
}


pub struct  Variant
{
    pub id:Identity,
    
    pub dtc_object_props:HashMap<String,Box<DTCDOP>>,
    
    pub data_object_props:HashMap<String,Box<DataObjectProp>>,
    
    pub env_data_descs:HashMap<String,Box<EnvDataDesc>>,
    /*
    pub units:HashMap<String,DataObjectProp>,
    pub structures:HashMap<String,DataObjectProp>,
    pub diag_comms:HashMap<String,DataObjectProp>,
    pub diag_comms_name_map:HashMap<String,DataObjectProp>,
    pub requests:HashMap<String,DataObjectProp>,
    pub pos_responses:HashMap<String,DataObjectProp>,
    pub neg_responses:HashMap<String,DataObjectProp>,
    pub comparam_refs:HashMap<String,DataObjectProp>,     */
    pub func_classes:HashMap<String,Box<FunctionClass>>,

}
#[derive(Debug)]
pub struct ScaleLinear;
#[derive(Debug)]
pub struct Identical;
#[derive(Debug)]
pub struct Textable;
#[derive(Debug)]
pub struct Linear;
#[derive(Debug)]
pub struct InternalConstrain;
#[derive(Debug)]
pub struct InternalConstrainScale;
#[derive(Debug)]
pub struct Unit
{
    pub ident:Identity
}
#[derive(Debug)]

pub struct Param
{
    pub shortname:String,
    pub longname:Option<String>,
    pub codedvalues:Vec<u32>,
    pub physical_constant_value:Option<u32>,
    pub diag_coded_type:
}



impl ComputeMethod for ScaleLinear{}
impl ComputeMethod for Identical{}
impl ComputeMethod for Textable{}
impl ComputeMethod for Linear{}
#[derive(Debug)]
pub struct FunctionClass
{
    pub ident:Identity,
    pub description:String,
}

#[derive(Debug)]
pub struct DTC 
{
    pub ident:Identity,
    pub trouble_code:u64,
    pub display_trouble_code:String,
    pub text:String,

}
#[derive(Debug)]
pub struct DiagCodedType
{
    pub aa_type:Option<String>,
    pub base_type:Option<String>,
    pub bit_length:Option<u32>,
    pub ishighbyteorder:Option<bool>
}
#[derive(Debug)]
pub struct PhysicalType{
  pub base_data_type:Option<String>,
  pub display_radix:Option<String>
}
#[derive(Debug)]
pub struct ComParam
{
    pub ref_id:Option<u32>,
    pub doc_type:String,
    pub value:u32,
}

pub trait  DataType {
    fn create_data_instance(&self,name:&str,byte_postion:u32,bit_position:u32);
}

pub struct DataObjectProp
{
    pub physical_type:Option<PhysicalType>,
    pub diag_coded_type:Option<DiagCodedType>,
    pub ident:Identity,
    pub compute_method:Option<Box<dyn ComputeMethod>>,
    pub unit_ref:Option<String>
}

pub struct Structure
{
    pub params:Vec<Box<Param>>,
    pub ident:Identity,
    pub bytesize:Option<u32>,
    ///Weak 可以用来解决循环引用赵成的内存如法释放，Variant拥有struct，struct又有variant，可能会照成循环引用
    pub variant:Weak<Variant>,
    dataObjectProp:DataObjectProp,
}


pub struct EnvDataDesc{
    pub ident:Identity
}

pub struct EnvData
{

}
pub struct MuxCase
{

}
pub struct MuxSwitch
{

}

pub struct EndOfPDUField
{
    dataObjectProp:DataObjectProp,
}

pub struct Mux
{
    dataObjectProp:DataObjectProp,
}

pub struct StaticField
{
    dataObjectProp:DataObjectProp,
}

pub struct DynamicLengthField
{
    dataObjectProp:DataObjectProp,
}


pub struct DTCDOP
{
    pub ident:Identity,
    pub dataObjectProp:DataObjectProp,
    pub dtcs:Vec<Box<DTC>>
}