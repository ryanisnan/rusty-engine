#[derive(Debug)]
pub struct Camera {
    // Position in the world
    x: f32,
    y: f32,

    // Boundaries of the viewport on the world
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,

    // Amounts that the camera should move
    horiz_scroll: f32,
    vert_scroll: f32,

    // Bounds related to an environment - can also be the size of the world
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,

    // Viewport Information
    viewport_width: u32,
    viewport_height: u32,
}

impl Camera {
    pub fn new(viewport_height: u32, viewport_width: u32, max_x: u32, max_y: u32) -> Camera {
        let cam = Camera {
            x: viewport_width as f32 / 2.0,
            y: viewport_height as f32 / 2.0,

            left: 0.0,
            right: viewport_width as f32,
            top: 0.0,
            bottom: viewport_height as f32,

            horiz_scroll: 32.0,
            vert_scroll: 32.0,

            min_x: 0.0,
            max_x: max_x as f32,
            min_y: 0.0,
            max_y: max_y as f32,

            viewport_width,
            viewport_height
        };

        // println!("Camera instantiated: {:#?}", cam);
        cam
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_left(&self) -> f32 {
        self.left
    }

    pub fn get_right(&self) -> f32 {
        self.right
    }

    pub fn get_top(&self) -> f32 {
        self.top
    }

    pub fn get_bottom(&self) -> f32 {
        self.bottom
    }

    fn position_updated(&mut self) {
        self.left = self.x - self.viewport_width as f32 / 2.0;
        self.right = self.x + self.viewport_width as f32 / 2.0;
        self.top = self.y - self.viewport_height as f32 / 2.0;
        self.bottom = self.y + self.viewport_height as f32 / 2.0;
    }

    pub fn move_left(&mut self) {
        if (self.x - self.viewport_width as f32 / 2.0 - self.horiz_scroll) >= 0.0 {
            self.x -= self.horiz_scroll;
        } else {
            self.x = self.viewport_width as f32 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_left() called - {:#?}", self);
    }

    pub fn move_right(&mut self) {
        if (self.x + self.viewport_width as f32 / 2.0 + self.horiz_scroll) <= (self.max_x) {
            self.x += self.horiz_scroll;
        } else {
            self.x = self.max_x - self.viewport_width as f32 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_right() called - {:#?}", self);
    }

    pub fn move_up(&mut self) {
        if (self.y - self.viewport_height as f32 / 2.0 - self.vert_scroll) >= 0.0 {
            self.y -= self.vert_scroll;
        } else {
            self.y = self.viewport_height as f32 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_up() called - {:#?}", self);
    }

    pub fn move_down(&mut self) {
        if (self.y + self.viewport_height as f32 / 2.0 + self.vert_scroll) <= self.max_y {
            self.y += self.vert_scroll;
        } else {
            self.y = self.max_y - self.viewport_height as f32 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_down() called - {:#?}", self);
    }
}
