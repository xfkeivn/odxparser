use std::collections::HashMap;
use std::process::id;
use std::rc::Weak;
use std::rc::Rc;
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
    pub variants:HashMap<String, Rc<Variant>>,
    odxfile:String

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
        let longname = ele.children().find(|n|n.tag_name().name() == "LONG-NAME").map(|n|n.text().unwrap());
        
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
        let dop = DataObjectProp{
            ident :identitity,
            physical_type:physicaltype,
            diag_coded_type:diagcodetype,
            compute_method:computemethod,
            unit_ref:unit_ref
        };
        
        println!("{:p}",&dop);
        return dop
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

    pub fn __get_env_data_desc(& mut self,node:&Node)->EnvDataDesc
    {
        let identitity = self.__get_ident(node);
        
        EnvDataDesc{ident:identitity}

    }

    pub fn __get_param(& mut self,node:&Node)->Param
    {
        let longname = match node.children().find(|n|n.tag_name().name() == "LONG-NAME")
        {
            Some(node)=>Some(node.text().unwrap()),
            _=>None
            
        };
        let shortname = match node.children().find(|n|n.tag_name().name() == "SHORT-NAME")
        {
            Some(node)=>node.text().unwrap(),
            _=>""
        };

        let bytepos = self.__get_descendantText(node, "BYTE-POSITION").map(|s|s.parse::<u32>().unwrap());
        let bitpos = self.__get_descendantText(node, "BIT-POSITION").map(|s|s.parse::<u32>().unwrap());
        let bitlen = self.__get_descendantText(node, "BIT-LENGTH").map(|s|s.parse::<u32>().unwrap());
        let dop_ref = node.children().find(|n|n.tag_name().name() == "DOP-REF").map(|node|node.attribute("ID-REF").unwrap());
        let phys_constant_value = self.__get_descendantText(node, "PHYS-CONSTANT-VALUE");
        let aatype = node.attribute("AA:type").map(|s|String::from(s));
        let sematic = node.attribute("SEMANTIC").map(|s|String::from(s));
        let mut codevalues = Vec::<u32>::new();
        for n in  node.descendants()
        {
           if n.tag_name().name() == "CODED-VALUE"
           {
            let codevalue = n.text().unwrap().parse::<u32>().unwrap();
            codevalues.push(codevalue);
           }

        };
    
        return Param { 
                        aa_type:aatype,
                        sematic:sematic,
                        variant_id:String::new(),
                        shortname: String::from(shortname), 
                        longname:  longname.map(|s|String::from(s)), 
                        byte_position:bytepos,
                        bit_position:bitpos,
                        bit_length:bitlen,
                        codedvalues:codevalues,
                        dop_ref:dop_ref.map(|n|String::from(n)),
                        physical_constant_value:phys_constant_value.map(|s|s.parse::<u32>().unwrap()),
                        diag_coded_type:self.__get_diag_coded_type(node)}
       
    }

    pub fn __get_struct (& mut self,node:&Node)->Structure
    {
        let identitity = self.__get_ident(node);
        let bytesize = self.__get_descendantText(node, "BYTE-SIZE");
        let mut struct_obj =  Structure{
            ident:identitity,
            bytesize:bytesize.map(|s|s.parse::<u32>().unwrap()),
            params:Vec::new(),
            variantId:String::new()

        };
        for ele in node.descendants()
        {
           if ele.tag_name().name() == "PARAM"
           {
            let param = self.__get_param(&ele);
            struct_obj.params.push(Box::new(param));

           }
        }

        return struct_obj;
    }

    pub fn __get_static_field(&mut self,node:&Node)->StaticField
    {

        let identitity = self.__get_ident(node);
        let numberofitem = self.__get_descendantText(node, "FIXED-NUMBER-OF-ITEMS").map(|s|s.parse::<u32>().unwrap());
        let structref = node.children().find(|n|n.tag_name().name() == "BASIC-STRUCTURE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let itemsize = self.__get_descendantText(node, "ITEM-BYTE-SIZE").map(|s|s.parse::<u32>().unwrap());

        StaticField{
            ident:identitity,
            ref_struct_id:structref,
            size:numberofitem,
            item_size:itemsize,
            variant_id:String::new()
        }
    }

    pub fn __get_dynamic_field(&mut self,node:&Node)->DynamicLengthField
    {

        let identitity = self.__get_ident(node);
        let numberofitem = self.__get_descendantText(node, "FIXED-NUMBER-OF-ITEMS").map(|s|s.parse::<u32>().unwrap());
        let offset_of_first_basicstructure = self.__get_descendantText(node, "OFFSET").map(|s|s.parse::<u32>().unwrap());
        let dopref = node.children().find(|n|n.tag_name().name() == "DATA-OBJECT-PROP-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let structref = node.children().find(|n|n.tag_name().name() == "BASIC-STRUCTURE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let itemsize = self.__get_descendantText(node, "ITEM-BYTE-SIZE").map(|s|s.parse::<u32>().unwrap());

        DynamicLengthField{
            ident:identitity,
            ref_struct_id:structref,
            byte_pos_length_determined_dop:dopref,
            offset_of_first_basic_structure:offset_of_first_basicstructure,
            //length_determind_dop_refid:dopref,
            variant_id:String::new()
        }
    }

    pub fn __get_endofpdu_field(&mut self,node:&Node)->EndOfPDUField
    {

        let identitity = self.__get_ident(node);
        let structref = node.children().find(|n|n.tag_name().name() == "BASIC-STRUCTURE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let maxitem = self.__get_descendantText(node, "MAX_NUMBER_OF_ITEMS").map(|s|s.parse::<u32>().unwrap());
        let minitem = self.__get_descendantText(node, "MIN_NUMBER_OF_ITEMS").map(|s|s.parse::<u32>().unwrap());

        EndOfPDUField{
            ident:identitity,
            max_item_number:maxitem,
            min_item_number:minitem,
            basic_struct_ref:structref,
            //length_determind_dop_refid:dopref,
            variant_id:String::new()
        }
    }

    pub fn __get_mux_case(&mut self,node:&Node)->MuxCase
    {
        let shortname = self.__get_descendantText(node, "SHORT-NAME").unwrap();
        let lowlimit = self.__get_descendantText(node, "LOWER-LIMIT").map(|s|s.parse::<u32>().unwrap());
        let hilimit = self.__get_descendantText(node, "UPPER-LIMIT").map(|s|s.parse::<u32>().unwrap());
        let structref = node.children().find(|n|n.tag_name().name() == "STRUCTURE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        MuxCase { shortname: String::from(shortname), ref_structure_id:structref, switch_lower_lim: lowlimit, switch_upper_lim: hilimit, is_default: false }
    }

    pub fn __get_mux(&mut self,node:&Node)->Mux
    {

        let identitity = self.__get_ident(node);
        let case_start_byte_offset = self.__get_descendantText(node, "BYTE-POSITION").map(|s|s.parse::<u32>().unwrap());
        let switchKeyNode = node.children().find(|n|n.tag_name().name() == "SWITCH-KEY").unwrap();

        let  ms = self.__get_mux_key(&switchKeyNode);
        let default_casenode = node.descendants().find(|n| n.tag_name().name() == "DEFAULT-CASE");
        let mut cases = Vec::<MuxCase>::new();
        let default_case = match default_casenode
        {
            Some(node)=>{
                
                let mut case =   self.__get_mux_case(&node);
                case.is_default = true;
                Some(case)
            },
            _=>None
        };
        

        for desdentnode in node.descendants()
        {

            if desdentnode.tag_name().name() == "CASE"
            {
                let case = self.__get_mux_case(&desdentnode);
                cases.push(case);
            
                

            }
        }

        Mux{
            ident:identitity,
            switch_key:ms,
            cases:cases,
            default_case:default_case,
            case_start_byte_offset:case_start_byte_offset,
            //length_determind_dop_refid:dopref,
            variant_id:String::new()
        }
    }
    pub fn __get_mux_key(&mut self,node:&Node)->MuxSwitch
    {

        let dopref = node.children().find(|n|n.tag_name().name() == "DATA-OBJECT-PROP-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let bytePos = self.__get_descendantText(node, "BYTE-POSITION").map(|s|s.parse::<u32>().unwrap());
        let bitPos = self.__get_descendantText(node, "BIT-POSITION").map(|s|s.parse::<u32>().unwrap());


        MuxSwitch{
          
            byte_position:bytePos,
            bit_position:bitPos,
            ref_data_prop_id:dopref
        }
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
                let mut variant = Variant{
                    id:ident,
                    func_classes:HashMap::new(),
                    dtc_object_props:HashMap::new(),
                    data_object_props:HashMap::new(),
                    env_data_descs:HashMap::new(),
                    structures:HashMap::new(),
                    static_fileds:HashMap::new(),
                    dynamic_fileds:HashMap::new(),
                    endofpdu_fileds:HashMap::new(),

                };
                
                
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
                        println!("{:p}",&dataprop);
                       
                        variant.data_object_props.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    else if  desdentnode.tag_name().name() == "ENV-DATA-DESC"
                    {
                        let dataprop = self.__get_env_data_desc(&desdentnode);
                        variant.env_data_descs.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    else if  desdentnode.tag_name().name() == "STRUCTURE"
                    {
                        let mut dataprop = self.__get_struct(&desdentnode);
                        dataprop.variantId = variant.id.id.clone();
                        variant.structures.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    else if  desdentnode.tag_name().name() == "STATIC-FIELD"
                    {
                        let mut dataprop = self.__get_static_field(&desdentnode);
                        dataprop.variant_id = variant.id.id.clone();
                        variant.static_fileds.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    else if  desdentnode.tag_name().name() == "DYNAMIC_LENGTH_FIELD"
                    {
                        let mut dataprop = self.__get_dynamic_field(&desdentnode);
                        dataprop.variant_id = variant.id.id.clone();
                        variant.dynamic_fileds.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    else if  desdentnode.tag_name().name() == "END-OF-PDU-FIELD"
                    {
                        let mut dataprop = self.__get_endofpdu_field(&desdentnode);
                        dataprop.variant_id = variant.id.id.clone();
                        variant.endofpdu_fileds.insert(dataprop.ident.id.clone(), Box::new(dataprop));
                    }
                    

                }
                self.variants.insert(variant.id.id.clone(), Rc::new(variant));
        
                
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