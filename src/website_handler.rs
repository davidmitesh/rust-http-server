use super::server::Handler;
use super::http::{Request,Response,StatusCode,Method};
use std::fs;
pub struct WebsiteHandler{
    public_path:String
}

//All the routing logic to specific request based on the path will be implemented here

impl WebsiteHandler{
    pub fn new(public_path:String) -> Self{
        Self{
            public_path
        }
    }

    fn read_file(&self,file_path : &str) -> Option<String>{

        //To prevent the directory traversal vulnerability, we will calculate the absolute canonical path and check whether that starts with public or not

        let path = format!("{}/{}",self.public_path,file_path);
        match fs::canonicalize(path){
            Ok(path) => {
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                }else {
                    println!("Directory Traversal Attack Attempted : {}",file_path);
                    None
                }
            }
            Err(_) => None
        }
        //ok() is very handly method for conversion between Result<T,E> to Option<>.
        // fs::read_to_string(path).ok() 
    }
}


impl Handler for WebsiteHandler{
    fn handle_request(&mut self,request:&Request) -> Response {
         match request.method(){
            Method::GET => match request.path(){
                "/" => Response::new(StatusCode::Ok,self.read_file("index.html")),
                path => match self.read_file(path){ //This is the terrible security vulnerability, directory traversal vulnerability
                    Some(contents) => Response::new(StatusCode::Ok,Some(contents)),
                    None => Response::new(StatusCode::NotFound,None)

                },
                
            } ,
            _ => Response::new(StatusCode::NotFound,None)
         }
    }
}