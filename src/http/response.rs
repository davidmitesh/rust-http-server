use super::StatusCode;


use std::io::{Write,Result as IoResult}; 
#[derive(Debug)]
pub struct Response{
    status_code : StatusCode,
    body : Option<String>
}


impl Response{
    pub fn new(status_code : StatusCode,body : Option<String>) -> Self{
        Response{
            status_code,
            body
        }
    }

    pub fn send(&self,stream : &mut impl Write) -> IoResult<()>{//dynamic and static dispatch in rust, we are using the static dispatch here, because it doesnot involve the looking for the v-table, and everthing is done at compile not adding overhead at the runtime
        let body = match &self.body {
            Some(b) => b,
            None => "" 
        };
        write!(stream,"HTTP/1.1 {} {}\r\n\r\n{}",self.status_code,self.status_code.reason_phrase(),body)
    }
}

