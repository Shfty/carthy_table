use comfy_table::Table;
use frunk_core::hlist::{HCons, HNil};

use carth::{hlist::FieldList, htable::Transpose};

use super::{cell_list::CellList, table::ToTable, to_cell_list::ToCellList};

pub trait OuterNamedTable<'a, Width> {
    fn to_table(&'a self) -> Table;
}

impl<'a, Width, Head, Tail> OuterNamedTable<'a, Width> for HCons<Head, Tail>
where
    Self: FieldList<'a>,
    <Self as FieldList<'a>>::NamesOutput: ToCellList,
    <Self as FieldList<'a>>::ValuesOutput: Transpose<Width>,
    <<Self as FieldList<'a>>::ValuesOutput as Transpose<Width>>::Output: ToTable,
    Tail: OuterNamedTable<'a, Width>,
{
    fn to_table(&'a self) -> Table {
        let mut table = self.values().transpose().to_table();
        let header = self.names().list_to_cells().to_row();
        table.set_header(header);
        table
    }
}

impl<'a, Width> OuterNamedTable<'a, Width> for HNil {
    fn to_table(&'a self) -> Table {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use frunk_core::{field, hlist};

    use super::OuterNamedTable;

    #[test]
    fn test_outer_named_table() {
        let outer_named_table = hlist![
            field!(i32, hlist![1, 2, 3]),
            field!(f32, hlist![4.1, 5.2, 6.3]),
            field!(char, hlist!['7', '8', '9']),
        ];

        let values_table = outer_named_table.to_table();
        println!("\nTable:\n{}", values_table);
    }
}
