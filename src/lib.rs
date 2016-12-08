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

    pub fn to_string(self) -> String {
        let mut ret = String::new();

        if let Some(name) = self.name {
            ret += &format!("'{}' ", name);
        }

        ret + &format!("<{}>", self.num)
    }
}

pub struct CallFile {
    pub channel: String,
    pub caller_id: CallerId,
    pub wait_time_secs: Option<i16>,
    pub max_retries: Option<i16>,
    pub retry_time_secs: Option<i16>,
    pub account: Option<String>,
}

impl CallFile {
    pub fn to_string(self) -> String {
        let mut ret = format!("Channel: {}\r\n", self.channel).to_owned();
        ret += &format!("CallerID: \"{}\"\r\n", self.caller_id.to_string());

        if let Some(t) = self.wait_time_secs {
            ret += &format!("WaitTime: {}\r\n", t);
        }

        if let Some(r) = self.max_retries {
            ret += &format!("MaxRetries: {}\r\n", r);
        }

        if let Some(rt) = self.retry_time_secs {
            ret += &format!("RetryTime: {}\r\n", rt);
        }

        if let Some(a) = self.account {
            ret += &format!("Account: {}\r\n", a);
        }

        ret
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

        assert_eq!(cf.to_string(),
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

        assert_eq!(cf.to_string(),
                   "Channel: SIP/just-chickens/1234567890\r\nCallerID: \"'Johnny' \
                    <0987654321>\"\r\n");
    }
}
