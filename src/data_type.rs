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

    pub structures:HashMap<String,Box<Structure>>,

    pub static_fileds:HashMap<String,Box<StaticField>>,

    pub dynamic_fileds:HashMap<String,Box<DynamicLengthField>>,
    pub endofpdu_fileds:HashMap<String,Box<EndOfPDUField>>,
    /*
    pub units:HashMap<String,DataObjectProp>,
    
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
    pub variantId:String
   
}


pub struct EnvDataDesc{
    pub ident:Identity,
    //pub param_snref:Option<String>,
   // pub env_data_refs:Vec<String>,
    //pub env_datas:Vec<EnvData>
}

pub struct EnvData
{
    pub structure:Structure,
    pub dtc_values:Vec<u64>

}
pub struct MuxCase
{
    pub shortname:String,
    pub ref_structure_id:Option<String>,
    pub switch_lower_lim:Option<u32>,
    pub switch_upper_lim:Option<u32>,
    pub is_default:bool
}
pub struct MuxSwitch
{
    pub byte_position:Option<u32>,
    pub ref_data_prop_id:Option<String>,
    pub bit_position:Option<u32>

}

pub struct EndOfPDUField
{
    pub ident:Identity,
    pub max_item_number:Option<u32>,
    pub min_item_number:Option<u32>,
    pub basic_struct_ref:Option<String>,
    pub variant_id:String,
}

pub struct Mux
{
    pub ident:Identity,
    pub variant_id:String,
    pub cases:Vec<MuxCase>,
    pub default_case:Option<MuxCase>,
    pub switch_key:MuxSwitch,
    pub case_start_byte_offset:Option<u32>
}

pub struct StaticField
{
   pub   ident:Identity,
   pub   ref_struct_id:Option<String>,
   pub   size:Option<u32>,
   pub   item_size:Option<u32>,
   pub   variant_id:String

}

pub struct DynamicLengthField
{
    pub ident:Identity,
    pub ref_struct_id:Option<String>,
    pub variant_id:String,
    pub offset_of_first_basic_structure:Option<u32>,
    //length_determind_dop_refid:Option<String>
    pub byte_pos_length_determined_dop:Option<String>,
}


pub struct DTCDOP
{
    pub ident:Identity,
    pub dataObjectProp:DataObjectProp,
    pub dtcs:Vec<Box<DTC>>
}