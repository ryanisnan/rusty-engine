use ggez::graphics::Rect;

#[derive(Debug)]
pub struct Camera {
    // Represents our current "view" into an area
    rect: Rect,

    // Represents the area in which we can move around
    // `rect` above must always fit inside of `boundaries`
    boundaries: Rect,

    // Amounts that the camera should move
    horiz_scroll: f32,
    vert_scroll: f32,
}

impl Camera {
    pub fn new(rect: Rect, boundaries: Rect) -> Camera {
        Camera {
            rect,
            boundaries,
            horiz_scroll: 32.0,
            vert_scroll: 32.0,
        }
    }

    pub fn left(&self) -> f32 {
        self.rect.left()
    }

    pub fn right(&self) -> f32 {
        self.rect.right()
    }

    pub fn top(&self) -> f32 {
        self.rect.top()
    }

    pub fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    pub fn to_rect(&self) -> Rect {
        self.rect.clone()
    }

    pub fn move_left(&mut self) {
        if (self.rect.left() - self.horiz_scroll) >= self.boundaries.left() {
            self.rect.x -= self.horiz_scroll;
        } else {
            self.rect.x = self.boundaries.left() + self.rect.w / 2.0;
        }

        println!("Camera::move_left() called - {:#?}", self);
    }

    pub fn move_right(&mut self) {
        if (self.rect.right() + self.horiz_scroll) <= self.boundaries.right() {
            self.rect.x += self.horiz_scroll;
        } else {
            self.rect.x = self.boundaries.right() - self.rect.w / 2.0;
        }

        println!("Camera::move_right() called - {:#?}", self);
    }

    pub fn move_up(&mut self) {
        if (self.rect.top() - self.vert_scroll) >= self.boundaries.top() {
            self.rect.y -= self.vert_scroll;
        } else {
            self.rect.y = self.boundaries.top() + self.rect.h / 2.0;
        }

        println!("Camera::move_up() called - {:#?}", self);
    }

    pub fn move_down(&mut self) {
        if (self.rect.bottom() + self.vert_scroll) <= self.boundaries.bottom() {
            self.rect.y += self.vert_scroll;
        } else {
            self.rect.y = self.boundaries.bottom() - self.rect.h / 2.0;
        }

        println!("Camera::move_down() called - {:#?}", self);
    }
}
