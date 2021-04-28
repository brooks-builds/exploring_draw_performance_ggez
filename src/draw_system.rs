use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::Context;

pub fn draw_system(context: &mut Context, world: &World) -> Result<()> {
    let query;
    let (locations, meshes, colors) = query!(world, query, "location", "mesh", "color");

    for (index, mesh) in meshes.iter().enumerate() {
        let mesh: &DataWrapper<Mesh> = mesh.cast()?;
        let location: &DataWrapper<Point> = locations[index].cast()?;
        let color: &DataWrapper<Color> = colors[index].cast()?;

        graphics::draw(
            context,
            &*mesh.borrow(),
            DrawParam::new()
                .dest(location.borrow().to_array())
                .color(*color.borrow()),
        )?;
    }
    Ok(())
}
