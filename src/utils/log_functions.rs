use mailparse::{parse_mail,parse_content_disposition,MailHeaderMap,parse_content_type,};
use std::io::Write;
use std::fs::File; 
use mailparse::{ParsedMail,dateparse};

pub fn create_attachment_files(parsed:&ParsedMail){
    for i in 1..=parsed.subparts.len()-1{
        let filename:Vec<String>=parsed.subparts[i].get_headers().get_all_values("Content-Type");
    let single_header=&filename[0].to_string();
    eprintln!("Attachment File Name:{}",&single_header);
    let ctype:String=parse_content_type(&single_header).params.get("name").unwrap().to_string();
   let attached_file=parsed.subparts[i].get_body_raw().unwrap();
    let mut out_file = File::create(&ctype).unwrap();
    out_file.write_all(&attached_file).unwrap();}
 
     }
 pub fn  log_message_body(parsed:&ParsedMail){
   let content_vec=parsed.subparts[0].get_headers().get_all_values("Content-Type");
   let content=&content_vec[0].to_string();
   eprintln!("Message_Content_Type: {}",content);
    eprintln!("Message: { }",&parsed.subparts[0].get_body().unwrap());
 }
 pub fn log_date(parsed:&ParsedMail){
    eprintln!("{}",dateparse(&parsed.headers.get_first_value("Date").unwrap().as_str()).unwrap());
 }
 pub fn log_others(parsed:&ParsedMail){
    eprintln!("Content-Transfer-Encoding:{}",&parsed.subparts[1].headers[1].get_value());
    eprintln!("{}",&parsed.subparts[1].ctype.mimetype);
    
 }    