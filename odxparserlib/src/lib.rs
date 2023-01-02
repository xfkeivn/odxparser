extern crate bv;
pub mod parser;
pub mod data_instance;
pub mod data_type;

static mut COUNTER:u32 = 100;
#[cfg(test)]
mod tests {
    use std::{rc::Rc, borrow::Borrow};

    use crate::{data_type::{Structure, ServiceMsgType}, data_instance::{DiagServiceInstance, ServiceMessageInstance, StructureDataInstance, DataInstanceCore}};

    use super::*;

    #[test]
    fn test_unsafe()
    {
        let a = String::from("123");
        let a2 = &String::from("123");
        //stack address for string object, has len, cap and raw pointer to the p2-2
        let p2 = a2 as *const String;
        //heap address for the 123
        let p2_2 = a2.as_ptr();
        let p = a.as_ptr();
        
        unsafe
        { 
            //raw pointer dereference
            //static mut change
            //union
            //unsafe function / method 
            // unsafe trait
            
            let v = *p.offset(1);
            println!("{}",v);
            COUNTER = 200;
        }
        

    }

    #[test]
    fn test_odx_parser()
    {
        //let odxpath = r"D:\Workspace\RustApp\odxparser\odxparserlib\src\CN180S_V1.0.80.odx";
        
        let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\odxparserlib\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        let result = parser.parse(odxpath);
        assert_eq!(result,true);

    }


    #[test]
    fn test() {
       
    
    
    }

    

}
