use crate::error::{Error, Result};
use crate::models::*;
use crate::soap::SoapClient;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use std::collections::HashMap;

pub struct StudentVueClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl StudentVueClient {
    pub fn new(base_url: String, username: String, password: String) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .user_agent("StudentVUE/8.0.26")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            username,
            password,
        }
    }

    async fn make_request(
        &self,
        service_handle: &str,
        method_name: &str,
        params: &HashMap<String, String>,
        multi_web: bool,
    ) -> Result<String> {
        let param_str = SoapClient::build_params(params);
        let soap_request = SoapClient::create_request(
            &self.username,
            &self.password,
            service_handle,
            method_name,
            &param_str,
            multi_web,
        );

        let endpoint = if service_handle == "HDInfoServices" {
            format!("{}/Service/HDInfoCommunication.asmx", self.base_url)
        } else {
            format!("{}/Service/PXPCommunication.asmx", self.base_url)
        };

        let response = self
            .client
            .post(&endpoint)
            .header("Content-Type", "text/xml; charset=utf-8")
            .header(
                "SOAPAction",
                "http://edupoint.com/webservices/ProcessWebServiceRequest",
            )
            .body(soap_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Authentication(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let response_text = response.text().await?;
        SoapClient::parse_response(&response_text)
    }

    pub async fn get_districts_by_zip(&self, zip_code: &str) -> Result<Vec<DistrictInfo>> {
        let mut params = HashMap::new();
        params.insert(
            "Key".to_string(),
            "5E4B7859-B805-474B-A833-FDB15D205D40".to_string(),
        );
        params.insert("MatchToDistrictZipCode".to_string(), zip_code.to_string());

        let client = Client::new();
        let soap_request = SoapClient::create_request(
            "EdupointDistrictInfo",
            "Edup01nt",
            "HDInfoServices",
            "GetMatchingDistrictList",
            &SoapClient::build_params(&params),
            false,
        );

        let response = client
            .post("https://support.edupoint.com/Service/HDInfoCommunication.asmx")
            .header("Content-Type", "text/xml; charset=utf-8")
            .header(
                "SOAPAction",
                "http://edupoint.com/webservices/ProcessWebServiceRequest",
            )
            .body(soap_request)
            .send()
            .await?;

        let response_text = response.text().await?;
        let xml_data = SoapClient::parse_response(&response_text)?;

        self.parse_districts(&xml_data)
    }

    pub async fn get_messages(&self) -> Result<Vec<Message>> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request("PXPWebServices", "GetPXPMessages", &params, false)
            .await?;

        self.parse_messages(&xml_data)
    }

    pub async fn get_calendar(&self, date: &str) -> Result<Vec<CalendarEvent>> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());
        params.insert("RequestDate".to_string(), date.to_string());

        let xml_data = self
            .make_request("PXPWebServices", "StudentCalendar", &params, false)
            .await?;

        self.parse_calendar(&xml_data)
    }

    pub async fn get_attendance(&self) -> Result<Vec<Absence>> {
        let mut params = HashMap::new();
        params.insert("ChildIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request("PXPWebServices", "Attendance", &params, false)
            .await?;

        self.parse_attendance(&xml_data)
    }

    pub async fn get_gradebook(&self, report_period: Option<usize>) -> Result<Vec<Course>> {
        let mut params = HashMap::new();
        params.insert("ChildIntID".to_string(), "0".to_string());
        if let Some(rp) = report_period {
            params.insert("ReportPeriod".to_string(), rp.to_string());
        }

        let xml_data = self
            .make_request("PXPWebServices", "Gradebook", &params, false)
            .await?;

        self.parse_gradebook(&xml_data)
    }

    pub async fn get_class_notes(&self) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());

        self.make_request("PXPWebServices", "StudentHWNotes", &params, false)
            .await
    }

    pub async fn get_student_info(&self) -> Result<StudentInfo> {
        let mut params = HashMap::new();
        params.insert("ChildIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request("PXPWebServices", "StudentInfo", &params, false)
            .await?;

        self.parse_student_info(&xml_data)
    }

    pub async fn get_class_schedule(
        &self,
        term_index: Option<usize>,
    ) -> Result<Vec<ClassSchedule>> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());
        if let Some(ti) = term_index {
            params.insert("TermIndex".to_string(), ti.to_string());
        }

        let xml_data = self
            .make_request("PXPWebServices", "StudentClassList", &params, false)
            .await?;

        self.parse_class_schedule(&xml_data)
    }

    pub async fn get_school_info(&self) -> Result<SchoolInfo> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request("PXPWebServices", "StudentSchoolInfo", &params, false)
            .await?;

        self.parse_school_info(&xml_data)
    }

    pub async fn list_report_cards(&self) -> Result<Vec<ReportCard>> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request("PXPWebServices", "GetReportCardInitialData", &params, false)
            .await?;

        self.parse_report_cards(&xml_data)
    }

    pub async fn get_report_card(&self, document_gu: &str) -> Result<DocumentData> {
        let mut params = HashMap::new();
        params.insert("DocumentGU".to_string(), document_gu.to_string());

        let xml_data = self
            .make_request(
                "PXPWebServices",
                "GetReportCardDocumentData",
                &params,
                false,
            )
            .await?;

        self.parse_document_data(&xml_data)
    }

    pub async fn list_documents(&self) -> Result<Vec<Document>> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "0".to_string());

        let xml_data = self
            .make_request(
                "PXPWebServices",
                "GetStudentDocumentInitialData",
                &params,
                false,
            )
            .await?;

        self.parse_documents(&xml_data)
    }

    pub async fn get_document(&self, document_gu: &str) -> Result<DocumentData> {
        let mut params = HashMap::new();
        params.insert("DocumentGU".to_string(), document_gu.to_string());

        let xml_data = self
            .make_request("PXPWebServices", "GetContentOfAttachedDoc", &params, false)
            .await?;

        self.parse_document_data(&xml_data)
    }

    pub async fn get_message_attachment(&self, attachment_gu: &str) -> Result<DocumentData> {
        let mut params = HashMap::new();
        params.insert("childIntID".to_string(), "".to_string());
        params.insert("SmAttachmentGU".to_string(), attachment_gu.to_string());

        let xml_data = self
            .make_request("PXPWebServices", "SynergyMailGetAttachment", &params, true)
            .await?;

        self.parse_attachment_data(&xml_data)
    }

    pub async fn mark_message_read(&self, message_id: &str, message_type: &str) -> Result<String> {
        let param_str = format!(
            "&lt;Parms&gt;&lt;MessageListing ID=\"{message_id}\" Type=\"{message_type}\" MarkAsRead=\"true\" /&gt;&lt;/Parms&gt;"
        );

        let soap_request = SoapClient::create_request(
            &self.username,
            &self.password,
            "PXPWebServices",
            "UpdatePXPMessage",
            &param_str,
            true,
        );

        let endpoint = format!("{}/Service/PXPCommunication.asmx", self.base_url);

        let response = self
            .client
            .post(&endpoint)
            .header("Content-Type", "text/xml; charset=utf-8")
            .header(
                "SOAPAction",
                "http://edupoint.com/webservices/ProcessWebServiceRequestMultiWeb",
            )
            .body(soap_request)
            .send()
            .await?;

        let response_text = response.text().await?;
        SoapClient::parse_response(&response_text)
    }

    pub async fn get_student_health_info(
        &self,
        health_conditions: bool,
        health_visits: bool,
        health_immunizations: bool,
    ) -> Result<Vec<HealthImmunization>> {
        let mut params = HashMap::new();
        params.insert("ChildIntID".to_string(), "0".to_string());
        params.insert(
            "HealthConditions".to_string(),
            health_conditions.to_string(),
        );
        params.insert("HealthVisits".to_string(), health_visits.to_string());
        params.insert(
            "HealthImmunizations".to_string(),
            health_immunizations.to_string(),
        );

        let xml_data = self
            .make_request("PXPWebServices", "StudentHealthInfo", &params, true)
            .await?;

        self.parse_health_immunizations(&xml_data)
    }

    fn parse_districts(&self, xml: &str) -> Result<Vec<DistrictInfo>> {
        let mut districts = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"DistrictInfo" => {
                    let mut name = String::new();
                    let mut address = String::new();
                    let mut url = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"Name" => {
                                name = String::from_utf8_lossy(&attr.value).to_string();
                            }
                            b"Address" => {
                                address = String::from_utf8_lossy(&attr.value).to_string();
                            }
                            b"PvueURL" => {
                                url = String::from_utf8_lossy(&attr.value).to_string();
                            }
                            _ => {}
                        }
                    }

                    districts.push(DistrictInfo { name, address, url });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(districts)
    }

    fn parse_messages(&self, xml: &str) -> Result<Vec<Message>> {
        let mut messages = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"MessageListing" => {
                    let mut id = String::new();
                    let mut subject = String::new();
                    let mut content = String::new();
                    let mut from = String::new();
                    let mut date = String::new();
                    let mut read = false;
                    let mut deletable = false;
                    let mut message_type = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"ID" => id = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Subject" => {
                                subject = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"Content" => {
                                content = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"From" => from = String::from_utf8_lossy(&attr.value).to_string(),
                            b"BeginDate" => date = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Read" => read = String::from_utf8_lossy(&attr.value) == "true",
                            b"Deletable" => {
                                deletable = String::from_utf8_lossy(&attr.value) == "true"
                            }
                            b"Type" => {
                                message_type = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    messages.push(Message {
                        id,
                        subject,
                        content,
                        from,
                        date,
                        read,
                        deletable,
                        message_type,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(messages)
    }

    fn parse_calendar(&self, xml: &str) -> Result<Vec<CalendarEvent>> {
        let mut events = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"EventList" => {
                    let mut date = String::new();
                    let mut title = String::new();
                    let mut icon = String::new();
                    let mut day_type = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"Date" => date = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Title" => title = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Icon" => icon = String::from_utf8_lossy(&attr.value).to_string(),
                            b"DayType" => {
                                day_type = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    events.push(CalendarEvent {
                        date,
                        title,
                        icon,
                        day_type,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(events)
    }

    fn parse_attendance(&self, xml: &str) -> Result<Vec<Absence>> {
        let mut absences = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut current_absence: Option<Absence> = None;
        let mut in_periods = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"Absence" => {
                    let mut date = String::new();
                    let mut reason = String::new();
                    let mut note = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"AbsenceDate" => {
                                date = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"Reason" => reason = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Note" => note = String::from_utf8_lossy(&attr.value).to_string(),
                            _ => {}
                        }
                    }

                    current_absence = Some(Absence {
                        date,
                        reason,
                        note,
                        periods: Vec::new(),
                    });
                }
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"Periods" => {
                    in_periods = true;
                }
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"Period" && in_periods => {
                    if let Some(ref mut absence) = current_absence {
                        let mut number = String::new();
                        let mut name = String::new();
                        let mut reason = String::new();
                        let mut course = String::new();
                        let mut staff = String::new();
                        let mut staff_email = String::new();

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"Number" => {
                                    number = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"Name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                                b"Reason" => {
                                    reason = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"Course" => {
                                    course = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"Staff" => {
                                    staff = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"StaffEMail" => {
                                    staff_email = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                _ => {}
                            }
                        }

                        absence.periods.push(AbsencePeriod {
                            number,
                            name,
                            reason,
                            course,
                            staff,
                            staff_email,
                        });
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"Periods" => {
                    in_periods = false;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"Absence" => {
                    if let Some(absence) = current_absence.take() {
                        absences.push(absence);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(absences)
    }

    fn parse_gradebook(&self, xml: &str) -> Result<Vec<Course>> {
        let mut courses = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"Course" => {
                    let mut period = String::new();
                    let mut title = String::new();
                    let mut room = String::new();
                    let mut staff = String::new();
                    let mut staff_email = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"Period" => period = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Title" => title = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Room" => room = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Staff" => staff = String::from_utf8_lossy(&attr.value).to_string(),
                            b"StaffEMail" => {
                                staff_email = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    courses.push(Course {
                        period,
                        title,
                        room,
                        staff,
                        staff_email,
                        marks: Vec::new(),
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(courses)
    }

    fn parse_student_info(&self, xml: &str) -> Result<StudentInfo> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut name = String::new();
        let mut perm_id = String::new();
        let mut gender = String::new();
        let mut grade = String::new();
        let mut address = String::new();
        let mut birth_date = String::new();
        let mut email = String::new();
        let mut phone = String::new();
        let mut current_school = String::new();
        let mut home_room_teacher = String::new();
        let mut home_room_teacher_email = String::new();
        let mut counselor = String::new();

        let mut current_tag = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    current_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e
                        .unescape()
                        .map_err(|e| Error::XmlParse(e.to_string()))?
                        .to_string();
                    match current_tag.as_str() {
                        "FormattedName" => name = text,
                        "PermID" => perm_id = text,
                        "Gender" => gender = text,
                        "Grade" => grade = text,
                        "Address" => address = text,
                        "BirthDate" => birth_date = text,
                        "EMail" => email = text,
                        "Phone" => phone = text,
                        "CurrentSchool" => current_school = text,
                        "HomeRoomTch" => home_room_teacher = text,
                        "HomeRoomTchEMail" => home_room_teacher_email = text,
                        "CounselorName" => counselor = text,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(StudentInfo {
            name,
            perm_id,
            gender,
            grade,
            address,
            birth_date,
            email,
            phone,
            current_school,
            home_room_teacher,
            home_room_teacher_email,
            counselor,
        })
    }

    fn parse_class_schedule(&self, xml: &str) -> Result<Vec<ClassSchedule>> {
        let mut schedules = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"ClassListing" => {
                    let mut period = String::new();
                    let mut course_title = String::new();
                    let mut room_name = String::new();
                    let mut teacher = String::new();
                    let mut teacher_email = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"Period" => period = String::from_utf8_lossy(&attr.value).to_string(),
                            b"CourseTitle" => {
                                course_title = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"RoomName" => {
                                room_name = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"Teacher" => {
                                teacher = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"TeacherEmail" => {
                                teacher_email = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    schedules.push(ClassSchedule {
                        period,
                        course_title,
                        room_name,
                        teacher,
                        teacher_email,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(schedules)
    }

    fn parse_school_info(&self, xml: &str) -> Result<SchoolInfo> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"StudentSchoolInfoListing" => {
                    let mut school = String::new();
                    let mut principal = String::new();
                    let mut address = String::new();
                    let mut city = String::new();
                    let mut state = String::new();
                    let mut zip = String::new();
                    let mut phone = String::new();
                    let mut url = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"School" => school = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Principal" => {
                                principal = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"SchoolAddress" => {
                                address = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"SchoolCity" => {
                                city = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"SchoolState" => {
                                state = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"SchoolZip" => zip = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Phone" => phone = String::from_utf8_lossy(&attr.value).to_string(),
                            b"URL" => url = String::from_utf8_lossy(&attr.value).to_string(),
                            _ => {}
                        }
                    }

                    return Ok(SchoolInfo {
                        school,
                        principal,
                        address,
                        city,
                        state,
                        zip,
                        phone,
                        url,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Err(Error::InvalidResponse("No school info found".to_string()))
    }

    fn parse_report_cards(&self, xml: &str) -> Result<Vec<ReportCard>> {
        let mut report_cards = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"RCReportingPeriod" => {
                    let mut document_gu = String::new();
                    let mut reporting_period = String::new();
                    let mut end_date = String::new();
                    let mut message = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"DocumentGU" => {
                                document_gu = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"ReportingPeriodName" => {
                                reporting_period = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"EndDate" => {
                                end_date = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"Message" => {
                                message = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    report_cards.push(ReportCard {
                        document_gu,
                        reporting_period,
                        end_date,
                        message,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(report_cards)
    }

    fn parse_documents(&self, xml: &str) -> Result<Vec<Document>> {
        let mut documents = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"StudentDocumentData" => {
                    let mut document_gu = String::new();
                    let mut file_name = String::new();
                    let mut date = String::new();
                    let mut document_type = String::new();
                    let mut comment = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"DocumentGU" => {
                                document_gu = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"DocumentFileName" => {
                                file_name = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"DocumentDate" => {
                                date = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"DocumentType" => {
                                document_type = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"DocumentComment" => {
                                comment = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    documents.push(Document {
                        document_gu,
                        file_name,
                        date,
                        document_type,
                        comment,
                    });
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(documents)
    }

    fn parse_document_data(&self, xml: &str) -> Result<DocumentData> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut document_gu = String::new();
        let mut file_name = String::new();
        let mut doc_type = String::new();
        let mut base64_content = String::new();
        let mut current_tag = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"DocumentData" => {
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"DocumentGU" => {
                                document_gu = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"FileName" => {
                                file_name = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"DocType" => {
                                doc_type = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::Start(ref e)) => {
                    current_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) if current_tag == "Base64Code" => {
                    base64_content = e
                        .unescape()
                        .map_err(|e| Error::XmlParse(e.to_string()))?
                        .to_string();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(DocumentData {
            document_gu,
            file_name,
            doc_type,
            base64_content,
        })
    }

    fn parse_attachment_data(&self, xml: &str) -> Result<DocumentData> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut file_name = String::new();
        let mut base64_content = String::new();
        let mut current_tag = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"AttachmentXML" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"DocumentName" {
                            file_name = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                }
                Ok(Event::Start(ref e)) => {
                    current_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                }
                Ok(Event::Text(e)) if current_tag == "Base64Code" => {
                    base64_content = e
                        .unescape()
                        .map_err(|e| Error::XmlParse(e.to_string()))?
                        .to_string();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(DocumentData {
            document_gu: String::new(),
            file_name,
            doc_type: "unknown".to_string(),
            base64_content,
        })
    }

    fn parse_health_immunizations(&self, xml: &str) -> Result<Vec<HealthImmunization>> {
        let mut immunizations = Vec::new();
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut current_immunization: Option<HealthImmunization> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"HealthImmunizationListing" => {
                    let mut name = String::new();
                    let mut compliant = false;
                    let mut compliant_message = String::new();
                    let mut required_doses = String::new();

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"Name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                            b"Compliant" => {
                                compliant = String::from_utf8_lossy(&attr.value) == "true"
                            }
                            b"CompliantMessage" => {
                                compliant_message = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"NumReqDoses" => {
                                required_doses = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }

                    current_immunization = Some(HealthImmunization {
                        name,
                        compliant,
                        compliant_message,
                        required_doses,
                        dates: Vec::new(),
                    });
                }
                Ok(Event::Empty(ref e)) if e.name().as_ref() == b"ImmunizationDate" => {
                    if let Some(ref mut imm) = current_immunization {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"ImmunizationDt" {
                                imm.dates
                                    .push(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"HealthImmunizationListing" => {
                    if let Some(imm) = current_immunization.take() {
                        immunizations.push(imm);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(immunizations)
    }
}
