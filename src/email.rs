//! Send email

use crate::types::*;
use lettre::sendmail::SendmailTransport;
use lettre::Transport;
use lettre_email::{EmailBuilder, MimeMultipartType, PartBuilder};

/// Dispatch the provided HTML email
pub fn send(config: &Config, html: String) -> Result<(), Box<dyn std::error::Error>> {
    // Dynamic fields other than html body
    let plaintext = format!(
        "New {} photos are available at {}",
        config.album_name,
        config.album_id.url()
    );
    let subject = format!("New {} photos", config.album_name);

    // Construct email
    let email =
        // We need to fold over the vector of recipients, to update the builder
        // value with each one...
        (config.recipient_email_addrs)
        .iter()
        .fold(EmailBuilder::new(), |builder, recipient| {
            builder.to(recipient.clone())
        })
        // ...then add the other scalar fields normally...
        .from((config.sender_email_addr.clone(), config.sender_email_name.clone()))
        .bcc(config.sender_email_addr.clone())
        .subject(subject)
        .alternative_body(html, plaintext)
        // ...aaand build.
        .build()?;

    // Send it
    SendmailTransport::new_with_command(&config.sendmail_path)
        .send(email.into())
        .map_err(|e| e.into())
}

/// Encode an email as alternaive text/plain and text/html, but with a
/// content-transfer-encoding of quoted-printable for the html, due to RFC
/// 5322's maximum line limit of 998 characters excluding CRLF (and
/// recommendation of 78 characters).
///
/// This is copied and modified from lettre_email::alternative().
trait AddAlt {
    fn alternative_body(self, body_html: String, body_text: String) -> EmailBuilder;
}

impl AddAlt for EmailBuilder {
    fn alternative_body(self, body_html: String, body_text: String) -> EmailBuilder {
        let text = PartBuilder::new()
            .body(body_text)
            .header(("Content-Type", mime::TEXT_PLAIN_UTF_8.to_string()))
            .build();

        let html = PartBuilder::new()
            .body(quoted_printable::encode_to_str(body_html))
            .header(("Content-Type", mime::TEXT_HTML_UTF_8.to_string()))
            .header(("Content-Transfer-Encoding", "quoted-printable"))
            .build();

        let alternate = PartBuilder::new()
            .message_type(MimeMultipartType::Alternative)
            .child(text)
            .child(html);

        self.message_type(MimeMultipartType::Mixed)
            .child(alternate.build())
    }
}
