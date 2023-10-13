use super::web_app::WebAppInfo;

pub enum MenuButton {
    Commands {
        r#type: String,
    },
    WebApp {
        r#type: String,
        text: String,
        web_app: WebAppInfo,
    },
    Default {
        r#type: String,
    },
}
