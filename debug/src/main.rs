use eframe::egui;
use egui::{DragValue, Rect, Sense, Vec2, emath::RectTransform, global_theme_preference_buttons};
use egui_snarl::Snarl;
use nodes::{Node, Renderable, show_nodes};

mod nodes;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(AthenaVisualize::new(cc)))),
    )
}

struct AthenaVisualize {
    snarl: Snarl<Node>,
    show: Vec<Renderable>,
    view_scale: f32,
    view_offset: Vec2,
}

impl AthenaVisualize {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        AthenaVisualize {
            snarl: Snarl::new(),
            show: Vec::new(),
            view_scale: 1.0,
            view_offset: Vec2::ZERO,
        }
    }
}

impl eframe::App for AthenaVisualize {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            global_theme_preference_buttons(ui);
        });

        let size = ctx.viewport(|v| v.this_pass.available_rect);

        egui::TopBottomPanel::top("Graph")
            .exact_height(size.height() / 2.0)
            .show(ctx, |ui| {
                ui.label("This is the graph area");
                self.show = show_nodes(&mut self.snarl, ui);
            });

        egui::SidePanel::left("View Config").show(ctx, |ui| {
            let s = self.view_scale.abs().max(1.0) * 0.001;
            ui.add(
                DragValue::new(&mut self.view_scale)
                    .speed(s)
                    .range(0.001..=10.0)
                    .prefix("scale: "),
            );
            let s = self.view_offset.x.abs().max(1.0) * 0.001;
            ui.add(
                DragValue::new(&mut self.view_offset.x)
                    .speed(s)
                    .prefix("x: "),
            );
            let s = self.view_offset.y.abs().max(1.0) * 0.001;
            ui.add(
                DragValue::new(&mut self.view_offset.y)
                    .speed(s)
                    .prefix("y: "),
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let (rect, _response) = ui.allocate_exact_size(size, Sense::empty());

            let painter = ui.painter_at(rect);

            let ar = rect.aspect_ratio();

            for renderable in &self.show {
                renderable.draw(
                    &painter,
                    RectTransform::from_to(
                        Rect::from_min_max(
                            egui::pos2(-self.view_scale * ar, -self.view_scale) + self.view_offset,
                            egui::pos2(self.view_scale * ar, self.view_scale) + self.view_offset,
                        ),
                        rect,
                    ),
                );
            }
        });
    }
}
