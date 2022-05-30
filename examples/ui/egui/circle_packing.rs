use nannou::{color::rgb_u32, rand::thread_rng};
use nannou::{prelude::*, rand::prelude::SliceRandom};
use nannou_egui::{self, egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Hsv,
}

struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

struct Model {
    circles: Vec<Circle>,
    settings: Settings,
    egui: Egui,
    abc: bool,
    name: String,
    spacing: Point2,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {
        circles: Vec::new(),
        egui,
        settings: Settings {
            min_radius: 10.0,
            max_radius: 100.0,
            circle_count: 10,
        },
        abc: false,
        name: String::new(),
        spacing: pt2(40., 4.)
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut circles,
        ref mut abc,
        ref mut name,
        ref mut spacing,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Workshop window").show(&ctx, |ui| {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([spacing.x, spacing.y])
            .striped(true)
            .show(ui, |ui| {
                let mut changed = false;
                ui.label("Your name: ").changed();
                changed |= ui.text_edit_singleline(name).changed();
                ui.end_row();

                ui.label("min radius: ");
                changed |= ui
                    .add(egui::Slider::new(&mut settings.min_radius, 0.0..=20.0))
                    .changed();
                ui.end_row();
                
                ui.label("max radius: ");
                changed |= ui
                    .add(egui::Slider::new(&mut settings.max_radius, 0.0..=200.0).text("max radius"))
                    .changed();
                ui.end_row();

                ui.label("circle count: ");
                changed |= ui
                    .add(egui::Slider::new(&mut settings.circle_count, 0..=2000).text("circle count"))
                    .changed();
                
                ui.end_row();

                ui.label("Spacing x: ");
                changed |= ui
                    .add(egui::Slider::new(&mut spacing.x, 5.0..=100.0))
                    .changed();
                
                ui.end_row();

                ui.label("Spacing y: ");
                changed |= ui
                    .add(egui::Slider::new(&mut spacing.y, 1.0..=50.0))
                    .changed();
                
                ui.end_row();
                // changed |= ui.button("Generate").clicked();
        
                ui.label("111111 rect:\t");
                changed |= ui.add(toggle(abc)).changed();
                ui.end_row();

                ui.label("2222222 rect:\t");
                changed |= ui.add(toggle(abc)).changed();
                ui.end_row();
                
                // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                //     if ui.button("test").clicked() {
                //         println!("Shit!@!@!")
                //     }
                // }); 
                // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                //     changed |= ui.button("test2").clicked();
                // });
                // ui.end_row();
                if changed {
                    *circles = generate_circles(settings);
                }
            });
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for circle in model.circles.iter() {
        draw.ellipse()
            .x_y(circle.x, circle.y)
            .radius(circle.radius)
            .color(circle.color);
    }
    let win = app.main_window().rect();
    if model.abc == true {
        let bl = win.bottom_left();
        draw.rect().x_y(bl.x + 20., bl.y + 20.).w_h(40., 40.).color(RED);
    }

    draw.text(model.name.as_str()).align_text_bottom().color(WHITE);

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}

fn intersects(circle: &Circle, circles: &Vec<Circle>) -> bool {
    for other in circles.iter() {
        let dist: f32 =
            ((other.x - circle.x).pow(2) as f32 + (other.y - circle.y).pow(2) as f32).sqrt();
        if dist < circle.radius + other.radius {
            return true;
        }
    }
    false
}

fn generate_circles(settings: &mut Settings) -> Vec<Circle> {
    let colors = [
        hsv_from_hex_rgb(0x264653),
        hsv_from_hex_rgb(0x2a9d8f),
        hsv_from_hex_rgb(0xe9c46a),
        hsv_from_hex_rgb(0xf4a261),
        hsv_from_hex_rgb(0xe76f51),
    ];

    let mut circles = Vec::new();

    let mut rng = thread_rng();

    let mut loops = 0;
    loop {
        let x = random_range(-WIDTH / 2.0, WIDTH / 2.0);
        let y = random_range(-HEIGHT / 2.0, HEIGHT / 2.0);
        let radius = random_range(settings.min_radius, settings.max_radius);
        let color = *colors.choose(&mut rng).unwrap();
        let mut circle = Circle {
            x,
            y,
            radius,
            color,
        };

        loops += 1;
        if loops > 20000 {
            break;
        }

        if intersects(&circle, &circles) {
            continue;
        }

        let mut prev_radius = circle.radius;
        while !intersects(&circle, &circles) {
            // Grow the circle
            prev_radius = circle.radius;
            circle.radius += 10.0;

            if circle.radius >= settings.max_radius {
                break;
            }
        }
        circle.radius = prev_radius;

        circles.push(circle);

        if circles.len() >= settings.circle_count {
            break;
        }
    }

    circles
}

fn hsv_from_hex_rgb(color: u32) -> Hsv {
    let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    // 4. Paint!
    // Make sure we need to paint:
    if ui.visible() && rect.intersects(ui.clip_rect()) {
        // Let's ask for a simple animation from egui.
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        let how_on = ui.ctx().animate_bool(response.id, *on);
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

/// Here is the same code again, but a bit more compact:
#[allow(dead_code)]
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.visible() && rect.intersects(ui.clip_rect()) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
