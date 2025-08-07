//! Story
//!
//! A story represents a thought or conception.
//!   a thought has a spatial, temporal, and agential component
//!   acausal agents changing objects deterministically
//! the story of X is a story where X are the objects
//! what is biggest story/thought?
//!   universe is shared and world is unique
//!   -> The Story of Being (Existence,Reality)
//!      God is the agent, objects are reality, all universes (known and unknown)
//!   -> The Story of Life on Earth (indirectly known universe)
//!   -> The Story of Humanity (directly known universe)
//!        all objects are sensations / sensory experience
//!        stories are attention/experience
//!        objects of thinking perception are thoughts/stories
//!        so thinking as perception is story of all stories
//!   -> The Story of My Life
//!        agent is person and roles
//!        time is the life/history/memory
//!        objects are the world (that person's world)
//!          a combination of that person and others worlds / imagination
//!
//! so God is story of reality, all possible universes
//!   only requirement, must include agents who know of stories

//- USE ------------------------------------------------------------------------

use super::space::Space;
use super::time::Time;
use super::agency::Agency;

//- DEF / Story --------------------------------------------------------------

/// Story
/// program when agent is a computer
/// certainty decreases from agency to time to space
///   materialized views are when space is always available
/// diff programs for different ID,DIV values?
/// program also represents the "center"
///   the point of control, the direction of attention
///   revolves around a particular space
///   there is a sense of gravity
///   why main story is story of life/lives/humanity (collective)
pub struct Story<UID,SIZ,DIV> {
    /// current state, now
    pub space: Space<UID,SIZ,DIV>,
    /// past
    /// so there is an asymmetry between past & future
    ///   as one would expect
    pub time: Time<UID,SIZ,DIV>,
    /// desires, decisions, future
    pub agency: Agency<UID,SIZ,DIV>,
}
type Program<UID,SIZ,DIV> = Story<UID,SIZ,DIV>;

