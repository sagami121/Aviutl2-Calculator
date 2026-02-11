use aviutl2::AnyResult;
use aviutl2::generic::*;
use aviutl2_eframe::EframeWindow;

mod gui;

#[aviutl2::plugin(GenericPlugin)]
pub struct CalcPlugin {
    window: Option<EframeWindow>,
}

impl GenericPlugin for CalcPlugin {
    fn new(_info: aviutl2::AviUtl2Info) -> AnyResult<Self> {
        Ok(Self { window: None })
    }

    fn register(&mut self, host: &mut HostAppHandle) {
        host.set_plugin_information("Rust製電卓プラグイン v0.2");

        let name = "電卓";
        let eframe_window = EframeWindow::new(name, |cc, handle| {
            Ok(Box::new(gui::CalcApp::new(cc, handle)))
        });

        if let Ok(w) = eframe_window {
            let _ = host.register_window_client(name, &w);
            self.window = Some(w);
        }
    }
}

aviutl2::register_generic_plugin!(CalcPlugin);