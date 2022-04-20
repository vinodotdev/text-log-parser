use log_parser::{LogFormat, parse};

#[test]
fn test_parse() {
    let format = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
    let log_format = LogFormat::new(format);
    let log_string = r#"167.71.134.212 - - [15/Apr/2022:14:04:30 +0000] "\xE6T\xA1\x12)\x05\x1D\xF6\xED\xAD\x84M\x10\xF5\x9Cj\xE1\x87\xB2\xF6\xC2#\xAF}\x90\x8D\xB3M\xB0:\xB7\xF9W\xEA\xF0\xBDc\x07\xC8Z\xC9p\x83\x1A%\xDD\xC6\x5C\x81\x91\xFD\xBAX7u\xE3\x80\xE8\xFCd\x9C\x93l&\xF2bDBr" 400 172 "-" "-""#;
    let log_message = parse(log_format, log_string);
    assert_eq!(log_message.len(),9);
    assert_eq!(log_message["remote_addr"],"167.71.134.212");
    assert_eq!(log_message["remote_user"],"-");
    assert_eq!(log_message["time_local"],"15/Apr/2022:14:04:30 +0000");
    assert_eq!(log_message["request"],r#"\xE6T\xA1\x12)\x05\x1D\xF6\xED\xAD\x84M\x10\xF5\x9Cj\xE1\x87\xB2\xF6\xC2#\xAF}\x90\x8D\xB3M\xB0:\xB7\xF9W\xEA\xF0\xBDc\x07\xC8Z\xC9p\x83\x1A%\xDD\xC6\x5C\x81\x91\xFD\xBAX7u\xE3\x80\xE8\xFCd\x9C\x93l&\xF2bDBr"#);
    assert_eq!(log_message["status"],"400");
    assert_eq!(log_message["body_bytes_sent"],"172");
    assert_eq!(log_message["http_referer"],"-");
    assert_eq!(log_message["http_user_agent"],"-");
}