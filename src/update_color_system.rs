use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::Color;

pub fn update_color_system(world: &World) -> Result<()> {
    let query;
    let (colors,) = query!(world, query, "color");

    for color in colors {
        let color: &DataWrapper<Color> = color.cast()?;
        color.borrow_mut().a -= 0.008;
    }
    Ok(())
}
