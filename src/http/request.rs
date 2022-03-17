// pub mod request {
    use super::method::{Method,MethodError};
    use super::query_string::{QueryString};
    use std::convert::TryFrom;
    use std::error::Error;
    use std::str;
    use std::fmt::{Display,Formatter,Result as FmtResult,Debug};
    use std::str::Utf8Error;

    #[derive(Debug)]
    pub struct Request<'buf>{
        path : &'buf str,
        query_string : Option<QueryString<'buf>>, //This overcomes the fear of null
        method :  Method //here by specifying super, we are referring to the parent module 
    }

    //Lifetimes doesnot allow us to choose how long a value will live but It allows us to communicate to the compiler that some references are "related" and are expected to share the same lifetime.

    impl<'buf> Request<'buf>{
        pub fn path(&self) -> &str{
            &self.path
        }

        pub fn method(&self) -> &Method {
            &self.method
        }

        pub fn query_string(&self) -> Option<&QueryString>{
            self.query_string.as_ref()
        }
    }
    
    impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
        type Error = ParseError;

        //A sample http request would look like this
        //GET /search?name=abc&sort=1 HTTP/1.1\r\n ... Headers

        fn try_from(buf : &'buf [u8]) -> Result<Self,Self::Error>{
            // match str::from_utf8(buf){
            //     Ok(request) => {},
            //     Err(_) => return Err(ParseError::InvalidEncoding)
            // }
            //the same thing above can also be achieved as:
            let request = str::from_utf8(buf)?;
            // match get_next_word(request){
            //     Some((method,request)) => {},
            //     None => return Err(ParseError::InvalidRequest)  
            // }
            //The above code can be compactly written as :
            let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;//here variable request is shadowed
            let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
            let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

            if protocol != "HTTP/1.1" {
                return Err(ParseError::InvalidProtocol);
            }
            let method:Method = method.parse()?;
            let mut query_string = None;


            // match path.find('?'){
            //     Some(i) =>{
            //         query_string = Some(&path[i+1..]);
            //         path = &path[..i];
            //     },
            //     None => {}
            // }

            // let q = path.find('?');
            // if q.is_some(){
            //     let i = q.unwrap();
            //     query_string = Some(&path[i+1..]);
            //     path = &path[..i];
            // }
             //There is also another method that rust provides in the case where we require only to evaluate on the some condition and ignore the none condition
            if let Some(i) = path.find('?'){
                query_string = Some(QueryString::from(&path[i+1..]));
                path = &path[..i];
            }

            Ok(Self{
                path ,
                query_string,
                method
            })   
        }
    }
// }

fn get_next_word(request:&str) -> Option<(&str,&str)>{
    for (i,c) in request.char_indices(){
        if c == ' ' || c == '\r' {
            return Some((&request[..i],&request[i+1..])); //this is potentially unsafe code because adding the 1 to the indices, will add 1 byte not the 1 charater to the count. But in this case, we know there is space at postition i. so this is safe.
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod 
}

impl ParseError{
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl Error for ParseError{

}


impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
         write!(f,"{}",self.message())
    }
}    

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
         write!(f,"{}",self.message())
    }
} 

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}



