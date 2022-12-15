use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::vec;
use crate::data_instance::*;
#[derive(Debug,PartialEq)]
#[derive(Default)]
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

#[derive(Default)]
pub struct  Variant
{
    pub id:Identity,
    pub dtc_object_props:HashMap<String,Box<DTCDOP>>,
    pub data_object_props:HashMap<String,Box<DataObjectProp>>,
    pub env_data_descs:HashMap<String,Box<EnvDataDesc>>,
    pub structures:HashMap<String,Box<Structure>>,
    pub static_fileds:HashMap<String,Box<StaticField>>,
    pub dynamic_fileds:HashMap<String,Box<DynamicLengthField>>,
    pub endofpdu_fileds:HashMap<String,Box<EndOfPDUField>>,
    pub units:HashMap<String,Box<Unit>>,
    pub diag_comms:HashMap<String,Box<DiagSerivce>>,
    pub requests:HashMap<String,Box<ServiceMsgType>>,
    pub pos_responses:HashMap<String,Box<ServiceMsgType>>,
    pub neg_responses:HashMap<String,Box<ServiceMsgType>>,
    pub comparam_refs:HashMap<String,Box<ComParam>>,     
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
pub struct InternalConstrain
{
    pub scales:Vec<InternalConstrainScale>
}
#[derive(Debug)]
pub struct InternalConstrainScale
{
    pub upper_limit:u64,
    pub lower_limit:u64
}
#[derive(Debug)]
pub struct Unit
{
    pub ident:Identity,
    pub display_name:String
}
#[derive(Debug)]

pub struct Param
{
    pub shortname:String,
    pub longname:Option<String>,
    pub codedvalues:Vec<u32>,
    pub dop_ref:Option<String>,
    pub byte_position:Option<u32>,
    pub bit_position:Option<u32>,
    pub bit_length:Option<u32>,
    pub sematic:Option<String>,
    pub aa_type:Option<String>,
    pub variant_id:String,
    pub physical_constant_value:Option<u32>,
    pub diag_coded_type:Option<DiagCodedType>
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
    pub ref_id:String,
    pub doc_type:Option<String>,
    pub doc_ref:Option<String>,
    pub value:Option<String>,
}



pub trait  InstanceType<I> {
    fn create_data_instance(&self,name:&str,byte_postion:u32,bit_position:u32)->I;
    fn is_high_low_byte_order(&self)->bool
    {return false;}
}
#[derive(Default)]
pub struct DataObjectProp
{
    pub physical_type:Option<PhysicalType>,
    pub diag_coded_type:Option<DiagCodedType>,
    pub ident:Identity,
    pub compute_method:Option<Box<dyn ComputeMethod>>,
    pub unit_ref:Option<String>
}
#[derive(Default)]
pub struct Structure
{
    pub params:Vec<Box<Param>>,
    pub ident:Identity,
    pub bytesize:Option<u32>,
    ///Weak 可以用来解决循环引用赵成的内存如法释放，Variant拥有struct，struct又有variant，可能会照成循环引用
    pub variantId:String
   
}




#[derive(Default)]
pub struct EnvDataDesc{
    pub ident:Identity,
    pub param_snref:Option<String>,
    pub env_data_refs:Vec<String>,
    pub env_datas:Vec<EnvData>
}
#[derive(Default)]
pub struct EnvData
{
    pub ident:Identity,
    pub params:Vec<Param>

}
#[derive(Default)]
pub struct MuxCase
{
    pub shortname:String,
    pub ref_structure_id:Option<String>,
    pub switch_lower_lim:Option<u32>,
    pub switch_upper_lim:Option<u32>,
    pub is_default:bool
}
#[derive(Default)]
pub struct MuxSwitch
{
    pub byte_position:Option<u32>,
    pub ref_data_prop_id:Option<String>,
    pub bit_position:Option<u32>

}
#[derive(Default)]
pub struct EndOfPDUField
{
    pub ident:Identity,
    pub max_item_number:Option<u32>,
    pub min_item_number:Option<u32>,
    pub basic_struct_ref:Option<String>,
    pub variant_id:String,
}
#[derive(Default)]
pub struct Mux
{
    pub ident:Identity,
    pub variant_id:String,
    pub cases:Vec<MuxCase>,
    pub default_case:Option<MuxCase>,
    pub switch_key:MuxSwitch,
    pub case_start_byte_offset:Option<u32>
}
#[derive(Default)]
pub struct StaticField
{
   pub   ident:Identity,
   pub   ref_struct_id:Option<String>,
   pub   size:Option<u32>,
   pub   item_size:Option<u32>,
   pub   variant_id:String

}
#[derive(Default)]
pub struct DynamicLengthField
{
    pub ident:Identity,
    pub ref_struct_id:Option<String>,
    pub variant_id:String,
    pub offset_of_first_basic_structure:Option<u32>,
    //length_determind_dop_refid:Option<String>
    pub byte_pos_length_determined_dop:Option<String>,
}

#[derive(Default)]
pub struct DTCDOP
{
    pub ident:Identity,
    pub dataObjectProp:DataObjectProp,
    pub dtcs:Vec<Box<DTC>>
}
#[derive(Default)]
pub struct SeviceMsgPayload
{
    pub ident:Identity,
    pub params:Vec<Param>
}

pub enum ServiceMsgType {
    Request(SeviceMsgPayload),
    PositiveResponse(SeviceMsgPayload),
    NegativeReponse(SeviceMsgPayload)
}

#[derive(Default)]
pub struct DiagSerivce
{
    pub ident:Identity,
    pub semantic:Option<String>,
    pub request_ref:String,
    pub pos_response_ref:Option<String>,
    pub neg_response_ref:Option<String>,
    pub func_class_ref:Option<String>    
}

pub enum DataTypeEnum<'a> {
    DiagCodedType(&'a DiagCodedType),
    StaticField(&'a StaticField),
    DynamicLengthField(&'a DynamicLengthField),
    Structure(&'a Structure),
    Mux(&'a Mux),
    DataObjectProp(&'a DataObjectProp),
    EndOfPDUField(&'a EndOfPDUField)
}