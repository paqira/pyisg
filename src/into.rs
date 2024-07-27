use libisg::*;

use crate::*;

macro_rules! impl_from {
    ($src:tt => $dis:tt) => {
        impl From<$src> for $dis {
            fn from(value: $src) -> Self {
                value.0
            }
        }
    };
}

impl_from!(HeaderWrapper => Header);
impl_from!(DataWrapper => Data);
impl_from!(ModelTypeWrapper => ModelType);
impl_from!(DataTypeWrapper => DataType);
impl_from!(DataUnitsWrapper => DataUnits);
impl_from!(DataFormatWrapper => DataFormat);
impl_from!(DataOrderingWrapper => DataOrdering);
impl_from!(TideSystemWrapper => TideSystem);
impl_from!(CoordTypeWrapper => CoordType);
impl_from!(CoordUnitsWrapper => CoordUnits);
impl_from!(CreationDateWrapper => CreationDate);
impl_from!(CoordWrapper => Coord);
