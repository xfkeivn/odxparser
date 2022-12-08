extern crate bv;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod parser;
pub mod data_instance;
pub mod data_type;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
       let odxpath = r"D:\Workspace\RustApp\odxparser\src\CN180S_V1.0.80.odx";
       let mut parser = parser::ODXParser::new();
       parser.parse(odxpath);
       for (key,value) in parser.variants.iter()
       {
        for (k,v) in value.dtc_object_props.iter()
        {
            println!("{}", v.dtcs.len());
            for dtc in &v.dtcs
            {
                println!("{}",dtc.display_trouble_code);

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

}
