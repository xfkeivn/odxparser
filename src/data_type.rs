use std::collections::HashMap;

#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Identity<'a>
{
    pub short_name:&'a str,
    pub long_name:&'a str,
    pub id:&'a str
}

pub trait ComputeMethod {
    fn get_physical_value(&self,rawvalue:&[u8])->f64
    {
        return 0.0;
    }
}


pub struct  Variant<'a>
{
    pub id:Identity<'a>,
    /*
    dtc_object_props:HashMap<&'a str,DTCDOP<'a>>,
    data_object_props:HashMap<&'a str,DTCDOP<'a>>,
    env_data_descs:HashMap<&'a str,DTCDOP<'a>>,
    units:HashMap<&'a str,DTCDOP<'a>>,
    structures:HashMap<&'a str,DTCDOP<'a>>,
    diag_comms:HashMap<&'a str,DTCDOP<'a>>,
    diag_comms_name_map:HashMap<&'a str,DTCDOP<'a>>,
    requests:HashMap<&'a str,DTCDOP<'a>>,
    pos_responses:HashMap<&'a str,DTCDOP<'a>>,
    neg_responses:HashMap<&'a str,DTCDOP<'a>>,
    comparam_refs:HashMap<&'a str,DTCDOP<'a>>,
    func_classes:HashMap<&'a str,DTCDOP<'a>>,
     */
}

pub struct ScaleLinear;
pub struct Identical;
pub struct Textable;
pub struct Linear;
pub struct InternalConstrain;
pub struct InternalConstrainScale;
pub struct Unit;

pub struct Param;


impl ComputeMethod for ScaleLinear{}
impl ComputeMethod for Identical{}
impl ComputeMethod for Textable{}
impl ComputeMethod for Linear{}

pub struct FunctionClass<'a>
{
    ident:Identity<'a>,
    description:&'a str
}


pub struct DTC <'a>
{
    ident:Identity<'a>,
    trouble_code:u64,
    display_trouble_code:&'a str,
    text:&'a str,
    ref_id:Option<u32>
}

pub struct DiagCodedType<'a>
{
    pub aa_type:&'a str,
    pub base_type:&'a str,
    pub bit_length:u32,
    pub ishighbyteorder:bool
}

pub struct PhysicalType<'a>{
  base_data_type:&'a str
}
pub struct ComParam<'a>
{
    pub ref_id:Option<u32>,
    pub doc_type:&'a str,
    pub value:u32,
}

pub trait  DataType {
    fn create_data_instance(&self,name:&str,byte_postion:u32,bit_position:u32);
}

pub struct DataObjectProp<'a>
{
    diagCodeType:Option<DiagCodedType<'a>>,
    physical_type:&'a PhysicalType<'a>,
    diag_coded_type:&'a DiagCodedType<'a>,
    compute_method:&'a dyn ComputeMethod,
    ident:Identity<'a>,
    ref_id:Option<u32>,
}

pub struct Structure<'a>
{
    dataObjectProp:DataObjectProp<'a>,
    params:Vec<Box<Param>>,
    id:Identity<'a>,
    bytesize:u32,
    variant:&'a Variant<'a>
}

pub struct EndOfPDUField
{

}
pub struct MUX
{

}

pub struct StaticField
{

}

pub struct DynamicLengthField
{

}

pub struct DTCDOP <'a>
{
    ident:Identity<'a>,
    dataObjectProp:DataObjectProp<'a>,
}