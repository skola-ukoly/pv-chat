use eframe::egui;
use regex::Regex;
use std::net::UdpSocket;

/// Describes what page ui is in and thus what should be displayed
enum Page {
    Login,
    Chat,
}

/// stores the main state of the app
pub struct ChatApp {
    pub messages: Vec<String>,
    pub socket: UdpSocket,
    chat_input: String,
    username: String,
    server_addr: String,
    server_port: String,
    ui_state: UiState,
}

impl eframe::App for ChatState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state {
            Login => self.login_ui(ui),
            Chat => self.chat_ui(ui),
        });
    }
}

impl ChatState {
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            messages: Vec::new(),
            socket,
            username: String::new(),
            server_addr: String::new(),
            server_port: String::new(),
            chat_input: String::new(),
            ui_state: UiState::Login,
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
                }
                if !check_port(&self.server_port) {
                    return;
                }
            }
        });
    }

    /// Describes the chat Ui
    fn chat_ui(&mut self, ui: &mut egui::Ui) {}
}

/// checks if a string is valid IPv4 address
fn check_addr(addr: &str) -> bool {
    let addr_pattern = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();

    if addr_pattern.is_match(addr) {
        true
    } else {
        false
    }
}

/// checks if string is a valid port
fn check_port(port: &str) -> bool {
    match port.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
