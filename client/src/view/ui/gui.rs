use client::{check_port, check_addr};
use crate::controller::Controller;
use crate::types::Message;
use eframe::{egui::{self, TextStyle, ScrollArea, Ui}, epaint::Color32};

const ALERT_COLOR: Color32 = Color32::from_rgb(255, 69, 58);



enum Page {
    Login,
    Chat,
}


/// stores the main state of the app
pub struct ChatApp {
    page: Page,
    login: LoginPage,
    chat: ChatPage,
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            page: Page::Login,
            login: LoginPage {
                path: None,
                username: String::new(),
                server_addr: String::new(),
                server_port: String::new(),
            },
            chat: ChatPage {
                chat_input: String::new(),
                messages: Vec::new(),
            },
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let page = match &self.page {
                Page::Login => self.login.login_ui(ui),
                Page::Chat => self.chat.chat_ui(ui),
            };
            self.page = page;
        });
    }
}



struct LoginPage {
    username: String,
    server_addr: String,
    server_port: String,
    path: Option<String>,
}

impl LoginPage {
    /// Describes the Login Ui
    fn login_ui(&mut self, ui: &mut egui::Ui) -> Page {
        let mut return_page = Page::Login;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let server_addr_label = ui.label("server address:");
                ui.text_edit_singleline(&mut self.server_addr)
                    .labelled_by(server_addr_label.id);
            });
            ui.horizontal(|ui| {
                let server_port_label = ui.label("server port:");
                ui.text_edit_singleline(&mut self.server_port)
                    .labelled_by(server_port_label.id);
            });
            ui.horizontal(|ui| {
                let username_label = ui.label("username:");
                ui.text_edit_singleline(&mut self.username)
                    .labelled_by(username_label.id);
            });
            ui.horizontal(|ui| {
                let otp_label = ui.label("One timne pad:");
                if ui.button("choose file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path = Some(path.display().to_string());
                    };
                };
            });

            // user wants to connect to server
            if ui.button("Connect!").clicked() {
                if !check_addr(&self.server_addr) {
                    println!("invalid addr");
                    return; 
                };
                if !check_port(&self.server_port) {
                    println!("invalid port");
                    return;
                };

                return_page = Page::Chat;
            };


        });

        return_page
    }
}

struct ChatPage {
    messages: Vec<Message>,
    chat_input: String,
}

impl ChatPage {
    /// Describes the chat Ui
    fn chat_ui(&mut self, ui: &mut egui::Ui) -> Page {
        let mut return_page = Page::Chat;

        ui.vertical(|ui| {
            if ui.button("Disconnect!").clicked() {
                return_page = Page::Login;
                return;
            };

            ui.add_space(8.0);

            ui.separator();

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().stick_to_bottom(true).show_rows(
                ui,
                row_height,
                self.messages.len(),
                |ui, row_range| {
                    for row in row_range {
                        ui.label(format!("{}> {}", self.messages[row].sender, self.messages[row].body));
                    }
                },
            );

            ui.separator(); 

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.text_edit_singleline(&mut self.chat_input);
                    if ui.button("Send!").clicked() {
                        let new_message = self.chat_input.clone();
                        self.chat_input.truncate(0);
                    };
                });
            });
        });

        return_page
    }
}



