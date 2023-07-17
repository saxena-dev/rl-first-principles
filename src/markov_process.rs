use rand::seq::IteratorRandom;

use crate::distribution::{Distribution, FiniteDistribution};
use std::collections::binary_heap::Iter;
use std::collections::HashMap;
use std::hash::Hash;

// [ States ] ===========================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Terminal<S> {
    state: S,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonTerminal<S> {
    state: S,
}

#[derive(Clone, Copy, Debug)]
pub enum State<S> {
    Terminal(Terminal<S>),
    NonTerminal(NonTerminal<S>),
}

impl<S> State<S> {
    fn on_non_terminal<F, X>(self, f: F, default: X) -> X
    where
        F: Fn(&NonTerminal<S>) -> X,
    {
        match self {
            State::Terminal(_) => default,
            State::NonTerminal(state) => f(&state),
        }
    }
}

// --------------------------------------------------------------------------------------

// Trait: `MarkovProcess` ===============================================================

pub trait MarkovProcess<S> {
    fn transition<D>(self, state: NonTerminal<S>) -> D
    where
        D: Distribution<S>;

    fn simulate_iter<D, Z>(self, start_state_dist: D) -> Z
    where
        D: Distribution<S>,
        Z: Iterator<Item = State<S>>;

    fn traces_iter<D, Y, Z>(self, start_state_dist: D) -> Z
    where
        D: Distribution<S>,
        Y: Iterator<Item = State<S>>,
        Z: Iterator<Item = Y>;
}

// --------------------------------------------------------------------------------------

// Struct: `FiniteMarkovProcess` ========================================================

/// A markov process with finite state space which allows the use of tabular methods to
/// (dynamic programming) work with the process.
pub struct FiniteMarkovProcess<S, X>
where
    S: Eq + Hash,
    X: FiniteDistribution<S>,
{
    non_terminal_states: Vec<NonTerminal<S>>,
    transition_map: HashMap<NonTerminal<S>, X>,
}

impl<S, X> FiniteMarkovProcess<S, X>
where
    S: Eq + Hash,
    X: FiniteDistribution<S>,
{
    pub fn get_transition_matrix(self) -> Vec<f64> {
        todo!()
    }

    pub fn get_stationary_distribution(self) -> X {
        todo!()
    }
}

impl<S, X> MarkovProcess<S> for FiniteMarkovProcess<S, X>
where
    S: Eq + Hash,
    X: FiniteDistribution<S>,
{
    fn transition<D>(self, state: NonTerminal<S>) -> D
    where
        D: Distribution<S>,
    {
        todo!()
    }

    fn simulate_iter<D, Z>(self, start_state_dist: D) -> Z
    where
        D: Distribution<S>,
        Z: Iterator<Item = State<S>>,
    {
        todo!()
    }

    fn traces_iter<D, Y, Z>(self, start_state_dist: D) -> Z
    where
        D: Distribution<S>,
        Y: Iterator<Item = State<S>>,
        Z: Iterator<Item = Y>,
    {
        todo!()
    }
}
// --------------------------------------------------------------------------------------

// Trait: `MarkovRewardProcess` =========================================================
pub trait MarkovRewardProcess<S>: MarkovProcess<S> {
    fn transition_reward<D>(self, state: NonTerminal<S>) -> D
    where
        D: Distribution<(State<S>, f64)>;
}

// --------------------------------------------------------------------------------------
