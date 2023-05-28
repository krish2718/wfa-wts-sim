/// Unit tests for functions in `src/main.rs`
use mockstream::SyncMockStream;


#[test]
fn test_parse_internal_cmd() {
    let mut int_cmd: wfa_wts_sim::InternalCmd = wfa_wts_sim::InternalCmd {
        key: String::new(),
        value: String::new(),
    };

    let parsed: bool = wfa_wts_sim::parse_internal_cmd(&"!sleep!10!".to_string(), &mut int_cmd);
    assert_eq!(parsed, true);
    assert_eq!(int_cmd.key, "sleep");
    assert_eq!(int_cmd.value, "10");

    let parsed: bool = wfa_wts_sim::parse_internal_cmd(&"!sleep!10!20".to_string(), &mut int_cmd);
    assert_eq!(parsed, false);

    let parsed: bool = wfa_wts_sim::parse_internal_cmd(&"!sleep".to_string(), &mut int_cmd);
    assert_eq!(parsed, false);

    let parsed: bool = wfa_wts_sim::parse_internal_cmd(&"ca_get_version".to_string(), &mut int_cmd);
    assert_eq!(parsed, false);

    let parsed: bool = wfa_wts_sim::parse_internal_cmd(&"!sleep!".to_string(), &mut int_cmd);
    assert_eq!(parsed, false);
}


#[test]
fn test_send_one_cmd() {
    let input = "ca_get_version".to_string();
    let mut stream = SyncMockStream::new();
    stream.push_bytes_to_read(b"STATUS, RUNNING\n");
    stream.push_bytes_to_read(b"STATUS, COMPLETE\n");
    let sent: i64 = wfa_wts_sim::send_one_cmd(stream.clone(), &input);
    let exp_input = format!("{}{}", input, "   ");
    assert_eq!(stream.pop_bytes_written(), exp_input.as_bytes());
    // RUNNING + COMPLETE
    assert_eq!(sent, 33);
}

#[test]
fn test_send_one_cmd_timeout_full() {
    let input = "ca_get_version".to_string();
    let mut stream = SyncMockStream::new();
    let sent: i64 = wfa_wts_sim::send_one_cmd(stream.clone(), &input);
    let exp_input = format!("{}{}", input, "   ");
    assert_eq!(stream.pop_bytes_written(), exp_input.as_bytes());
    assert_eq!(sent, -1);
    // Cannot test the timeout because the mockstream does not support timeouts
}

#[test]
fn test_send_one_cmd_timeout_partial() {
    let input = "ca_get_version".to_string();
    let mut stream = SyncMockStream::new();
    stream.push_bytes_to_read(b"STATUS, RUNNING\n");
    let sent: i64 = wfa_wts_sim::send_one_cmd(stream.clone(), &input);
    let exp_input = format!("{}{}", input, "   ");
    assert_eq!(stream.pop_bytes_written(), exp_input.as_bytes());
    // RUNNING
    assert_eq!(sent, -1);
    // Cannot test the timeout because the mockstream does not support timeouts
}
