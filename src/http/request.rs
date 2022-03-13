// pub mod request {
    use super::method::Method;
    use std::convert::TryFrom;
    pub struct Request{
        path : String,
        query_string : Option<String>, //This overcomes the fear of null
        method :  Method //here by specifying super, we are referring to the parent module 
    }

    impl Request{
        fn from_byte_array(buf:&[u8]) -> Result<Self,String>{

        }
    }

    impl TryFrom<> for Request{
        
    }
// }