mod menu;
mod window;

use bevy::prelude::*;

fn main() {
  App::new()
  .add_plugins( (
    DefaultPlugins.set( window::fullscreen() ),
  ) )
  .add_systems( Startup, window::spawn_camera )
  .add_systems( Startup, menu::spawn_menu )
  .run();
}