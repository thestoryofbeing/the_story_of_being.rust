//! The Story of Being

//- USE ------------------------------------------------------------------------

use crate::representation::space::Object;
use crate::representation::story::Story;

//- DEF ------------------------------------------------------------------------
//------------------------------------------------------------------------------

//- DEF / STORY-----------------------------------------------------------------
//------------------------------------------------------------------------------


// immanence
pub trait Control<ID>
{

    fn interpret(&self);
    fn implement(&self);


    fn read(&self) -> Box<dyn Object<ID>>;
    fn write(&self, object: Box<dyn Object<ID>>);

    // only read and write. update and delete are just special writes
    // read
    // write

    // create new story
    //   in some format e.g. filesystem, python, postgress, webserver, etc

}

// non-control / letting go / transcendence
pub trait Grace 
{

}


struct Being
{
    ontology: i32,
    epistemology: i32,
    ethics: i32,
}

/// The story <of> Being
impl Story for Being {
}



// story of being  interface
//
// implementation
// interpretation
//
// claim
//   i am
//   i am here
//   i am now doing this
//   i saw this
//   i know this
//   i beliee this
//   i intend this
//



// so does form/var/harmony
// correspond to space/time/agency ? 
//
// well here at least it makes sense that basic objects
// are going to start with the props of reality
// so nothing, everything, and anything are objects for form/var/harm
// all the same actually, just used differently

pub enum Being
{
    /// Object version
    /// complete form 
    /// no information
    /// Complete variability means there *is* nothing
    Nothing,
    /// Variable version
    /// complete variability 
    /// all posible information
    /// complete var means there *could be* any info
    Everything,
    /// Type version
    /// complete harmony 
    /// *should be* any information
    /// nothing and everything
    Any,
}
type NonBeing = Being;

//- AWARENESS ------------------------------------------------------------------

/// also division, separation
pub struct Awareness;
type Consciousness = Awareness;

// could this help with definition components
//   also thinking of CSS divs when creating UI
// impl is context is context and content is variable
// the wrapper wraps any content
pub struct Outside<ID>
{
    wrapper: Dimension<ID>,
}

type Inside<ID> = Outside<ID>;


/// by using awareness as identity
/// we are saying all is the same
impl Object<Awareness> for Everything
{
    fn cls(&self) -> Vec<Awareness>
    {
        return Vec::new();
    }
    
    fn ctx(&self) -> Box<Dimension<Awareness>>
    {
        return Box::new(Dimension {
            var: None,
            lit: None,
            typ: None,
            key: Key(Awareness),
        })
    }
    
    fn con(&self) -> Vec<Box<Dimension<Awareness>>>
    {
        return Box::new(Dimension {
            var: None,
            lit: None,
            typ: None,
            key: Key(Awareness),
        })
    }
    
    fn key(&self) -> ID 
    {
        return A
    }

    // fn union(
    //     &self, 
    //     other: Box<dyn Object<ID>>, 
    //     key: Box<dyn Fn(ID, ID) -> ID + 'static>,
    // ) -> Box<dyn Object<ID>>;
    //
    // fn inter(
    //     &self, 
    //     other: Box<dyn Object<ID>>, 
    //     key: Box<dyn Fn(ID, ID) -> ID + 'static>,
    // ) -> Box<dyn Object<ID>>;
    //
    // fn merge(&self) -> Box<dyn Object<ID>>;
    //
    // fn split(&self) -> Box<dyn Object<ID>>;
    //
    // fn join(&self, other: Box<dyn Object<ID>>) -> Box<dyn Object<ID>>;
}

