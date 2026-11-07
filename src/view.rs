use crate::login::{LoginResult, login2};
use iced::widget::{button, column, container, text, text_input};
use iced::{Element, Length, Task};

#[derive(Default)]
pub struct MigreetApp {
    username: String,
    password: String,
    info: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
}

impl MigreetApp {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UsernameChanged(content) => {
                self.username = content;
            }
            Message::PasswordChanged(content) => {
                self.password = content;
            }
            Message::Submit => {
                let command = String::from("dbus-run-session niri --session");
                let _: Task<Message> =
                    match login2(self.username.as_str(), self.password.as_str(), &command) {
                        Ok(LoginResult::Success) => iced::exit(),
                        Ok(LoginResult::Failure) => {
                            eprintln!("LoginResult Failed");
                            self.info = "LoginResult Failed".to_string();
                            Task::none()
                        }
                        Err(e) => {
                            eprintln!("login error: {e}");
                            self.info = e.to_string();
                            Task::none()
                        }
                    };
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let username_inp: Element<Message> = text_input("Username", &self.username)
            .on_input(Message::UsernameChanged)
            .into();

        let password_inp: Element<Message> = text_input("Password", &self.password)
            .on_input(Message::PasswordChanged)
            .into();

        let submit_btn = button("submit").on_press(Message::Submit);
        let info = text(&self.info);

        let form = column![username_inp, password_inp, submit_btn, info].spacing(12);

        let form_card = container(form)
            .width(Length::Fill)
            .max_width(320)
            .padding(20)
            .center_x(Length::Fill);

        // Outer container: fills the window and centers the card both ways.
        container(form_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
