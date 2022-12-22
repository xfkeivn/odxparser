extern crate bv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Arc;
use data_type::Variant;
use std::sync::Mutex;
use parser::ODXParser;


pub mod parser;
pub mod data_instance;
pub mod data_type;
use clap::Parser;
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


#[cfg(test)]
mod tests {
    use std::{rc::Rc, borrow::Borrow};

    use crate::data_type::{Structure, ServiceMsgType};

    use super::*;

    #[test]
    fn parsing()
    {
        //let odxpath = r"D:\Workspace\RustApp\odxparser\src\CN180S_V1.0.80.odx";
        let args = Cli::parse();
        let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        parser.parse(odxpath);
        for (key,variant) in parser.variants.iter()
        {
            let var = &*variant.as_ref().borrow();

            for (k,v) in var.diag_comms.iter()
            { 
                let diag_service = var.diag_comms.get(k);

                let mut serviceInstance = DiagServiceInstance{..Default::default()};

                if let Some(p)= var.requests.get(&diag_service.unwrap().as_ref().borrow().request_ref)
                {   
                    if let ServiceMsgType::Request(p2) = &*p.as_ref().borrow()
                    {
                        let mut request_instance = ServiceMessageInstance{..Default::default()};

                    for mut param in p2.params.iter()
                    {   
                        let param_instance = param.create_data_instance(Some(variant.clone()));
                        request_instance.param_instances.push(param_instance);
                        
                    }
                    serviceInstance.request_instance = request_instance;

                    }
                    

                }
                if diag_service.unwrap().as_ref().borrow().pos_response_ref.is_some()
                {
                    let key = &*diag_service.unwrap().as_ref().borrow();
                    if let Some(p)= var.pos_responses.get::<String>(key.pos_response_ref.as_ref().unwrap())
                    {   
                        if let ServiceMsgType::PositiveResponse(p2) = &*p.as_ref().borrow()
                        {
                            let mut response_instance = ServiceMessageInstance{..Default::default()};
    
                        for param in p2.params.iter()
                        {
                            let param_instance = param.create_data_instance(Some(variant.clone()));
                            response_instance.param_instances.push(param_instance);
                            
                        }
                        serviceInstance.positive_response_instance = response_instance;
    
                        }
                        
    
                    }
                }
                if diag_service.unwrap().as_ref().borrow().neg_response_ref.is_some()
                {
                    let key =&diag_service;
                    if let Some(p)= var.neg_responses.get("")
                    {   
                        if let ServiceMsgType::NegativeReponse(p2) = &*p.as_ref().borrow()
                        {
                            let mut neg_response_instance = ServiceMessageInstance{..Default::default()};
    
                        for param in p2.params.iter()
                        {
                            let param_instance = param.create_data_instance(Some(variant.clone()));
                            neg_response_instance.param_instances.push(param_instance);
                            
                        }
                        serviceInstance.negative_response_instance = neg_response_instance;
    
                        }
                        
    
                    }
                }
              
            }
         
         
         

        }

    }


    #[test]
    fn test() {
        #[derive(Default,Debug)]
        struct  A
        {
            a:u32
            
        }

        let a = A{..Default::default()};

        let b = &a;
        let c = &a;
        let h = c;

        println!("{:?}",a);
        println!("{:?}",a);
        println!("{:?}",c);
        println!("{:?}",b);

        let mut d = a;
        let e = &d;
        let f = &mut d;
        println!("{:?}",d);
        //f.a = 200;
        //d.a = 100;
       // 








        //let parent = DataInstanceCore::default();
        let parent_instance:StructureDataInstance=StructureDataInstance{
            instance_core:DataInstanceCore{
                ..Default::default()
            },
            ..Default::default()};
        let name = parent_instance.get_full_name();
        println!("{}",name);
    
    
    //Rc::new();
    //println!("{}",Rc::strong_count(&parent));
    //let parent2 = parent.clone();
    //println!("{}",Rc::strong_count(&parent2));

      let parent = RefCell::new(parent_instance);
       
      
      let mut child_instance:StructureDataInstance=StructureDataInstance{
        instance_core:DataInstanceCore{
            
            ..Default::default()
            },
            ..Default::default()};
            child_instance.set_parent(Arc::new(parent));

            let currnetparent = child_instance.get_parent();
            
            //child_instance.set_parent(&parent);        
        
        println!("11");
    
    
    }

    

}
