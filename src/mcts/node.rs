use crate::array::Array;

use super::{action_list::ActionList, edge::Edge, state::State};

type ChildArray = Array<usize, 24>;
//type ChildArray = Vec<usize>;

pub struct Node<T: State> {
    edge: Option<Edge<T::Action, usize>>,
    parent_id: Option<usize>,
    //child_ids: Vec<usize>,
    child_ids: ChildArray,
    tried_actions: T::ActionList,
    is_terminal: bool,

    num_sims: usize,
    avails: usize,
    score: f32,
}

impl<T> Node<T>
where
    T: State,
{
    pub fn new(
        edge: Option<Edge<T::Action, usize>>,
        parent_id: Option<usize>,
        is_terminal: bool,
    ) -> Self {
        Node {
            edge,
            parent_id,
            tried_actions: T::ActionList::uninit(),
            //child_ids: Vec::with_capacity(23),
            child_ids: Default::default(),
            num_sims: 0,
            avails: 1,
            score: 0.,
            is_terminal,
        }
    }

    pub fn add_child(&mut self, child_id: usize) {
        self.child_ids.push(child_id)
    }

    pub fn has_untried_actions(&self, legal_actions: &T::ActionList) -> bool {
        !legal_actions.without(&self.tried_actions).is_empty()
    }

    pub fn pop_action(&mut self, legal_actions: &T::ActionList) -> Option<T::Action> {
        let mut actions = legal_actions.without(&self.tried_actions);
        let action = actions.pop_random();

        if let Some(action) = action.clone() {
            self.tried_actions.push(action);
        }

        action
    }

    pub const fn increase_availability(&mut self) {
        self.avails += 1;
    }

    pub fn update(&mut self, reward: f32) {
        self.num_sims += 1;
        self.score += reward;
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    pub fn edge(&self) -> Option<Edge<T::Action, usize>> {
        self.edge.clone()
    }

    pub fn child_ids_ref(&self) -> &ChildArray {
        &self.child_ids
    }

    pub const fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    pub const fn num_sims(&self) -> usize {
        self.num_sims
    }

    pub const fn avg_score(&self) -> f32 {
        self.score / self.num_sims as f32
    }

    pub fn uct_score(&self) -> f32 {
        let n = self.num_sims as f32;
        self.score / n + 4.5 * (2. * (self.avails as f32).ln() / n).sqrt()
    }
}
