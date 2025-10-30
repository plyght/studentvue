# StudentVue API Client

A comprehensive, type-safe Rust client library for the StudentVue SOAP API. Provides programmatic access to student information, grades, attendance, schedules, messages, and documents through the official StudentVue web services.

## Features

- 15+ fully implemented API endpoints
- Type-safe data models with proper error handling
- Async/await support via Tokio runtime
- Automatic SOAP envelope construction and XML parsing
- Production-ready with comprehensive test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
studenvue = "0.1.0"
tokio = { version = "1.35", features = ["full"] }
```

## Configuration

Create a `.env` file in your project root with your credentials:

```bash
cp env.example .env
# Edit .env with your actual credentials
```

Required environment variables:
- `STUDENTVUE_PORTAL` - Your district's StudentVue portal URL
- `STUDENTVUE_USERNAME` - Your student ID or username
- `STUDENTVUE_PASSWORD` - Your StudentVue password

## Quick Start

```rust
use studenvue::{StudentVueClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let portal = env::var("STUDENTVUE_PORTAL").expect("STUDENTVUE_PORTAL not set");
    let username = env::var("STUDENTVUE_USERNAME").expect("STUDENTVUE_USERNAME not set");
    let password = env::var("STUDENTVUE_PASSWORD").expect("STUDENTVUE_PASSWORD not set");

    let client = StudentVueClient::new(portal, username, password);

    let info = client.get_student_info().await?;
    println!("Student: {} (Grade {})", info.name, info.grade);

    let courses = client.get_gradebook(None).await?;
    println!("Enrolled in {} courses", courses.len());

    Ok(())
}
```

## API Reference

### Student Information
- `get_student_info()` - Retrieve student profile including name, grade, school, and contact information
- `get_school_info()` - Retrieve school details including principal, address, and contact information

### Academic Records
- `get_gradebook(report_period)` - Retrieve current grades, assignments, and course information
- `get_class_schedule(term_index)` - Retrieve class schedule with periods, teachers, and room assignments
- `get_calendar(date)` - Retrieve calendar events and upcoming assignments for a specific date

### Attendance
- `get_attendance()` - Retrieve attendance records including absences, tardies, and reasons

### Communication
- `get_messages()` - Retrieve inbox messages from teachers and administrators
- `mark_message_read(message_id, message_type)` - Mark a specific message as read
- `get_message_attachment(attachment_gu)` - Download attachments from messages

### Documents
- `list_documents()` - List all available documents
- `get_document(document_gu)` - Download a specific document by GUID
- `list_report_cards()` - List available report cards by grading period
- `get_report_card(document_gu)` - Download a specific report card

### Health Records
- `get_student_health_info(conditions, visits, immunizations)` - Retrieve student health information

### Utilities
- `get_districts_by_zip(zip_code)` - Search for school districts by ZIP code
- `get_class_notes()` - Retrieve homework notes (district-dependent feature)

## Examples

### Getting Grades

```rust
let courses = client.get_gradebook(None).await?;
for course in courses {
    println!("Period {}: {}", course.period, course.title);
    println!("  Teacher: {} ({})", course.staff, course.staff_email);
    for mark in course.marks {
        println!("  {}: {}", mark.mark_name, mark.score);
    }
}
```

### Checking Attendance

```rust
let absences = client.get_attendance().await?;
for absence in absences {
    println!("Date: {}", absence.date);
    println!("Reason: {}", absence.reason);
    for period in absence.periods {
        println!("  Period {}: {} - {}", period.number, period.course, period.reason);
    }
}
```

### Getting Messages

```rust
let messages = client.get_messages().await?;
for msg in messages {
    println!("[{}] {}: {}", 
        if msg.read { "READ" } else { "UNREAD" },
        msg.from, 
        msg.subject
    );
}
```

### Downloading Documents

```rust
let documents = client.list_documents().await?;
if let Some(doc) = documents.first() {
    let data = client.get_document(&doc.document_gu).await?;
    let bytes = base64::decode(&data.base64_content)?;
    std::fs::write(&doc.file_name, bytes)?;
    println!("Downloaded: {}", doc.file_name);
}
```

## Testing

Set up environment variables in `.env` and run tests:

```bash
cargo test
```

All tests are integration tests that verify functionality against live API endpoints.

## Development

```bash
make format        # Format code with rustfmt
make lint          # Run clippy linter
make typecheck     # Run type checking
make test          # Run test suite
make quality-gates # Run all checks
```

## Finding Your District Portal

Use the district lookup utility:

```rust
let client = StudentVueClient::new(/* ... */);
let districts = client.get_districts_by_zip("12345").await?;
for district in districts {
    println!("{}: {}", district.name, district.url);
}
```

Your district portal URL will typically follow the pattern `https://[district-name].edupoint.com` or similar.

## Documentation

This library implements the StudentVue SOAP API based on documented endpoints. For detailed API specifications, refer to the [StudentVue API documentation](https://github.com/StudentVue/docs).

## Security Considerations

**Important:** Never commit credentials to version control. Always use environment variables or a secure credential management system. Add `.env` to your `.gitignore` file.

## License

MIT

## Contributing

Contributions are welcome. Please ensure all quality gates pass before submitting pull requests:
- Code must be formatted with `rustfmt`
- All clippy lints must pass
- All tests must pass
- New features should include tests

## Acknowledgments

Based on reverse-engineering and documentation of the StudentVue SOAP API by the community.

