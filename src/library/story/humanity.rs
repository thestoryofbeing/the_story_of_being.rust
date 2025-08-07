//! The Story of Humanity
//!
//! A story about existence
//! add rise, shine, combine here
//! this module will basically be the meditation framework
//! energy, emotions, attention
//! suffering, liberation
//! happiness, freedom, peace
//! all desire is for joy, freedom and peace

//- USE ------------------------------------------------------------------------

use crate::conception::common::space;
use crate::conception::common::time;
use crate::conception::common::agency;
use crate::conception::common::story;

use uuid::Uuid;

//- DEF ------------------------------------------------------------------------

//- DEF / Identities -----------------------------------------------------------

/// Universal Identifier
type U = Uuid;

/// Size
type S = usize;

/// Key
type K = String;

//- DEF | Setting --------------------------------------------------------------

type Space = space::Space<U,S,K>;
type Concept = space::Concept<U,S,K>;
type Knowledge = space::Knowledge<U,S,K>;
type Capability = space::Capability<U,S,K>;
type Agent = space::Agent<U,S,K>;

type Structure = space::Structure<U,S,K>;
type Object = space::Object<U,S,K>;
type Collection = space::Collection<U,S,K>;
type Hierarchy = space::Hierarchy<U,S,K>;

type Possibility = space::Possibility<U,S,K>;

type Time = time::Time<U,S,K>;

type Agency = agency::Agency<U,S,K>;
type Intention = agency::Intention<U,S,K>;

type Story = story::Story<U,S,K>;

//- DEF | Definitions ----------------------------------------------------------

/// Infinite Space
/// There is Nothing/Everything/Anything
/// the emptiness test, just give story as proof, stories are empty
///    empty test, hand reaching into box icon
/// awareness is empty because it has a whole story with it
/// this the story of perception?
/// awareness can be things, it can be "for" something, like to enable
///   self learning or operating in world
///   and can itself have an agenda when seen as an entity in itself
///     as in common in meditation
const Awareness: Space = space::Space(
    space::Concept::Knowledge {
        id: Experience,
        structure: Structure::Unit(Unit::None),
    }
);

enum Percept
{
    Physical(PhysicalPercept),
    Mental(MentalPercept),
}

const Percept: Knowledge = 
    space::Knowledge::array_of_values(
        id,
        vec![
            
        ],
    )


enum SenseMode
{
    Sight,
    Sound,
    Feeling,
    Taste,
    Smell,
}


struct Sensation
{
    sense: SenseMode,
    valence: Valence,
}

//struct Pattern(Vec<Sensation>);

enum Pattern
{
    Physical(Vec<Sensation>),
    Mental(Vec<Sensation>),
}

// struct Percept {
//     pattern: Pattern,
//     valence: Valence,
// }

struct PhysicalPercept(Pattern);

enum MentalPercept
{
    Somatic(Pattern),
    Conceptual(Story),
}


enum Valence
{
    Good,
    Neutral,
    Bad,
}

/// Fabricate
/// also knowing??
enum Fabricate
{
    Contact,
    Internalize,
    Conceive,
}

/// Timelessness
/// Nothing [ever happened]/[will happen]
const PERCEPTION: Time = time::Time(Vec::new());


/// Action
/// is this just physics? 
/// maybe observation is physics
pub struct Change;
pub struct Observation;
pub struct Desire;

/// Love
/// Nothing/Everything/Anything is desired
pub const ATTENTION: Agency = agency::Agency(Vec::new());

/// Self
/// conventional happiness is when our observations match desires
/// they overlap in composition. when we can stitch together reality
/// the way we want
pub const SELF: Intention = agency::Intention {
    judgement: Observation,
    action: Change,
    objective: Desire,
}

//- DEF / Story ----------------------------------------------------------------

pub const HUMANITY: Story = Story {
    space: SENSATIONS,
    time: PERCEPTION,
    agency: ATTENTION,
};

// The Perception/Attention/PhysicalIntention triangle
//
//       Self (attention)
//      /             \
//     /               \
//   xp (awareness)     xp
//
