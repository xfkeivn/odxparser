
#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Identity<'a>
{
    short_name:&'a str,
    long_name:&'a str,
    id:u64
}

pub trait ComputeMethod {
    fn get_physical_value(rawvalue:&[u8])->f64
    {
        return 0.0;
    }
}

pub struct ScaleLinear;
pub struct Identical;
pub struct Textable;
pub struct Linear;

impl ComputeMethod for ScaleLinear{}
impl ComputeMethod for Identical{}
impl ComputeMethod for Textable{}
impl ComputeMethod for Linear{}

pub struct FunctionClass<'a>
{
    ident:Identity<'a>,
    description:&'a str
}

pub struct DTC <'a>
{
    ident:Identity<'a>,
    trouble_code:u64,
    display_trouble_code:&'a str,
    text:&'a str,
    ref_id:Option<u64>
}


pub struct DTCDOP <'a,T:COMPU_METHOD>
{
    ident:Identity<'a>,
    diag_coded_type:&'a str,
    physical_type:&'a str,
    compute_method:Option<T>
}