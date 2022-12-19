extern crate bv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cell::RefCell;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod parser;
pub mod data_instance;
pub mod data_type;
use data_instance::*;


lazy_static! {
    pub static ref MAP:HashMap<u32,&'static str> ={
        let mut m = HashMap::new();
        m.insert(0,"foo");
        m
    };
}





#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::data_type::{Structure, ServiceMsgType};

    use super::*;

    #[test]
    fn parsing()
    {
        let odxpath = r"D:\Workspace\RustApp\odxparser\src\CN180S_V1.0.80.odx";
        //let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        parser.parse(odxpath);
        for (key,variant) in parser.variants.iter()
        {
         
            for (k,v) in variant.diag_comms.iter()
            {
               
                let request = variant.as_ref().requests.get(k);

                let mut serviceInstance = DiagServiceInstance{..Default::default()};

                if let ServiceMsgType::Request(p)= request.unwrap().as_ref()
                {   let mut request_instance = ServiceMessageInstance{..Default::default()};
                    for param in p.params.iter()
                    {
                        let param_instance = param.create_data_instance();
                        request_instance.param_instances.push(param_instance);
                        
                    }
                    serviceInstance.request_instance = request_instance;

                }
                else if let ServiceMsgType::PositiveResponse(p)= request.unwrap().as_ref()  {
                    let mut positive_instance = ServiceMessageInstance{..Default::default()};
                    for param in p.params.iter()
                    {
                        let param_instance = param.create_data_instance();
                        positive_instance.param_instances.push(param_instance);
                        
                    }
                    serviceInstance.positive_response_instance = positive_instance;

                }
                else if let ServiceMsgType::NegativeReponse(p)= request.unwrap().as_ref()  {
                    let mut negative_instance = ServiceMessageInstance{..Default::default()};
                    for param in p.params.iter()
                    {
                        let param_instance = param.create_data_instance();
                        negative_instance.param_instances.push(param_instance);
                        
                    }
                    serviceInstance.positive_response_instance = negative_instance;

                }
              
            }
         
         
         
         
         
            for (k,v) in variant.dtc_object_props.iter()
         {
             println!("{}", v.dtcs.len());
             for dtc in &v.dtcs
             {
                 println!("      {}",dtc.display_trouble_code);
 
             }
         }
         for (k,v) in variant.structures.iter()
         {
             
             println!("{}",v.ident.short_name);
             for param in v.params.iter()
             {
                 println!("       {}",param.shortname);
             }
         }
 
         for (k,v) in variant.data_object_props.iter()
         {
             let u = v.unit_ref.as_ref();
             match u{
                 Some(u)=>println!("{}",u),
                 _=>{}
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
            child_instance.set_parent(Rc::new(parent));

            let currnetparent = child_instance.get_parent();
            
            //child_instance.set_parent(&parent);        
        
        println!("11");
    
    
    }

    

}
