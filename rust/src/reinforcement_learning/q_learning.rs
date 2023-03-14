use plotters::prelude::*;
use rand::Rng;

pub struct QLearningAgent {
	random_move_probability: f64,
	q_table: [[f64; 4] ;25],
	discount: f64,
	learning_rate: f64,
	pub curve_dataset: Vec<i32>,
}

impl QLearningAgent {
	pub fn new(random_move_probability: f64) -> Self {
		QLearningAgent {
			random_move_probability,
			q_table: [[0.0; 4] ;25],
			discount: 0.9,
			learning_rate: 0.1,
			curve_dataset: Vec::<i32>::new(),
		}
	}

	pub fn choose_action(&mut self, curr_position: usize, available_actions: &Vec<String>) -> usize {
		let mut rng = rand::thread_rng();
		if rng.gen::<f64>() < self.random_move_probability {
			return rng.gen_range(0..available_actions.len());
		}
		
		self.find_max_in_q_table(self.q_table[curr_position]).1
	}

	pub fn update_q_table(&mut self, reward: i32, prev_position: usize, chosen_action: usize, new_position: usize) {
		self.q_table[prev_position][chosen_action] = 
		self.q_table[prev_position][chosen_action] + self.learning_rate *
		(reward as f64 + self.discount * self.find_max_in_q_table(self.q_table[new_position]).0 -
		self.q_table[prev_position][chosen_action])
	}

	pub fn get_q_table(&mut self) -> [[f64; 4] ;25] {
		self.q_table
	}

	pub fn add_to_curve(&mut self, reward: i32) {
		self.curve_dataset.push(reward);
	}

	pub fn plot_learning_curve(&self) {
		let mut curve_dataset_pos = vec![];
		for (ep, data) in self.curve_dataset.iter().enumerate() {
			curve_dataset_pos.push((ep as i32, *data));
		}

		let root = BitMapBackend::new("plots/q_learning_curve.png", (1024, 768)).into_drawing_area();

		root.fill(&WHITE);

		let mut chart = ChartBuilder::on(&root)
			.caption("Q Learning Agent", ("sans-serif", 40).into_font())
			.margin(5)
			.x_label_area_size(80)
			.y_label_area_size(80)
			.build_cartesian_2d(0i32..500i32, -120i32..5i32)
			.unwrap();

		chart.configure_mesh()
			.disable_mesh()
			.x_desc("Episode")
            .y_desc("Reward")
			.draw()
			.unwrap();
		chart.draw_series(LineSeries::new(
			curve_dataset_pos,
			&BLUE,
		)).unwrap();
		root.present().expect("Plot Err!");
	}

	fn find_max_in_q_table(&mut self, q_table_row: [f64; 4]) -> (f64, usize) {
		let mut max_q_value = q_table_row[0];
		let mut index = 0;
		for (i, n) in q_table_row.iter().enumerate() {
			if n > &max_q_value {
				max_q_value = *n;
				index = i;
			}
		}
		(max_q_value, index)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_build_agent() {
		let agent = QLearningAgent::new(0.2);
		assert_eq!(agent.random_move_probability, 0.2);
	}

	#[test]
	fn test_add_to_curve() {
		let mut agent = QLearningAgent::new(0.2);
		agent.add_to_curve(10);
		assert_eq!(agent.curve_dataset, vec![10])
	}
}