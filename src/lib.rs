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
       let odxpath = r"E:\WORKSPACE\RustApps\odx_parser\src\CN180S_V1.0.80.odx";
       let mut parser = parser::ODXParser::new();
       parser.parse(odxpath);

    }

}
