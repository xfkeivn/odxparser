use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc};
use crate::data_instance::*;
use std::cell::{RefCell, Ref};
pub trait DataType{
    fn is_high_low_byte_order(&self)->bool
    {return false;}

}


#[derive(Debug)]
#[derive(Default)]
pub struct Identity
{
    pub short_name:String,
    pub long_name:Option<String>,
    pub variant:Option<Arc<RefCell<Variant>>>,
    pub id:String
}



pub trait ComputeMethod {
    fn get_physical_value(&self,rawvalue:&[u8])->f64
    {
        return 0.0;
    }
}

#[derive(Default,Debug)]
pub struct  Variant
{
    pub id:Identity,
    pub dtc_object_props:HashMap<String,Arc<RefCell<DTCDOP>>>,
    pub data_object_props:HashMap<String,Arc<RefCell<DataObjectProp>>>,
    pub env_data_descs:HashMap<String,Arc<RefCell<EnvDataDesc>>>,
    pub structures:HashMap<String,Arc<RefCell<Structure>>>,
    pub static_fileds:HashMap<String,Arc<RefCell<StaticField>>>,
    pub dynamic_fileds:HashMap<String,Arc<RefCell<DynamicLengthField>>>,
    pub endofpdu_fileds:HashMap<String,Arc<RefCell<EndOfPDUField>>>,
    pub units:HashMap<String,Arc<RefCell<Unit>>>,
    pub diag_comms:HashMap<String,Arc<RefCell<DiagSerivce>>>,
    pub requests:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub pos_responses:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub neg_responses:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub comparam_refs:HashMap<String,Arc<RefCell<ComParam>>>,     
    pub func_classes:HashMap<String,Arc<RefCell<FunctionClass>>>,
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
#[derive(Default,Debug)]
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
    pub diag_coded_type:Option<Arc<RefCell<DiagCodedType>>>,
    pub variant:Option<Arc<RefCell<Variant>>>
}

impl Param {
    
    pub fn create_data_instance(& self,variant:Option<Arc<RefCell<Variant>>>)->Arc<RefCell<dyn TDataInstance>>
    {
        let name = &self.shortname;
        let byte_position = self.byte_position.unwrap();
        let bit_position = self.bit_position.unwrap_or(0);
        if self.dop_ref.is_none()
        {
            if let Some(p) = &self.diag_coded_type
            {

                return DiagCodedType::create_instance(p.clone(), name, byte_position, bit_position)
            }
            else {
                panic!("The ref is None and it is not diag coded type is not invalid mode");
            }
        }

        else if let Some(p) = self.variant.as_ref().unwrap().borrow().data_object_props.get(self.dop_ref.as_ref().unwrap())
        {
           DataObjectProp::create_instance(p.clone(), name, byte_position, bit_position)
           
        }
        else if let Some(p) =self.variant.as_ref().unwrap().borrow().structures.get(self.dop_ref.as_ref().unwrap())
        {
            Structure::create_instance(p.clone(), name, byte_position, bit_position)
            
        }
        else if let Some(p) =self.variant.as_ref().unwrap().borrow().static_fileds.get(self.dop_ref.as_ref().unwrap())
        {
            StaticField::create_instance(p.clone(), name, byte_position, bit_position)
            
        }
      
        else if let Some(p) =self.variant.as_ref().unwrap().borrow().env_data_descs.get(self.dop_ref.as_ref().unwrap())
        {
            EnvDataDesc::create_instance(p.clone(), name, bit_position, bit_position)
      
        }
        else if let Some(p) =self.variant.as_ref().unwrap().borrow().dynamic_fileds.get(self.dop_ref.as_ref().unwrap())
        {
            DynamicLengthField::create_instance(p.clone(), name, bit_position, bit_position)
        }
        else {
            panic!("")
        }
       

    }
    
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
#[derive(Debug,Default)]
pub struct DiagCodedType
{
    pub aa_type:Option<String>,
    pub base_type:Option<String>,
    pub bit_length:Option<u32>,
    pub ishighbyteorder:Option<bool>
}

impl DataType for DiagCodedType
{
}
impl DiagCodedType {
    fn create_instance(datatype:Arc<RefCell<DiagCodedType>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<CodedDataDataInstance>>
    {
        let di =  CodedDataDataInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
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

impl Debug for dyn ComputeMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ComputeMethod");
        Result::Ok(())
    }
}


#[derive(Default,Debug)]
pub struct DataObjectProp
{
    pub physical_type:Option<PhysicalType>,
    pub diag_coded_type:Option<Arc<RefCell<DiagCodedType>>>,
    pub ident:Identity,
    pub compute_method:Option<Box<dyn ComputeMethod>>,
    pub unit_ref:Option<String>,
  
}

impl DataObjectProp {
    fn get(self)->Self 
    {return self}
    
}

impl DataType for DataObjectProp
{

}

impl DataObjectProp {
    fn create_instance(datatype:Arc<RefCell<DataObjectProp>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<DataObjectPropDataInstance>>
    {
        let di =  DataObjectPropDataInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}



#[derive(Default,Debug)]
pub struct Structure
{

    pub params:Vec<Box<Param>>,
    pub ident:Identity,
    pub bytesize:Option<u32>,
    
   
}

impl DataType for Structure {
   
}

impl Structure {
    fn create_instance(datatype:Arc<RefCell<Structure>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<StructureDataInstance>>
    {
        let mut di =  Arc::new(RefCell::new(StructureDataInstance{
            instance_core:DataInstanceCore{datatype:datatype.clone() ,..Default::default()},
            ..Default::default()
        }));

        for param in datatype.borrow_mut().params.iter()
        {   
            let mut child = param.create_data_instance(datatype.borrow_mut().ident.variant.clone());
            
            child.borrow_mut().set_parent(di.clone());
            di.borrow_mut().children_instances.push(child);
            
        }
        return di

    }
}



#[derive(Default,Debug)]
pub struct EnvDataDesc{
    pub ident:Identity,
    pub param_snref:Option<String>,
    pub env_data_refs:Vec<String>,
    pub env_datas:Vec<EnvData>
}

impl DataType for EnvDataDesc
{
}

impl EnvDataDesc {
    fn create_instance(datatype:Arc<RefCell<EnvDataDesc>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<EnvDataDescInstance>>
    {
        let di =  EnvDataDescInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}


#[derive(Default,Debug)]
pub struct EnvData
{
    pub ident:Identity,
    pub params:Vec<Param>

}
#[derive(Default,Debug)]
pub struct MuxCase
{
    pub shortname:String,
    pub ref_structure_id:Option<String>,
    pub switch_lower_lim:Option<u32>,
    pub switch_upper_lim:Option<u32>,
    pub is_default:bool
}
#[derive(Default,Debug)]
pub struct MuxSwitch
{
    pub byte_position:Option<u32>,
    pub ref_data_prop_id:Option<String>,
    pub bit_position:Option<u32>

}
#[derive(Default,Debug)]
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
#[derive(Default,Debug)]
pub struct StaticField
{
   pub   ident:Identity,
   pub   ref_struct_id:Option<String>,
   pub   size:Option<u32>,
   pub   item_size:Option<u32>,
   pub   variant_id:String

}

impl DataType for StaticField
{
}

impl StaticField {
    fn create_instance(datatype:Arc<RefCell<StaticField>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<StaticFieldInstance>>
    {
        let di =  StaticFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}

#[derive(Default,Debug)]
pub struct DynamicLengthField
{
    pub ident:Identity,
    pub ref_struct_id:Option<String>,
    pub variant_id:String,
    pub offset_of_first_basic_structure:Option<u32>,
    //length_determind_dop_refid:Option<String>
    pub byte_pos_length_determined_dop:Option<String>,
}

impl DynamicLengthField {
    fn create_instance(datatype:Arc<RefCell<DynamicLengthField>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<DynamicLengthFieldInstance>>
    {
        let di =  DynamicLengthFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}

#[derive(Default,Debug)]
pub struct DTCDOP
{
    pub ident:Identity,
    pub dataObjectProp:DataObjectProp,
    pub dtcs:Vec<Box<DTC>>
}
#[derive(Default,Debug)]
pub struct SeviceMsgPayload
{
    pub ident:Identity,
    pub params:Vec<Param>
}

#[derive(Debug)]
pub enum ServiceMsgType {
    Request(SeviceMsgPayload),
    PositiveResponse(SeviceMsgPayload),
    NegativeReponse(SeviceMsgPayload)
}

#[derive(Default,Debug)]
pub struct DiagSerivce
{
    pub ident:Identity,
    pub semantic:Option<String>,
    pub request_ref:String,
    pub pos_response_ref:Option<String>,
    pub neg_response_ref:Option<String>,
    pub func_class_ref:Option<String>    
}


