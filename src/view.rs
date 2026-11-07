use crate::login::{LoginResult, login};
use iced::widget::operation::{self, focus};
use iced::widget::{Id, button, column, container, text, text_input};
use iced::{Element, Length, Task};

const USERNAME_INP_ID: Id = Id::new("Username");
const PASSWORD_INP_ID: Id = Id::new("Password");

#[derive(Default)]
pub struct MigreetApp {
    username: String,
    password: String,
    info: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    UsernameChanged(String),
    UsernameSubmitted,
    PasswordChanged(String),
    PasswordSubmitted,
    Submit,
}

impl MigreetApp {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), operation::focus(USERNAME_INP_ID))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UsernameChanged(content) => {
                self.username = content;
                Task::none()
            }
            Message::UsernameSubmitted => focus(PASSWORD_INP_ID),
            Message::PasswordChanged(content) => {
                self.password = content;
                Task::none()
            }
            Message::PasswordSubmitted | Message::Submit => {
                let command = String::from("dbus-run-session niri --session"); // FIXME
                let _: Task<Message> =
                    match login(self.username.as_str(), self.password.as_str(), &command) {
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
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let username_inp: Element<Message> = text_input("Username", &self.username)
            .id(USERNAME_INP_ID)
            .on_input(Message::UsernameChanged)
            .on_submit(Message::UsernameSubmitted)
            .into();

        let password_inp: Element<Message> = text_input("Password", &self.password)
            .id(PASSWORD_INP_ID)
            .secure(true)
            .on_input(Message::PasswordChanged)
            .on_submit(Message::PasswordSubmitted)
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
