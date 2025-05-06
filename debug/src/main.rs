use athena::{Matrix, Matrix4, Vector2, Vector4};
use eframe::egui;
use egui::{
    Color32, DragValue, Rect, Sense, Shape, Vec2, emath::RectTransform,
    global_theme_preference_buttons,
};
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

            let camera = camera(size.x / size.y, 70.0_f32.to_radians());

            let painter = ui.painter_at(rect);

            let rect_transform = RectTransform::from_to(
                Rect::from_min_max(egui::pos2(-1.0, -1.0), egui::pos2(1.0, 1.0)),
                rect,
            );

            for i in -10..=10 {
                let from = Vector4::new((i as f32) * 0.1, -1.0, 0.0, 1.0);
                let to = Vector4::new((i as f32) * 0.1, 1.0, 0.0, 1.0);

                let from2 = camera * from;
                let to2 = camera * to;

                let from3 =
                    rect_transform.transform_pos(egui::pos2(from2.x / from2.w, from2.y / from2.w));
                let to3 = rect_transform.transform_pos(egui::pos2(to2.x / to2.w, to2.y / to2.w));

                painter.add(Shape::line_segment(
                    [from3, to3],
                    (1.0, Color32::from_rgba_premultiplied(100, 100, 100, 100)),
                ));
            }

            for i in -10..=10 {
                let from = Vector4::new(-1.0, (i as f32) * 0.1, 0.0, 1.0);
                let to = Vector4::new(1.0, (i as f32) * 0.1, 0.0, 1.0);

                let from2 = camera * from;
                let to2 = camera * to;

                let from3 =
                    rect_transform.transform_pos(egui::pos2(from2.x / from2.w, from2.y / from2.w));
                let to3 = rect_transform.transform_pos(egui::pos2(to2.x / to2.w, to2.y / to2.w));

                painter.add(Shape::line_segment(
                    [from3, to3],
                    (1.0, Color32::from_rgba_premultiplied(100, 100, 100, 100)),
                ));
            }

            let ar = rect.aspect_ratio();

            for renderable in &self.show {
                renderable.draw(&painter, &camera, rect_transform);
            }
        });
    }
}

fn camera(aspect: f32, fov: f32) -> Matrix4<f32> {
    let near = 0.1;
    let far = 100.0;

    let f = 1.0 / (fov / 2.0).tan();

    let perspective = Matrix4::from_column_arrays([
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, far / (far - near), 1.0],
        [0.0, 0.0, -far * near / (far - near), 0.0],
    ]);

    let translate = Matrix4::from_column_arrays([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.5, 2.0, 1.0],
    ]);

    translate * perspective
}
