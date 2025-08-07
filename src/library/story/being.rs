//! The Story of Being
//!
//! A story about existence
//! include tetralemma somewhere
//!  this + not this, that + this and that + neither this nor that

//- USE ------------------------------------------------------------------------

use crate::conception::agency;
use crate::conception::space;
use crate::conception::story;
use crate::conception::time;

//- DEF ------------------------------------------------------------------------

//- DEF / Identities -----------------------------------------------------------

/// Experience
pub struct Experience;
type X = Experience;

/// Consciousness
pub struct Consciousness;
type C = Consciousness;

/// Infinity
pub struct Infinity;
type I = Infinity;

//- DEF / Setting --------------------------------------------------------------


// make this a macro, maybe one exists already

type Space = space::Space<X,C,I>;
type Knowledge = space::Knowledge<X,C,I>;
type Structure = space::Structure<X,C,I>;
type Unit = space::Unit<X,C,I>;

type Time = time::Time<X,C,I>;

type Agency = agency::Agency<X,C,I>;

type Story = story::Story<X,C,I>;

//- DEF / Definitions ----------------------------------------------------------

/// Infinite Space
/// There is Nothing/Everything/Anything
pub const INFINITE_SPACE: Space = space::Space(Knowledge {
    id: Experience,
    structure: Structure::Unit(Unit::None),
});

/// Timelessness
/// Nothing [ever happened]/[will happen]
pub const TIMELESSNESS: Time = time::Time(Vec::new());

/// Love
/// Nothing/Everything/Anything is desired
pub const LOVE: Agency = agency::Agency(Vec::new());

//- DEF / Story ----------------------------------------------------------------

pub const BEING: Story = Story {
    space: INFINITE_SPACE,
    time: TIMELESSNESS,
    agency: LOVE,
};


