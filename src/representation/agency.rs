//! Representation / Agency
//! Decisions, conditions, control flow

//- USE ------------------------------------------------------------------------

use super::space::Object;
use super::time::Function;

//- DEF ------------------------------------------------------------------------

//- DEF / ACTION -------------------------------------------------------------
//- ----------------------------------------------------------------------------

/// Action
pub struct Action<ID>
{
    cnd: Object<ID>,
    fun: Function<ID>,
}


