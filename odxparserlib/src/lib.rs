extern crate bv;
pub mod parser;
pub mod data_instance;
pub mod data_type;
use std::sync::Arc;
use std::cell::RefCell;



#[cfg(test)]
mod tests {
    use std::{rc::Rc, borrow::Borrow};

    use crate::{data_type::{Structure, ServiceMsgType}, data_instance::{DiagServiceInstance, ServiceMessageInstance, StructureDataInstance, DataInstanceCore}};

    use super::*;

    #[test]
    fn parsing()
    {
        let odxpath = r"D:\Workspace\RustApp\odxparser\odxparserlib\src\CN180S_V1.0.80.odx";
        
        //let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\odxparserlib\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        parser.parse(odxpath);
        for (key,variant) in parser.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            for (k,v) in var.diag_comms.iter()
            { 
                let diag_service = var.diag_comms.get(k);
                let diagservice = &*diag_service.unwrap().as_ref().borrow();
                let request_ref :&str= diagservice.request_ref.as_ref();
                let mut serviceInstance = DiagServiceInstance{..Default::default()};

                if let Some(p)=var.requests.get(request_ref)
                {   
                    if let ServiceMsgType::Request(p2) = &*p.as_ref().borrow_mut()
                    {
                        let mut request_instance = ServiceMessageInstance{..Default::default()};

                    for param in p2.params.iter()
                    {   
                        let param_instance = param.create_data_instance(Some(variant.clone()));
                        request_instance.param_instances.push(param_instance);
                        
                    }
                    serviceInstance.request_instance = request_instance;

                    }
                }
                if diagservice.pos_response_ref.is_some()
                {
                    if let Some(p)= var.pos_responses.get(diagservice.pos_response_ref.as_ref().unwrap())
                    {   
                        if let ServiceMsgType::PositiveResponse(p2) =  &*p.as_ref().borrow()
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
                if diagservice.neg_response_ref.is_some()
                {
                    if let Some(p)= var.neg_responses.get(diagservice.neg_response_ref.as_ref().unwrap())
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
       
    
    
    }

    

}
