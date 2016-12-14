extern crate mktemp;
use mktemp::Temp;

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

pub struct CallerId {
    pub num: String,
    pub name: Option<String>,
}

impl CallerId {
    pub fn new(num: String, name: Option<String>) -> CallerId {
        CallerId {
            num: num,
            name: name,
        }
    }

    pub fn to_text(&self) -> String {
        let mut ret = String::new();

        if let Some(ref name) = self.name {
            ret += &format!("'{}' ", name);
        }

        ret + &format!("<{}>", self.num)
    }
}

static DEFAULT_SPOOL_DIR: &'static str = "/var/spool/asterisk/outgoing";
static DEFAULT_TMP_DIR: &'static str = "/tmp";

pub struct CallFile {
    pub channel: String,
    pub caller_id: CallerId,
    pub wait_time_secs: Option<i16>,
    pub max_retries: Option<i16>,
    pub retry_time_secs: Option<i16>,
    pub account: Option<String>,
}

pub enum SpoolError {
    FailedToCreateTmpFile,
    FailedToOpenTmpFileForWriting,
    FailedToWriteToTmpFile,
    FailedToMoveToSpool,
}

impl CallFile {
    fn to_text(&self) -> String {
        let mut ret = format!("Channel: {}\r\n", self.channel).to_owned();
        ret += &format!("CallerID: \"{}\"\r\n", self.caller_id.to_text());

        if let Some(ref t) = self.wait_time_secs {
            ret += &format!("WaitTime: {}\r\n", t);
        }

        if let Some(r) = self.max_retries {
            ret += &format!("MaxRetries: {}\r\n", r);
        }

        if let Some(rt) = self.retry_time_secs {
            ret += &format!("RetryTime: {}\r\n", rt);
        }

        if let Some(ref a) = self.account {
            ret += &format!("Account: {}\r\n", a);
        }

        ret
    }

    fn _spool(self, tmp_dir: PathBuf, mut spool_dir: PathBuf) -> Result<(), SpoolError> {
        let text = self.to_text();
        let tmp_file = match Temp::new_file_in(tmp_dir.as_path()) {
            Ok(tmp) => tmp,
            Err(_) => return Err(SpoolError::FailedToCreateTmpFile),
        };

        let path_buf = tmp_file.to_path_buf();
        let file_name = path_buf.clone().file_name();
        let mut file = match File::open(path_buf) {
            Ok(f) => f,
            Err(_) => return Err(SpoolError::FailedToOpenTmpFileForWriting),
        };

        match file.write_all(text.as_bytes()) {
            Err(_) => return Err(SpoolError::FailedToWriteToTmpFile),
            _ => {}
        };

        let file_name = match file_name {
            Some(n) => n,
            None => return Err(SpoolError::FailedToMoveToSpool),
        };

        let new_file_name = spool_dir.push(file_name);
        println!("{:?}", new_file_name);
        Ok(())

        // std::fs::rename(path_buf,)
    }

    pub fn spool(self) -> Result<(), SpoolError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_call_file() {
        let cf = CallFile {
            channel: "SIP/just-chickens/1234567890".into(),
            caller_id: CallerId::new("0987654321".into(), None),
            wait_time_secs: None,
            max_retries: None,
            retry_time_secs: None,
            account: None,
        };

        assert_eq!(cf.to_text(),
                   "Channel: SIP/just-chickens/1234567890\r\nCallerID: \"<0987654321>\"\r\n");
    }

    #[test]
    fn caller_id_with_name() {
        let cf = CallFile {
            channel: "SIP/just-chickens/1234567890".into(),
            caller_id: CallerId::new("0987654321".into(), Some("Johnny".into())),
            wait_time_secs: None,
            max_retries: None,
            retry_time_secs: None,
            account: None,
        };

        assert_eq!(cf.to_text(),
                   "Channel: SIP/just-chickens/1234567890\r\nCallerID: \"'Johnny' \
                    <0987654321>\"\r\n");
    }
}
