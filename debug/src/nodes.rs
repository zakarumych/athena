use std::time::Duration;

use athena::*;
use egui::{Color32, DragValue, Painter, Shape, Ui, emath::RectTransform};
use egui_snarl::{
    InPin, NodeId, OutPin, Snarl,
    ui::{PinWireInfo, SnarlPin, SnarlViewer, SnarlWidget, WireStyle},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Type {
    Nil,
    Scalar,
    Point2,
    Line2,
    Motor2,
    Point3,
    Line3,
}

impl Type {
    fn name(&self) -> &'static str {
        match self {
            Type::Nil => "Nil",
            Type::Scalar => "Scalar",
            Type::Point2 => "Point2",
            Type::Line2 => "Line2",
            Type::Motor2 => "Motor2",
            Type::Point3 => "Point3",
            Type::Line3 => "Line3",
        }
    }
}

impl Type {
    fn pin_color(&self) -> Color32 {
        match self {
            Type::Nil => Color32::WHITE,
            Type::Scalar => Color32::GRAY,
            Type::Point2 => Color32::RED,
            Type::Line2 => Color32::GREEN,
            Type::Motor2 => Color32::BLUE,
            Type::Point3 => Color32::YELLOW,
            Type::Line3 => Color32::PURPLE,
        }
    }
}

/// Geometric object.
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Nil,
    Scalar(f32),
    Point2(Point2<f32>),
    Line2(Line2<f32>),
    Motor2(Motor2<f32>),
    Point3(Point3<f32>),
    Line3(Line3<f32>),
}

impl Value {
    fn ty(&self) -> Type {
        match self {
            Value::Nil => Type::Nil,
            Value::Scalar(_) => Type::Scalar,
            Value::Point2(_) => Type::Point2,
            Value::Line2(_) => Type::Line2,
            Value::Motor2(_) => Type::Motor2,
            Value::Point3(_) => Type::Point3,
            Value::Line3(_) => Type::Line3,
        }
    }

    fn show(&self, ui: &mut Ui) {
        match self {
            Value::Nil => {
                ui.label("Nil");
            }
            Value::Scalar(scalar) => {
                ui.label(format!("Scalar: {}", *scalar));
            }
            Value::Point2(point) if point.is_ideal() => {
                let (x, y) = point.coords();
                ui.label(format!("Point2 at infinity: ({}, {})", x, y));
            }
            Value::Point2(point) => {
                let (x, y) = point.normalized().coords();
                ui.label(format!("Point2: ({}, {})", x, y));
            }
            Value::Line2(line) => {
                let (a, b, c) = line.abc();
                ui.label(format!("{a}x + {b}y + {c} = 0"));
            }
            Value::Motor2 { .. } => {
                ui.label("Motor2");
            }
            Value::Point3(point) if point.is_ideal() => {
                let (x, y, z) = point.coords();
                ui.label(format!("Point3 at infinity: ({}, {}, {})", x, y, z));
            }
            Value::Point3(point) => {
                let (x, y, z) = point.coords();
                ui.label(format!("Point3: ({}, {}, {})", x, y, z));
            }
            Value::Line3(line) => {
                ui.label("Line3");
            }
        }
    }

    pub fn draw(
        &self,
        scale: f32,
        color: Color32,
        painter: &Painter,
        camera: &Matrix4<f32>,
        tr: RectTransform,
    ) {
        match self {
            Value::Nil => {}
            Value::Scalar(_) => {}
            Value::Point2(point) => {
                if point.is_ideal() {
                    return;
                }
                let (x, y) = point.normalized().coords();
                let pos = egui::pos2(x, y);
                let pos = tr.transform_pos(pos);
                painter.add(Shape::circle_filled(pos, scale, color));
            }
            Value::Line2(line) => {
                if !line.is_ideal() {
                    let (a, b, c) = line.abc();

                    let by = tr.from().bottom();
                    let bx = (b * by + c) / -a;
                    let bp = egui::pos2(bx, by);

                    let ty = tr.from().top();
                    let tx = (b * ty + c) / -a;
                    let tp = egui::pos2(tx, ty);

                    let lx = tr.from().left();
                    let ly = (a * lx + c) / -b;
                    let lp = egui::pos2(lx, ly);

                    let rx = tr.from().right();
                    let ry = (a * rx + c) / -b;
                    let rp = egui::pos2(rx, ry);

                    let p0 = match (bx > lx, bx < rx) {
                        (true, true) => bp,
                        (false, _) => lp,
                        (_, false) => rp,
                    };

                    let p1 = match (tx > lx, tx < rx) {
                        (true, true) => tp,
                        (false, _) => lp,
                        (_, false) => rp,
                    };

                    if p0 != p1 {
                        let p0 = tr.transform_pos(p0);
                        let p1 = tr.transform_pos(p1);

                        painter.add(Shape::line_segment([p0, p1], (scale, color)));
                    }
                } else {
                }
            }
            Value::Motor2 { .. } => {
                todo!()
            }
            Value::Point3(point) => {
                if point.is_ideal() {
                    return;
                }

                let (x, y, z) = point.coords();
                let v = *camera * Vector4::new(x, y, z, 1.0);
                let pos = egui::pos2(v.x / v.w, v.y / v.w);
                let pos = tr.transform_pos(pos);
                painter.add(Shape::circle_filled(pos, scale, color));
            }
        }
    }

    fn apply(&self, geom: &Value) -> Value {
        match (self, geom) {
            (Value::Nil, _) => Value::Nil,
            (Value::Line2(transform), Value::Point2(point)) => {
                Value::Point2(transform.reflect_point(*point))
            }
            (Value::Line2(transform), Value::Line2(line)) => {
                Value::Line2(transform.reflect_line(*line))
            }
            (Value::Point2(transform), Value::Point2(point)) => {
                Value::Point2(transform.reflect_point(*point))
            }
            (Value::Point2(transform), Value::Line2(line)) => {
                Value::Line2(transform.reflect_line(*line))
            }
            (Value::Motor2(motor), Value::Point2(point)) => Value::Point2(motor.move_point(*point)),
            (Value::Motor2(motor), Value::Line2(line)) => Value::Line2(motor.move_line(*line)),
            _ => Value::Nil,
        }
    }

    fn meet(&self, geom: &Value) -> Value {
        match (self, geom) {
            (Value::Line2(a), Value::Line2(b)) => Value::Point2(a.meet(*b)),
            _ => Value::Nil,
        }
    }

    fn join(&self, geom: &Value) -> Value {
        match (self, geom) {
            (Value::Point2(a), Value::Point2(b)) => Value::Line2(a.join(*b)),
            _ => Value::Nil,
        }
    }

    fn project(&self, geom: &Value) -> Value {
        match (self, geom) {
            (Value::Point2(a), Value::Line2(b)) => Value::Point2(a.project_to(*b)),
            (Value::Line2(a), Value::Point2(b)) => Value::Line2(a.project_to(*b)),
            _ => Value::Nil,
        }
    }
}

enum CtorNode {
    Scalar {
        scalar: f32,
    },
    Point2 {
        point: Point2<f32>,
    },
    Line2 {
        line: Line2<f32>,
    },
    PointPointMotor2 {
        mul: f32,
        a: Point2<f32>,
        b: Point2<f32>,
        motor: Motor2<f32>,
    },
    LineLineMotor2 {
        mul: f32,
        a: Line2<f32>,
        b: Line2<f32>,
        motor: Motor2<f32>,
    },
    ReconstructMotor2 {
        mul: f32,
        a: [Point2<f32>; 2],
        b: [Point2<f32>; 2],
        motor: Motor2<f32>,
    },
    Point3 {
        point: Point3<f32>,
    },
}

impl CtorNode {
    fn name(&self) -> &'static str {
        match self {
            CtorNode::Scalar { .. } => "Scalar",
            CtorNode::Point2 { .. } => "Point2",
            CtorNode::Line2 { .. } => "Line2",
            CtorNode::PointPointMotor2 { .. } => "Point to Point Motor2",
            CtorNode::LineLineMotor2 { .. } => "Line to Line Motor2",
            CtorNode::ReconstructMotor2 { .. } => "Reconstruct Motor2",
            CtorNode::Point3 { .. } => "Point3",
        }
    }

    fn inputs(&self) -> &[Type] {
        match self {
            CtorNode::Scalar { .. } => &[],
            CtorNode::Point2 { .. } => &[Type::Scalar, Type::Scalar],
            CtorNode::Line2 { .. } => &[Type::Scalar, Type::Scalar, Type::Scalar],
            CtorNode::PointPointMotor2 { .. } => &[Type::Scalar, Type::Point2, Type::Point2],
            CtorNode::LineLineMotor2 { .. } => &[Type::Scalar, Type::Line2, Type::Line2],
            CtorNode::ReconstructMotor2 { .. } => &[
                Type::Scalar,
                Type::Point2,
                Type::Point2,
                Type::Point2,
                Type::Point2,
            ],
            CtorNode::Point3 { .. } => &[Type::Scalar, Type::Scalar, Type::Scalar],
        }
    }

    fn output(&self) -> Type {
        match self {
            CtorNode::Scalar { .. } => Type::Scalar,
            CtorNode::Point2 { .. } => Type::Point2,
            CtorNode::Line2 { .. } => Type::Line2,
            CtorNode::PointPointMotor2 { .. } => Type::Motor2,
            CtorNode::LineLineMotor2 { .. } => Type::Motor2,
            CtorNode::ReconstructMotor2 { .. } => Type::Motor2,
            CtorNode::Point3 { .. } => Type::Point3,
        }
    }

    fn show_input(&mut self, idx: usize, value: Option<Value>, ui: &mut Ui) {
        match self {
            CtorNode::Scalar { .. } => unreachable!(),
            CtorNode::Point2 { point } => {
                let (mut x, mut y) = point.coords();

                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            x = scalar;
                            ui.add(DragValue::new(&mut x).speed(0.0));
                        }
                        _ => {
                            let s = x.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut x).speed(s));
                        }
                    },
                    1 => match value {
                        Some(Value::Scalar(scalar)) => {
                            y = scalar;
                            ui.add(DragValue::new(&mut y).speed(0.0));
                        }
                        _ => {
                            let s = y.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut y).speed(s));
                        }
                    },
                    _ => unreachable!(),
                };

                if point.is_ideal() {
                    *point = Point2::ideal(x, y);
                } else {
                    *point = Point2::at(x, y);
                }
            }
            CtorNode::Line2 { line } => {
                let (mut a, mut b, mut c) = line.abc();

                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            a = scalar;
                            ui.add(DragValue::new(&mut a).speed(0.0));
                        }
                        _ => {
                            let s = a.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut a).speed(s));
                        }
                    },
                    1 => match value {
                        Some(Value::Scalar(scalar)) => {
                            b = scalar;
                            ui.add(DragValue::new(&mut b).speed(0.0));
                        }
                        _ => {
                            let s = b.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut b).speed(s));
                        }
                    },
                    2 => match value {
                        Some(Value::Scalar(scalar)) => {
                            c = scalar;
                            ui.add(DragValue::new(&mut c).speed(0.0));
                        }
                        _ => {
                            let s = c.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut c).speed(s));
                        }
                    },
                    _ => unreachable!(),
                };

                *line = Line2::from_abc(a, b, c).normalized();
            }
            CtorNode::PointPointMotor2 { mul, a, b, motor } => {
                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            *mul = scalar;
                            *motor = Motor2::point_point(*a, *b) * *mul;
                        }
                        _ => {
                            let s = mul.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(mul).speed(s));
                            *motor = Motor2::point_point(*a, *b) * *mul;
                        }
                    },
                    1 => match value {
                        None => {}
                        Some(Value::Point2(point)) => {
                            *a = point;
                            *motor = Motor2::point_point(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    2 => match value {
                        None => {}
                        Some(Value::Point2(point)) => {
                            *b = point;
                            *motor = Motor2::point_point(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
            }
            CtorNode::LineLineMotor2 { mul, a, b, motor } => {
                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            *mul = scalar;
                            *motor = Motor2::line_line(*a, *b) * *mul;
                        }
                        _ => {
                            let s = mul.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(mul).speed(s));
                            *motor = Motor2::line_line(*a, *b) * *mul;
                        }
                    },
                    1 => match value {
                        None => {}
                        Some(Value::Line2(line)) => {
                            *a = line;
                            *motor = Motor2::line_line(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    2 => match value {
                        None => {}
                        Some(Value::Line2(line)) => {
                            *b = line;
                            *motor = Motor2::line_line(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
            }
            CtorNode::ReconstructMotor2 { mul, a, b, motor } => {
                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            *mul = scalar;
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                        _ => {
                            let s = mul.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(mul).speed(s));
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                    },
                    1 => match value {
                        None => {}
                        Some(Value::Point2(p)) => {
                            a[0] = p;
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    2 => match value {
                        None => {}
                        Some(Value::Point2(p)) => {
                            a[1] = p;
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    3 => match value {
                        None => {}
                        Some(Value::Point2(p)) => {
                            b[0] = p;
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    4 => match value {
                        None => {}
                        Some(Value::Point2(p)) => {
                            b[1] = p;
                            *motor = Motor2::reconstruct(*a, *b) * *mul;
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
            }
            CtorNode::Point3 { point } => {
                let (mut x, mut y, mut z) = point.coords();

                match idx {
                    0 => match value {
                        Some(Value::Scalar(scalar)) => {
                            x = scalar;
                            ui.add(DragValue::new(&mut x).speed(0.0));
                        }
                        _ => {
                            let s = x.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut x).speed(s));
                        }
                    },
                    1 => match value {
                        Some(Value::Scalar(scalar)) => {
                            y = scalar;
                            ui.add(DragValue::new(&mut y).speed(0.0));
                        }
                        _ => {
                            let s = y.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut y).speed(s));
                        }
                    },
                    2 => match value {
                        Some(Value::Scalar(scalar)) => {
                            z = scalar;
                            ui.add(DragValue::new(&mut z).speed(0.0));
                        }
                        _ => {
                            let s = z.abs().max(1.0) * 0.001;
                            ui.add(DragValue::new(&mut z).speed(s));
                        }
                    },
                    _ => unreachable!(),
                };

                if point.is_ideal() {
                    *point = Point3::ideal(x, y, z);
                } else {
                    *point = Point3::at(x, y, z);
                }
            }
        }
    }

    fn show_output(&mut self, ui: &mut Ui) {
        match self {
            CtorNode::Scalar { scalar } => {
                ui.add(DragValue::new(scalar).speed(0.0));
            }
            CtorNode::Point2 { point } => {
                Value::Point2(*point).show(ui);
            }
            CtorNode::Line2 { line } => {
                Value::Line2(*line).show(ui);
            }
            CtorNode::PointPointMotor2 { motor, .. } => {
                Value::Motor2(*motor).show(ui);
            }
            CtorNode::LineLineMotor2 { motor, .. } => {
                Value::Motor2(*motor).show(ui);
            }
            CtorNode::ReconstructMotor2 { motor, .. } => {
                Value::Motor2(*motor).show(ui);
            }
            CtorNode::Point3 { point } => {
                Value::Point3(*point).show(ui);
            }
        }
    }

    fn get(&self) -> Value {
        match self {
            CtorNode::Scalar { scalar } => Value::Scalar(*scalar),
            CtorNode::Point2 { point } => Value::Point2(*point),
            CtorNode::Line2 { line } => Value::Line2(*line),
            CtorNode::PointPointMotor2 { motor, .. } => Value::Motor2(*motor),
            CtorNode::LineLineMotor2 { motor, .. } => Value::Motor2(*motor),
            CtorNode::ReconstructMotor2 { motor, .. } => Value::Motor2(*motor),
            CtorNode::Point3 { point } => Value::Point3(*point),
        }
    }
}

enum Motor {
    Motor2(Motor2<f32>),
}

impl Motor {
    fn new(a: Value, b: Value) -> Motor {
        match (a, b) {
            (Value::Point2(a), Value::Point2(b)) => Motor::Motor2(Motor2::point_point(a, b)),
            (Value::Line2(a), Value::Line2(b)) => Motor::Motor2(Motor2::line_line(a, b)),
            _ => unreachable!(),
        }
    }

    fn move_object(&self, geom: Value) -> Value {
        match self {
            Motor::Motor2(motor) => match geom {
                Value::Point2(point) => Value::Point2(motor.move_point(point)),
                _ => unreachable!(),
            },
        }
    }
}

pub enum Node {
    // Node that outputs current time as f32 seconds.
    Time,

    /// Calculates modulo.
    Modulo {
        a: f32,
        b: f32,
        result: f32,
    },

    CtorNode(CtorNode),
    Apply {
        transform: Value,
        results: Vec<Value>,
    },
    Meet {
        a: Value,
        b: Value,
        result: Value,
    },
    Join {
        a: Value,
        b: Value,
        result: Value,
    },
    Project {
        a: Value,
        b: Value,
        result: Value,
    },
    Show {
        scale: f32,
        color: Color32,
    },
}

impl Node {
    fn inputs(&self) -> usize {
        match self {
            Node::Time => 0,
            Node::Modulo { .. } => 2,
            Node::CtorNode(ctor) => ctor.inputs().len(),
            Node::Apply { results, .. } => results.len() + 2,
            Node::Meet { .. } => 2,
            Node::Join { .. } => 2,
            Node::Project { .. } => 2,
            Node::Show { .. } => 1,
        }
    }

    fn input_type(&self, idx: usize) -> Option<Type> {
        match self {
            Node::Time => None,
            Node::Modulo { .. } => Some([Type::Scalar; 2][idx]),
            Node::CtorNode(ctor) => Some(ctor.inputs()[idx]),
            Node::Apply { .. } => None,
            Node::Meet { .. } => None,
            Node::Join { .. } => None,
            Node::Project { .. } => None,
            Node::Show { .. } => None,
        }
    }

    fn outputs(&self) -> usize {
        match self {
            Node::Time => 1,
            Node::Modulo { .. } => 1,
            Node::CtorNode(_) => 1,
            Node::Apply { results, .. } => results.len(),
            Node::Meet { .. } => 1,
            Node::Join { .. } => 1,
            Node::Project { .. } => 1,
            Node::Show { .. } => 0,
        }
    }

    fn output_type(&self, idx: usize) -> Type {
        match self {
            Node::Time => {
                assert_eq!(idx, 0);
                Type::Scalar
            }
            Node::Modulo { .. } => {
                assert_eq!(idx, 0);
                Type::Scalar
            }
            Node::CtorNode(ctor) => {
                assert_eq!(idx, 0);
                ctor.output()
            }
            Node::Apply { results, .. } => results[idx].ty(),
            Node::Meet { result, .. } => result.ty(),
            Node::Join { result, .. } => result.ty(),
            Node::Project { result, .. } => result.ty(),
            Node::Show { .. } => unreachable!(),
        }
    }

    fn get(&self, idx: usize, cx: &egui::Context) -> Value {
        match self {
            Node::Time => {
                assert_eq!(idx, 0);
                cx.request_repaint();
                Value::Scalar(cx.input(|i| i.time as f32)) // Placeholder for current time
            }
            Node::Modulo { result, .. } => {
                assert_eq!(idx, 0);
                Value::Scalar(*result)
            }
            Node::CtorNode(ctor) => {
                assert_eq!(idx, 0);
                ctor.get()
            }
            Node::Apply { results, .. } => results[idx],
            Node::Meet { result, .. } => {
                assert_eq!(idx, 0);
                *result
            }
            Node::Join { result, .. } => {
                assert_eq!(idx, 0);
                *result
            }
            Node::Project { result, .. } => {
                assert_eq!(idx, 0);
                *result
            }
            Node::Show { .. } => unreachable!(),
        }
    }
}

pub type Graph = Snarl<Node>;

struct ValueInPin(Type);

impl SnarlPin for ValueInPin {
    fn draw(
        self,
        _snarl_style: &egui_snarl::ui::SnarlStyle,
        _style: &egui::Style,
        rect: egui::Rect,
        painter: &egui::Painter,
    ) -> PinWireInfo {
        let radius = f32::min(rect.width(), rect.height()) * 0.5;

        painter.add(Shape::circle_filled(
            rect.center(),
            radius,
            self.0.pin_color(),
        ));

        PinWireInfo {
            color: self.0.pin_color(),
            style: WireStyle::Bezier5,
        }
    }
}

struct SinkPin;

impl SnarlPin for SinkPin {
    fn draw(
        self,
        _snarl_style: &egui_snarl::ui::SnarlStyle,
        _style: &egui::Style,
        rect: egui::Rect,
        painter: &egui::Painter,
    ) -> PinWireInfo {
        let corner_radius = f32::min(rect.width(), rect.height()) * 0.2;

        painter.add(Shape::rect_filled(rect, corner_radius, Color32::GRAY));
        PinWireInfo {
            color: Color32::GRAY,
            style: WireStyle::Bezier5,
        }
    }
}

enum InputPin {
    Value(ValueInPin),
    Sink(SinkPin),
}

impl SnarlPin for InputPin {
    fn draw(
        self,
        snarl_style: &egui_snarl::ui::SnarlStyle,
        style: &egui::Style,
        rect: egui::Rect,
        painter: &egui::Painter,
    ) -> PinWireInfo {
        match self {
            InputPin::Value(geom_pin) => geom_pin.draw(snarl_style, style, rect, painter),
            InputPin::Sink(sink_pin) => sink_pin.draw(snarl_style, style, rect, painter),
        }
    }
}

pub struct Renderable {
    pub color: Color32,
    pub scale: f32,
    pub value: Value,
}

impl Renderable {
    pub fn draw(&self, painter: &Painter, camera: &Matrix4<f32>, tr: RectTransform) {
        self.value.draw(self.scale, self.color, painter, camera, tr);
    }
}

struct AthenaViewer {
    show: Vec<Renderable>,
}

impl SnarlViewer<Node> for AthenaViewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Time => "Time".to_string(),
            Node::Modulo { .. } => "Modulo".to_string(),
            Node::CtorNode(ctor) => ctor.name().to_string(),
            Node::Apply { .. } => "Apply".to_string(),
            Node::Meet { .. } => "Meet".to_string(),
            Node::Join { .. } => "Join".to_string(),
            Node::Project { .. } => "Project".to_string(),
            Node::Show { .. } => "Show".to_string(),
        }
    }

    fn inputs(&mut self, node: &Node) -> usize {
        node.inputs()
    }

    fn outputs(&mut self, node: &Node) -> usize {
        node.outputs()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let v = pin
            .remotes
            .get(0)
            .map(|p| snarl[p.node].get(p.output, ui.ctx()));
        let node = &mut snarl[pin.id.node];

        match node {
            Node::Time => {
                unreachable!()
            }
            Node::Modulo { a, b, result } => match pin.id.input {
                0 => {
                    ui.label("A");

                    let Some(Value::Scalar(scalar)) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *a = scalar;
                    *result = *a % *b;

                    InputPin::Value(ValueInPin(Type::Scalar))
                }
                1 => {
                    ui.label("B");

                    let Some(Value::Scalar(scalar)) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *b = scalar;
                    *result = *a % *b;

                    InputPin::Value(ValueInPin(Type::Scalar))
                }
                _ => unreachable!(),
            },
            Node::CtorNode(ctor) => {
                ctor.show_input(pin.id.input, v, ui);
                InputPin::Value(ValueInPin(ctor.inputs()[pin.id.input]))
            }
            Node::Apply {
                transform, results, ..
            } => match pin.id.input {
                0 => {
                    ui.label("Transform");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    let Node::Apply {
                        ref mut transform, ..
                    } = snarl[pin.id.node]
                    else {
                        panic!();
                    };

                    *transform = v;

                    InputPin::Value(ValueInPin(v.ty()))
                }
                i if i < results.len() + 2 => {
                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    if i == results.len() + 1 {
                        results.push(Value::Nil);
                    }

                    let tv = transform.apply(&v);
                    results[i - 1] = tv;

                    InputPin::Value(ValueInPin(v.ty()))
                }
                _ => unreachable!(),
            },
            Node::Meet { a, b, result } => match pin.id.input {
                0 => {
                    ui.label("A");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *a = v;
                    *result = a.meet(b);

                    InputPin::Value(ValueInPin(a.ty()))
                }
                1 => {
                    ui.label("B");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *b = v;
                    *result = a.meet(b);

                    InputPin::Value(ValueInPin(b.ty()))
                }
                _ => unreachable!(),
            },
            Node::Join { a, b, result } => match pin.id.input {
                0 => {
                    ui.label("A");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *a = v;
                    *result = a.join(b);

                    InputPin::Value(ValueInPin(a.ty()))
                }
                1 => {
                    ui.label("B");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *b = v;
                    *result = a.join(b);

                    InputPin::Value(ValueInPin(b.ty()))
                }
                _ => unreachable!(),
            },
            Node::Project { a, b, result } => match pin.id.input {
                0 => {
                    ui.label("A");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *a = v;
                    *result = a.project(b);

                    InputPin::Value(ValueInPin(a.ty()))
                }
                1 => {
                    ui.label("B");

                    let Some(v) = v else {
                        return InputPin::Sink(SinkPin);
                    };

                    *b = v;
                    *result = a.project(b);

                    InputPin::Value(ValueInPin(b.ty()))
                }
                _ => unreachable!(),
            },
            Node::Show { scale, color } => {
                let scale = *scale;
                let color = *color;

                assert!(pin.id.input == 0);

                for r in pin.remotes.iter() {
                    let rnode = &snarl[r.node];

                    let value = rnode.get(r.output, ui.ctx());
                    self.show.push(Renderable {
                        color,
                        scale,
                        value,
                    });
                }

                InputPin::Sink(SinkPin)
            }
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let node = &mut snarl[pin.id.node];

        match node {
            Node::Time => {
                assert_eq!(pin.id.output, 0);
                let seconds = ui.ctx().input(|i| i.time as f32);
                ui.label(format!("Time: {:0.3}", seconds));
                ui.ctx().request_repaint();
                ValueInPin(Type::Scalar)
            }
            Node::Modulo { result, .. } => {
                assert_eq!(pin.id.output, 0);
                ui.label(format!("Result: {}", *result));
                ValueInPin(Type::Scalar)
            }
            Node::CtorNode(ctor) => {
                assert_eq!(pin.id.output, 0);
                ctor.show_output(ui);
                ValueInPin(ctor.output())
            }
            Node::Apply { results, .. } => {
                let result = &results[pin.id.output];
                result.show(ui);
                ValueInPin(result.ty())
            }
            Node::Meet { result, .. } => {
                assert_eq!(pin.id.output, 0);
                result.show(ui);
                ValueInPin(result.ty())
            }
            Node::Join { result, .. } => {
                assert_eq!(pin.id.output, 0);
                result.show(ui);
                ValueInPin(result.ty())
            }
            Node::Project { result, .. } => {
                assert_eq!(pin.id.output, 0);
                result.show(ui);
                ValueInPin(result.ty())
            }
            Node::Show { .. } => {
                unreachable!()
            }
        }
    }

    fn has_body(&mut self, node: &Node) -> bool {
        match node {
            Node::Show { .. } => true,
            _ => false,
        }
    }

    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) {
        let node = &mut snarl[node];

        match node {
            Node::Show { scale, color } => {
                ui.add(DragValue::new(scale).range(1.0..=10.0).prefix("scale: "));
                ui.color_edit_button_srgba(color);
            }
            _ => unreachable!(),
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, snarl: &mut Snarl<Node>) {
        ui.menu_button("Add node", |ui| {
            ui.vertical(|ui| {
                if ui.button("Show").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Show {
                            scale: 1.0,
                            color: Color32::from_rgb(
                                rand::random(),
                                rand::random(),
                                rand::random(),
                            ),
                        },
                    );
                    ui.close_menu();
                }
                if ui.button("Point2").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::Point2 {
                            point: Point2::ORIGIN,
                        }),
                    );
                    ui.close_menu();
                }
                if ui.button("Point3").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::Point3 {
                            point: Point3::ORIGIN,
                        }),
                    );
                    ui.close_menu();
                }
                if ui.button("Line2").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::Line2 {
                            line: Line2::from_abc(1.0, 1.0, 0.0),
                        }),
                    );
                    ui.close_menu();
                }
                if ui.button("Point to Point Motor2").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::PointPointMotor2 {
                            mul: 1.0,
                            a: Point2::ORIGIN,
                            b: Point2::ORIGIN,
                            motor: Motor2::point_point(Point2::ORIGIN, Point2::ORIGIN),
                        }),
                    );
                    ui.close_menu();
                }
                if ui.button("Line to Line Motor2").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::LineLineMotor2 {
                            mul: 1.0,
                            a: Line2::from_abc(1.0, 1.0, 0.0),
                            b: Line2::from_abc(1.0, 1.0, 0.0),
                            motor: Motor2::line_line(
                                Line2::from_abc(1.0, 1.0, 0.0),
                                Line2::from_abc(1.0, 1.0, 0.0),
                            ),
                        }),
                    );
                    ui.close_menu();
                }

                if ui.button("Reconstruct Motor2").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::CtorNode(CtorNode::ReconstructMotor2 {
                            mul: 1.0,
                            a: [Point2::ORIGIN, Point2::ORIGIN],
                            b: [Point2::ORIGIN, Point2::ORIGIN],
                            motor: Motor2::reconstruct(
                                [Point2::ORIGIN, Point2::ORIGIN],
                                [Point2::ORIGIN, Point2::ORIGIN],
                            ),
                        }),
                    );
                    ui.close_menu();
                }

                if ui.button("Apply").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Apply {
                            transform: Value::Nil,
                            results: Vec::new(),
                        },
                    );
                    ui.close_menu();
                }

                if ui.button("Meet").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Meet {
                            a: Value::Nil,
                            b: Value::Nil,
                            result: Value::Nil,
                        },
                    );
                    ui.close_menu();
                }

                if ui.button("Join").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Join {
                            a: Value::Nil,
                            b: Value::Nil,
                            result: Value::Nil,
                        },
                    );
                    ui.close_menu();
                }

                if ui.button("Project").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Project {
                            a: Value::Nil,
                            b: Value::Nil,
                            result: Value::Nil,
                        },
                    );
                    ui.close_menu();
                }

                if ui.button("Time").clicked() {
                    snarl.insert_node(pos, Node::Time);
                    ui.close_menu();
                }

                if ui.button("Modulo").clicked() {
                    snarl.insert_node(
                        pos,
                        Node::Modulo {
                            a: 0.0,
                            b: 1.0,
                            result: 0.0,
                        },
                    );
                    ui.close_menu();
                }
            });
        });
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let from_node = &snarl[from.id.node];
        let to_node = &snarl[to.id.node];

        assert!(from.id.output < from_node.outputs());
        assert!(to.id.input < to_node.inputs());

        match (
            from_node.output_type(from.id.output),
            to_node.input_type(to.id.input),
        ) {
            (_, None) => {}
            (from_ty, Some(to_ty)) => {
                if from_ty != to_ty {
                    return;
                }
            }
        }

        match to_node {
            Node::Show { .. } => {
                for old_to in from.remotes.iter() {
                    let old_to_node = &snarl[old_to.node];

                    if let Node::Show { .. } = old_to_node {
                        snarl.disconnect(from.id, *old_to);
                    }
                }
                snarl.connect(from.id, to.id);
            }
            _ => {
                for old_from in to.remotes.iter() {
                    snarl.disconnect(*old_from, to.id);
                }
                snarl.connect(from.id, to.id);
            }
        }
    }
}

pub fn show_nodes(snarl: &mut Snarl<Node>, ui: &mut Ui) -> Vec<Renderable> {
    let mut viewer = AthenaViewer { show: Vec::new() };

    SnarlWidget::new()
        .id_salt("AthenaSnarl")
        .show(snarl, &mut viewer, ui);

    viewer.show
}
