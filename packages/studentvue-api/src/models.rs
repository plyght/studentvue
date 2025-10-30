use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistrictInfo {
    pub name: String,
    pub address: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub subject: String,
    pub content: String,
    pub from: String,
    pub date: String,
    pub read: bool,
    pub deletable: bool,
    pub message_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub date: String,
    pub title: String,
    pub icon: String,
    pub day_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absence {
    pub date: String,
    pub reason: String,
    pub note: String,
    pub periods: Vec<AbsencePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsencePeriod {
    pub number: String,
    pub name: String,
    pub reason: String,
    pub course: String,
    pub staff: String,
    pub staff_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub period: String,
    pub title: String,
    pub room: String,
    pub staff: String,
    pub staff_email: String,
    pub marks: Vec<Mark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mark {
    pub mark_name: String,
    pub score: String,
    pub score_raw: Option<String>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub measure: String,
    pub assignment_type: String,
    pub date: String,
    pub due_date: String,
    pub score: String,
    pub score_type: String,
    pub points: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentInfo {
    pub name: String,
    pub perm_id: String,
    pub gender: String,
    pub grade: String,
    pub address: String,
    pub birth_date: String,
    pub email: String,
    pub phone: String,
    pub current_school: String,
    pub home_room_teacher: String,
    pub home_room_teacher_email: String,
    pub counselor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSchedule {
    pub period: String,
    pub course_title: String,
    pub room_name: String,
    pub teacher: String,
    pub teacher_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolInfo {
    pub school: String,
    pub principal: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCard {
    pub document_gu: String,
    pub reporting_period: String,
    pub end_date: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub document_gu: String,
    pub file_name: String,
    pub date: String,
    pub document_type: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    pub document_gu: String,
    pub file_name: String,
    pub doc_type: String,
    pub base64_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthImmunization {
    pub name: String,
    pub compliant: bool,
    pub compliant_message: String,
    pub required_doses: String,
    pub dates: Vec<String>,
}
