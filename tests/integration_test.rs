use std::env;
use studenvue::StudentVueClient;

fn get_test_credentials() -> (String, String, String) {
    let portal =
        env::var("STUDENTVUE_PORTAL").expect("STUDENTVUE_PORTAL must be set for integration tests");
    let username = env::var("STUDENTVUE_USERNAME")
        .expect("STUDENTVUE_USERNAME must be set for integration tests");
    let password = env::var("STUDENTVUE_PASSWORD")
        .expect("STUDENTVUE_PASSWORD must be set for integration tests");
    (portal, username, password)
}

#[tokio::test]
async fn test_get_messages() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_messages().await;
    match result {
        Ok(messages) => {
            println!("✓ Found {} messages", messages.len());
            for (i, msg) in messages.iter().take(3).enumerate() {
                println!("  Message {}: {} (from: {})", i + 1, msg.subject, msg.from);
            }
        }
        Err(e) => println!("✗ Error getting messages: {}", e),
    }
}

#[tokio::test]
async fn test_get_student_info() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_student_info().await;
    match result {
        Ok(info) => {
            println!("✓ Student Info:");
            println!("  Name: {}", info.name);
            println!("  Grade: {}", info.grade);
            println!("  School: {}", info.current_school);
            println!("  Email: {}", info.email);
        }
        Err(e) => println!("✗ Error getting student info: {}", e),
    }
}

#[tokio::test]
async fn test_get_gradebook() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_gradebook(None).await;
    match result {
        Ok(courses) => {
            println!("✓ Found {} courses", courses.len());
            for course in courses.iter() {
                println!(
                    "  Period {}: {} ({})",
                    course.period, course.title, course.staff
                );
            }
        }
        Err(e) => println!("✗ Error getting gradebook: {}", e),
    }
}

#[tokio::test]
async fn test_get_attendance() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_attendance().await;
    match result {
        Ok(absences) => {
            println!("✓ Found {} absences", absences.len());
            for absence in absences.iter().take(3) {
                println!(
                    "  {}: {} ({} periods)",
                    absence.date,
                    absence.reason,
                    absence.periods.len()
                );
            }
        }
        Err(e) => println!("✗ Error getting attendance: {}", e),
    }
}

#[tokio::test]
async fn test_get_calendar() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_calendar("10/30/2025").await;
    match result {
        Ok(events) => {
            println!("✓ Found {} calendar events", events.len());
            for event in events.iter().take(5) {
                println!("  {}: {}", event.date, event.title);
            }
        }
        Err(e) => println!("✗ Error getting calendar: {}", e),
    }
}

#[tokio::test]
async fn test_get_class_schedule() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_class_schedule(None).await;
    match result {
        Ok(schedule) => {
            println!("✓ Found {} classes", schedule.len());
            for class in schedule.iter() {
                println!(
                    "  Period {}: {} in {} ({})",
                    class.period, class.course_title, class.room_name, class.teacher
                );
            }
        }
        Err(e) => println!("✗ Error getting class schedule: {}", e),
    }
}

#[tokio::test]
async fn test_get_school_info() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_school_info().await;
    match result {
        Ok(info) => {
            println!("✓ School Info:");
            println!("  Name: {}", info.school);
            println!("  Principal: {}", info.principal);
            println!(
                "  Address: {}, {}, {} {}",
                info.address, info.city, info.state, info.zip
            );
            println!("  Phone: {}", info.phone);
            println!("  Website: {}", info.url);
        }
        Err(e) => println!("✗ Error getting school info: {}", e),
    }
}

#[tokio::test]
async fn test_list_documents() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.list_documents().await;
    match result {
        Ok(docs) => {
            println!("✓ Found {} documents", docs.len());
            for doc in docs.iter().take(3) {
                println!("  {}: {} ({})", doc.date, doc.file_name, doc.document_type);
            }
        }
        Err(e) => println!("✗ Error listing documents: {}", e),
    }
}

#[tokio::test]
async fn test_list_report_cards() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.list_report_cards().await;
    match result {
        Ok(cards) => {
            println!("✓ Found {} report cards", cards.len());
            for card in cards.iter() {
                println!(
                    "  {}: {} (ends {})",
                    card.reporting_period, card.message, card.end_date
                );
            }
        }
        Err(e) => println!("✗ Error listing report cards: {}", e),
    }
}

#[tokio::test]
async fn test_get_districts_by_zip() {
    let (portal, username, password) = get_test_credentials();
    let client = StudentVueClient::new(portal, username, password);

    let result = client.get_districts_by_zip("94102").await;
    match result {
        Ok(districts) => {
            println!("✓ Found {} districts for test ZIP code", districts.len());
            for district in districts.iter().take(3) {
                println!("  {}: {}", district.name, district.address);
            }
        }
        Err(e) => println!("✗ Error getting districts: {}", e),
    }
}
