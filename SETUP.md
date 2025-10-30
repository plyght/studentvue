# Setup Guide

## Prerequisites

- Rust 1.70 or later
- Active StudentVue account
- Your district's StudentVue portal URL

## Installation

1. Clone or add the library to your project:

```bash
# Add to Cargo.toml
[dependencies]
studenvue = "0.1.0"
tokio = { version = "1.35", features = ["full"] }
```

2. Set up environment variables:

```bash
# Copy the example environment file
cp env.example .env

# Edit .env with your credentials
nano .env  # or use your preferred editor
```

3. Configure your credentials in `.env`:

```env
STUDENTVUE_PORTAL=https://your-district.edupoint.com
STUDENTVUE_USERNAME=your_student_id
STUDENTVUE_PASSWORD=your_password
```

## Finding Your District Portal

If you don't know your district's portal URL:

1. Use the district lookup feature:

```rust
use studenvue::StudentVueClient;

#[tokio::main]
async fn main() {
    // Create a temporary client (credentials not needed for this)
    let client = StudentVueClient::new(
        "https://support.edupoint.com".to_string(),
        "temp".to_string(),
        "temp".to_string(),
    );
    
    // Search by your ZIP code
    let districts = client.get_districts_by_zip("YOUR_ZIP").await.unwrap();
    
    for district in districts {
        println!("{}", district.name);
        println!("  Portal: {}", district.url);
        println!("  Address: {}\n", district.address);
    }
}
```

2. Contact your school district's IT department

3. Check your school's website for StudentVue login links

## Verifying Installation

Run the example to test your setup:

```bash
cargo run --example basic_usage
```

If configured correctly, you should see your student information, grades, and schedule.

## Troubleshooting

### Authentication Errors

- Verify your credentials are correct
- Check that your portal URL is complete (including `https://`)
- Ensure your StudentVue account is active

### Connection Issues

- Verify you have internet access
- Check if your district's portal is accessible via web browser
- Some districts may have IP restrictions or maintenance windows

### Environment Variable Issues

```bash
# Verify variables are set
echo $STUDENTVUE_PORTAL
echo $STUDENTVUE_USERNAME

# Load .env file if not automatically loaded
export $(cat .env | xargs)
```

## Security Best Practices

1. Never commit your `.env` file to version control
2. Add `.env` to `.gitignore` (already configured)
3. Use environment-specific configurations for different deployments
4. Consider using a secrets manager for production deployments
5. Rotate passwords regularly

## Next Steps

- Read the [API Reference](README.md#api-reference) for available endpoints
- Check the [examples](examples/) directory for usage patterns
- Run the test suite to explore functionality: `cargo test`

