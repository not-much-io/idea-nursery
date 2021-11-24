#[cfg(test)]
mod tests {
    use rand::Rng;
    use rayon::prelude::*;
    use std::{env, fs::File, io::Write, path::PathBuf};

    const SAMPLE_SIZE: i32 = 1000000;

    fn d(die_size: &i32) -> i32 {
        rand::thread_rng().gen_range(1..=*die_size)
    }

    fn resolve_to_story_progress(
        spec_die_size: &i32,
        action_modifier: &i32,
        player_entity_level: &i32,
        world_entity_level: &i32,
    ) -> i32 {
        let d6_result = d(&6);
        let dn_result = d(spec_die_size);
        let roll_result = d6_result + dn_result;

        if roll_result == 2 {
            return -(world_entity_level + 1) * 2;
        }
        if roll_result >= 12 {
            return player_entity_level * 2;
        }

        if roll_result + action_modifier <= 6 {
            return -world_entity_level;
        }
        if roll_result + action_modifier <= 9 {
            return player_entity_level - world_entity_level;
        }
        if roll_result + action_modifier >= 10 {
            return *player_entity_level;
        }

        panic!("should never happen");
    }

    #[derive(Clone, Copy)]
    struct SimulationResult {
        ability_modifier:    i32,
        mastery_modifier:    i32,
        player_entity_level: i32,
        world_entity_level:  i32,
        average_progress:    f64,
    }

    fn run_simulations(
        ability_modifiers: impl IntoIterator<Item = i32> + Clone,
        mastery_modifiers: impl IntoIterator<Item = i32> + Clone,
        player_entity_levels: impl IntoIterator<Item = i32>,
        world_entity_levels: impl IntoIterator<Item = i32> + Clone,
    ) -> Vec<SimulationResult> {
        let mut results = Vec::new();

        for player_entity_level in player_entity_levels {
            for world_entity_level in world_entity_levels.clone() {
                for mastery_modifier in mastery_modifiers.clone() {
                    for ability_modifier in ability_modifiers.clone() {
                        let average_progress = (0..SAMPLE_SIZE)
                            .into_par_iter()
                            .map(|_| {
                                resolve_to_story_progress(
                                    &mastery_modifier,
                                    &ability_modifier,
                                    &player_entity_level,
                                    &world_entity_level,
                                )
                            })
                            .sum::<i32>() as f64
                            / SAMPLE_SIZE as f64;

                        results.push(SimulationResult {
                            ability_modifier,
                            mastery_modifier,
                            player_entity_level,
                            world_entity_level,
                            average_progress,
                        });
                    }
                }
            }
        }

        results
    }

    fn export_simple_progression_plots(results: Vec<SimulationResult>) {
        let ability_progression_plot = results
            .clone()
            .into_iter()
            .filter(|res| {
                res.player_entity_level == 1
                    && res.world_entity_level == 1
                    && res.mastery_modifier == 6
            });
        export_data(ability_progression_plot, "ability_progression");

        let mastery_progression_plot = results
            .clone()
            .into_iter()
            .filter(|res| {
                res.player_entity_level == 1
                    && res.world_entity_level == 1
                    && res.ability_modifier == 0
            });
        export_data(mastery_progression_plot, "mastery_progression");
    }

    fn export_data(data: impl Iterator<Item = SimulationResult>, plot_name: &str) {
        let report_path = format!(
            "{}/data_exports/{}",
            env::current_dir()
                .expect("unable to get current directory")
                .to_string_lossy(),
            format!("{}.csv", plot_name)
        );
        let mut file = File::create(report_path).expect("unable to open csv file");

        file.write_all("avg_story_progession,player_entity_progression".as_bytes())
            .expect("failed to write header to csv file");
        for result in data {
            file.write_all(
                format!(
                    "\n{},{}",
                    result.average_progress,
                    format!(
                        "d{}+{}@Lvl{}",
                        result.mastery_modifier,
                        result.ability_modifier,
                        result.player_entity_level
                    ),
                )
                .as_bytes(),
            )
            .expect("failed to write line to csv file");
        }
    }

    #[test]
    fn run_simulation() {
        let mut simulation_results = run_simulations(0..=3, (6..=12).step_by(2), 1..=9, 1..=9);
        simulation_results.sort_by(|pr1, pr2| {
            pr1.average_progress
                .partial_cmp(&pr2.average_progress)
                .expect("failed partial_cmp of floats")
        });

        export_simple_progression_plots(simulation_results.to_vec());
    }
}
