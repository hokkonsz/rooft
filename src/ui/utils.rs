use bevy::ui::{UiRect, Val};

pub trait ValExt {
    fn px(&self) -> f32;
}

impl ValExt for Val {
    fn px(&self) -> f32 {
        match &self {
            Val::Px(value) => *value,
            _ => 0.,
        }
    }
}

pub trait UiRectExt {
    fn px(&self) -> (f32, f32, f32, f32);
    fn px_horizontal(&self) -> f32;
    fn px_vertical(&self) -> f32;
}

impl UiRectExt for UiRect {
    fn px(&self) -> (f32, f32, f32, f32) {
        (
            self.left.px(),
            self.right.px(),
            self.top.px(),
            self.bottom.px(),
        )
    }

    fn px_horizontal(&self) -> f32 {
        self.left.px() + self.right.px()
    }

    fn px_vertical(&self) -> f32 {
        self.top.px() + self.bottom.px()
    }
}
