use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use rand::{random, thread_rng, Rng};

pub fn accelerate_system(world: &World) -> Result<()> {
    let query;
    let (accelerations,) = query!(world, query, "acceleration");
    let mut rng = thread_rng();
    for acceleration in accelerations.iter() {
        let acceleration: &DataWrapper<Point> = acceleration.cast()?;

        let wind_force = Point::new(rng.gen_range(-0.02..0.02), rng.gen_range(-0.03..-0.01));
        *acceleration.borrow_mut() += wind_force;
    }

    Ok(())
}
