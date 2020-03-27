pub mod sender_mail {
    // --------------- //
    use lettre_email::Email;
    use lettre::smtp::authentication::Credentials;
    use lettre::{SmtpClient, Transport};
    use lettre_email::{
        Mailbox,
    };
    use crate::mail::models::mod_sender::{
        Sender,
        ReceiveItem,
        EmailFails,
    };
    // --------------- //

    pub fn send_mail (sender:  &mut Sender, receive: ReceiveItem, i: usize) {
        let email = Email::builder()
            .to(
                Mailbox::new_with_name(
                    receive.name.clone(),
                    receive.email.clone()
                )
            )
            .from(
                Mailbox::new_with_name(
                    sender.from.name.clone(),
                    sender.from.email.clone()
                )
            )
            .subject(
                sender.mail.subject.clone()
            )
            .html(
                sender.mail.template.clone()
            )
            .build()
            .unwrap();

        let creds = Credentials::new(
            sender.from.email.clone(),
            sender.from.password.clone(),
        );

        let mut mailer = SmtpClient::new_simple(&sender.from.host)
            .unwrap()
            .credentials(creds)
            .transport();

        let result = mailer.send(email.into());

        if result.is_ok() {
            sender.to[i].sended = true;
        } else {
            sender.to[i].fails = EmailFails {
                tries  : sender.to[i].fails.tries + 1,
                message: format!("{:?}", result),
            };
        }
    }
}
