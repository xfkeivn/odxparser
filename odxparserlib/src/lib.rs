extern crate bv;
pub mod parser;
pub mod data_instance;
pub mod data_type;

static mut COUNTER:u32 = 100;
#[cfg(test)]
mod tests {
    use std::{rc::Rc, borrow::Borrow, cell::RefCell};
    use crate::{data_type::{Structure, ServiceMsgType}, data_instance::{DiagServiceInstance, ServiceMessageInstance, StructureDataInstance, DataInstanceCore, StructInstance,TDataInstance}};
    use bitvec::{prelude::*, view::BitView};
    use super::*;
    use data_instance::*;
    use std::sync::Arc;
    

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
    fn test_structure_fixed_sized() {
        let odxpath = r"D:\Workspace\RustApp\odxparser\odxparserlib\src\CN180S_V1.0.80.odx";
        
       // let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\odxparserlib\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        parser.parse(odxpath); 
        //let pos_service_instance = service_instance.positive_response_instance.as_ref().unwrap();
        let data  = 0x55u8;
        let pending_value = data.view_bits::<Lsb0>();
        println!("{}",pending_value);
        let bv = BitVecU8::from(pending_value);
        println!("{:?}",bv.as_raw_slice());
        parser.set_pending("RQ_FaultMemory_Read_identified_errors.DtcStatusbyte_STRUCTURE",&BitVecU8::from_bitslice(pending_value) );
        
        let mut bit1 = BitVecU8::new();
        bit1.push(true);
        //parser.set_pending("RQ_FaultMemory_Read_identified_errors.DtcStatusbyte_STRUCTURE.TestFailedSinceLastClear",&bit1);
        let service_instance = parser.get_diag_service_instance("CN180S_PEPS", "FaultMemory_Read_identified_errors");
        let request_instance = &service_instance.request_instance;
        
        // test the expected value 
        let expected = BitVecU8::from_vec(vec![25u8,0x2,0x55u8]);
        assert_eq!(request_instance.as_ref().borrow().get_pending().unwrap(),expected);

        for instance in request_instance.as_ref().borrow().as_struct().children_instances.iter()
        {
            print_child_instance(instance);
        }

        let pos_instance = &service_instance.positive_response_instance;
        let response_data = [89u8,2,0xff,0xff,0xff,0xff,0xff];
        parser.update_positive_response("FaultMemory_Read_identified_errors", response_data.to_vec());
        //
        

    
    }

    fn print_child_instance(instance:&Arc<RefCell<dyn TDataInstance>>)
    {
        
        let pending_data = &instance.as_ref().borrow().get_pending();
        println!("{},Bit_Length={},value={:#?}",instance.as_ref().borrow().get_full_name(),pending_data.as_ref().unwrap().len(),pending_data);
        if let Some(p) =  instance.as_ref().borrow().get_children()
        {
           for child in  p.iter()
           {
                print_child_instance(child);
           }
        }

    }

    

}
