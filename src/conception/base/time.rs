//! Time
//!
//! All functions between objects

//- USE ------------------------------------------------------------------------

use super::space;


// "built in" functions (mutations?)
//   fundamental temporal divisions
// create 
//   can always create a new object, structure, or register
//   works at level of whatever its creating, neutral
// compose/combine
//   combine two objects/structures/registers to create a new one
//   works at structural level

//- DEF / TIME -----------------------------------------------------------------

// should this actually be a function??
//   represents a "real" function
// +1 function leads to this concept
//   represents expansion? 
pub struct Time<ID> {
    transactions: Vec<time::Application<ID>>,
}

// data then functions then IO ? 
// turing machine represents the scope of thoughts/stories
//   or whatever is just spacetime

/// A transaction (diff, mutation)
///    is a function between Space
///  Space -> Space is a difference of Space
///    we define that
pub struct Transaction {
    what: space::Path, 
    change: Change,
}

// what is this exactly???
pub struct Path<KEY>(Vec<KEY>);

/// A change between Space, the unit of time
/// represents abstractly anything that can change
///   as space represents anything that can be defined
pub enum Change {
    /// new, assumed information, axioms (from other context except tsob)
    Create(space::Object),
    /// neutral, 0, preserve information
    Compose(Compose),
    /// increase, add info
    Expand(Expand),
    /// decrease, remove info
    Contract(Contract),
}

/// Compose
/// neutral change
pub enum Compose
{
    Merge,
    Split,
}

/// Expand
/// positive change
pub enum Expand
{
    Append,
    Substitute,
}

/// Contract
/// negative change
pub enum Contract
{
    Remove,
    Clear,
}

//- DEF / SPATIAL --------------------------------------------------------------



// registers
// only has update
//   that which can be moved

// structure
// that which can be combined
// combining structures
//   horizontal (same level)
//     and split
//   nesting (deeper level)
//     and unnest/collapse
//         extend/retract
//    is merging same as retract?
//  done in place? 
//    put functions, data etc next to each other, then merge
//    zero information loss/add operations
//      well all that exists really



fn merge(
    s: Structure<ID,KEY>,
    t: Structure<ID,KEY>,
) -> Structure<ID,KEY>
{
    match (s,t) {
        (Structure::None(none), t                    ) => {
            t
        },
        (s                    , Structure::None(none)) => {
            s
        },
        (Structure::One(one)   ,Structure::One(one)   ) => {

        },
        (Structure::One(one)   ,Structure::Set(one)   ) => {

        },
        (Structure::Set(set_s)   ,Structure::One(set_t)   ) => {
            Structure::Set(merged_sets(set_s, set_t))
        },
        (Structure::Set(set)   ,Structure::Set(set)   ) => {

        },
    }
}

fn merged_sets<ID,KEY>(
    a: Set<ID,KEY>,
    b: Set<ID,KEY>,
) -> Set<ID,KEY>
{
    return a;
}


pub fn split<ID,CLS,KEY>(
    this: Concept<ID,CLS,KEY>, 
    that: Concept<ID,CLS,KEY>,
) -> Concept<ID,CLS,KEY>
{

}


/// combining objects
fn compose<ID,KEY>(
    o: Object<ID,KEY>,
    p: Object<ID,KEY>,
) -> Object<ID,KEY>
{

}



//- DEF / TEMPORAL -------------------------------------------------------------

/// Join
fn join<ID>(
    left: Box<Object<ID>>, 
    right: Box<Object<ID>>, 
) -> Function<ID
{
    // just takes cross product

}

/// Bind
fn bind<ID>(
    left: Box<Object<ID>>, 
    right: Vec<Function<ID>>, 
) -> Monad<ID
{
    // just takes cross product
}

/// Event
fn trigger<ID>(
    left: Box<Object<ID>>, 
    right: Vec<Function<ID>>, 
) -> Monad<ID
{
    // just takes cross product
}

/// Event
fn apply<ID>(
    left: Box<Object<ID>>, 
    right: Vec<Function<ID>>, 
) -> Monad<ID
{
    // just takes cross product
}

