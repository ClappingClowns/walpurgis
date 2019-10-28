use ggez::{Context, GameResult};
use ggez::graphics::{self, Drawable, DrawParam, Rect, BlendMode, Mesh, DrawMode};
use ggez::nalgebra as na;

use crate::{physics};

/// Denotes a static section of the `Arena`. Implements `ggez::Drawable`.
#[derive(Debug)]
pub struct Platform {
    mode: Option<BlendMode>,
    /// The portion occupied by the platform.
    body: physics::BoundingBox,
    /// If a player is allowed to move through the platform.
    can_move_through: bool,
    // TODO: Add storage for the assets' handles.
}

impl physics::Collidable for Platform {
    fn get_hitboxes<'tick>(&'tick self) -> &'tick[physics::BoundingBox] {
        std::slice::from_ref(&self.body)
    }
    fn get_effects(&self) {}
}

impl Drawable for Platform {
    fn draw(&self, ctx: &mut Context, mut param: DrawParam) -> GameResult {
        let rect = Rect::new(0f32, 0f32, 1.0, 1.0);

        let body = &self.body;
        param.rotation += body.ori;
        param.scale.x *= body.size[0];
        param.scale.y *= body.size[1];
        param.dest.x += body.pos[0];
        param.dest.y += body.pos[1];

        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &mesh, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.mode = mode;
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.mode
    }
}