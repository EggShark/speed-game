/// Code taken from p5 collide
/// not sure if its the best but ive used it before

use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;

pub fn point_in_rect(point: Vec2<f32>, r_pos: Vec2<f32>, r_size: Vec2<f32>) -> bool {
    (point.x >= r_pos.x && point.x <= r_pos.x + r_size.x) && 
        (point.y >= r_pos.y && point.y <= r_pos.y + r_size.y)
}

pub fn rect_in_rect(r1_pos: Vec2<f32>, r1_size: Vec2<f32>, r2_pos: Vec2<f32>, r2_size: Vec2<f32>) -> bool {
    r1_pos.x + r1_size.x >= r2_pos.x && 
        r1_pos.x <= r2_pos.x + r2_size.x &&
        r1_pos.y + r1_size.y >= r2_pos.y &&
        r1_pos.y <= r2_pos.y + r2_size.y
}

pub fn line_line(l1_start: Vec2<f32>, l1_end: Vec2<f32>, l2_start: Vec2<f32>, l2_end: Vec2<f32>) -> bool {
    let ua: f32 = ((l2_end.x-l2_start.x)*(l1_start.y-l2_start.y) - (l2_end.y-l2_start.y)*(l1_start.x-l2_start.x)) / ((l2_end.y-l2_start.y)*(l1_end.x-l1_start.x) - (l2_end.x-l2_start.x)*(l1_end.y-l1_start.y));
    let ub: f32 = ((l1_end.x-l1_start.x)*(l1_start.y-l2_start.y) - (l1_end.y-l1_start.y)*(l1_start.x-l2_start.x)) / ((l2_end.y-l2_start.y)*(l1_end.x-l1_start.x) - (l2_end.x-l2_start.x)*(l1_end.y-l1_start.y));

    ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0
}

pub fn line_in_rect(l_start: Vec2<f32>, l_end: Vec2<f32>, r_pos: Vec2<f32>, r_size: Vec2<f32>) -> bool {
    // left right top then bottom
    line_line(l_start, l_end, r_pos, vec2!(r_pos.x, r_pos.y + r_size.y)) ||
    line_line(l_start, l_end, vec2!(r_pos.x + r_size.x, r_pos.y), vec2!(r_pos.x + r_size.x, r_pos.y + r_size.y)) ||
    line_line(l_start, l_end, r_pos, vec2!(r_pos.x + r_size.x, r_pos.y)) ||
    line_line(l_start, l_end, vec2!(r_pos.x, r_pos.y + r_size.y), vec2!(r_pos.x + r_size.x, r_pos.y + r_size.y))
}