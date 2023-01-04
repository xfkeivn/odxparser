extern crate bv;
pub mod parser;
pub mod data_instance;
pub mod data_type;

static mut COUNTER:u32 = 100;
#[cfg(test)]
mod tests {
    use std::{rc::Rc, borrow::Borrow};

    use crate::{data_type::{Structure, ServiceMsgType}, data_instance::{DiagServiceInstance, ServiceMessageInstance, StructureDataInstance, DataInstanceCore, StructInstance, AsCoreInstance}};

    use super::*;

    #[test]
    fn test_odx_parser()
    {
        let odxpath = r"D:\Workspace\RustApp\odxparser\odxparserlib\src\CN180S_V1.0.80.odx";
        
        //let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\odxparserlib\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        let result = parser.parse(odxpath);
        assert_eq!(result,true);

        let variant = parser.variants.values().nth(0).unwrap().as_ref().borrow();
        let structure = variant.structures.get("_186").unwrap().as_ref().borrow();
        assert_eq!(structure.params.len(),8);
        let param= &structure.params.as_slice()[5];
        assert_eq!(param.as_ref().shortname,"CfgByte5_STRUCTURE");
        assert_eq!(param.as_ref().byte_position.unwrap(),5);
        let dop_ref = param.as_ref().dop_ref.as_ref();


    }


    #[test]
    fn test_end_of_pdu() {
        let odxpath = r"D:\Workspace\RustApp\odxparser\odxparserlib\src\CN180S_V1.0.80.odx";
        
        //let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\odxparserlib\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        let result = parser.parse(odxpath);
        assert_eq!(result,true);

        let variant = parser.variants.values().nth(0).unwrap().as_ref().borrow();
        let service_instances = parser.variant_service_instances.get(variant.id.short_name.as_str()).unwrap();
       
        let service_instance = service_instances.iter().find(|s|s.positive_response_instance.as_ref().unwrap().id.as_str() == "_441" ).unwrap();
        //let pos_service_instance = service_instance.positive_response_instance.as_ref().unwrap();
        parser.set_pending("RQ_FaultMemory_Read_identified_errors.DtcStatusbyte_STRUCTURE", vec![0b10101010]);
        let request_instance = &service_instance.request_instance;
        for child_instance in request_instance.as_struct().children_instances.iter()
        {
            let instance = child_instance.as_ref().borrow();
        }
    
    }

    

}
