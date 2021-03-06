//! Internal module defining the `Cell` type.

use super::free::State as FreeState;
use super::wall::State as WallState;

/// The [`Cell`][cell] type represents all possible states of a
/// single Rokkakari game cell.
///
/// See [the module level documentation for more][cell_module].
///
/// [cell]: crate::cell::Cell
/// [cell_module]: crate::cell#cells
pub enum Cell {
    /// A *free* game cell.
    /// All the free cell specific state is represented by [`FreeState`][free_state].
    ///
    /// [free_state]: crate::cell::FreeState
    Free(FreeState),

    /// A *wall* game cell.
    /// All the wall cell specific state is represented by [`WallState`][wall_state].
    ///
    /// [wall_state]: crate::cell::WallState
    Wall(WallState),
}
