extern crate bv;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod parser;
pub mod data_instance;
pub mod data_type;
use data_instance::*;
#[cfg(test)]
mod tests {
    use crate::data_type::Structure;

    use super::*;


    fn parsing()
    {
        let odxpath = r"D:\Workspace\RustApp\odxparser\src\CN180S_V1.0.80.odx";
        //let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\src\CN180S_V1.0.80.odx";
        let mut parser = parser::ODXParser::new();
        parser.parse(odxpath);
        for (key,value) in parser.variants.iter()
        {
         for (k,v) in value.dtc_object_props.iter()
         {
             println!("{}", v.dtcs.len());
             for dtc in &v.dtcs
             {
                 println!("      {}",dtc.display_trouble_code);
 
             }
         }
         for (k,v) in value.structures.iter()
         {
             
             println!("{}",v.ident.short_name);
             for param in v.params.iter()
             {
                 println!("       {}",param.shortname);
             }
         }
 
         for (k,v) in value.data_object_props.iter()
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
        //let parent = DataInstanceCore::default();
        let struct_instance:StructureDataInstance=StructureDataInstance{
            instance_core:DataInstanceCore{
                name:String::from("TestInstance"),
                ..Default::default()
            },
            ..Default::default()};
        let name = struct_instance.get_full_name();
        println!("{}",name);

    }

}
