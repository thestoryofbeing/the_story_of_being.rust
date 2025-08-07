//! Concept
//!
//! More about objects
//! building a language which reflects the symmetry of experience
//! all the possible conceptual divisions
//!    everything we know is some permutation of conceptual divisions

//- USE ------------------------------------------------------------------------

use std::fmt::Debug;

//- DEF ------------------------------------------------------------------------

//- DEF | Space ----------------------------------------------------------------

pub struct Space<U,S,K>(
    pub Knowledge<U,S,K>
);

//- DEF | Identity -------------------------------------------------------------

//trait Identity: Clone + Debug + Eq + Ord + PartialEq + PartialOrd;

//- DEF | Concept ---------------------------------------------------------------

/// Concept
/// capability vs affordance?
#[derive(Clone, Debug)]
pub enum Concept<U,S,K>
{
    Knowledge(Knowledge<U,S,K>),
    Capability(Capability<U,S,K>),
    Agent(Agent<U,S,K>),
}

//- DEF | Concept.Knowledge -----------------------------------------------------

#[derive(Clone, Debug)]
pub struct Knowledge<U,S,K> 
{
    pub id: U,
    pub structure: Structure<U,S,K>,
}

//- DEF | Concept.Knowledge | Constructors --------------------------------------

/// Knowledge constructors
impl<U> Knowledge<U,usize,usize>
{
    pub fn array_of_values<OS>(
        id: U,
        objects: OS,
    ) -> Knowledge<U,usize,usize>
    where
        OS: IntoIterator<Item = Concept<U,usize,usize>>
    {
        return Knowledge {
            id: id,
            structure: Structure::Collection(
                Collection::from_iter(
                    objects
                        .into_iter()
                        .enumerate()
                        .map(|(idx,obj)| Possibility::value(idx, obj))
                )
            ),
        }
    }
}

// could just generate these with a macro based on a mapping

//- DEF / Concept / Capability --------------------------------------------------

#[derive(Clone, Debug)]
pub struct Capability<U,S,K> {
    pub id: U,
    pub before: Structure<U,S,K>,
    pub after: Structure<U,S,K>,
}

//- DEF / Concept / Agent --------------------------------------------------------

// this actually models attention reacting to thoughts 
// as if they were perceptions??
#[derive(Clone, Debug)]
pub struct Agent<U,S,K> {
    pub id: U,
    pub input: Capability<U,S,K>, // if id func then same as func
    pub process: Capability<U,S,K>,
    pub output: Capability<U,S,K>,
}

//- DEF | Structure ------------------------------------------------------------

// quantity vs size? vs limit
#[derive(Clone, Debug)]
pub enum Structure<U,S,K>
{
    /// Object (set)
    /// kind of makes sense with perception since 
    /// this is the unit of perception (reflected here as structure)
    ///   and the unit of perception is an object/thing
    Object(Object<U,S,K>),
    /// Set (collection, things)
    Collection(Collection<U,S,K>),
    /// Hierarchy Hierarchy
    Hierarchy(Hierarchy<U,S,K>),
}

//- DEF | Structure.Object -------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Object<U,S,K>
{
    /// None, No-thing, 0
    None,
    /// One, Some-thing, 1
    One(Box<Possibility<U,S,K>>),
}

//- DEF | Structure.Collection -------------------------------------------------

#[derive(Clone, Debug)]
pub struct Collection<U,S,K>
{
    size: S,
    members: Vec<Possibility<U,S,K>>,
}

impl<U,K> FromIterator<Possibility<U,usize,K>> for Collection<U,usize,K>
{
    fn from_iter<IT: IntoIterator<Item=Possibility<U,usize,K>>>(iter: IT) -> Self 
    {
        let members: Vec<Possibility<U,usize,K>> = iter.into_iter().collect();
        let size = members.len();
        Collection { size, members }
    }
}

//- DEF | Structure.Hierarchy --------------------------------------------------

#[derive(Clone, Debug)]
pub struct Hierarchy<U,S,K> 
{
    size: S,
    node: Object<U,S,K>,
    children: Collection<U,S,K>,
}

//- DEF | Possibility ----------------------------------------------------------

/// Possibility
/// 3+1 Inceptual attributes
#[derive(Clone, Debug)]
pub struct Possibility<U,S,K>
{
    key: K,
    value: Option<Concept<U,S,K>>,
    lock: Option<Concept<U,S,K>>,
    typ: Option<Concept<U,S,K>>,
}

//- DEF | Possibility | Constructors -------------------------------------------

/// Constructors
impl<U,S,K> Possibility<U,S,K>
{
    /// Pure Value
    fn value(
        key: K,
        object: Concept<U,S,K>
    ) -> Possibility<U,S,K>
    {
        return Possibility
        {
            key,
            value: Some(object),
            lock: None,
            typ: None,
        }

    }

    /// Pure Type
    fn typ(
        key: K,
        typ: Concept<U,S,K>
    ) -> Possibility<U,S,K>
    {
        return Possibility
        {
            key,
            value: None,
            lock: None,
            typ: Some(typ),
        }

    }

    /// Pure Lock (no lock)
    fn lock(
        key: K,
        lock: Concept<U,S,K>
    ) -> Possibility<U,S,K>
    {
        return Possibility {
            key,
            value: None,
            lock: Some(lock),
            typ: None,
        }

    }
}


