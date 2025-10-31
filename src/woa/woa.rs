use super::super::entity::graph::Graph;
use super::whale::Whale;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::f64::consts::PI;



#[derive(Debug,Clone)]
pub struct WOA {
    size_population : usize,
    population : Vec<Whale>,
    idx_best_whale : usize,
    max_iteration : usize,
    convergence_curve : Vec<f64>,
    random : StdRng,
}

impl WOA {
    pub fn new(size_population : usize, max_iteration : usize, lb : f64, ub : f64, seed: u64, k : usize, graph : &Graph) -> Self {
        let mut random = StdRng::seed_from_u64(seed);
        let mut population: Vec<Whale> = vec![];
        for i in 0..size_population {
            let whale = Whale::new(graph, lb, ub, &mut random, k);
            population.push(whale);
        }
        let mut idx_best_whale : usize = 0;
        let mut best_whale = population[idx_best_whale].get_cost(graph);
        println!("Initialized WOA with population size: {}, max iterations: {}, lb: {}, ub: {}, seed: {}, k: {}", size_population, max_iteration, lb, ub, seed, k);
        println!("Initial best whale cost: {}", best_whale);
        for i in 0..size_population {
            println!("W{}: {}", i, population[i].cost);
            if population[i].get_cost(graph) < best_whale {
                idx_best_whale = i;
                best_whale = population[i].get_cost(graph);
            }
        }

        let convergence_curve = vec![f64::INFINITY;max_iteration];
        Self {
            size_population,
            population,
            idx_best_whale,
            max_iteration,
            convergence_curve,
            random,
        } 
    }

    pub fn get_best_whale(&self) -> Whale {
        self.population[self.idx_best_whale].clone()
    }

    pub fn get_idx_best_whale(&self) -> usize {
        self.idx_best_whale
    }

    pub fn get_convergence(&self) -> Vec<f64> {
        self.convergence_curve.clone()
    }

    pub fn woa(&mut self, graph : &Graph) -> usize {
        for i in 0..self.max_iteration {
            //println!("Iteration {}: Best cost = {}", i, self.population[self.idx_best_whale].cost);
            self.convergence_curve[i] = self.population[self.idx_best_whale].cost; 

            let i_f64 = i as f64;
            let max_iteration_f64 = self.max_iteration as f64;

            let a1 = 2.0 - i_f64 * (2.0 / max_iteration_f64); 
            let a2 = -1.0 + i_f64 * (-1.0 / max_iteration_f64);
            
            self.update_position(graph,a1,a2);
            self.recalculate_cost(graph); 
        }

        for i in 0..self.size_population {
            println!("W{}: {}", i, self.population[i].cost);
        }

        self.idx_best_whale
    }

    fn calculate_new_position(
        actual_whale_pos: f64, 
        best_solution_pos: f64,
        random_whale_pos: f64, 
        a: f64, 
        c: f64, 
        b: f64, 
        l: f64, // Coeficiente l
        p: f64,
    ) -> f64 {
        let new_position : f64;
        if p < 0.5 {
            if a.abs() < 1.0 {
                let d_leader = c * best_solution_pos - actual_whale_pos;
                new_position = best_solution_pos - a * d_leader;
            } else {
                let d_random = c * random_whale_pos - actual_whale_pos;
                new_position = random_whale_pos - a * d_random;
            }
        } else {
            let d_leader = best_solution_pos - actual_whale_pos;
            new_position = d_leader * (b * l).exp() * (2.0 * PI * l).cos() + actual_whale_pos;
        }

        new_position
    }

    fn update_position(&mut self, graph : &Graph, a1 : f64, a2 : f64) {
        let b = 1.0;        
        let best_whale_idx = self.idx_best_whale;
        let best_whale_ref = self.population[best_whale_idx].clone();
        let population_size = self.size_population;    

        for i in 0..population_size  {
            //println!("Updating position of whale {}...", i);
            // Recalcular parámetros r1, r2, a, c, l, p para cada ballena
            let r1 : f64 = self.random.gen_range(0.0..1.0); // Factor r1 en [0, 1]
            let r2 : f64 = self.random.gen_range(0.0..1.0); // Factor r2 en [0, 1]
            let a = 2.0 * a1 * r1 - a1;
            let c = 2.0 * r2;
            let l = a2 + (a1 - a2) * self.random.gen_range(0.0..1.0); // Coeficiente 'l' (entre a2 y a1)
            let p = self.random.gen_range(0.0..1.0);

            let random_idx = if p < 0.5 && a.abs() >= 1.0 {
                self.random.gen_range(0..population_size)
            } else {
                best_whale_idx
            };

            let random_pos_ref = self.population[random_idx].clone();


            // 1. Elegir y actualizar la posición del nodo a REMOVER
            let idx_remove_node = self.population[i].get_index_node_in_tree(&mut self.random);
            let actual_pos_remove = self.population[i].get_position(idx_remove_node);
            let best_pos_remove = best_whale_ref.get_position(idx_remove_node);
            let random_pos_remove = random_pos_ref.get_position(idx_remove_node);

            let mut new_pos_remove = WOA::calculate_new_position(
                actual_pos_remove,
                best_pos_remove,
                random_pos_remove,
                a, c, b, l, p,
            );

            //println!("New position for node {} to remove: {}", idx_remove_node, new_pos_remove);

            let actual_whale = &mut self.population[i];
            
            // Clamping (Ajuste de límites)
            new_pos_remove = new_pos_remove.clamp(actual_whale.lb, actual_whale.ub);
            actual_whale.set_position(idx_remove_node, new_pos_remove);
            actual_whale.set_node(idx_remove_node, false);

            // 2. Elegir y actualizar la posición del nodo a AÑADIR
            let mut idx_new_node = actual_whale.get_index_node_nin_tree(&mut self.random);

            while !actual_whale.get_node(idx_new_node).1 {
                let actual_pos_new = actual_whale.get_position(idx_new_node);
                let best_pos_new = best_whale_ref.get_position(idx_new_node);
                let random_pos_new = if i == random_idx {
                    actual_whale.get_position(idx_new_node)
                } else {
                    random_pos_ref.get_position(idx_new_node)
                };

                let new_position = WOA::calculate_new_position(
                        actual_pos_new,
                        best_pos_new,
                        random_pos_new,
                        a, c, b, l, p,
                );
                
                // Clamping (Ajuste de límites)
                let clamped_position = new_position.clamp(actual_whale.lb, actual_whale.ub);
                
                // Decisión binaria (Sigmoid)
                let sigmoid_value = 1.0 / (1.0 + (-clamped_position).exp()); 
                let limit = 0.5; // Decisión binaria no estocástica (como en el original)

                if sigmoid_value >= limit { 
                    // Aceptar nueva posición y nodo
                    actual_whale.set_position(idx_new_node, clamped_position);
                    actual_whale.set_node(idx_new_node, true);
                } else {
                    // Rechazar, buscar un nuevo nodo a añadir y reintentar
                    idx_new_node = actual_whale.get_index_node_nin_tree(&mut self.random);
                }
            }
            //println!("New position for node {} to add: {}", idx_new_node, actual_whale.get_position(idx_new_node));

            // 3. Reconstruir/Actualizar el árbol (K-MST)
            let new_node = actual_whale.get_node(idx_new_node).0.clone();
            let remove_node = actual_whale.get_node(idx_remove_node).0.clone();
            let _ = actual_whale.tree.get_neighbor(graph, &new_node, &remove_node);
            actual_whale.tree.recover_solution();
            actual_whale.cost = actual_whale.tree.get_cost(graph);
        }
    } 

    fn adjust_position(&mut self, agent_index : usize) {
        let agent = &mut self.population[agent_index];
        let size_agent = agent.get_len_position();
        for i in 0..size_agent {
            let clamped_position = agent.get_position(i).clamp(agent.lb, agent.ub);
            agent.set_position(i, clamped_position);
        }
    }

    fn recalculate_cost(&mut self, graph : &Graph) {
        let mut current_best_cost = self.population[self.idx_best_whale].cost;
        
        for i in 0..self.population.len() {
            self.adjust_position(i); 
            let new_value = self.population[i].get_cost(graph);
            
            if new_value < current_best_cost {
                self.idx_best_whale = i;
                current_best_cost = new_value;
            } 
        }
    }

}
