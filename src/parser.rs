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

pub struct DiagService
{
    id:Identity,
    semantic:String,
    request_ref:Option<u32>,
    positive_response_ref:Option<u32>,
    negative_response_ref:Option<u32>,
    func_class_ref:Option<u32>,
    parserContext:ODXParser
}

pub struct ODXParser
{
    pub variants:HashMap<String, Box<Variant>>,
    odxfile:String,

}
impl<'b> ODXParser
{
    pub fn new()->ODXParser
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


    pub fn __get_descendantText<'a>(&mut self,parentnode:&Node<'a,'_>,textnodename:&str)->Option<&'a str>
    {
        let descText = match parentnode.descendants().find(|n| n.tag_name().name() == textnodename)
        {
            Some(node)=>node.text(),
            None=>None
        };
        return descText;
    }

    pub fn  __get_ident<'a>(&mut self,ele:&Node)->Identity
    {
        let shortname = match ele.children().find(|n|n.tag_name().name() == "SHORT-NAME")
        {
            Some(node)=>node.text().unwrap(),
            _=>""
            
        };
        let longname = ele.children().find(|n|n.tag_name().name() == "LONG-NAME").map(|n|n.text()).unwrap();
        
        let ident = Identity
        {
            short_name:shortname.to_string(),
            long_name:longname.map(|s|String::from(s)),
            id:ele.attribute("ID").unwrap().to_string()
        };
        return ident;
    }

    pub fn __get_func_class(&mut self,funcele:&Node)->FunctionClass
    {

        let identitity = self.__get_ident(funcele);

        let funclass = FunctionClass{
            ident:identitity,
            description:String::new()
        };
        return funclass

    }

    pub fn __get_unit_ref(&mut self,parentunit:&Node)->Option<String>
    {
        let result = match parentunit.descendants().find(|n|n.tag_name().name() == "UNIT-REF")
        {
            Some(diagcodetype)=>Some({
                let idref = diagcodetype.attribute("ID-REF").unwrap();
                String::from(idref)
            }),
            _=>None
        };
        return result;

    }
    

    /// get diag coded type from the parent node, return the option 
    pub fn __get_diag_coded_type(&mut self,diagcodetypeParent:&Node)->Option<DiagCodedType>
    {

        let result = match diagcodetypeParent.descendants().find(|n|n.tag_name().name() == "DIAG-CODED-TYPE")
        {
            Some(diagcodetype)=>Some({
                let aatype = diagcodetype.attribute("AA:type");
                let basedatatype = diagcodetype.attribute("BASE-DATA-TYPE");
                let ishighlow = diagcodetype.attribute("IS-HIGHLOW-BYTE-ORDER").map(|ishighlow|ishighlow=="false");
                let bitlength = self.__get_descendantText(&diagcodetype,"BIT-LENGTH").map(|bitlength|bitlength.parse::<u32>().unwrap());
                DiagCodedType {
                   aa_type:aatype.map(|n|String::from(n)),
                   base_type:basedatatype.map(|n|String::from(n)),
                   ishighbyteorder:ishighlow,
                   bit_length:bitlength  
                }
            }),
            _=>None
        };
        return result;
        
    }

    /// get diag coded type from the parent node, return the option 
    pub fn __get_physcial_type(&mut self,physicalTypeParent:&Node)->Option<PhysicalType>
    {

        let result = match physicalTypeParent.descendants().find(|n|n.tag_name().name() == "PHYSICAL-TYPE")
        {
            Some(physicaltypenode)=>Some({
                let base_data_type = physicaltypenode.attribute("DISPLAY-RADIX");
                let display_radix = physicaltypenode.attribute("BASE-DATA-TYPE");
                PhysicalType {
                   base_data_type:base_data_type.map(|n|String::from(n)),
                   display_radix:display_radix.map(|n|String::from(n)),
                
                }
            }),
            _=>None
        };
        return result;
        
    }

    pub fn __get_compute_method(&mut self,computeMethodParent:&Node)->Option<Box<dyn ComputeMethod>>
    {
        let result = match computeMethodParent.descendants().find(|n|n.tag_name().name() == "COMPU-METHOD")
        {
            Some(physicaltypenode)=>{
                let category = self.__get_descendantText(&physicaltypenode, "CATEGORY").unwrap();
                let cm:Box<dyn ComputeMethod>;
                if (category == "LINEAR")
                {
                    cm = Box::new(Linear{});

                }
                else if (category == "SCALE-LINEAR")
                {
                    cm = Box::new(ScaleLinear{});
                    
                }
                else if (category == "TEXTTABLE")
                {
                    cm = Box::new(Textable{});
                    
                }
                else {
                    cm = Box::new(Identical{});
                }
              Some(cm)
               
            },
            _=>None
        };
        return result;

    }


    pub fn __get_data_prop(&mut self,datapropnode:&Node)->DataObjectProp
    {
        let identitity = self.__get_ident(datapropnode);

        let diagcodetype = self.__get_diag_coded_type(datapropnode);

        let physicaltype = self.__get_physcial_type(datapropnode);
       
        let computemethod:Option<Box<dyn ComputeMethod>> = self.__get_compute_method(datapropnode);

        let unit_ref = self.__get_unit_ref(datapropnode);
        DataObjectProp{
            ident :identitity,
            physical_type:physicaltype,
            diag_coded_type:diagcodetype,
            compute_method:computemethod,
            unit_ref:unit_ref
        }
    }

    pub fn __get_dtc(&mut self,dtcnode:&Node)->DTC
    {
        let Identity = self.__get_ident(dtcnode);
        let troublecode = self.__get_descendantText(dtcnode, "TROUBLE-CODE").unwrap().parse::<u64>().unwrap();
        let displaytroublecode = self.__get_descendantText(dtcnode, "DISPLAY-TROUBLE-CODE").unwrap();
        DTC{ident:Identity,trouble_code:troublecode,display_trouble_code:String::from(displaytroublecode),text:String::from("")}

    }

    pub fn __get_dtc_dop(&mut self,dtcdopNode:&Node)->DTCDOP
    {
        let identitity = self.__get_ident(dtcdopNode);
        let dataprop = self.__get_data_prop(dtcdopNode);
        let mut dtcdop = DTCDOP{
            ident:identitity,
            dataObjectProp:dataprop,
            dtcs:Vec::new()
           
        };
        for ele in dtcdopNode.descendants()
        {
            if ele.tag_name().name() == "DTC"
            {

                let dtc = self.__get_dtc(&ele);
                dtcdop.dtcs.push(Box::new(dtc));

            
            }

        }
        return dtcdop;

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
                let mut variant = Box::new(Variant{
                    id:ident,
                    func_classes:HashMap::new(),
                    dtc_object_props:HashMap::new(),
                    data_object_props:HashMap::new()
                });
                for desdentnode in ele.descendants()
                {
                    //println!("{}",desdentnode.tag_name().name());
                    if desdentnode.tag_name().name() == "FUNCT-CLASS"
                    {
                        let funclass = self.__get_func_class(&desdentnode);
                        variant.func_classes.insert(variant.id.id.clone(), Box::new(funclass));
                    }

                    else if  desdentnode.tag_name().name() == "DTC-DOP"
                    {
                        let dtcdop = self.__get_dtc_dop(&desdentnode);
                        variant.dtc_object_props.insert(dtcdop.ident.id.clone(), Box::new(dtcdop));
                    }
                    else if  desdentnode.tag_name().name() == "DATA-OBJECT-PROP"
                    {
                        let dataprop = self.__get_data_prop(&desdentnode);
                        variant.data_object_props.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }

                }
                
        
                self.variants.insert(variant.id.id.clone(), variant);
            }
        }

    }
}
pub struct ServiceMsg
{
    id:Identity,
    params:Vec<Box<Param>>
}

pub struct PosResponse
{
    msg:ServiceMsg,
}
pub struct NegResponse
{
    msg:ServiceMsg,
}
pub struct Request
{
    msg:ServiceMsg,
}

pub struct DiagSerivce
{
    id:Identity,
    
}