use crate::data_instance::*;
use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;
use bitvec::vec::BitVec;
use roxmltree::Document;
use roxmltree::Node;
use crate::data_type::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::cell::RefCell;
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
#[derive(Default)]
pub struct ODXParser
{
    pub variants:HashMap<String, Arc<RefCell<Variant>>>,
    pub current_variant_name:String,
    odxfile:String,
    pub variant_service_instances:HashMap<String,Vec<DiagServiceInstance>>

}
impl<'b> ODXParser
{
    pub fn new()->ODXParser
    {
        return ODXParser{variants:HashMap::new(),odxfile:String::new(),current_variant_name:String::default(),..Default::default()}
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
            self.__create_instances();

            return true;
        },
        Err(e) =>false
         }
    }

    fn get_service_instance_by_name(&self,service_name:&str)->Option<&DiagServiceInstance>
    {
        for (_key,variant) in self.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            let serviceinstances  = self.variant_service_instances.get(var.id.short_name.as_str()).unwrap();
            let service_instance = serviceinstances.iter().find(|s|s.diag_service_name.as_str() == service_name);
            return service_instance;
        }
        panic!("there si no reqeust instance found for this servcie !")

    }

    fn get_service_request_instance(&self,reqeust_name:&str)->&ServiceMessageInstance
    {
        for (_key,variant) in self.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            let serviceinstances  = self.variant_service_instances.get(var.id.short_name.as_str()).unwrap();
            let service_instance = serviceinstances.iter().find(|s|s.request_instance.short_name.as_str() == reqeust_name);
            return service_instance.map(|s|&s.request_instance).unwrap();
        }
        panic!("there si no reqeust instance found for this servcie !")
    }

    fn get_service_pos_response_instance(&self,service_name:&str)->Option<&ServiceMessageInstance>
    {
        for (_key,variant) in self.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            let serviceinstances  = self.variant_service_instances.get(var.id.short_name.as_str()).unwrap();
            let service_instance = serviceinstances.iter().find(|s|s.positive_response_instance.as_ref().unwrap().short_name.as_str() == service_name);
            let pos_response = service_instance.map(|s|&s.positive_response_instance);
            return pos_response.unwrap().as_ref();
        }
        panic!("there si no reqeust instance found for this servcie !")
    }

    fn get_service_negative_response_instance(&self,service_name:&str)->Option<&ServiceMessageInstance>
    {
        for (_key,variant) in self.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            let serviceinstances  = self.variant_service_instances.get(var.id.short_name.as_str()).unwrap();
            let service_instance = serviceinstances.iter().find(|s|s.negative_response_instance.as_ref().unwrap().short_name.as_str() == service_name);
            let neg_response = service_instance.map(|s|&s.negative_response_instance);
            return neg_response.unwrap().as_ref();
        }
        panic!("there si no reqeust instance found for this servcie !")
    }

    pub fn set_pending(&mut self,param:&str,pending_value:&BitVec)
    {
        let dot_index = param.find(".");
        match dot_index
        {
            Some(index)=>{
                let request_name = &param[0..index];
                let remainding = &param[index+1..];
                let request_instance = self.get_service_request_instance(request_name);
                request_instance.as_mut_struct().set_pending(remainding, pending_value);

            },
            _=>{
                let request_instance = self.get_service_request_instance(param);
                request_instance.as_mut_struct().set_pending("", pending_value);
            }
        }
    }

    fn __create_instances(&mut self)
    {
        for (key,variant) in self.variants.iter()
        {
            let var = &*variant.as_ref().borrow();
            
            let mut service_instances = Vec::<DiagServiceInstance>::new(); 
            for (k,v) in var.diag_comms.iter()
            { 
                let diag_service = var.diag_comms.get(k);
                let diagservice = &*diag_service.unwrap().as_ref().borrow();
                let request_ref :&str= diagservice.request_ref.as_ref();
                let mut serviceInstance = DiagServiceInstance{..Default::default()};
                serviceInstance.diag_service_name = diagservice.ident.short_name.clone();

                if let Some(p)=var.requests.get(request_ref)
                {   
                    if let ServiceMsgType::Request(p2) = &*p.as_ref().borrow_mut()
                    {
                        let mut request_instance = ServiceMessageInstance{..Default::default()};
                       

                    for param in p2.params.iter()
                    {   
                        let param_instance = param.create_data_instance(Some(variant.clone()));
                        request_instance.as_mut_struct().children_instances.push(param_instance);
                        
                        
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
                            let mut response_instance = ServiceMessageInstance{short_name:p2.ident.short_name.clone(),long_name:p2.ident.long_name.clone(),id:p2.ident.id.clone(),..Default::default()};
                            
                            
    
                        for param in p2.params.iter()
                        {
                            let param_instance = param.create_data_instance(Some(variant.clone()));
                            response_instance.as_mut_struct().children_instances.push(param_instance);
                            
                        }
                        serviceInstance.positive_response_instance = Some(response_instance);
    
                        }
                        
    
                    }
                }
                if diagservice.neg_response_ref.is_some()
                {
                    if let Some(p)= var.neg_responses.get(diagservice.neg_response_ref.as_ref().unwrap())
                    {   
                        if let ServiceMsgType::NegativeReponse(p2) = &*p.as_ref().borrow()
                        {
                            let mut neg_response_instance = ServiceMessageInstance{short_name:p2.ident.short_name.clone(),long_name:p2.ident.long_name.clone(),id:p2.ident.id.clone(),..Default::default()};
    
                        for param in p2.params.iter()
                        {
                            let param_instance = param.create_data_instance(Some(variant.clone()));
                            neg_response_instance.as_mut_struct().children_instances.push(param_instance);
                            
                        }
                        serviceInstance.negative_response_instance = Some(neg_response_instance);
    
                        }
                        
    
                    }
                }
                service_instances.push(serviceInstance);  
            }
            self.variant_service_instances.insert(var.id.short_name.clone(), service_instances); 
            

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
            id:ele.attribute("ID").unwrap().to_string(),
            ..Default::default()

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
    
    pub fn __get_unit(&mut self,parentunit:&Node)->Unit
    {
      let ident = self.__get_ident(parentunit);
      let displayname = self.__get_descendantText(parentunit, "DISPLAY-NAME").unwrap_or("");
      Unit { ident: ident,display_name:String::from(displayname) }

    }

    /// get diag coded type from the parent node, return the option 
    pub fn __get_diag_coded_type(&mut self,diagcodetypeParent:&Node)->Option<Arc<RefCell<DiagCodedType>>>
    {

        let result = match diagcodetypeParent.descendants().find(|n|n.tag_name().name() == "DIAG-CODED-TYPE")
        {
            Some(diagcodetype)=>Some({
                let aatype = diagcodetype.attribute("AA:type");
                let basedatatype = diagcodetype.attribute("BASE-DATA-TYPE");
                let ishighlow = diagcodetype.attribute("IS-HIGHLOW-BYTE-ORDER").map(|ishighlow|ishighlow=="false");
                let bitlength = self.__get_descendantText(&diagcodetype,"BIT-LENGTH").map(|bitlength|bitlength.parse::<u32>().unwrap());
                
                Arc::new(RefCell::new(DiagCodedType {
                   aa_type:aatype.map(|n|String::from(n)),
                   base_type:basedatatype.map(|n|String::from(n)),
                   ishighbyteorder:ishighlow,
                   bit_length:bitlength  
                }))
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


    pub fn __get_linear(&mut self,linearnode:&Node)->Linear
    {
        let scalnode = linearnode.descendants().find(|n|n.tag_name().name() == "COMPU-SCALE").unwrap();
        let lowlimit = linearnode.descendants().find(|n|n.tag_name().name() == "LOWER-LIMIT");
        let hightlimit = linearnode.descendants().find(|n|n.tag_name().name() == "UPPER-LIMIT");

        Linear{}

    }
    pub fn __get_scale_linear(&mut self,computeMethodParent:&Node)->ScaleLinear
    {

        ScaleLinear{}
    }
    pub fn __get_textable(&mut self,computeMethodParent:&Node)->Textable
    {
        Textable{}


    }

    pub fn __get_compute_method(&mut self,computeMethodParent:&Node)->Option<Box<dyn ComputeMethod>>
    {
        let result = match computeMethodParent.descendants().find(|n|n.tag_name().name() == "COMPU-METHOD")
        {
            Some(computenode)=>{
                let category = self.__get_descendantText(&computenode, "CATEGORY").unwrap();
                let cm:Box<dyn ComputeMethod>;
                if (category == "LINEAR")
                {
                    cm = Box::new(self.__get_linear(&computenode));

                }
                else if (category == "SCALE-LINEAR")
                {

                    cm = Box::new(self.__get_scale_linear(&computenode));
                    
                }
                else if (category == "TEXTTABLE")
                {
                    cm = Box::new(self.__get_textable(&computenode));
                    
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
        
        //println!("{:p}",&dop);
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
        let mut id_refs = Vec::<String>::new();
        let mut envdatas = Vec::<EnvData>::new();
        for n in  node.descendants()
        {
           if n.tag_name().name() == "ENV-DATA-REF"
           {
                let id_ref = n.attribute("ID-REF").map(|s|String::from(s)).unwrap();
                id_refs.push(id_ref);
           }
           if n.tag_name().name() == "ENV-DATA"
           {
            let envdata = self.__get_env_data(&n);
            envdatas.push(envdata);
           }
          
        }

        EnvDataDesc{ident:identitity,env_data_refs:id_refs,env_datas:envdatas,..Default::default()}

    }





    pub fn __get_env_data(& mut self,node:&Node)->EnvData
    {
        let envdata_ident = self.__get_ident(&node);
        let byte_size  = self.__get_descendantText(&node, "BYTE-SIZE");
        let mut params = Vec::new();
        for paramnode in  node.descendants()
        {
            if paramnode.tag_name().name() == "PARAM"
            {
            let mut param = self.__get_param(&paramnode);
            param.variant_id = self.current_variant_name.clone();
            params.push(param);
            }
        }
        EnvData { ident: envdata_ident, params: params }

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
        let mut reversed;
        if dop_ref.is_none()
        {
            reversed = Some(Arc::new(RefCell::new(Reversed{})));
        }
        else
        {
            reversed = Option::None;
        }
        
    
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
                        diag_coded_type:(self.__get_diag_coded_type(node)),
                        variant:Option::None,
                        reversed:reversed}
       
    }

    pub fn __get_struct (& mut self,node:&Node)->Structure
    {
        let identitity = self.__get_ident(node);
        let bytesize = self.__get_descendantText(node, "BYTE-SIZE");
        let mut struct_obj =  Structure{
            ident:identitity,
            bytesize:bytesize.map(|s|s.parse::<u32>().unwrap()),
            params:Vec::new(),
            

        };
        for ele in node.descendants()
        {
           if ele.tag_name().name() == "PARAM"
           {
            let mut param = self.__get_param(&ele);
            param.variant_id = self.current_variant_name.clone();
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

    pub fn __get_diag_service(&mut self,node:&Node)->DiagSerivce
    {
        let ident = self.__get_ident(node);
        let semantic = node.attribute("SEMANTIC").map(|s|String::from(s));
        let request_ref = node.children().find(|n|n.tag_name().name() == "REQUEST-REF").map(|node|String::from(node.attribute("ID-REF").unwrap())).unwrap();
        let positive_resp_ref = node.children().find(|n|n.tag_name().name() == "POS-RESPONSE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let negative_resp_ref = node.children().find(|n|n.tag_name().name() == "NEG-RESPONSE-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
        let func_class_ref = node.children().find(|n|n.tag_name().name() == "FUNCT-CLASS-REF").map(|node|String::from(node.attribute("ID-REF").unwrap()));
       
        DiagSerivce{
            ident:ident,
            semantic:semantic,
            request_ref:request_ref,
            pos_response_ref:positive_resp_ref,
            neg_response_ref:negative_resp_ref,
            func_class_ref:func_class_ref,
            ..Default::default()
        }
    }

    pub fn __get_serive_msg(&mut self,node:&Node)->SeviceMsgPayload
    {
        let ident = self.__get_ident(node);

        let mut smp=  SeviceMsgPayload{ident:ident,params:Vec::new()};
        let mut params = Vec::new();
        for ele in node.descendants()
        {
           if ele.tag_name().name() == "PARAM"
           {
            let mut param = self.__get_param(&ele);
            param.variant_id = self.current_variant_name.clone();
            params.push(param);

           }
        }
        smp.params = params;
        return smp;

    }


    pub fn __get_comparm_ref(&mut self,node:&Node)->ComParam
    {
        let id_ref = node.attribute("ID-REF").map(|s|String::from(s)).unwrap();
        let doc_ref = node.attribute("DOCREF").map(|s|String::from(s));
        let doc_type = node.attribute("DOCTYPE").map(|s|String::from(s));
        let mut value = self.__get_descendantText(node, "VALUE").map(|s|String::from(s));
        if value == None
        {
            value = self.__get_descendantText(node, "SIMPLE-VALUE").map(|s|String::from(s));
        }
        ComParam { ref_id: id_ref, doc_type: doc_type, doc_ref:doc_ref,value: value }

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
                let mut variant = Variant::default();
                self.current_variant_name = ident.short_name.clone();
                variant.id = ident;
                let variant_id = variant.id.id.clone();
                let arc_variant = Arc::new(RefCell::new(variant));
                
                for desdentnode in ele.descendants()
                {
                    //println!("{}",desdentnode.tag_name().name());
                    if desdentnode.tag_name().name() == "FUNCT-CLASS"
                    {
                        let funclass = self.__get_func_class(&desdentnode);
                        arc_variant.as_ref().borrow_mut().func_classes.insert(variant_id.clone(), Arc::new(RefCell::new(funclass)));
                    }

                    else if  desdentnode.tag_name().name() == "DTC-DOP"
                    {
                        let dtcdop = self.__get_dtc_dop(&desdentnode);
                        arc_variant.as_ref().borrow_mut().dtc_object_props.insert(dtcdop.ident.id.clone(), Arc::new(RefCell::new(dtcdop)));
                    }
                    else if  desdentnode.tag_name().name() == "DATA-OBJECT-PROP"
                    {
                        let dataprop = self.__get_data_prop(&desdentnode);
                        //println!("{:p}",&dataprop);
                       
                        arc_variant.as_ref().borrow_mut().data_object_props.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "ENV-DATA-DESC"
                    {
                        let dataprop = self.__get_env_data_desc(&desdentnode);
                        arc_variant.as_ref().borrow_mut().env_data_descs.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "STRUCTURE"
                    {
                        let mut dataprop = self.__get_struct(&desdentnode);
                        dataprop.ident.variant = Some(arc_variant.clone());
                        arc_variant.as_ref().borrow_mut().structures.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "STATIC-FIELD"
                    {
                        let mut dataprop = self.__get_static_field(&desdentnode);
                        dataprop.variant_id = variant_id.clone();
                        arc_variant.as_ref().borrow_mut().static_fileds.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "DYNAMIC_LENGTH_FIELD"
                    {
                        let mut dataprop = self.__get_dynamic_field(&desdentnode);
                        dataprop.ident.variant = Some(arc_variant.clone());
                        arc_variant.as_ref().borrow_mut().dynamic_fileds.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "END-OF-PDU-FIELD"
                    {
                        let mut dataprop = self.__get_endofpdu_field(&desdentnode);
                        dataprop.ident.variant = Some(arc_variant.clone());
                        arc_variant.as_ref().borrow_mut().endofpdu_fileds.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "UNIT"
                    {
                        let mut dataprop = self.__get_unit(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().units.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }

                    else if  desdentnode.tag_name().name() == "DIAG-SERVICE"
                    {
                        let mut dataprop = self.__get_diag_service(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().diag_comms.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "REQUEST"
                    {
                        let mut dataprop = self.__get_serive_msg(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().requests.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(ServiceMsgType::Request(dataprop))));
                    }
                    else if  desdentnode.tag_name().name() == "POS-RESPONSE"
                    {
                        let mut dataprop = self.__get_serive_msg(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().pos_responses.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(ServiceMsgType::Request(dataprop))));
                    }
                    else if  desdentnode.tag_name().name() == "NEG-RESPONSE"
                    {
                        let mut dataprop = self.__get_serive_msg(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().neg_responses.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(ServiceMsgType::Request(dataprop))));
                    }
                    else if  desdentnode.tag_name().name() == "COMPARAM-REF"
                    {
                        let mut dataprop = self.__get_comparm_ref(&desdentnode);
                        
                        arc_variant.as_ref().borrow_mut().comparam_refs.insert(dataprop.ref_id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                    else if  desdentnode.tag_name().name() == "MUX"
                    {
                        let mut dataprop = self.__get_mux(&desdentnode);
                        dataprop.ident.variant = Some(arc_variant.clone());
                        arc_variant.as_ref().borrow_mut().muxs.insert(dataprop.ident.id.clone(), Arc::new(RefCell::new(dataprop)));
                    }
                }

                
                self.variants.insert(variant_id.clone(), arc_variant.clone());
        
                
            }
        }

    }
}
