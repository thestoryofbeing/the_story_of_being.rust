//! Representation / Time
//!
//! functions / time add a new fundamental dimension / variable
//!   the parameters divide space 
//!   table vs row of table
//! depends, causes, controls
//! relation, ordering

//- USE ------------------------------------------------------------------------

use super::space::Object;

//- DEF ------------------------------------------------------------------------

//- DEF / FUNCTION -------------------------------------------------------------
//- ----------------------------------------------------------------------------

// tables and database correlation??
// functions as vector spaces?
//
// func with no params? is just object. not a function
// id function, A -> A
//
// maybe function here is diff than math function
// more like db table. any column can map to any other set
//
// so what is natural ordering?

// functions here are more like function types
// they are just mappings from domain -> codomain
// so we can use dimension for mapping
// hmm this actually makes a lot of sense

/// Function
/// table
pub trait Function<ID> : Object<ID>
{
    fn filter(&self, query: Box<dyn Object<ID>>) -> Box<dyn Object<ID>>;
}


// define:
// identity function
// inverse ? flip? 
// function composition 
//   vert vs horz
// functions of fucntions (functors)
// 

//pub struct Static()

/// Function Application
/// row of table
/// if need deps, then FA must be in same context as Function needs
///   must be called in env that fun needs
///   and also ofc where params are
pub struct Transaction<ID>
{
    par: Vec<Object<ID>>,
    fun: Function<ID>,
}

/// Functor
/// data transformation
pub struct Functor<ID>
{
    inp: Function<ID>,
    out: Function<ID>,
}
