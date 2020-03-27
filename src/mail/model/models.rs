pub mod mod_sender {

    #[derive(Debug, Deserialize, Serialize, Clone, Default)]
    pub struct EmailFails {
        pub tries  : u32,
        pub message: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ReceiveItem {
        pub name  : String,
        pub email : String,
        pub fails : EmailFails,
        pub sended: bool,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct EMailService {
        pub host    : String,
        pub name    : String,
        pub email   : String,
        pub password: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct EMailTemplate {
        pub subject : String,
        pub template: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Sender {
        pub from  : EMailService,
        pub to    : Vec<ReceiveItem>,
        pub mail  : EMailTemplate
    }
}
