use rand::Rng;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub struct GridWorld {
	num_row: usize,
	num_col: usize,
	random_move_probability: f64,
	agent_position: usize,
	bomb_position: usize,
	gold_position: usize,
	actions: Vec<String>,
}

impl GridWorld {
	pub fn new(num_row: usize, num_col: usize, random_move_probability: f64) -> Self {
		let mut rng = rand::thread_rng();

		GridWorld {
			num_row,
			num_col,
			random_move_probability,
			// choose agent to start randomly among the first row
			agent_position: rng.gen_range(0..num_row),
			// choose position of bomb and gold
			bomb_position: 18,
			gold_position: 23,
			// specify available actions
			actions: vec![
				"UP".to_string(),
				"RIGHT".to_string(),
				"DOWN".to_string(),
				"LEFT".to_string()
			],
		}
	}

	pub fn make_step(&mut self, mut action_index: usize) -> (i32, usize) {
		// randomly sample action_index if world is stochastic
		let mut rng = rand::thread_rng();
		if rng.gen::<f64>() < self.random_move_probability {
			let mut action_indices = Vec::<usize>::new();
			
			for i in 0..self.actions.len() {
				action_indices.push(i);
			}
			action_indices.remove(
				action_indices
				.iter()
				.position(|x| *x == action_index)
				.expect("not found")
			);
			action_index = *action_indices.choose(&mut rng).unwrap();
		}
		let action = &self.actions[action_index];

		// determine new position and check whether the agent hits a wall
		let old_position = self.agent_position;
		let mut new_position = self.agent_position;
		if action == "UP" {
			let candidate_position = old_position + self.num_col;
			if candidate_position < self.num_row * self.num_col {
				new_position = candidate_position;
			}
		} else if action == "RIGHT" {
			let candidate_position = old_position + 1;
			if candidate_position % self.num_col > 0 {
				new_position = candidate_position;
			}
		} else if action == "DOWN" {
			let candidate_position = old_position.saturating_sub(self.num_col);
			if candidate_position >= 0 {
				new_position = candidate_position;
			}
		} else if action == "LEFT" {
			let candidate_position = old_position.saturating_sub(1);
			if candidate_position % self.num_col < self.num_col - 1 {
				new_position = candidate_position;
			}
		} else {
			panic!("Invalid action!");
		}

		// update env state
		self.agent_position = new_position;

		let mut reward = 0;
		if self.agent_position == self.bomb_position {
			reward -= 10;
		}
		if self.agent_position == self.gold_position {
			reward += 10;
		}
		reward -= 1;

		(reward, new_position)
	}

	pub fn get_agent_position(&mut self) -> usize {
		self.agent_position
	}

	pub fn is_game_over(&mut self) -> bool {
		if self.agent_position == self.bomb_position || self.agent_position == self.gold_position {
			return true;
		}
		false
	}

	pub fn get_available_actions(&mut self) -> Vec<String> {
		self.actions.to_vec()
	}

	pub fn reset(&mut self) {
		let mut rng = rand::thread_rng();
		self.agent_position = rng.gen_range(0..self.num_row) as usize;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_build_env() {
		let env = GridWorld::new(5, 5, 0.2);
		assert_eq!(25, env.num_cell);
		assert_eq!(18, env.bomb_position);
		assert_eq!(23, env.gold_position);
	}

	#[test]
	fn test_total_stochastic_env() {
		let mut env = GridWorld::new(5, 5, 1.0);
		let prev = env.agent_position;
		let (_, next) = env.make_step(2);
		assert_ne!(prev, next);
	}

	#[test]
	fn test_agent_moves() {
		let mut env = GridWorld::new(5, 5, 0.0);
		let prev = env.agent_position;
		let (_, next) = env.make_step(0);
		assert_ne!(prev, next);
	}

	#[test]
	fn test_agent_hits_wall() {
		let mut env = GridWorld::new(5, 5, 0.0);
		let prev = env.agent_position;
		let (_, next) = env.make_step(2);
		assert_eq!(prev, next);
	}

	#[test]
	#[should_panic]
	fn test_invalid_action() {
		let mut env = GridWorld::new(5, 5, 0.0);
		env.make_step(5);
	}

	#[test]
	fn test_reset() {
		let mut env = GridWorld::new(5, 5, 0.0);
		let (_, prev) = env.make_step(0);
		env.reset();
		assert_ne!(prev, env.agent_position);
	}

	#[test]
	fn test_get_available_actions() {
		let env = GridWorld::new(5, 5, 0.2);
		let actions = vec![
				"UP".to_string(),
				"RIGHT".to_string(),
				"DOWN".to_string(),
				"LEFT".to_string(),
			];
		assert_eq!(&actions, env.get_available_actions());
	}

	#[test]
	fn test_reached_terminal_state() {
		let mut rng = rand::thread_rng();
		let mut env = GridWorld::new(5, 5, 0.2);
		let num_actions = env.get_available_actions().len();

		while !env.is_game_over() {
			let step = rng.gen_range(0..num_actions);
			env.make_step(step);
		}

		assert_eq!(true, env.is_game_over());
	}
}