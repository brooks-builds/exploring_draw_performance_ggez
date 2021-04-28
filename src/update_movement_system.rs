use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

pub fn update_movement_system(world: &World) -> Result<()> {
    let query;
    let (locations, velocities, accelerations) =
        query!(world, query, "location", "velocity", "acceleration");

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let velocity: &DataWrapper<Point> = velocities[index].cast()?;
        let acceleration: &DataWrapper<Point> = accelerations[index].cast()?;

        *velocity.borrow_mut() += *acceleration.borrow();
        *location.borrow_mut() += *velocity.borrow();
        acceleration.borrow_mut().multiply_scalar(0.0);
    }
    Ok(())
}
