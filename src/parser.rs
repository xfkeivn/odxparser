use std::collections::HashMap;
use roxmltree::Document;
use roxmltree::Node;

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

pub struct ODXParser
{
    variants:HashMap<&'b str, Box<Variant<'b>>>,
    odxfile:String,

}
impl<'b> ODXParser<'b,'c>
{
    pub fn new()->ODXParser<'b>
    {
        return ODXParser{variants:HashMap::new(),odxfile:String::new()}
    }
    pub fn parse(&mut self,odxfile:&str)->bool
    {
        self.variants.clear();
        self.odxfile = odxfile.to_string();
        let mut f = File::open(&self.odxfile).unwrap();
        let mut s = String::new();
        let doc ;
        match f.read_to_string(&mut s) {
        Ok(_) => {
            

           doc = roxmltree::Document::parse(&s).unwrap();
            self.__parseDocument(&doc);
            
            return true;
        },
        Err(e) =>false
         }
    }

    pub fn  __get_ident<'a>(&mut self,ele:&Node<'a,'_>)->Identity<'a>
    {
        let shortname = match ele.children().find(|n|n.tag_name().name() == "SHORT-NAME")
        {
            Some(node)=>node.text().unwrap(),
            _=>""
            
        };
        let longname = match ele.children().find(|n|n.tag_name().name() == "LONG-NAME")
        {
            Some(node)=>node.text().unwrap(),
            _=>""
            
        };

        let ident = Identity
        {
            short_name:shortname,
            long_name:longname,
            id:ele.attribute("ID").unwrap()
        };
        return ident;
    }

    pub fn __parseDocument<'c>(&mut self,doc:&'c Document)->()
    {
        let rootElem = doc.descendants().find(|n| n.tag_name().name() == "ODX").unwrap();
        for ele in rootElem.descendants()
        {
            let name = ele.tag_name().name();
            if name == "BASE-VARIANT"
            {
                let ident = self.__get_ident(&ele);
               
                let variant = Box::new(Variant{id:ident});
                self.variants.insert(ident.id, variant);
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