#![allow(unused_imports)]
#[path = "utils/log_functions.rs"] mod log_functions;
extern crate  mailin;

 
 use log_functions::log_date;
use anyhow::Context;
use mailparse::{parse_mail,ParsedMail,MailHeaderMap,parse_content_type};
use mailin::Handler;
use anyhow::{Result,bail};
use std::{
    io::{self,},
    net::{SocketAddr,  TcpStream},
   
    time::Duration,
   
};
use std::io::Write; 
use async_io::{block_on, Async, Timer};
use std::fs::File;
use futures_lite::{io::BufReader, AsyncBufReadExt, FutureExt};
use std::string::String;



pub struct MailHandler<> {
    peer_addr: SocketAddr,
    //inbox: PathBuf,
    //data:Option<EmailWrite>,
    
    message:Vec<String>,
}
impl mailin::Handler for MailHandler{
    fn helo(&mut self, _ip: std::net::IpAddr, _domain: &str) -> mailin::Response {
         //maillog!(self.peer_addr, "HELO {} {}", ip, domain);
        mailin::response::OK
    }

    fn mail(&mut self, _ip: std::net::IpAddr, _domain: &str, from: &str) -> mailin::Response {
    println!("From:{}",from);
        mailin::response::OK
    }

    fn rcpt(&mut self, to: &str) -> mailin::Response {
        println!( "to {}", to);
        mailin::response::OK
    }
    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
           let con=String::from_utf8_lossy(buf);
           //dbg!(&con);
self.message.push(con.to_string());
            
        
       
         Ok(())
        
    }
    fn data_end(&mut self) -> mailin::Response {
        
   //dbg!(&self.message);
    let con:String=self.message.iter().map(String::as_str).collect();
    let parsed:ParsedMail=parse_mail(con.as_bytes()).unwrap();
    //dbg!("{}",&parsed);
    let header=parsed.get_headers();
   //dbg!(&header);
   log_functions::create_attachment_files(&parsed);
 log_functions::log_message_body(&parsed);
 log_functions::log_date(&parsed);
log_functions::log_others(&parsed);


    //dbg!(keyvec);

    
      
        mailin::response::OK
    }
}

 pub fn receive_updates_on_socket(mut stream: TcpStream) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    let remote_addr = stream.peer_addr().unwrap();
    let handler = MailHandler {
        peer_addr,
        //inbox: inbox.as_ref().to_path_buf(),
        //data:None,
        message:Vec::new(),
    };
      
    let mut session = mailin::SessionBuilder::new("localhost").build(remote_addr.ip(), handler);//change localhost value to actual domain address.
    session.greeting().write_to(&mut stream)?;

    let mut buf_read = BufReader::new(Async::new(stream.try_clone()?)?);
    let mut command = String::new();
 
    loop {
        command.clear();
        let len = block_on(buf_read.read_line(&mut command).or(async {
            Timer::after(Duration::from_secs(5 * 60)).await;
            Err(std::io::ErrorKind::TimedOut.into())
        }))?;
        let command = if len == 0 {
            break;
        } else {
            &command[..]
        };
        let result = session.process(command.as_bytes());
        match result.action {
            mailin::Action::Close => {
                //maillog!(peer_addr, "CLOSE");
                result.write_to(&mut stream)?;
                break;
            }
            mailin::Action::UpgradeTls => bail!("TLS requested"),
            mailin::Action::NoReply =>{  continue},
            mailin::Action::Reply => result.write_to(&mut stream).context(format!(
                "{}: Writing SMTP reply failed when responding to '{}' with '{:?}'",
                peer_addr, command, result
            ))?,
        }
    }
Ok(())
}




