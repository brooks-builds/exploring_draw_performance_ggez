use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World};
use bbecs::{get_resource, query};
use eyre::Result;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::Context;

pub fn draw_system(context: &mut Context, world: &World) -> Result<()> {
    let query;
    let (locations, colors) = query!(world, query, "location", "color");
    let mut mesh_builder = MeshBuilder::new();

    if locations.len() == 0 {
        return Ok(());
    }

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let color: &DataWrapper<Color> = colors[index].cast()?;
        mesh_builder.circle(
            DrawMode::fill(),
            location.borrow().to_array(),
            5.0,
            0.1,
            *color.borrow(),
        );
    }

    let mesh = mesh_builder.build(context)?;
    graphics::draw(context, &mesh, DrawParam::new())?;
    Ok(())
}
