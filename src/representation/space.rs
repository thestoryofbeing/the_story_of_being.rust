//! Object
//!
//! The unit of representation. 
//! ALL of space gets divided by time and ALL of time gets divided by agency
//!
//! this is a programmable metaphysics
//! any true metaphysics must be programmable because
//! we see reality through the lens of intention
//! and so any image of reality will be intentional
//!
//! so created prog metaphysics that when used will 
//! allow indv and coll intelligence to converge toward clear seeing and truth


// an object is just composition / decomposition
//
// every division has one parent, at least one unique structure and one class


//- DEF ------------------------------------------------------------------------

//- DEF / OBJECT ---------------------------------------------------------------
//- ----------------------------------------------------------------------------
    
/// Object
/// <and>
/// object represents known
/// id represents unknown
/// every object is a number / quantity / size
///   call these natural numbers, repr quantity
///   0 is just duality, no-quantity, but also is context
/// thought can come in many different forms
///   why object is trait. can say more about that?
///
/// ID can also be substrate
/// it is the source of uniqueness or identification in that domain
/// for example molecules are Object<Atom>
/// software is Object<Bits>
pub trait Object<ID>
{
    //- INTERNAL (ONE / CONSTITUTION / STATIC)
    //- | 3+1 attributes of objects
    
    /// [01] Classes
    /// equilibrium
    /// network growth/contraction
    /// 0 cost
    fn cls(&self) -> Vec<ID>;
    
    /// [02] Context
    /// contraction / simplification
    /// size reduction
    /// neg cost
    fn ctx(&self) -> Box<Dimension<ID>>;
    
    /// [03] Content 
    /// expansion / complication
    /// size increase
    /// pos cost
    fn con(&self) -> Vec<Box<Dimension<ID>>>;
    
    /// [+1] Identifier
    /// equilibrium + contraction + expansion
    fn key(&self) -> ID;

    //- EXTERNAL (MANY / COMPOSITION / EXPANSION-CONTRACTION)
    //- | Information-preserving  (should be prop of composition?)
    
    //- EXTERNAL / HORIZONTAL

    /// [++] Union
    /// expansion
    fn union(
        &self, 
        other: Box<dyn Object<ID>>, 
        key: Box<dyn Fn(ID, ID) -> ID + 'static>,
    ) -> Box<dyn Object<ID>>;

    /// [--] Intersection
    /// contraction
    fn inter(
        &self, 
        other: Box<dyn Object<ID>>, 
        key: Box<dyn Fn(ID, ID) -> ID + 'static>,
    ) -> Box<dyn Object<ID>>;

    //- EXTERNAL / VERTICAL

    /// [<>] Merge
    /// expansion
    fn merge(&self) -> Box<dyn Object<ID>>;
    
    /// [><] Split
    /// contraction
    fn split(&self) -> Box<dyn Object<ID>>;

    //- TRANSCENDENT
    //- lift object to function or action

    /// [**] Join
    /// turns object into function object
    /// returns cross product of object content
    fn join(&self, other: Box<dyn Object<ID>>) -> Box<dyn Object<ID>>;
}

/// Dimension
/// <or>
/// 3 + 1 ? 
pub struct Dimension<ID>
{
    /// [01] Variable
    var: Option<Var>,
    /// [02] Literal
    lit: Option<Lit<ID>>,
    /// [03] Type
    typ: Option<Typ<ID>>,
    /// [+1] Key
    key: Key<ID>,
}

// names for diff combinations of var/lit/typ
//   kind of dimensionality
// ind
// dep
// static / constant / cns

pub struct Var();
pub struct Lit<ID>(Box<dyn Object<ID>>);
pub struct Typ<ID>(Box<Dimension<ID>>);
pub struct Key<ID>(ID);

