#![feature(new_range_api)]

use bevy::{
	color::palettes::css::GOLD,
	diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
	prelude::*,
};

use crate::ball::Ball;
use ball::BallPlugin;
use settings::*;

// CRATES
mod ball;
mod settings;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct BallText;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Ball Simulation".into(),
						resolution: (SCREENSIZE.x, SCREENSIZE.y).into(),
						resizable: false,
						..default()
					}),
					..default()
				})
				.build(),
			FrameTimeDiagnosticsPlugin,
		))
		.add_systems(Startup, setup)
		.add_systems(Update, text_update_system)
		.add_plugins(BallPlugin)
		.run();
}

fn setup(mut commands: Commands) {
	let camera = Camera2dBundle::default();
	commands.spawn(camera);

	commands.spawn((
		TextBundle::from_sections([
			TextSection::new(
				"FPS: ",
				TextStyle {
					font_size: 30.0,
					..default()
				},
			),
			TextSection::from_style(TextStyle {
				font_size: 30.0,
				color: GOLD.into(),
				..default()
			}),
		]),
		FpsText,
	));

	commands.spawn((
		TextBundle::from_sections([
			TextSection::new(
				"Ball: ",
				TextStyle {
					font_size: 30.0,
					..default()
				},
			),
			TextSection::from_style(TextStyle {
				font_size: 30.0,
				color: GOLD.into(),
				..default()
			}),
		])
		.with_style(Style {
			position_type: PositionType::Absolute,
			top: Val::Px(50.0),
			..default()
		}),
		BallText,
	));
}

#[allow(clippy::type_complexity)]
fn text_update_system(
	diagnostics: Res<DiagnosticsStore>,
	mut set: ParamSet<(
		Query<&mut Text, With<FpsText>>,
		Query<&mut Text, With<BallText>>,
	)>,
	ball_query: Query<&Ball>,
) {
	// let mut text = query.single_mut();
	// let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
	// let value = fps.expect("REASON").smoothed();

	// text.sections[1].value = value.expect("REASON").to_string();

	for mut fps_text in set.p0().iter_mut() {
		if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
			&& let Some(value) = fps.smoothed()
		{
			// Update the value of the second section
			fps_text.sections[1].value = format!("{:.2}", value);
		}
	}
	for mut ball_text in set.p1().iter_mut() {
		ball_text.sections[1].value = format!("{}", ball_query.iter().count());
	}

	// for mut text in &mut fpsQuery {
	//     if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
	//         if let Some(value) = fps.smoothed() {
	//             // Update the value of the second section
	//             text.sections[1].value = format!("{:.2}", value);
	//         }
	//     }
	// }

	// let mut text = ballQuery.single_mut();

	// text.sections[1].value = format!("{}", 2);
}
