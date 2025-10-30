use studenvue::{Result, StudentVueClient};

#[tokio::main]
async fn main() -> Result<()> {
    let portal_url = std::env::var("STUDENTVUE_PORTAL")
        .expect("STUDENTVUE_PORTAL environment variable must be set");
    let username = std::env::var("STUDENTVUE_USERNAME")
        .expect("STUDENTVUE_USERNAME environment variable must be set");
    let password = std::env::var("STUDENTVUE_PASSWORD")
        .expect("STUDENTVUE_PASSWORD environment variable must be set");

    let client = StudentVueClient::new(portal_url, username, password);

    println!("=== StudentVue API Demo ===\n");

    println!("📚 Getting student info...");
    match client.get_student_info().await {
        Ok(info) => {
            println!("  Name: {}", info.name);
            println!("  Grade: {}", info.grade);
            println!("  School: {}", info.current_school);
            println!("  Email: {}", info.email);
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n📖 Getting gradebook...");
    match client.get_gradebook(None).await {
        Ok(courses) => {
            println!("  Found {} courses:", courses.len());
            for course in courses.iter() {
                println!(
                    "    • Period {}: {} ({})",
                    course.period, course.title, course.staff
                );
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n📅 Getting class schedule...");
    match client.get_class_schedule(None).await {
        Ok(schedule) => {
            println!("  Found {} classes:", schedule.len());
            for class in schedule.iter() {
                println!(
                    "    • Period {}: {} in Room {} - {}",
                    class.period, class.course_title, class.room_name, class.teacher
                );
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n📊 Getting attendance...");
    match client.get_attendance().await {
        Ok(absences) => {
            println!("  Total absences: {}", absences.len());
            if !absences.is_empty() {
                println!("  Recent absences:");
                for absence in absences.iter().take(5) {
                    println!(
                        "    • {}: {} ({} periods)",
                        absence.date,
                        absence.reason,
                        absence.periods.len()
                    );
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n💌 Getting messages...");
    match client.get_messages().await {
        Ok(messages) => {
            println!("  Total messages: {}", messages.len());
            if !messages.is_empty() {
                println!("  Recent messages:");
                for msg in messages.iter().take(5) {
                    let read_status = if msg.read { "READ" } else { "UNREAD" };
                    println!("    • [{}] {} - {}", read_status, msg.from, msg.subject);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n🏫 Getting school info...");
    match client.get_school_info().await {
        Ok(info) => {
            println!("  School: {}", info.school);
            println!("  Principal: {}", info.principal);
            println!(
                "  Address: {}, {}, {} {}",
                info.address, info.city, info.state, info.zip
            );
            println!("  Phone: {}", info.phone);
            println!("  Website: {}", info.url);
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n✅ Demo complete!");

    Ok(())
}
