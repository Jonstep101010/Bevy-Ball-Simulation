#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::Wireframe2dPlugin;
use bevy::window::PrimaryWindow;
use bevy::{
	prelude::*,
	sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use crate::settings::*;

pub struct BallPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum SimulationState {
	#[default]
	Running,
	Paused,
}

impl Plugin for BallPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			#[cfg(not(target_arch = "wasm32"))]
			Wireframe2dPlugin,
		))
		.init_state::<SimulationState>()
		.add_systems(Startup, spawn_ball_parent)
		.add_systems(
			Update,
			(update_gravity_velocity, update_processes).run_if(in_state(SimulationState::Running)),
		)
		.add_systems(Update, (spawn_ball, interact))
		.add_systems(Update, update_ball_draw_position)
		.register_type::<Ball>();
	}
}

#[derive(Component)]
pub struct BallParent;

#[derive(Component, Default, Reflect, Clone, Copy)]
pub struct Ball {
	pub size: f32,
	pub pos: Vec3,
	pub velocity: Vec3,
	pub elasticity: f32,
	pub id: i32,
	pub pressure_stat: f32,
}

fn spawn_ball_parent(mut commands: Commands) {
	commands.spawn((
		SpatialBundle::default(),
		BallParent,
		Name::new("Ball Parent"),
	));
}

fn spawn_ball(
	mut commands: Commands,
	input: Res<ButtonInput<KeyCode>>,
	parent: Query<Entity, With<BallParent>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	if !input.pressed(KeyCode::Space) {
		return;
	}

	let parent = parent.single();

	let shape = Mesh2dHandle(meshes.add(Circle {
		radius: BALL_SIZE / 2.0,
	}));
	let color = Color::srgb(
		rand::thread_rng().gen_range(0.0..1.0),
		rand::thread_rng().gen_range(0.0..1.0),
		rand::thread_rng().gen_range(0.0..1.0),
	);

	// Spawns the ball
	commands.entity(parent).with_children(|commands| {
		commands.spawn((
			MaterialMesh2dBundle {
				mesh: shape,
				material: materials.add(color),
				transform: Transform {
					translation: Vec3::new(0.0, 180.0, 0.0),
					..default()
				},
				..default()
			},
			Ball {
				size: BALL_SIZE / 2.0,
				pos: Vec3::new(0.0, 180.0, 0.0),
				velocity: Vec3::new(10.0, 10.0, 0.0),
				elasticity: 0.3,
				id: rand::thread_rng().gen_range(1..=100000000),
				pressure_stat: 0.0,
			},
			Name::new("Ball"),
		));
	});

	info!("Spawned new Ball");
}

#[allow(clippy::too_many_arguments)]
fn interact(
	mut commands: Commands,
	parent: Query<Entity, With<BallParent>>,
	mut ball_object_query: Query<(Entity, &mut Ball, &mut Handle<ColorMaterial>)>,
	time: Res<Time>,
	q_windows: Query<&Window, With<PrimaryWindow>>,
	mouse_input: Res<ButtonInput<MouseButton>>,
	key_input: Res<ButtonInput<KeyCode>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	current_state: Res<State<SimulationState>>,
	mut next_state: ResMut<NextState<SimulationState>>,
) {
	let parent = parent.single();

	let mut position = Vec3::new(0.0, 0.0, 0.0);

	if let Some(mouse_position) = q_windows.single().cursor_position() {
		// println!("Cursor is inside the primary window, at {:?}", position);
		position = Vec3::new(mouse_position.x, mouse_position.y, 0.0);
	} else {
		// println!("Cursor is not in the game window.");
	}

	let mut rel_position: Vec3 = position - HALF_DIM;
	rel_position.y = -rel_position.y;

	if key_input.just_pressed(KeyCode::KeyP) {
		next_state.set(match current_state.get() {
			SimulationState::Running => SimulationState::Paused,
			SimulationState::Paused => SimulationState::Running,
		});
	}

	if mouse_input.just_pressed(MouseButton::Left) {
		let shape = Mesh2dHandle(meshes.add(Circle {
			radius: BALL_SIZE / 2.0,
		}));
		let color = Color::srgb(
			rand::thread_rng().gen_range(0.0..1.0),
			rand::thread_rng().gen_range(0.0..1.0),
			rand::thread_rng().gen_range(0.0..1.0),
		);

		// Spawns the ball

		for x in -5..5 {
			for y in -5..5 {
				let ball_spawn_pos: Vec3 =
					rel_position + Vec3::new(x as f32, y as f32, 0.0) * BALL_SIZE;

				commands.entity(parent).with_children(|commands| {
					commands.spawn((
						MaterialMesh2dBundle {
							mesh: shape.clone(),
							material: materials.add(color),
							transform: Transform {
								translation: ball_spawn_pos,
								..default()
							},
							..default()
						},
						Ball {
							size: BALL_SIZE / 2.0,
							pos: ball_spawn_pos,
							velocity: Vec3::new(0.0, 0.0, 0.0),
							elasticity: 0.3,
							id: rand::thread_rng().gen_range(1..=100000000),
							pressure_stat: 0.0,
						},
						Name::new("Ball"),
					));
				});
			}
		}

		info!("Spawned Ball Cluster");
	}

	if mouse_input.pressed(MouseButton::Right) {
		for (_, mut ball_object, _) in &mut ball_object_query {
			let rel_pos = (rel_position - ball_object.pos).normalize();
			// info!("{}",relPos);
			ball_object.velocity += rel_pos * MOUSE_STRENGTH * time.delta_seconds();
		}

		let mouse_pos_i: IVec3 = position.as_ivec3() / CHUNK_SIZE;

		info!("NEW CHECK");
		for x in -1..2 {
			for y in -1..2 {
				let offset_vec: IVec3 = IVec3::new(x, y, 0);
				let check_chunk_pos: IVec3 = mouse_pos_i + offset_vec;

				if !in_bounds(&check_chunk_pos) {
					continue;
				}
			}
		}
	}

	if key_input.pressed(KeyCode::KeyS) {
		for (ball_entity, ball_struct, _) in &mut ball_object_query {
			let gradient: f32 = ball_struct.velocity.length() / 300.0;
			let color = Color::srgb(gradient, gradient * 0.1, gradient * 0.1);
			commands.entity(ball_entity).insert(materials.add(color));
		}
	}

	if key_input.pressed(KeyCode::KeyD) {
		for (ball_entity, ball_struct, _) in &mut ball_object_query {
			let gradient: f32 = ball_struct.pressure_stat / 100.0;
			let color = Color::srgb(gradient * 0.1, gradient, gradient * 0.1);
			commands.entity(ball_entity).insert(materials.add(color));
		}
	}

	// too much of a hassle to record original colours, so we make new ones
	if key_input.just_released(KeyCode::KeyS) || key_input.just_released(KeyCode::KeyD) {
		for (ball_entity, _, _) in &mut ball_object_query {
			let color = Color::srgb(
				rand::thread_rng().gen_range(0.0..1.0),
				rand::thread_rng().gen_range(0.0..1.0),
				rand::thread_rng().gen_range(0.0..1.0),
			);
			commands.entity(ball_entity).insert(materials.add(color));
		}
	}

	// remove balls (left click + backspace)
	if mouse_input.pressed(MouseButton::Left) && key_input.pressed(KeyCode::Backspace) {
		for (ball_entity, ball_object, _) in &mut ball_object_query {
			if ball_object.pos.distance_squared(rel_position) <= REMOVE_RADIUS_SQUARED {
				commands.entity(parent).remove_children(&[ball_entity]);
				commands.entity(ball_entity).despawn();
			}
		}
	}

	// remove all balls (shift + backspace)
	if (key_input.pressed(KeyCode::ShiftLeft) || key_input.pressed(KeyCode::ShiftRight))
		&& key_input.pressed(KeyCode::Backspace)
	{
		for (ball_entity, _, _) in &mut ball_object_query {
			commands.entity(parent).remove_children(&[ball_entity]);
			commands.entity(ball_entity).despawn();
		}
	}
}

fn update_processes(mut ball_object_query: Query<&mut Ball>, time: Res<Time>) {
	// reset the pressure stat
	for mut ball in &mut ball_object_query {
		ball.pressure_stat = 0.0;
	}

	let mut i = 0;

	while i < ITERATION_COUNT {
		update_ball_position(&mut ball_object_query, &time, ITERATION_DELTA);
		container_collision(&mut ball_object_query);
		// ball_collision_physics(&mut ball_object_query);
		ball_collision_physics_optimised(&mut ball_object_query);
		i += 1;
	}
}

fn update_ball_position(
	ball_object_query: &mut Query<&mut Ball>,
	time: &Res<Time>,
	iteration_delta: f32,
) {
	for mut ball_object in ball_object_query {
		let ball_velocity = ball_object.velocity;
		ball_object.pos += ball_velocity * time.delta_seconds() * iteration_delta;
	}
}

fn update_gravity_velocity(mut ball_object_query: Query<&mut Ball>, time: Res<Time>) {
	for mut ball_object in &mut ball_object_query {
		ball_object.velocity -= GRAVITY * time.delta_seconds();
	}
}

fn container_collision(ball_object_query: &mut Query<&mut Ball>) {
	for mut ball_object in ball_object_query {
		if ball_object.pos.y - ball_object.size < -HALF_DIM.y {
			ball_object.pos.y = -HALF_DIM.y + ball_object.size;
			ball_object.velocity.y = -ball_object.velocity.y * ball_object.elasticity;
		} else if ball_object.pos.y + ball_object.size > HALF_DIM.y {
			ball_object.pos.y = HALF_DIM.y - ball_object.size;
			ball_object.velocity.y = -ball_object.velocity.y * ball_object.elasticity;
		}

		if ball_object.pos.x + ball_object.size > HALF_DIM.x {
			ball_object.pos.x = HALF_DIM.x - ball_object.size;
			ball_object.velocity.x = -ball_object.velocity.x * ball_object.elasticity - 0.1;
		} else if ball_object.pos.x - ball_object.size < -HALF_DIM.x {
			ball_object.pos.x = -HALF_DIM.x + ball_object.size;
			ball_object.velocity.x = -ball_object.velocity.x * ball_object.elasticity + 0.1;
		}
	}
}

fn is_ball_collision(ball1: &Ball, ball2: &Ball) -> bool {
	let distance_vec: Vec3 = ball1.pos - ball2.pos;
	let radius_sum: f32 = ball1.size + ball2.size;

	distance_vec.length_squared() <= radius_sum * radius_sum
}

fn vec2d_to_index(vector: &IVec3) -> i32 {
	BALL_CHUNK_ARRAY_DIM.x * vector.y.clamp(0, BALL_CHUNK_ARRAY_DIM.y)
		+ vector.x.clamp(0, BALL_CHUNK_ARRAY_DIM.x)
}

const BALL_CHUNK_ARRAY_DIM: IVec3 = IVec3::new(
	(SCREENSIZE.x as i32) / CHUNK_SIZE + 1,
	(SCREENSIZE.y as i32) / CHUNK_SIZE + 1,
	0,
);
const BALL_CHUNK_ARRAY_LENGTH: usize =
	(BALL_CHUNK_ARRAY_DIM.x * BALL_CHUNK_ARRAY_DIM.y + BALL_CHUNK_ARRAY_DIM.x) as usize;
use core::range::Range;
fn in_bounds(vector: &IVec3) -> bool {
	Range::from(0..BALL_CHUNK_ARRAY_DIM.x).contains(&vector.x)
		&& Range::from(0..BALL_CHUNK_ARRAY_DIM.y).contains(&vector.y)
}

fn ball_collision_physics_optimised(
	ball_object_query: &mut Query<&mut Ball>,
	// mut query_set: ParamSet<(Query<&mut Ball>, Query<&Ball>)>
) {
	let mut ball_chunk_array: [Vec<Ball>; BALL_CHUNK_ARRAY_LENGTH] =
		std::array::from_fn(|_| Vec::new());

	for ball_object in ball_object_query.iter() {
		let screen_pos: IVec3 = (ball_object.pos + HALF_DIM).as_ivec3();
		let chunk_pos: IVec3 = screen_pos / CHUNK_SIZE;
		let chunk_index: usize = vec2d_to_index(&chunk_pos) as usize;

		ball_chunk_array[chunk_index].push(*ball_object);
	}
	// info!("START COLLISION");
	for mut ball_object1 in ball_object_query.iter_mut() {
		// info!("new ball col");
		let screen_pos: IVec3 = (ball_object1.pos + HALF_DIM).as_ivec3();
		let chunk_pos: IVec3 = screen_pos / CHUNK_SIZE;

		let mut temp_pos: Vec3 = ball_object1.pos;
		let mut temp_vec: Vec3 = ball_object1.velocity;

		for x in -1..2 {
			for y in -1..2 {
				let offset_vec: IVec3 = IVec3::new(-x, -y, 0);
				let check_chunk_pos: IVec3 = chunk_pos + offset_vec;

				// info!("{} {} {}", check_chunk_pos, ball_object1.pos, chunk_pos);

				if !in_bounds(&check_chunk_pos) {
					continue;
				}

				let chunk_index = BALL_CHUNK_ARRAY_DIM.x
					* check_chunk_pos.y.clamp(0, BALL_CHUNK_ARRAY_DIM.y)
					+ check_chunk_pos.x.clamp(0, BALL_CHUNK_ARRAY_DIM.x);

				for ball_object2 in &ball_chunk_array[chunk_index as usize] {
					// same object
					if ball_object1.id == ball_object2.id {
						continue;
					}

					if is_ball_collision(&ball_object1, ball_object2) {
						let ball_rel_vec: Vec3 = ball_object1.pos - ball_object2.pos;

						if ball_rel_vec.length() == 0.0 {
							ball_object1.pos +=
								Vec3::new(rand::thread_rng().gen_range(0.0..1.0), 0.0, 0.0);
							continue;
						}

						// info!("COLLISION HERE");

						let rel_distance: f32 =
							ball_object1.size + ball_object2.size - ball_rel_vec.length();
						let ball_rel_vec_normalised: Vec3 = ball_rel_vec.normalize();
						let average_elastivity: f32 =
							(ball_object1.elasticity + ball_object2.elasticity) / 2.0;

						let d1: Vec3 = ball_rel_vec_normalised * (rel_distance / 2.0);
						let d2: Vec3 =
							ball_rel_vec_normalised * rel_distance * average_elastivity * 10.0;

						temp_pos += d1;
						temp_vec += d2;

						ball_object1.pressure_stat += d1.length_squared();

						if ball_object1.pos.x.is_nan() {
							info!(
								"PROBLEM: {} {} {}",
								chunk_index, chunk_pos, ball_object1.pos
							);
						}
					}
				}
			}
		}

		ball_object1.pos = temp_pos;
		ball_object1.velocity = temp_vec;
	}
}

fn update_ball_draw_position(mut ball_transform_query: Query<(&mut Transform, &Ball)>) {
	for (mut transform, ball_object) in &mut ball_transform_query {
		//info!("{}: {}", counter, ball_object.pos);
		transform.translation = ball_object.pos;
	}
}
