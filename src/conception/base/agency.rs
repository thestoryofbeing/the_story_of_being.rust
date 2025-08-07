//! Agency
//!
//! More about agency

//- USE ------------------------------------------------------------------------

use super::time::Time;

//- DEF / Agency ---------------------------------------------------------------

/// Agency
pub struct Agency<UID,SIZ,DIV>(
    pub Vec<Intention<UID,SIZ,DIV>>
);

/// Intention
pub struct Intention<UID,SIZ,DIV>
{
    // interpretation, condition
    pub judgement: Time<UID,SIZ,DIV>,
    pub action: Time<UID,SIZ,DIV>,
    pub objective: Time<UID,SIZ,DIV>,
}

// have to connect programs to each other
// so that judgement can overlap


