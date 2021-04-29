use accelerate_system::accelerate_system;
use bbecs::data_types::point::Point;
use bbecs::query;
use bbecs::world::{World, WorldMethods, ENTITY_ID};
use draw_system::draw_system;
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::{timer, Context};
use rand::Rng;
use repurpose_spark_system::repurpose_spark_system;
use update_color_system::update_color_system;
use update_movement_system::update_movement_system;

mod accelerate_system;
mod draw_system;
mod repurpose_spark_system;
mod update_color_system;
mod update_meshes_system;
mod update_movement_system;

pub struct MainState {
    world: World,
    sparks_created: u32,
}

impl MainState {
    pub fn new(context: &mut Context) -> Result<Self> {
        let mut world = World::new();
        let sparks_created = 0;

        world.register("location")?;
        world.register("velocity")?;
        world.register("acceleration")?;
        world.register("mesh")?;
        world.register("color")?;

        let spark_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 5.0, 0.1, WHITE)
            .build(context)?;

        world.add_resource("mesh".to_owned(), spark_mesh);

        Ok(Self {
            world,
            sparks_created,
        })
    }

    fn create_particle(&mut self, context: &mut Context) -> Result<()> {
        let (width, height) = graphics::drawable_size(context);
        let mut rng = rand::thread_rng();
        let world = &mut self.world;
        let location = Point::new(rng.gen_range(0.0..width), height + 5.0);
        let velocity = Point::new(0.0, 0.0);
        let acceleration = Point::new(0.0, 0.0);
        let color = Color::new(rng.gen_range(0.5..1.0), 0.0, 0.0, 1.0);
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 5.0, 0.1, WHITE)
            .build(context)?;

        world
            .spawn_entity()?
            .with_component("location", location)?
            .with_component("mesh", mesh)?
            .with_component("velocity", velocity)?
            .with_component("acceleration", acceleration)?
            .with_component("color", color)?;

        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        if self.sparks_created == 0 {
            for _ in 0..500 {
                self.create_particle(context).unwrap();
                self.sparks_created += 500;
            }
        }
        while timer::check_update_time(context, 60) {
            let fps = timer::fps(context);
            update_movement_system(&self.world).unwrap();
            accelerate_system(&self.world).unwrap();
            update_color_system(&self.world).unwrap();
            repurpose_spark_system(&self.world, context).unwrap();
            self.world.update().unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        draw_system(context, &self.world).unwrap();
        graphics::present(context)
    }
}
