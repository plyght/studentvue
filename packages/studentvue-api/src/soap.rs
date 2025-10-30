use crate::error::{Error, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

pub struct SoapClient;

impl SoapClient {
    pub fn create_request(
        user_id: &str,
        password: &str,
        service_handle: &str,
        method_name: &str,
        param_str: &str,
        multi_web: bool,
    ) -> String {
        let user_id = Self::escape_xml(user_id);
        let password = Self::escape_xml(password);
        let service_handle = Self::escape_xml(service_handle);
        let method_name = Self::escape_xml(method_name);

        if multi_web {
            format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
<soap:Body>
<ProcessWebServiceRequestMultiWeb xmlns="http://edupoint.com/webservices/">
<userID>{user_id}</userID>
<password>{password}</password>
<skipLoginLog>1</skipLoginLog>
<parent>0</parent>
<webDBName></webDBName>
<webServiceHandleName>{service_handle}</webServiceHandleName>
<methodName>{method_name}</methodName>
<paramStr>{param_str}</paramStr>
</ProcessWebServiceRequestMultiWeb>
</soap:Body>
</soap:Envelope>"#
            )
        } else {
            format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
<soap:Body>
<ProcessWebServiceRequest xmlns="http://edupoint.com/webservices/">
<userID>{user_id}</userID>
<password>{password}</password>
<skipLoginLog>1</skipLoginLog>
<parent>0</parent>
<webServiceHandleName>{service_handle}</webServiceHandleName>
<methodName>{method_name}</methodName>
<paramStr>{param_str}</paramStr>
</ProcessWebServiceRequest>
</soap:Body>
</soap:Envelope>"#
            )
        }
    }

    pub fn parse_response(response_text: &str) -> Result<String> {
        let mut reader = Reader::from_str(response_text);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut in_result = false;
        let mut result_content = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let local_name = std::str::from_utf8(name.as_ref()).unwrap_or("");
                    if local_name.contains("ProcessWebServiceRequestResult")
                        || local_name.contains("ProcessWebServiceRequestMultiWebResult")
                    {
                        in_result = true;
                    }
                }
                Ok(Event::Text(e)) if in_result => {
                    result_content
                        .push_str(&e.unescape().map_err(|e| Error::XmlParse(e.to_string()))?);
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let local_name = std::str::from_utf8(name.as_ref()).unwrap_or("");
                    if local_name.contains("ProcessWebServiceRequestResult")
                        || local_name.contains("ProcessWebServiceRequestMultiWebResult")
                    {
                        in_result = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(result_content)
    }

    pub fn build_params(params: &HashMap<String, String>) -> String {
        if params.is_empty() {
            return "&lt;Parms/&gt;".to_string();
        }

        let mut param_parts = String::from("&lt;Parms&gt;");
        for (key, value) in params {
            param_parts.push_str(&format!("&lt;{key}&gt;{value}&lt;/{key}&gt;"));
        }
        param_parts.push_str("&lt;/Parms&gt;");
        param_parts
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
