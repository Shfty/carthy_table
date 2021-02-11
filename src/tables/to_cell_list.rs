use comfy_table::{Cell, ToCell};
use frunk_core::hlist::{HCons, HNil};

use super::cell_list::CellList;

/// A `HList` of `ToCell` types
pub trait ToCellList {
    type Output: CellList;

    fn list_to_cells(&self) -> Self::Output;
}

impl<Head, Tail> ToCellList for HCons<Head, Tail>
where
    Head: Copy + ToCell,
    Tail: ToCellList,
{
    type Output = HCons<Cell, Tail::Output>;

    fn list_to_cells(&self) -> Self::Output {
        HCons {
            head: self.head.to_cell(),
            tail: self.tail.list_to_cells(),
        }
    }
}

impl<Head, Tail> ToCellList for &HCons<Head, Tail>
where
    Head: Copy + ToCell,
    Tail: ToCellList,
{
    type Output = HCons<Cell, Tail::Output>;

    fn list_to_cells(&self) -> Self::Output {
        HCons {
            head: self.head.to_cell(),
            tail: self.tail.list_to_cells(),
        }
    }
}

impl ToCellList for HNil {
    type Output = HNil;

    fn list_to_cells(&self) -> Self::Output {
        HNil
    }
}

#[cfg(test)]
pub mod tests {
    use frunk_core::hlist;

    use super::ToCellList;

    #[test]
    fn to_cell_list() {
        let list = hlist![1, 2, 3];
        let cells = list.list_to_cells();
        println!("\nCells:\n{:?}", cells);
    }
}