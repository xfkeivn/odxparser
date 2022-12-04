
#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Identity<'a>
{
    short_name:&'a str,
    long_name:&'a str,
    id:u64
}

pub trait COMPU_METHOD {
    fn get_physical_value(rawvalue:&[u8])->f64;
}

pub struct ScaleLinear
{
    scale_coefficient_arr:[u32;4]
}


impl COMPU_METHOD for ScaleLinear
{
    fn get_physical_value(rawvalue:&[u8])->f64
    {
        return 0.0;
    }
}

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