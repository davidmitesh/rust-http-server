use std::collections::HashMap;
pub struct QueryString<'buf>{
    data : HashMap<&'buf str,Value<'buf>>
}

//In rust, heap allocated dynamic array is called Vector.
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self,key:&str) -> Option<&Value>{
        self.data.get(key)
    }  
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s:&'buf str) -> Self {
        let mut data = HashMap :: new();
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find("="){
                key = &sub_str[]
            }
        }
        QueryString{
            data
        }
    }
}

