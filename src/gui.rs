use aviutl2_eframe::{eframe, egui, AviUtl2EframeHandle};

pub struct CalcApp {
    _handle: AviUtl2EframeHandle,
    pub display: String,
    pub equation: String,
    pub last_val: f64,
    pub operator: Option<String>,
    pub waiting_for_next: bool,
    backspace_icon: Option<egui::TextureHandle>,
}

impl CalcApp {
    pub fn new(cc: &eframe::CreationContext<'_>, handle: AviUtl2EframeHandle) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "meiryo".to_owned(),
            egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\meiryo.ttc")).into(),
        );
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "meiryo".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let icon_data = include_bytes!("icon/backspace.png");
        let backspace_icon = match image::load_from_memory(icon_data) {
            Ok(img) => {
                let size = [img.width() as _, img.height() as _];
                let pixels = img.to_rgba8().into_raw();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                Some(cc.egui_ctx.load_texture("backspace_icon", color_image, Default::default()))
            }
            Err(_) => None,
        };

        cc.egui_ctx.all_styles_mut(|style| {
            style.visuals = aviutl2_eframe::aviutl2_visuals();
            style.spacing.item_spacing = egui::vec2(2.0, 2.0);
        });

        Self {
            _handle: handle,
            display: "0".to_string(),
            equation: String::new(),
            last_val: 0.0,
            operator: None,
            waiting_for_next: false,
            backspace_icon,
        }
    }

    fn add_digit(&mut self, digit: &str) {
        if self.display == "0" || self.waiting_for_next {
            self.display = digit.to_string();
            self.waiting_for_next = false;
        } else {
            self.display.push_str(digit);
        }
    }

    fn set_operator(&mut self, op: &str) {
        if self.operator.is_some() && !self.waiting_for_next {
            self.calculate_internal();
        }

        let current: f64 = self.display.parse().unwrap_or(0.0);
        self.last_val = current;
        self.operator = Some(op.to_string());
        self.equation = format!("{} {}", current, op);
        self.waiting_for_next = true;
    }

    fn calculate(&mut self) {
        if let Some(op) = self.operator.clone() {
            let current: f64 = self.display.parse().unwrap_or(0.0);
            let prev_val = self.last_val; 

            self.calculate_internal();

            self.equation = format!("{} {} {} =", prev_val, op, current);
            self.operator = None;
        }
    }

    fn calculate_internal(&mut self) {
        if let Some(ref op) = self.operator {
            let current: f64 = self.display.parse().unwrap_or(0.0);
            let result = match op.as_str() {
                "+" => self.last_val + current,
                "-" => self.last_val - current,
                "×" => self.last_val * current,
                "÷" => if current != 0.0 { self.last_val / current } else { 0.0 },
                _ => current,
            };
            
            self.display = result.to_string();
            self.last_val = result;
            self.waiting_for_next = true;
        }
    }

    fn calc_button(&self, ui: &mut egui::Ui, size: egui::Vec2, label: &str, text_size: f32) -> egui::Response {
        ui.add_sized(size, egui::Button::new(egui::RichText::new(label).size(text_size)))
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(260.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(&self.equation).size(16.0).weak());
                    ui.heading(egui::RichText::new(&self.display).size(48.0).strong());
                    ui.add_space(10.0);
                });

                let btn_size = egui::vec2(62.0, 52.0);
                egui::Grid::new("calc_grid").spacing(egui::vec2(3.0, 3.0)).show(ui, |ui| {
                    if self.calc_button(ui, btn_size, "%", 18.0).clicked() {
                        if let Ok(val) = self.display.parse::<f64>() { self.display = (val / 100.0).to_string(); }
                    }
                    if self.calc_button(ui, btn_size, "CE", 18.0).clicked() { self.display = "0".to_string(); }
                    if self.calc_button(ui, btn_size, "C", 18.0).clicked() { 
                        self.display = "0".to_string(); self.equation.clear(); self.last_val = 0.0; self.operator = None; 
                    }
                    
                    let bs_btn = if let Some(ref icon) = self.backspace_icon {
                        ui.add_sized(btn_size, egui::Button::image(egui::Image::new(icon).max_size(egui::vec2(22.0, 22.0))))
                    } else {
                        self.calc_button(ui, btn_size, "←", 18.0)
                    };
                    if bs_btn.clicked() {
                        if !self.waiting_for_next && !self.display.is_empty() && self.display != "0" {
                            self.display.pop();
                            if self.display.is_empty() || self.display == "-" { self.display = "0".to_string(); }
                        }
                    }
                    ui.end_row();

                    let layout = [["7", "8", "9", "÷"], ["4", "5", "6", "×"], ["1", "2", "3", "-"]];
                    for row in layout {
                        for (i, label) in row.iter().enumerate() {
                            let is_op = i == 3;
                            if self.calc_button(ui, btn_size, label, if is_op { 24.0 } else { 20.0 }).clicked() {
                                if is_op { self.set_operator(label); } else { self.add_digit(label); }
                            }
                        }
                        ui.end_row();
                    }

                    if self.calc_button(ui, btn_size, "±", 18.0).clicked() {
                        if let Ok(val) = self.display.parse::<f64>() { self.display = (-val).to_string(); }
                    }
                    if self.calc_button(ui, btn_size, "0", 20.0).clicked() { self.add_digit("0"); }
                    if self.calc_button(ui, btn_size, ".", 20.0).clicked() { 
                        if self.waiting_for_next { self.display = "0.".to_string(); self.waiting_for_next = false; }
                        else if !self.display.contains('.') { self.display.push('.'); } 
                    }
                    if self.calc_button(ui, btn_size, "+", 24.0).clicked() { self.set_operator("+"); }
                    ui.end_row();

                    ui.label(""); ui.label(""); ui.label("");
                    let accent = ctx.style().visuals.selection.bg_fill;
                    if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new("=").size(26.0)).fill(accent)).clicked() {
                        self.calculate();
                    }
                    ui.end_row();
                });
            });
        });
    }
}