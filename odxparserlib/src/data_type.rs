use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc};
use crate::data_instance::*;
use std::cell::{RefCell, Ref};

pub trait DataType{
    type InstanceType;
    fn create_instance(datatype:Arc<RefCell<Self>>,name:&str,byte_postion:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<Self::InstanceType>>;
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
    pub muxs:HashMap<String,Arc<RefCell<Mux>>>,
    pub units:HashMap<String,Arc<RefCell<Unit>>>,
    pub diag_comms:HashMap<String,Arc<RefCell<DiagSerivce>>>,
    pub requests:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub pos_responses:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub neg_responses:HashMap<String,Arc<RefCell<ServiceMsgType>>>,
    pub comparam_refs:HashMap<String,Arc<RefCell<ComParam>>>,     
    pub func_classes:HashMap<String,Arc<RefCell<FunctionClass>>>,
}


impl Variant
{
   pub fn get_data_type_by_ref_id(&self,ref_id:&str)->Option<DataTypeEnum>
   {
    
    if let Some(p) = self.data_object_props.get(ref_id)
    {
       return Some(DataTypeEnum::DATA_OBJECT_PROP(p.clone()));
       
    }
    else if let Some(p) =self.structures.get(ref_id)
    {
        return  Some(DataTypeEnum::STRUCTURE(p.clone()));
        
    }
    else if let Some(p) =self.static_fileds.get(ref_id)
    {
        return  Some(DataTypeEnum::STATIC_FIELD(p.clone()));   
    }
    else if let Some(p) =self.env_data_descs.get(ref_id)
    {
       
        return Some(DataTypeEnum::ENV_DATA_DESC(p.clone()));
    }
    else if let Some(p) =self.dynamic_fileds.get(ref_id)
    {
        return  Some(DataTypeEnum::DYNAMIC_FIELD(p.clone()));
    }
    else if let Some(p) =self.endofpdu_fileds.get(ref_id)
    {
        return Some(DataTypeEnum::END_OF_PDU_FIELD(p.clone()));
    }
    else if let Some(p) =self.muxs.get(ref_id)
    {
        return Some(DataTypeEnum::MUX(p.clone()));
    }
    else{
        return Option::None;
    }
    
   }
}

pub enum DataTypeEnum
{
    DTC_OBJECT_PROP(Arc<RefCell<DTCDOP>>),
    DATA_OBJECT_PROP(Arc<RefCell<DataObjectProp>>),
    ENV_DATA_DESC(Arc<RefCell<EnvDataDesc>>),
    STRUCTURE(Arc<RefCell<Structure>>),
    STATIC_FIELD(Arc<RefCell<StaticField>>),
    DYNAMIC_FIELD(Arc<RefCell<DynamicLengthField>>),
    END_OF_PDU_FIELD(Arc<RefCell<EndOfPDUField>>),
    MUX(Arc<RefCell<Mux>>)
    
}

impl DataTypeEnum
{
    pub fn create_data_instance(&self,name:&str,byte_postion:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<dyn TDataInstance>>
    {
        match self
        {
            DataTypeEnum::STRUCTURE(p)=>{
            let instance = Structure::create_instance(p.clone(), name, byte_postion, bit_position,bit_length);
            return instance;

            }
            DataTypeEnum::DATA_OBJECT_PROP(p)=>{
                let instance = DataObjectProp::create_instance(p.clone(), name, byte_postion, bit_position,bit_length);
                return instance;
            }
            DataTypeEnum::STATIC_FIELD(p)=>{
                let instance = StaticField::create_instance(p.clone(), name, byte_postion, bit_position,bit_length);
                return instance;
            }
            DataTypeEnum::END_OF_PDU_FIELD(p)=>{
                let instance = EndOfPDUField::create_instance(p.clone(), name, byte_postion, bit_position,bit_length);
                return instance;

            }
            DataTypeEnum::DYNAMIC_FIELD(p)=>{
                let instance = DynamicLengthField::create_instance(p.clone(), name, byte_postion, bit_position,bit_length);
                return instance;

            }
            _=>{
                panic!("");
            }
        }
}
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
    pub reversed:Option<Arc<RefCell<Reversed>>>,
    pub variant:Option<Arc<RefCell<Variant>>>
}

impl Param {
    
    pub fn create_data_instance(& self,variant:Option<Arc<RefCell<Variant>>>)->Arc<RefCell<dyn TDataInstance>>
    {
        let name = &self.shortname;
        //for param there must be byte_position, to define the byte position, but the bit position is optional
        let byte_position = self.byte_position.unwrap();
        //if bit_position is none, it should be the first bit of the byte
        let bit_position = self.bit_position.unwrap_or_default();
        let variantref = variant.as_ref().unwrap().as_ref().borrow();

        if variant.is_none()
        {
            panic!("Variant is None");
        }

        if self.dop_ref.is_none()
        {
            if let Some(p) = &self.diag_coded_type
            {
                
                let mut coded_type_instance =  DiagCodedType::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length);
               
                coded_type_instance.as_ref().borrow_mut().coded_values = self.codedvalues.clone();
               
                return coded_type_instance
            }
            else {

                if let Some(p) = &self.reversed
                {
                    return Reversed::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
                }
                panic!("The ref is None and it is not diag coded type is not invalid mode");
            }
        }

        else if let Some(p) = variantref.data_object_props.get(self.dop_ref.as_ref().unwrap())
        {
           DataObjectProp::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
           
        }
        else if let Some(p) =variantref.structures.get(self.dop_ref.as_ref().unwrap())
        {
            Structure::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
            
        }
        else if let Some(p) =variantref.static_fileds.get(self.dop_ref.as_ref().unwrap())
        {
            StaticField::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
            
        }
      
        else if let Some(p) =variantref.env_data_descs.get(self.dop_ref.as_ref().unwrap())
        {
            EnvDataDesc::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
      
        }
        else if let Some(p) =variantref.dynamic_fileds.get(self.dop_ref.as_ref().unwrap())
        {
            DynamicLengthField::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
        }
        else if let Some(p) =variantref.endofpdu_fileds.get(self.dop_ref.as_ref().unwrap())
        {
            EndOfPDUField::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
        }
        else if let Some(p) =variantref.muxs.get(self.dop_ref.as_ref().unwrap())
        {
            Mux::create_instance(p.clone(), name, byte_position, bit_position,self.bit_length)
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
    type InstanceType = CodedDataDataInstance ;
    fn create_instance(datatype:Arc<RefCell<DiagCodedType>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<CodedDataDataInstance>>
    {
        let di =  CodedDataDataInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
            ..Default::default()
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
    //pub bit_length:Option<u32>,
    
  
}

impl DataObjectProp {
    fn get(self)->Self 
    {return self}
    
}

impl DataType for DataObjectProp
{
    type InstanceType = DataObjectPropDataInstance;
    fn create_instance(datatype:Arc<RefCell<DataObjectProp>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<DataObjectPropDataInstance>>
    {
        let di =  DataObjectPropDataInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
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
   
    type InstanceType = StructureDataInstance;
    fn create_instance(datatype:Arc<RefCell<Structure>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<StructureDataInstance>>
    {
        let dt = datatype.clone();
        let mut di =  Arc::new(RefCell::new(StructureDataInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
            ..Default::default()
        }));
        
        for param in dt.as_ref().borrow().params.iter()
        {   
            let mut child = param.create_data_instance(dt.as_ref().borrow().ident.variant.clone());
            
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
    type InstanceType = EnvDataDescInstance;
    fn create_instance(datatype:Arc<RefCell<EnvDataDesc>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<EnvDataDescInstance>>
    {
        let di =  EnvDataDescInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}

#[derive(Debug,Default)]
pub struct Reversed
{

}

impl DataType for Reversed {
    type InstanceType = ReversedInstance;
    fn create_instance(datatype:Arc<RefCell<Reversed>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<ReversedInstance>>
    {
        let di =  ReversedInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position:byte_position,bit_length:bit_length.unwrap(),..Default::default()},
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


impl  EndOfPDUField
{
    fn create_instance(datatype:Arc<RefCell<EndOfPDUField>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<EndOfPDUFieldInstance>>
    {
        let di =  EndOfPDUFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position:byte_position,..Default::default()},
            ..Default::default()
        };
        return Arc::new(RefCell::new(di));

    }




}

#[derive(Default,Debug)]
pub struct Mux
{
    pub ident:Identity,
    pub variant_id:String,
    pub cases:Vec<MuxCase>,
    pub default_case:Option<MuxCase>,
    pub switch_key:MuxSwitch,
    pub case_start_byte_offset:Option<u32>
}


impl Mux {
    fn create_instance(datatype:Arc<RefCell<Mux>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<MuxInstance>>
    {

        let mut di =  MuxInstance{
            instance_core:DataInstanceCore{datatype:datatype.clone() ,name:String::from(name),bit_position:bit_position,byte_position:byte_position,..Default::default()},
            ..Default::default()
        };
        let datatype_ref = datatype.as_ref().borrow();
        let variant_ref = datatype_ref.ident.variant.as_ref().unwrap().as_ref().borrow();
        for case_instance in datatype_ref.cases.iter()
        {
            let case_type_ref = case_instance.ref_structure_id.as_ref().unwrap().as_str();
           
            let case_data_type = variant_ref.get_data_type_by_ref_id(case_type_ref).unwrap();
            di.children_case_instances.push(case_data_type.create_data_instance(name, byte_position, bit_position,Option::None));
        }

        
        let switch_data_type = variant_ref.get_data_type_by_ref_id(datatype_ref.switch_key.ref_data_prop_id.as_ref().unwrap().as_str());
        let byte_position = datatype_ref.switch_key.byte_position.unwrap();
        let bit_position = datatype_ref.switch_key.bit_position.unwrap_or_default();
        let switch_datatype = switch_data_type.as_ref().unwrap();
        di.mux_switch_case_instance = Some(switch_datatype.create_data_instance("key",byte_position, bit_position,Option::None));
        
        
        let default_case_data_type = variant_ref.get_data_type_by_ref_id(datatype_ref.default_case.as_ref().unwrap().ref_structure_id.as_ref().unwrap()).unwrap();
        //di.default_case_instance = Some(default_case_data_type.create_data_instance("default_key",datatype_ref.default_case.as_ref().unwrap()., datatype_ref.switch_key.bit_position.unwrap()));
        return Arc::new(RefCell::new(di));

    }
    
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
    type InstanceType = StaticFieldInstance;
    fn create_instance(datatype:Arc<RefCell<StaticField>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<StaticFieldInstance>>
    {
        let datatype_ref = datatype.clone();
        let vv = (&datatype_ref).as_ref().borrow();
        let variant_ref = vv.ident.variant.as_ref().unwrap();
        let mut byte_position = byte_position;
        let mut di =  StaticFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
            ..Default::default()
        };
        
        for item in (0..datatype_ref.as_ref().borrow().size.unwrap())
        {
            let itemname = format!("{}[{}]",name,item);
            let ref_id = vv.ref_struct_id.as_ref();
            let itemdatatype = variant_ref.as_ref().borrow().get_data_type_by_ref_id(ref_id.unwrap().as_str());
            di.children_instances.push(itemdatatype.as_ref().unwrap().create_data_instance(itemname.as_str(), byte_position, 0,Option::None));
            byte_position+=vv.item_size.as_ref().unwrap();

        }
        return Arc::new(RefCell::new(di));

    }
}

#[derive(Default,Debug)]
pub struct DynamicLengthField
{
    pub ident:Identity,
    pub ref_struct_id:Option<String>,
    pub offset_of_first_basic_structure:Option<u32>,
    
    //length_determind_dop_refid:Option<String>
    pub byte_pos_length_determined_dop:Option<String>,
}

impl DynamicLengthField {
    fn create_instance(datatype:Arc<RefCell<DynamicLengthField>>,name:&str,byte_position:u32,bit_position:u32,bit_length:Option<u32>)->Arc<RefCell<DynamicLengthFieldInstance>>
    {
        let datatype_ref = datatype.clone();
        let vv = (&datatype_ref).as_ref().borrow();
        let ref_id = vv.ref_struct_id.as_ref();
        let variant_ref = vv.ident.variant.as_ref().unwrap();
        let mut di =  DynamicLengthFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,name:String::from(name),bit_position:bit_position,byte_position,..Default::default()},
            ..Default::default()
        };

        let length_instance = ListLengthInstance{
            instance_core:DataInstanceCore{datatype:Arc::new(RefCell::new(0)) ,name:String::from("Length"),..Default::default()}
        };
        let itemdatatype = variant_ref.as_ref().borrow().get_data_type_by_ref_id(ref_id.unwrap().as_str());
        let item_instance = itemdatatype.as_ref().unwrap().create_data_instance(format!("{}[i]",name).as_str(), 0, 0,Option::None);
        di.children_instances.push(Arc::new(RefCell::new(length_instance)));
        di.children_instances.push(item_instance);
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


