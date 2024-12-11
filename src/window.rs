use bevy::prelude::*;

pub fn fullscreen() -> WindowPlugin {
  WindowPlugin{
    primary_window: Some( Window{
      mode: bevy::window::WindowMode::BorderlessFullscreen( MonitorSelection::Primary ),
      ..Default::default()
    } ),
    ..Default::default()
  }
}

pub fn spawn_camera( mut commands: Commands, ) {
  commands.spawn( Camera2d::default() );
}