// use serde::*;
// use serde_derive::*;
// use serde_json;
// use serde_json::{Value};
// use reqwest;
// use failure::Error;
// use networking;

// //use jsonrpc_core::*;
// use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ASReport {
//     pub id : String, 
//     pub timestamp : String,
//     pub isvEnclaveQuoteStatus : String,
//     pub platformInfoBlob : String,
//     pub isvEnclaveQuoteBody : String
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct ASResult {
//     pub ca : String, 
//     pub certificate : String,
//     pub report : ASReport,
//     pub report_string : String,
//     pub signature : String, 
//     pub validate : bool
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct ASResponse {
//     pub id : i64,
//     pub jsonrpc : String, 
//     pub result : ASResult
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Params {
//     pub quote : String,
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct QuoteRequest {
//     pub jsonrpc : String, 
//     pub method : String, 
//     pub params : Params, 
//     pub id : i32, 
// }

// fn build_request() -> QuoteRequest{
//     QuoteRequest{
//         jsonrpc : "2.0".to_string(),
//         method : "validate".to_string(),
//         params : Params{
//             quote : "AgAAANoKAAAHAAYAAAAAABYB+Vw5ueowf+qruQGtw+54eaWW7MiyrIAooQw/uU3eBAT/////AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwAAAAAAAAAHAAAAAAAAALcVy53ugrfvYImaDi1ZW5RueQiEekyu/HmLIKYvg6OxAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACGcCDM4cgbYe6zQSwWQINFsDvd21kXGeteDakovCXPDwjJ31WG0K+wyDDRo8PFi293DtIr6DgNqS/guQSkglPJqAIAALbvs91Ugh9/yhBpAyGQPth+UWXRboGkTaZ3DY8U+Upkb2NWbLPkXbcMbB7c3SAfV4ip/kPswyq0OuTTiJijsUyOBOV3hVLIWM4f2wVXwxiVRXrfeFs/CGla6rGdRQpFzi4wWtrdKisVK5+Cyrt2y38Ialm0NqY9FIjxlodD9D7TC8fv0Xog29V1HROlY+PvRNa+f2qp858w8j+9TshkvOAdE1oVzu0F8KylbXfsSXhH7d+n0c8fqSBoLLEjedoDBp3KSO0bof/uzX2lGQJkZhJ/RSPPvND/1gVj9q1lTM5ccbfVfkmwdN0B5iDA5fMJaRz5o8SVILr3uWoBiwx7qsUceyGX77tCn2gZxfiOICNrpy3vv384TO2ovkwvhq1Lg071eXAlxQVtPvRYOGgBAABydn7bEWdP2htRd46nBkGIAoNAnhMvbGNbGCKtNVQAU0N9f7CROLPOTrlw9gVlKK+G5vM1X95KTdcOjs8gKtTkgEos021zBs9R+whyUcs9npo1SJ8GzowVwTwWfVz9adw2jL95zwJ/qz+y5x/IONw9iXspczf7W+bwyQpNaetO9xapF6aHg2/1w7st9yJOd0OfCZsowikJ4JRhAMcmwj4tiHovLyo2fpP3SiNGzDfzrpD+PdvBpyQgg4aPuxqGW8z+4SGn+vwadsLr+kIB4z7jcLQgkMSAplrnczr0GQZJuIPLxfk9mp8oi5dF3+jqvT1d4CWhRwocrs7Vm1tAKxiOBzkUElNaVEoFCPmUYE7uZhfMqOAUsylj3Db1zx1F1d5rPHgRhybpNpxThVWWnuT89I0XLO0WoQeuCSRT0Y9em1lsozSu2wrDKF933GL7YL0TEeKw3qFTPKsmUNlWMIow0jfWrfds/Lasz4pbGA7XXjhylwum8e/I".to_string(),
//         },
//         id : 1
//     }
// }

// fn unwrap_report_obj(r : &Value) -> ASReport {
//     let report_str = r["result"]["report"].as_str().unwrap();
//     let report_obj : ASReport = serde_json::from_str(report_str).unwrap();
//     report_obj
// }

// fn unwrap_result(r : & Value) -> ASResult{
//     let ca = r["result"]["ca"].as_str().unwrap();
//     let certificate = r["result"]["certificate"].as_str().unwrap();
//     let signature = r["result"]["signature"].as_str().unwrap();
//     let validate = match r["result"]["validate"].as_str() {
//         Some(v)=>{
//             if v == "True"{
//                 true
//             }else{
//                 false
//             }
//         },
//         None =>{
//             false
//         },
//     };
//     let report : ASReport =  unwrap_report_obj(r);
//     let result_obj : ASResult  = ASResult{
//         ca: ca.to_string(), 
//         certificate : certificate.to_string(), 
//         signature : signature.to_string(), 
//         validate : validate , 
//         report : report , 
//         report_string :  r["result"]["report"].as_str().unwrap().to_string()
//     };
//     result_obj
// }

// fn unwrap_response(r : & Value) -> ASResponse{
//     let result : ASResult = unwrap_result(r);
//     let id = r["id"].as_i64().unwrap();
//     let jsonrpc = r["jsonrpc"].as_str().unwrap();
//     let response_obj : ASResponse = ASResponse {
//         id : id , 
//         jsonrpc : jsonrpc.to_string(),
//         result : result
//     };
//     response_obj
// }
// fn print_response(r : &Value){
    
//     let response : ASResponse = unwrap_response(r);
//     println!("report to be signed string => {}",response.result.report_string );
//     println!("report isv enclave quote status  => {}",response.result.report.isvEnclaveQuoteStatus );
    
// }
// fn send_request(quote_req : QuoteRequest){
//     let client = reqwest::Client::new();
//     let mut res = client.post("https://sgx.enigma.co/api")
//     .json(&quote_req)
//     .send().unwrap();
//     let response_str =  res.text().unwrap();
//     let json_resonse : Value = serde_json::from_str(response_str.as_str()).unwrap();
//     print_response(&json_resonse);

//     if res.status().is_success(){
//         println!("success! {:?}" ,res.status());
//     }else if res.status().is_server_error(){
//         println!("sever error1 {:?}" ,res.status());
//     }else{
//         println!("unkown error {:?}", res.status() );
//     }
// }

// // new 

// pub struct AttestationService {

// }
// impl AttestationService{
//     pub fn get_report(&self){
//         let req = build_request();
//     }
//     // input: encrypted enclave quote 
//     // output : JSON-RPC request object
//     fn build_request(&self, quote : &String) -> QuoteRequest{
//         QuoteRequest{
//             jsonrpc : "2.0".to_string(),
//             method : "validate".to_string(),
//             params : Params{
//                 quote : quote.to_string(),
//             },
//             id : 1
//         }
//     }
//     // request the report object 
//     fn send_request(quote_req : QuoteRequest)-> Result<Value,Error>{
        
//         let client = reqwest::Client::new();
//         let mut res = client.post(networking::constants::ATTESTATION_SERVICE_URL)
//             .json(&quote_req)
//             .send().unwrap();
//         let response_str =  res.text().unwrap();
//         let json_resonse : Value = serde_json::from_str(response_str.as_str()).unwrap();
//         Ok(json_resonse)
//     }
//     // parse the response json into an ASResponse

// }
// pub fn run(){
//     let mut req = build_request();
//     send_request(req);
// }

//  #[cfg(test)]  
//  mod test {
//     use esgx::general::init_enclave;
//     use esgx::quote_utils;
//      #[test]
//      fn test_get_response_attestation_service(){ 
//             // initiate the enclave 
//             let enclave = match init_enclave() {
//             Ok(r) => {
//                 println!("[+] Init Enclave Successful {}!", r.geteid());
//                 r
//             },
//             Err(x) => {
//                 println!("[-] Init Enclave Failed {}!", x.as_str());
//                 assert_eq!(0,1);
//                 return;
//             },
//         };
//         // post a quote to the AS (Attestation Service)

//         // parse the report 
//         enclave.destroy();
//      }
//  }