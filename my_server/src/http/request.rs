use super::method::{Method, MethodError};
use super::QueryString;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::str;

use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: HashMap<String, &'buf str>,
    body: Option<&'buf str>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn get_header(&self, key: &str) -> Option<&&str> {
        self.headers.get(&key.to_lowercase())
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Received a request: GET / HTTP/1.1
    // Host: 127.0.0.1:8080
    // Connection: keep-alive
    // Cache-Control: max-age=0
    //
    //
    // Body
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?; //.or(Err(ParseError::InvalidEncoding))?;

        let (info, body_str) = request.split_once("\r\n\r\n").unwrap();
        let (request_line, header_lines) = info.split_once("\r\n").unwrap();

        let (method, request_line) =
            get_next_word(request_line).ok_or(ParseError::InvalidRequest)?;
        let (mut path, protocol) = get_next_word(request_line).ok_or(ParseError::InvalidUri)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidVersion);
        }

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let mut body = None;
        if !body_str.is_empty() {
            body = Some(body_str);
        }

        let mut headers = HashMap::new();

        for header_line in header_lines.lines() {
            let (key, value) = header_line
                .split_once(": ")
                .ok_or(ParseError::InvalidHeader)?;
            headers.insert(key.to_lowercase(), value);
        }

        let method: Method = method.parse()?;
        /*let (info, body) = str.split_once("\r\n\r\n").unwrap();
        let (request_line, header_lines) = info.split_once("\r\n").unwrap();
        let mut lines = info.split("\r\n");

        // GET / HTTP/1.1
        // Method, path, protocol
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap();
        let path = parts.next().unwrap();
        let protocol = parts.next().unwrap();

        let headers = header_lines
            .lines()
            .map(|line| line.to_owned().split_once(": "))
            .collect::<Option<HashMap<_, _>>>()
            .ok_or_else(|| ParseError::InvalidHeader);

        let method_parsed: Method = method.parse().or(Err(ParseError::InvalidMethod))?;
        */
        Ok(Self {
            path,
            query_string,
            method,
            headers,
            body,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.split_once(" ");
    let (next, rest) = iter?;

    Some((next, rest))
}
#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidUri,
    InvalidVersion,
    InvalidHeader,
    InvalidBody,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidUri => "Invalid uri",
            Self::InvalidVersion => "Invalid version",
            Self::InvalidHeader => "Invalid header",
            Self::InvalidBody => "Invalid body",
        }
    }
}

impl Error for ParseError {}
