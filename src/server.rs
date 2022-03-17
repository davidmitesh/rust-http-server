
//every file in rust is treated as a module. so no need to define it explicitly
use std::io::{Read};
use std::net::{TcpListener};
use crate::http::{Request,Response,StatusCode,ParseError};
use std::convert::TryFrom;

pub trait Handler{
    fn handle_request(&mut self,request:&Request) -> Response;

    fn handle_bad_request(&mut self,e:&ParseError) -> Response{
        println!("Failed to parse the request: {}",e);
        Response::new(StatusCode::BadRequest,None)
    }
}

// mod server{
    pub struct Server {
        addr : String
    }   
    impl Server{
        pub fn new(addr : String) -> Self {
            Server{
                addr
            }
        }
    
        pub fn run(&self,mut handler : impl Handler){//if this method is called without &, this function takes the ownership of the passed Server instance and the Server instance will be deallocated when this call is finished.
            //To avoid that we use the reference by adding &self.
            println!("Listening on {}",self.addr);
            let listener = TcpListener::bind(&self.addr).unwrap();//here we just want to pass the reference to the addr not move the addr to this function in entirely,like passing ownership, so use &

            //the infinite loop in rust can be achieved by using the loop keyword
            // 'outer:loop{
            //     loop{
            //         break 'outer;
            //     }
            // }

            // loop{
            //     let res = listener.accept();
            //     if res.is_err(){
            //         continue;
            //     }

            //     let (stream,address) = res.unwrap();

            // } 

            //We can rewrite the above more efficiently using the match expression

            loop{
                match listener.accept(){
                    Ok((mut stream,_)) => { // _ is used when we dont care about the parameter, we can also use Ok(_) => {}
                        let mut buffer:[u8;1024] = [0; 1024]; //allocating 1024 bytes
                        match stream.read(&mut buffer){
                            Ok(_) => {
                                println!("Received a request : {}",String::from_utf8_lossy(&buffer));
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        // dbg!(request);
                                        // Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string()))
                                        handler.handle_request(&request)
                                    },
                                    Err(e) => {
                                        // println!("Failed to parse the request : {}",e);
                                        // Response::new(StatusCode::BadRequest, None)
                                        handler.handle_bad_request(&e)
                                    }
                                }; 

                                if let Err(e) = response.send(&mut stream){
                                    println!("Failed to send the response: {}",e);
                                }
                                // let res:&Result<Request,_> = &buffer[..].try_into(); //This is the another alternative to the conversion function
                            },
                            Err(e) => println!("Failed to read from connection : {}",e)
                        } 
                    }
                    Err(e) => println!("Failed to establish the connection: {}",e)
                }
            }
        }
    }
// }