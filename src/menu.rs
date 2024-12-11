use bevy::prelude::*;

pub fn spawn_menu(
  mut commands: Commands
) {
  let screen = commands.spawn(
    Node {
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      height: Val::Percent( 100.0 ),
      width: Val::Percent( 100.0 ),
      ..Default::default()
    } 
  ).id();

  let menu = commands.spawn( (
    Node {
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      height: Val::Px( 900.0 ),
      width: Val::Px( 1200.0 ),
      padding: UiRect::all( Val::Px( 8.0 ) ),
      row_gap: Val::Px( 8.0 ),
      ..Default::default()
    },
    BackgroundColor( Color::oklch( 0.4, 0.1, 270.0 ) ),
    BorderRadius::all( Val::Px( 8.0 ) ),
  ) ).id();
  commands.entity( menu ).set_parent( screen );

  let tile_a = commands.spawn( (
    Node {
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      height: Val::Px( 50.0 ),
      width: Val::Px( 100.0 ),
      ..Default::default()
    },
    Text( "A".into() ),
    BackgroundColor( Color::BLACK ),
    BorderRadius::all( Val::Px( 4.0 ) ),
  ) ).id();
  commands.entity( tile_a ).set_parent( menu );

  let tile_b = commands.spawn( (
    Node {
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      height: Val::Px( 50.0 ),
      width: Val::Px( 100.0 ),
      ..Default::default()
    },
    BackgroundColor( Color::BLACK ),
    BorderRadius::all( Val::Px( 4.0 ) ),
  ) ).id();
  commands.entity( tile_b ).set_parent( menu );

  let tile_text_b = commands.spawn( Text( "B".into() ) ).id();
  commands.entity( tile_text_b ).set_parent( tile_b );
}
