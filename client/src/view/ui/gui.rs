use client::{check_port, check_addr};
use crate::controller::Controller;
use eframe::{egui::{self, TextStyle, ScrollArea, Ui}, epaint::Color32};

const ALERT_COLOR: Color32 = Color32::from_rgb(255, 69, 58);

/// Describes what page ui is in and thus what should be displayed
pub enum Page {
    Login,
    Chat(Controller),
}



/// stores the main state of the app
pub struct ChatApp {
//    pub controller: Controller,
    pub messages: Vec<String>,
    chat_input: String,
    username: String,
    server_addr: String,
    server_port: String,
    ui_state: Page,
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state {
            Page::Login => self.login_ui(ui),
            Page::Chat(controller) => self.chat_ui(ui, &controller),
        });
    }
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            username: String::new(),
            server_addr: String::new(),
            server_port: String::new(),
            chat_input: String::new(),
            ui_state: Page::Login,
        }
    }

    /// Describes the Login Ui
    fn login_ui(&mut self, ui: &mut egui::Ui) {
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

            // user wants to connect to server
            if ui.button("Connect!").clicked() {
                if !check_addr(&self.server_addr) {
                    return;
                };
                if !check_port(&self.server_port) {
                    return;
                };

                // chat is free to start, got: server_addr, server_port, username, one_time_pad
                
                
            }
        });
    }

    /// Describes the chat Ui
    fn chat_ui(&mut self, ui: &mut egui::Ui, controller: &Controller) {
        ui.vertical(|ui| {
            if ui.button("Disconnect!").clicked() {
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
                        ui.label(self.messages[row].clone());
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
    }

    fn alert(ui: &mut Ui, content: String) {


    }
}

