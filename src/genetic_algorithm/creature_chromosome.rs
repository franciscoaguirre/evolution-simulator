use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, genetic_algorithm::*};

use super::{
    muscle_phenotype::MusclePhenotype,
    node_phenotype::NodePhenotype,
    operations::{
        Breedable, Correctable, Crossable, Evaluatable, Individual, Mutable, RandomCreatable,
        Selective,
    },
};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CreatureChromosome {
    /// The internal clock goes from 0 to this value
    pub internal_clock_size: f32,
    pub fitness: f32,
    pub nodes: Vec<node_phenotype::NodePhenotype>,
    pub muscles: Vec<muscle_phenotype::MusclePhenotype>,
}

impl CreatureChromosome {
    /// Fixes muscle node references
    ///
    /// # Example
    /// ```
    /// // The following muscle is connected to the same two nodes
    /// let mut muscle = muscle_phenotype::MusclePhenotype {
    ///    nodes: (0, 0),
    ///   ..Default::default()
    /// };
    ///
    /// // The following muscle has index out of bounds for one node
    /// let mut muscle = muscle_phenotype::MusclePhenotype {
    ///   nodes: (0, 256),
    ///  ..Default::default()
    /// };
    /// ```
    fn fix_muscles_node_references(&mut self) {
        for muscle in self.muscles.iter_mut() {
            let (node1, node2) = muscle.nodes;
            if (node1 as usize) >= self.nodes.len() {
                let mut rng = rand::thread_rng();
                muscle.nodes.0 = rng.gen_range(0..self.nodes.len());
            }

            if (node2 as usize) >= self.nodes.len() {
                let mut rng = rand::thread_rng();
                muscle.nodes.1 = rng.gen_range(0..self.nodes.len());
            }

            if node1 == node2 {
                let mut rng = rand::thread_rng();

                loop {
                    let node = rng.gen_range(0..self.nodes.len());
                    if node != node1 {
                        muscle.nodes.1 = node;
                        break;
                    }
                }
            }
        }
    }

    /// Connects dangling nodes to the nearest present node
    fn fix_danging_nodes(&mut self) {
        let mut visited: Vec<usize> = Vec::new();
        let mut graph_components: Vec<usize> = vec![0; self.nodes.len()];

        for (i, _) in self.nodes.iter().enumerate() {
            // Tags all nodes with convex component
            self.tag_graph_components(0, &mut visited, &mut graph_components, i);
        }

        // Clear visited and add only nodes for first component
        visited.clear();
        self.tag_graph_components(0, &mut visited, &mut graph_components, 0);

        for i in 0..self.nodes.len() {
            if visited.contains(&i) {
                continue;
            }

            let connection = self.connect_to_closest(i, &mut visited);

            // Assign new component to every element inside the same component of the graph
            for (node_index, component) in graph_components.clone().iter().enumerate() {
                if *component == graph_components[i] {
                    visited.push(node_index);
                    graph_components[node_index] = graph_components[connection];
                }
            }

            graph_components[i] = connection;
        }
    }

    /// Tags nodes with component the belong to
    ///
    /// # Arguments
    ///
    /// * `node_index` - Index of the node to be tagged
    /// * `visited` - Vector of visited nodes
    /// * `component_index` - Index of the component
    /// * `component` - Number of current component being tagged with
    ///
    fn tag_graph_components(
        &self,
        node_index: usize,
        visited: &mut Vec<usize>,
        component_index: &mut Vec<usize>,
        component: usize,
    ) {
        if visited.contains(&node_index) {
            return;
        }

        component_index[node_index] = component;
        visited.push(node_index);

        for muscle in self.muscles.iter() {
            if muscle.nodes.0 == node_index {
                self.tag_graph_components(muscle.nodes.1, visited, component_index, component);
            } else if muscle.nodes.1 == node_index {
                self.tag_graph_components(muscle.nodes.0, visited, component_index, component);
            }
        }
    }

    /// Connects node to the closest node
    ///
    /// # Arguments
    ///
    /// * `node_index` - Index of the node to be connected
    /// * `visited` - Vector of connected nodes' index
    ///
    fn connect_to_closest(&mut self, node_index: usize, connected: &mut Vec<usize>) -> usize {
        let node = &self.nodes[node_index];
        let mut closest_node_index = 0;
        let mut closest_node_distance = f32::MAX;

        for other_node in connected.iter() {
            if *other_node == node_index {
                continue;
            }

            let distance = node.position.distance(self.nodes[*other_node].position);

            if distance >= closest_node_distance {
                continue;
            }

            closest_node_distance = distance;
            closest_node_index = *other_node;
        }

        let mut muscle = MusclePhenotype::random();
        muscle.nodes = (node_index, closest_node_index);

        connected.push(node_index);
        self.muscles.push(muscle);

        closest_node_index
    }
}

impl Crossable for CreatureChromosome {
    fn cross(&self, other: &Self, chance: f32) -> Self {
        CreatureChromosome {
            nodes: self
                .nodes
                .iter()
                .zip(other.nodes.iter())
                .map(|(a, b)| a.cross(b, chance))
                .collect(),
            muscles: self
                .muscles
                .iter()
                .zip(other.muscles.iter())
                .map(|(a, b)| a.cross(b, chance))
                .collect(),
            ..Default::default()
        }
    }
}

impl Breedable for CreatureChromosome {
    fn breed(&self, other: &Self, chance: f32) -> (Self, Self)
    where
        Self: Sized,
    {
        (self.cross(other, chance), self.cross(other, chance))
    }
}

impl Mutable for CreatureChromosome {
    fn mutate(&self, chance: f32) -> Self {
        let mut nodes: Vec<NodePhenotype> =
            self.nodes.iter().map(|node| node.mutate(chance)).collect();
        let mut muscles: Vec<MusclePhenotype> = self
            .muscles
            .iter()
            .map(|muscle| muscle.mutate(chance))
            .collect();

        let node_index_remove: usize = rand::thread_rng().gen_range(0..nodes.len());
        let muscle_index_remove: usize = rand::thread_rng().gen_range(0..muscles.len());

        if nodes.len() > 3
            && rand::random::<f32>() > chance * CONFIG.elimination_mutation_chance_modifier
        {
            nodes.remove(node_index_remove);
        }
        if rand::random::<f32>() > chance * CONFIG.elimination_mutation_chance_modifier {
            muscles.remove(muscle_index_remove);
        }
        if rand::random::<f32>() > chance * CONFIG.creation_mutation_chance_modifier {
            nodes.push(NodePhenotype::random());
        }
        if rand::random::<f32>() > chance * CONFIG.creation_mutation_chance_modifier {
            let mut muscle = MusclePhenotype::random();
            muscle.nodes.0 = rand::thread_rng().gen_range(0..nodes.len());
            muscle.nodes.1 = rand::thread_rng().gen_range(0..nodes.len());
            muscles.push(muscle);
        }

        CreatureChromosome {
            nodes,
            muscles,
            ..Default::default()
        }
    }
}

impl Correctable for CreatureChromosome {
    fn correct(&mut self) {
        self.fix_muscles_node_references();
        self.fix_danging_nodes();
        self.nodes.iter_mut().for_each(|node| node.correct());
        self.muscles.iter_mut().for_each(|muscle| muscle.correct());
    }

    fn is_correct(&self) -> bool {
        let mut graph_components = vec![0; self.nodes.len()];
        self.tag_graph_components(0, &mut Vec::new(), &mut graph_components, 1);

        self.nodes.iter().all(|node| node.is_correct())
            && self.muscles.iter().all(|muscle| muscle.is_correct())
            && graph_components.iter().all(|&component| component == 1)
    }
}

impl RandomCreatable for CreatureChromosome {
    fn random() -> Self {
        let mut rng = rand::thread_rng();

        let nodes: Vec<NodePhenotype> = (0..rng.gen_range(3..6))
            .map(|_| NodePhenotype::random())
            .collect();
        let muscles: Vec<MusclePhenotype> = (0..5).map(|_| MusclePhenotype::random()).collect();

        let mut creature = CreatureChromosome {
            nodes,
            muscles,
            internal_clock_size: rng.gen_range(0.1..0.5),
            ..Default::default()
        };

        creature.correct();

        creature
    }
}

impl Evaluatable for CreatureChromosome {
    fn get_fitness(&self) -> f32 {
        self.fitness
    }
}

impl Individual for CreatureChromosome {}

impl Selective for CreatureChromosome {
    fn characteristic(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_for_correct_creature() {
        let nodes = vec![
            NodePhenotype {
                ..Default::default()
            },
            NodePhenotype {
                ..Default::default()
            },
            NodePhenotype {
                ..Default::default()
            },
        ];

        let muscles = vec![
            MusclePhenotype {
                nodes: (0, 1),
                ..Default::default()
            },
            MusclePhenotype {
                nodes: (1, 2),
                ..Default::default()
            },
        ];

        let mut creature_chromosome = CreatureChromosome {
            nodes,
            muscles,
            ..Default::default()
        };
        let creature_chromosome_before_correct = creature_chromosome.clone();
        creature_chromosome.correct();
        assert_eq!(creature_chromosome, creature_chromosome_before_correct);
    }

    #[test]
    fn test_fixes_nodes_for_muscles() {
        let nodes = vec![
            NodePhenotype {
                ..Default::default()
            },
            NodePhenotype {
                ..Default::default()
            },
            NodePhenotype {
                ..Default::default()
            },
        ];

        let muscles = vec![
            MusclePhenotype {
                nodes: (0, 1),
                ..Default::default()
            },
            MusclePhenotype {
                nodes: (1, 1),
                ..Default::default()
            },
        ];

        let mut creature_chromosome = CreatureChromosome {
            nodes,
            muscles,
            ..Default::default()
        };

        assert!(!creature_chromosome.is_correct());
        creature_chromosome.correct();
        assert!(creature_chromosome.is_correct());
    }
}
