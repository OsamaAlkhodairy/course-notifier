use std::fs;

use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

/// Assumes email_address is a gmail account
pub fn create_mailer(email_address: String) -> SmtpTransport {
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    let creds = Credentials::new(email_address.to_string(), password);

    SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build()
}

pub fn send_email(mailer: &SmtpTransport, course_name: &str) {
    println!("Sending update email for {}", course_name);

    let from_email = fs::read_to_string("from_email.txt").unwrap();
    let to_email = fs::read_to_string("to_email.txt").unwrap();
    let email = Message::builder()
        .from(from_email.parse::<Mailbox>().unwrap())
        .to(to_email.parse::<Mailbox>().unwrap())
        .subject("Course Availability Update")
        .body(format!(
            "There has been an update to {}'s availability",
            course_name
        ))
        .unwrap();

    mailer.send(&email).expect("Unable to send email");
}
