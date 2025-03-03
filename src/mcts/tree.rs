use std::fmt::Debug;

use super::{action_list::ActionList, edge::Edge, node::Node, state::State};

const TREE_SIZE: usize = 500_000;

pub struct Tree<T: State> {
    nodes: Vec<Node<T>>,
    index: usize,
}

impl<T> Tree<T>
where
    T: State + Clone,
{
    pub fn new() -> Self {
        Tree {
            nodes: Vec::with_capacity(TREE_SIZE),
            index: 0,
        }
    }

    pub fn size(&mut self) -> (usize, usize, usize) {
        (self.index, self.nodes.len(), self.nodes.capacity())
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.nodes = Vec::with_capacity(TREE_SIZE);
    }

    pub fn add_node(
        &mut self,
        state: &T,
        edge: Option<Edge<T::Action, usize>>,
        parent_id: Option<usize>,
    ) -> usize {
        let node_id = self.index;

        if let Some(parent_id) = parent_id {
            self.nodes[parent_id].add_child(node_id);
        }

        let is_terminal = state.is_terminal();
        let node = Node::new(edge, parent_id, is_terminal);

        self.nodes.push(node);
        self.index += 1;

        node_id
    }

    pub fn select(&self, mut node_id: usize, state: &mut T) -> usize {
        let mut legal_actions = state.possible_actions();

        // TODO: replace with state.is_terminal, so we can remove Tree::is_terminal
        // and Node::is_terminal
        while !self.is_terminal(node_id) && self.is_fully_expanded(node_id, &legal_actions) {
            node_id = self.uct_select_child(node_id, &legal_actions).unwrap();
            //node_id = self.select_random_child(node_id, &legal_actions).unwrap();

            let action = self.get_edge(node_id).unwrap().action();
            state.apply_action(action);

            if state.is_terminal() {
                break;
            }
            legal_actions = state.possible_actions();
        }

        node_id
    }

    fn select_random_child(&self, node_id: usize, legal_actions: &T::ActionList) -> Option<usize> {
        let options = self.nodes[node_id]
            .child_ids()
            .filter(|&&child_id| legal_actions.has(&self.get_edge(child_id).unwrap().action()))
            .collect::<Vec<_>>();
        if options.is_empty() {
            None
        } else {
            let i = romu::range_usize(0..options.len());
            Some(*options[i])
        }
    }

    fn uct_select_child(&self, node_id: usize, legal_actions: &T::ActionList) -> Option<usize> {
        let n = self.nodes[node_id].num_sims();

        self.nodes[node_id]
            .child_ids()
            .filter(|&&child_id| legal_actions.has(&self.get_edge(child_id).unwrap().action()))
            .max_by(|&&x, &&y| {
                self.nodes[x]
                    .uct_score(n)
                    .partial_cmp(&self.nodes[y].uct_score(n))
                    .unwrap()
            })
            .cloned()
    }

    pub fn expand(&mut self, node_id: usize, state: &mut T) -> usize {
        if state.is_terminal() {
            return node_id;
        }

        let legal_actions = state.possible_actions();

        match self.nodes[node_id].pop_action(&legal_actions) {
            None => node_id,
            Some(action) => {
                let actor = state.turn();
                let edge = Edge::new(action.clone(), actor);

                state.apply_action(action);
                self.add_node(state, Some(edge), Some(node_id))
            }
        }
    }

    pub fn best_action(&self, node_id: usize, state: &T) -> Option<T::Action> {
        let legal_actions = state.possible_actions();
        let child_id = self.nodes[node_id]
            .child_ids()
            .filter(|&&child_id| legal_actions.has(&self.get_edge(child_id).unwrap().action()))
            .max_by_key(|&&child_id| self.nodes[child_id].num_sims())
            //.max_by(|&&x, &&y| {
            //    self.nodes[x]
            //        .avg_score()
            //        .partial_cmp(&self.nodes[y].avg_score())
            //        .unwrap()
            //})
            .unwrap();

        self.get_edge(*child_id).map(|e| e.action())
    }

    pub fn update_node(&mut self, node_id: usize, reward: f32) {
        self.nodes[node_id].update(reward);
    }

    pub fn is_fully_expanded(&self, node_id: usize, legal_actions: &T::ActionList) -> bool {
        !self.nodes[node_id].has_untried_actions(legal_actions)
    }

    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.nodes[node_id].parent_id()
    }

    pub fn get_edge(&self, node_id: usize) -> Option<Edge<T::Action, usize>> {
        self.nodes[node_id].edge()
    }

    pub fn is_terminal(&self, node_id: usize) -> bool {
        self.nodes[node_id].is_terminal()
    }

    pub fn dbg_actions(&self, node_id: usize, state: &T)
    where
        T::Action: Debug,
    {
        let n = self.nodes[node_id].num_sims();
        let legal_actions = state.possible_actions();
        self.nodes[node_id]
            .child_ids()
            .filter(|&&child_id| legal_actions.has(&self.get_edge(child_id).unwrap().action()))
            .for_each(|&child_id| {
                println!(
                    "{:?}: uct: {:?}, sims: {}, score: {}",
                    self.get_edge(child_id).map(|e| e.action()),
                    self.nodes[child_id].uct_score(n),
                    self.nodes[child_id].num_sims(),
                    self.nodes[child_id].avg_score(),
                )
            });
    }
}
