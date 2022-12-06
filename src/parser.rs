use std::collections::HashMap;
use roxmltree::Document;

use crate::data_instance;
use crate::data_type::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub struct DiagService<'a>
{
    id:Identity<'a>,
    semantic:&'a str,
    request_ref:Option<u32>,
    positive_response_ref:Option<u32>,
    negative_response_ref:Option<u32>,
    func_class_ref:Option<u32>,
    parserContext:&'a ODXParser<'a>
}

pub struct ODXParser<'a>
{
    variants:HashMap<&'a str, &'a Variant<'a>>,
    odxfile:String,

}
impl<'a> ODXParser<'a>
{
    pub fn new()->ODXParser<'a>
    {
        return ODXParser{variants:HashMap::new(),odxfile:String::new()}
    }
    pub fn parse(&mut self,odxfile:&'a str)->bool
    {
        self.variants.clear();
        self.odxfile = odxfile.to_string();
        let mut f = File::open(&self.odxfile).unwrap();
        let mut s = String::new();
        match f.read_to_string(&mut s) {
        Ok(_) => {
            let doc = roxmltree::Document::parse(&s).unwrap();
            self.__parseDocument(&doc);
            
            return true;
        },
        Err(e) =>false
         }
    }
    pub fn __parseDocument(&mut self,doc:&Document)
    {
        let rootElem = doc.descendants().find(|n| n.tag_name().name() == "ODX").unwrap();
        for ele in rootElem.descendants()
        {
            if ele.tag_name().name() == "BASE-VARIANT"
            {
                
            }
        }

    }
}
pub struct ServiceMsg<'a>
{
    id:Identity<'a>,
    params:Vec<Box<Param>>
}

pub struct PosResponse<'a>
{
    msg:ServiceMsg<'a>,
}
pub struct NegResponse<'a>
{
    msg:ServiceMsg<'a>,
}
pub struct Request<'a>
{
    msg:ServiceMsg<'a>,
}

pub struct DiagSerivce<'a>
{
    id:Identity<'a>,
    
}