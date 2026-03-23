use eframe::{egui, App, Frame};
use std::time::Instant;

mod doh_manager;
use doh_manager::{Browser, DohStatus, get_doh_status, set_doh_status};

struct DohToggleApp {
    chrome_status: DohStatus,
    edge_status: DohStatus,
    last_check_time: Instant,
    message: Option<String>,
    is_loading: bool,
}

impl Default for DohToggleApp {
    fn default() -> Self {
        Self {
            chrome_status: DohStatus::Unknown,
            edge_status: DohStatus::Unknown,
            last_check_time: Instant::now(),
            message: None,
            is_loading: false,
        }
    }
}

impl App for DohToggleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 每500毫秒检查一次状态
        if self.last_check_time.elapsed().as_millis() >= 500 {
            self.chrome_status = get_doh_status(Browser::Chrome);
            self.edge_status = get_doh_status(Browser::Edge);
            self.last_check_time = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // 提示信息
            ui.horizontal(|ui| {
                ui.label("关闭DOH会增强国外网络的使用流畅度");
            });
            ui.separator();

            // 浏览器状态显示区
            ui.heading("浏览器DOH状态");
            ui.horizontal(|ui| {
                ui.label("谷歌浏览器(Google Chrome):");
                match self.chrome_status {
                    DohStatus::Enabled => ui.label(egui::RichText::new("已开启").color(egui::Color32::GREEN)),
                    DohStatus::Disabled => ui.label(egui::RichText::new("已关闭").color(egui::Color32::RED)),
                    DohStatus::Unknown => ui.label(egui::RichText::new("未知").color(egui::Color32::YELLOW)),
                };
            });
            ui.horizontal(|ui| {
                ui.label("微软Edge浏览器:");
                match self.edge_status {
                    DohStatus::Enabled => ui.label(egui::RichText::new("已开启").color(egui::Color32::GREEN)),
                    DohStatus::Disabled => ui.label(egui::RichText::new("已关闭").color(egui::Color32::RED)),
                    DohStatus::Unknown => ui.label(egui::RichText::new("未知").color(egui::Color32::YELLOW)),
                };
            });
            ui.separator();

            // 独立控制区
            ui.heading("独立控制");
            ui.horizontal(|ui| {
                ui.label("谷歌浏览器:");
                if self.chrome_status == DohStatus::Enabled {
                    if ui.button("关闭").clicked() {
                        self.is_loading = true;
                        match set_doh_status(Browser::Chrome, false) {
                            Ok(_) => {
                                self.message = Some("谷歌浏览器DOH已关闭".to_string());
                                self.chrome_status = DohStatus::Disabled;
                            },
                            Err(e) => {
                                self.message = Some(format!("操作失败: {}", e));
                            },
                        }
                        self.is_loading = false;
                    }
                } else {
                    if ui.button("开启").clicked() {
                        self.is_loading = true;
                        match set_doh_status(Browser::Chrome, true) {
                            Ok(_) => {
                                self.message = Some("谷歌浏览器DOH已开启".to_string());
                                self.chrome_status = DohStatus::Enabled;
                            },
                            Err(e) => {
                                self.message = Some(format!("操作失败: {}", e));
                            },
                        }
                        self.is_loading = false;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("微软Edge浏览器:");
                if self.edge_status == DohStatus::Enabled {
                    if ui.button("关闭").clicked() {
                        self.is_loading = true;
                        match set_doh_status(Browser::Edge, false) {
                            Ok(_) => {
                                self.message = Some("Edge浏览器DOH已关闭".to_string());
                                self.edge_status = DohStatus::Disabled;
                            },
                            Err(e) => {
                                self.message = Some(format!("操作失败: {}", e));
                            },
                        }
                        self.is_loading = false;
                    }
                } else {
                    if ui.button("开启").clicked() {
                        self.is_loading = true;
                        match set_doh_status(Browser::Edge, true) {
                            Ok(_) => {
                                self.message = Some("Edge浏览器DOH已开启".to_string());
                                self.edge_status = DohStatus::Enabled;
                            },
                            Err(e) => {
                                self.message = Some(format!("操作失败: {}", e));
                            },
                        }
                        self.is_loading = false;
                    }
                }
            });
            ui.separator();

            // 批量控制区
            ui.heading("批量操作");
            ui.horizontal(|ui| {
                if ui.button("一键全开").clicked() {
                    self.is_loading = true;
                    let chrome_result = set_doh_status(Browser::Chrome, true);
                    let edge_result = set_doh_status(Browser::Edge, true);
                    
                    match (chrome_result, edge_result) {
                        (Ok(_), Ok(_)) => {
                            self.message = Some("所有浏览器DOH已开启".to_string());
                            self.chrome_status = DohStatus::Enabled;
                            self.edge_status = DohStatus::Enabled;
                        },
                        (Err(e), Err(_)) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                        (Err(e), _) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                        (_, Err(e)) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                    }
                    self.is_loading = false;
                }
                
                if ui.button("一键全关").clicked() {
                    self.is_loading = true;
                    let chrome_result = set_doh_status(Browser::Chrome, false);
                    let edge_result = set_doh_status(Browser::Edge, false);
                    
                    match (chrome_result, edge_result) {
                        (Ok(_), Ok(_)) => {
                            self.message = Some("所有浏览器DOH已关闭".to_string());
                            self.chrome_status = DohStatus::Disabled;
                            self.edge_status = DohStatus::Disabled;
                        },
                        (Err(e), Err(_)) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                        (Err(e), _) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                        (_, Err(e)) => {
                            self.message = Some(format!("操作失败: {}", e));
                        },
                    }
                    self.is_loading = false;
                }
            });
            ui.separator();

            // 操作结果提示
            if let Some(message) = &self.message {
                ui.label(egui::RichText::new(message).color(egui::Color32::BLUE));
            }

            // 加载状态
            if self.is_loading {
                ui.label("操作中...");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 400.0)),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "一键开关DOH",
        options,
        Box::new(|_cc| Box::new(DohToggleApp::default())),
    ).unwrap();
}
