use std::collections::HashMap;
use std::hash::Hash;

// Trait: `Distribution` ================================================================

/// A probability distribution that can be sampled
pub trait Distribution<T> {
    /// Returns a random sample from the distribution.
    fn sample(&self) -> T;

    /// Create an iterator that generates random values of `T`.
    ///
    /// Note: This function takes `Self` by value.
    fn sample_iter(self) -> DistIter<Self, T>
    where
        Self: Sized,
    {
        DistIter {
            dist: self,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Apply a function to the outcomes of this distribution by mapping the output of
    /// `Self` through the closure `F`.
    ///
    /// Note: This function takes `Self` by value.
    fn map<Func, U>(self, f: Func) -> DistMap<Self, T, Func, U>
    where
        Self: Sized,
        Func: Fn(T) -> U,
    {
        DistMap {
            dist: self,
            func: f,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Apply a function that returns a distribution to the outcomes of this distribution.
    /// Allows expression of 'dependent random variables'.
    ///
    /// Note: This function takes `Self` by value.
    fn apply<D, Func, U, X>(self, f: Func) -> SampledDist<Self, T, Func, U>
    where
        Self: Sized,
        D: Distribution<T>,
        X: Distribution<U>,
        Func: Fn() -> X,
    {
        SampledDist {
            dist: self,
            func: f,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Return the expectation of f(X) where X is the random variable for
    /// the distribution and f is an arbitrary function from X to f64.
    fn expectation<Func>(&self, f: Func, sample_size: usize) -> f64
    where
        Func: Fn(&T) -> f64;
}

// --------------------------------------------------------------------------------------

// Struct: `DistIter` ===================================================================

/// An iterator that generates random values of `T` with distribution `D`.
///
/// This struct is created by the [`Distribution::sample_iter`] method.
#[derive(Debug)]
pub struct DistIter<D, T>
where
    D: Distribution<T>,
{
    dist: D,
    _phantom: std::marker::PhantomData<T>,
}

impl<D, T> Iterator for DistIter<D, T>
where
    D: Distribution<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.dist.sample())
    }
}

// --------------------------------------------------------------------------------------

// Struct: `DistMap` ====================================================================

/// A distribution of values of type `U` derived from the distribution `D` by mapping its
/// output of type `T` through the closure `F`.
///
/// This struct is created by the [`Distribution::map`] method.
#[derive(Debug)]
pub struct DistMap<D, T, F, U> {
    dist: D,
    func: F, // : Fn(T) -> U
    _phantom: std::marker::PhantomData<fn(T) -> U>,
}

impl<D, T, F, U> Distribution<U> for DistMap<D, T, F, U>
where
    D: Distribution<T>,
    F: Fn(T) -> U,
{
    fn sample(&self) -> U {
        (self.func)(self.dist.sample())
    }

    fn expectation<Func>(&self, f: Func, sample_size: usize) -> f64
    where
        Func: Fn(&U) -> f64,
    {
        let sum: f64 = (0..sample_size).map(|_| f(&self.sample())).sum();
        sum / sample_size as f64
    }
}

// --------------------------------------------------------------------------------------

// Struct: `SampledDist` ================================================================

/// A distribution defined by a function to sample it
pub struct SampledDist<D, T, F, U> {
    dist: D,
    func: F, // : Fn(T) -> Distribution<U>
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<D, T, F, U, X> Distribution<U> for SampledDist<D, T, F, U>
where
    D: Distribution<T>,
    X: Distribution<U>,
    F: Fn(T) -> X,
{
    fn sample(&self) -> U {
        (self.func)(self.dist.sample()).sample()
    }

    /// Return a sampled approximation of the expectation of f(X) for some f
    fn expectation<Func>(&self, f: Func, sample_size: usize) -> f64
    where
        Func: Fn(&U) -> f64,
    {
        let sum: f64 = (0..sample_size).map(|_| f(&self.sample())).sum();
        sum / sample_size as f64
    }
}

// Trait: `FiniteDistribution` ==========================================================

/// A probability distribution with a finite number of outcomes,
/// which means we can render it as a PDF or CDF table
pub trait FiniteDistribution<T>: Distribution<T>
where
    T: Eq + Hash,
{
    /// Returns a tabular representation of the probability density function (PDF) for
    /// this distribution.
    fn table(&self) -> &HashMap<&T, f64>;

    /// Returns the probability of the given outcome according to this distribution.
    fn probability(&self, outcome: &T) -> f64 {
        self.table()
            .get(outcome)
            .map(|x| x.to_owned())
            .unwrap_or(0.0)
    }

    // Calculate the expected value of the distribution, using the given function
    fn expectation<Func>(&self, f: Func, sample_size: usize) -> f64
    where
        Self: Sized,
        Func: Fn(&T) -> f64,
    {
        let sum: f64 = self.table().into_iter().map(|(&k, &v)| v * f(k)).sum();
        sum / sample_size as f64
    }
}

// --------------------------------------------------------------------------------------

// [ Finite Distributions ] =============================================================

// pub struct Categorical<'a, A> {
//     probabilities: HashMap<&'a A, f64>,
// }

// impl<'a, A: Eq + Hash> Distribution<A> for Categorical<'a, A> {
//     fn sample(&self) -> A {
//         let (a, _) = self.probabilities.iter().next().unwrap();
//         *a.clone()
//     }
// }

// impl<'a, A: Eq + Hash> FiniteDistribution<A> for Categorical<'a, A> {
//     fn table(&self) -> &HashMap<&A, f64> {
//         &self.probabilities
//     }
// }

// --------------------------------------------------------------------------------------
