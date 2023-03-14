use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

mod reinforcement_learning;
mod sorting;


fn run_sorting() {
    let size = 500;
    let sorted: Vec<i32> = (0..size).collect();
    let mut nums: Vec<i32> = (0..size).collect();

    let mut rng = thread_rng();
    nums.shuffle(&mut rng);

    let start = Instant::now();
    sorting::quicksort::quicksort(&mut nums);
    let duration = start.elapsed();

    assert_eq!(nums, sorted);
    println!("Sorting took {duration:?}.");
}

fn run_q_learning() {
    let episodes = 500;
    let row = 5;
    let col = 5;
    let random_move_probability = 0.2;

    let mut env = reinforcement_learning::gridworld::GridWorld::new(row, col, random_move_probability);
    let mut agent = reinforcement_learning::q_learning::QLearningAgent::new(random_move_probability);
    let actions = env.get_available_actions();

    for e in 0..episodes {
        let mut curr_reward = 0;

        while !env.is_game_over() {
            let prev_position = env.get_agent_position();
            let chosen_action = agent.choose_action(prev_position, &actions);
            let (reward, new_position) = env.make_step(chosen_action);

            agent.update_q_table(reward, prev_position, chosen_action, new_position);
            curr_reward += reward;
        }

        agent.add_to_curve(curr_reward);
        env.reset();
    }
    agent.plot_learning_curve();
    println!("Q Learning Agent trained for {episodes} episodes.")
}

fn main() {
    run_sorting();
    run_q_learning();
}