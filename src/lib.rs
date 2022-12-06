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

    }

}
