use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    t: Point2,
    t_start: Point2,
    rect_t: Point2,
    rot: f32,
    rot_start: f32,
    t_set: bool,
    r_set: bool,
    moving: bool,
    scale: f32,
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_wheel(mouse_wheel)
        .view(view)
        .build()
        .unwrap();
    
    Model { 
        t: pt2(0., 0.), 
        t_start: pt2(0., 0.), 
        rect_t: pt2(0., 0.),
        rot: 0.,
        rot_start: 0.,
        t_set: true,
        r_set: true,
        moving: false,
        scale: 1.0
    }
}

fn good_angle(angle: f32) -> f32 {
    if angle > std::f32::consts::PI {
        return angle - std::f32::consts::PI * 2.;
    } else if angle < -std::f32::consts::PI {
        return angle + std::f32::consts::PI * 2.;
    }
    angle
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    let point = _app.mouse.position();
    match _button {
        MouseButton::Middle => {
            _model.t_start = point;
            _model.t_set = false;
        },
        MouseButton::Left => {
            _model.rot_start = point.y.atan2(point.x);
            _model.r_set = false;
        },
        MouseButton::Right => {
            _model.t_start = point;
            _model.moving = true;
        },
        _ => {}
    }
}

#[inline(always)]
fn get_rotation(angle: &f32) -> Mat2 {
    let cosa = angle.cos();
    let sina = angle.sin();
    Mat2::from_cols_array(&[cosa, sina, -sina, cosa])
}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {
    match _key {
        // _model.scrn and _model.obj are mutex to be true
        Key::Space => {
            _model.t = pt2(0., 0.);
            _model.rect_t = pt2(0., 0.);
            _model.rot = 0.;
        },
        _ => {},
    }
}

// mouse release will determine the initial angle
fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {
    let point = _app.mouse.position();
    match _button {
        MouseButton::Middle => {
            _model.t += point - _model.t_start;
            _model.t_set = true;
        },
        MouseButton::Left => {
            let delta_angle = good_angle(point.y.atan2(point.x) - _model.rot_start);
            _model.rot = good_angle(delta_angle + _model.rot);
            _model.r_set = true;
            _model.t = get_rotation(&delta_angle).mul_vec2(_model.t);
        },
        MouseButton::Right => {
            _model.rect_t += get_rotation(&-_model.rot).mul_vec2(point - _model.t_start) / _model.scale;
            _model.moving = false;
        },
        _ => {}
    }
}

// pid angle control
fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {
    let point = _app.mouse.position();
    if _model.t_set == false {
        _model.t += point - _model.t_start;
        _model.t_start = point;
    } else if _model.moving == true {
        _model.rect_t += get_rotation(&-_model.rot).mul_vec2(point - _model.t_start) / _model.scale;
        _model.t_start = point;
    }
    if _model.r_set == false {
        let current_angle = point.y.atan2(point.x);
        let delta_angle = good_angle(current_angle - _model.rot_start);
        _model.rot_start = current_angle;
        _model.rot = good_angle(delta_angle + _model.rot);
        _model.t = get_rotation(&delta_angle).mul_vec2(_model.t);
    }
}

// change velocity
fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {
    match _dt {
        MouseScrollDelta::LineDelta(_, iy) => {
            _model.scale = (_model.scale + iy * 0.05).min(2.).max(0.5);
        },
        _ => {
            println!("Mouse scroll data returned type: PixelDelta, which is not implemented.");
        }
    }
}

fn update(_app: &App, _model: &mut Model, _: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut draw = app.draw();

    draw = draw
        .x_y(_model.t.x, _model.t.y)
        .rotate(_model.rot)
        .scale_x(_model.scale)
        .scale_y(_model.scale);
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    // 100-step and 10-step grids.
    draw_grid(&draw, &win, 100.0, 1.0, 0.02);
    draw_grid(&draw, &win, 25.0, 0.5, 0.02);

    // Crosshair.
    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text.
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);

    

    // Window and monitor details.
    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        let text = format!(
            "
        Window size: [{:.0}, {:.0}]
        Window ratio: {:.2}
        Window scale factor: {:.2}
        Monitor size: [{:.0}, {:.0}]
        Monitor ratio: {:.2}
        Monitor scale factor: {:.2}
        ",
            win.w(),
            win.h(),
            win.w() / win.h(),
            w_scale_factor,
            mon_w,
            mon_h,
            mon_w / mon_h,
            m_scale_factor
        );
        let pad = 6.0;
        draw.text(&text)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .font_size(14)
            .align_text_bottom()
            .color(crosshair_color)
            .left_justify();

        // Ellipse at mouse.
        
        // Mouse position text.

        let original_mouse = app.mouse.position();
        let mut mouse = original_mouse;
        mouse -= _model.t;
        let rotation_inv= get_rotation(&-_model.rot);
        mouse = rotation_inv.mul_vec2(mouse);
        mouse /= _model.scale;

        draw.ellipse().wh([5.0; 2].into()).xy(mouse);
        let pos = format!("[{:.1}, {:.1}, {:.1}, {:.1}]", mouse.x, mouse.y, original_mouse.x, original_mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE).rotate(-_model.rot);

        draw.rect()
            .x_y(_model.rect_t.x, _model.rect_t.y)
            .w(100.).h(100.).color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32, alpha: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .rgba(1., 1., 1., alpha)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .rgba(1., 1., 1., alpha)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}
