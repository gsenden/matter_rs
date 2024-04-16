use super::sprite::Sprite;

#[derive(Clone, Copy, Default)]
pub struct Render {
    visible: bool,
    opacity: f64,
    //stroke_style: Option<?>,
    //fill_style: Option<?>,
    //line_width: Option<?>,
    sprite: Sprite,
}

impl Render {
    pub fn new(visible: bool, opacity: f64, sprite: Sprite) -> Self {
        Render {
            visible: visible,
            opacity: opacity,
            sprite: sprite,
        }
    }

    pub fn get_visible(&self) -> bool {
        self.visible
    }

    pub fn get_opacity(&self) -> f64 {
        self.opacity
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
