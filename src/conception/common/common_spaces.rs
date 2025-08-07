//! Space - common definitions
//!
//! Spatial constructors and methods for specific but commonly used type 
//! parameters.

//- USE ------------------------------------------------------------------------

use crate::conception::base::space::*;

//- DEF ------------------------------------------------------------------------

//- DEF | Space ----------------------------------------------------------------

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

