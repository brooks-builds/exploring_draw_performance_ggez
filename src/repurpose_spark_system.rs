use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::Color;
use ggez::{graphics, Context};
use rand::{thread_rng, Rng};

pub fn repurpose_spark_system(world: &World, context: &mut Context) -> Result<()> {
    let query;
    let (locations, velocities, colors) = query!(world, query, "location", "velocity", "color");

    for (index, location) in locations.iter().enumerate() {
        let wrapped_location: &DataWrapper<Point> = location.cast()?;
        let wrapped_velocity: &DataWrapper<Point> = velocities[index].cast()?;
        let wrapped_color: &DataWrapper<Color> = colors[index].cast()?;
        let mut color = wrapped_color.borrow_mut();

        if color.a > 0.0 {
            continue;
        }

        let mut rng = thread_rng();
        let mut location = wrapped_location.borrow_mut();
        let mut velocity = wrapped_velocity.borrow_mut();
        let (width, height) = graphics::drawable_size(context);

        color.r = rng.gen_range(0.5..1.0);
        color.a = 1.0;
        location.x = rng.gen_range(0.0..width);
        location.y = height + 5.0;
        velocity.multiply_scalar(0.0);
    }
    Ok(())
}
