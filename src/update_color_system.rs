use bbecs::components::CastComponents;
use bbecs::query;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::graphics::Color;

pub fn update_color_system(world: &World) -> Result<()> {
    let query;
    let (colors, ids) = query!(world, query, "color", ENTITY_ID);

    for (index, color) in colors.iter().enumerate() {
        let color: &DataWrapper<Color> = color.cast()?;
        color.borrow_mut().a -= 0.008;

        if color.borrow().a <= 0.0 {
            let id: &DataWrapper<u32> = ids[index].cast()?;
            world.delete_by_id(*id.borrow())?;
        }
    }
    Ok(())
}
