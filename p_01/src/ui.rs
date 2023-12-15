use ggez::mint::Point2;

pub struct SelectionRect {
    pub start: Point2<f32>,
    pub end: Point2<f32>,
}

impl SelectionRect {
    pub fn new() -> SelectionRect {
        SelectionRect {
            start: Point2 {x: 9999.0, y: 9999.0},
            end: Point2 {x: 9999.0, y: 9999.0},
        }
    }
}