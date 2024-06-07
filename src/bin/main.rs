use color_eyre::eyre::Result;

use game_engine::{
    app::App,
    components::{position::Position, velocity::Velocity},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();

    let player_id = app
        .create_entity()
        .add_component(Position { x: 0.0, y: 0.0 })
        .add_component(Velocity { x: 1.0, y: 1.0 })
        .id();

    println!("Player with ID {player_id} successfully created");
    println!("Position: {:?}", app.get_component::<Position>(player_id));
    println!("Velocity: {:?}", app.get_component::<Velocity>(player_id));

    Ok(())
}
