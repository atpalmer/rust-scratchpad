use std::string::FromUtf8Error;
use curl::easy::Easy;

struct Response {
    headers: String,
    content: String,
}

struct ResponseError {
    message: String,
}

impl From<curl::Error> for ResponseError {
    fn from(e: curl::Error) -> ResponseError {
        return ResponseError {
            message: format!("{}", e)
        };
    }
}

impl From<FromUtf8Error> for ResponseError {
    fn from(e: FromUtf8Error) -> ResponseError {
        return ResponseError {
            message: format!("{}", e)
        };
    }
}

fn get(url :&str) -> Result<Response, ResponseError> {
    let mut headerbuff = Vec::<u8>::new();
    let mut respbuff = Vec::<u8>::new();
    let mut curl = Easy::new();

    curl.url(url)?;

    {
        let mut transfer = curl.transfer();
        transfer.header_function(|data| {
            headerbuff.extend_from_slice(data);
            true
        })?;
        transfer.write_function(|data| {
            respbuff.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    return Ok(Response {
        headers: String::from_utf8(headerbuff)?,
        content: String::from_utf8(respbuff)?,
     });
}

fn main() {
    let resp = get("https://www.example.com/");
    match resp {
        Ok(resp) => {
            println!("Headers:\n{}", resp.headers);
            println!("Content:\n{}", resp.content);
        },
        Err(e) => panic!("{}", e.message),
    }
}

