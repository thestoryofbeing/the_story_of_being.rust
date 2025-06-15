//! Representation / Story


//- USE ------------------------------------------------------------------------

use super::space;
use super::time;
use super::agency;

//- DEF ------------------------------------------------------------------------


// story is a collection of objects, functions and actions
// but also collection of functions and actions seens as objects
// and also actions seen as functions

pub trait Story
{
    //- STORY ------------------------------------------------------------------

    /// takes a query
    fn substory(&self) -> Option<Story>;

    //- OBJECTS ----------------------------------------------------------------

    /// All objects in the story
    fn objects(&self) -> Vec<space::Object>;

    //- FUNCTIONS --------------------------------------------------------------

    /// All functions in the story
    fn functions(&self) -> Vec<time::Function>;

    //- ACTIONS ----------------------------------------------------------------

    /// All actions in the story
    fn actions(&self) -> Vec<agency::Intention>;
}


// nested stories
// just subsets of objects/fucntions/actions
// life is subset of being
//   because it's human perception and it's perception not nothing
//   so both somethign instead of nothign
//     and human perception specifically, not other types
//
// what about the current story. how does that make sense?
//
//
// need the story of the story of being entity/dataset
//   treat entire entity as an object in xp
