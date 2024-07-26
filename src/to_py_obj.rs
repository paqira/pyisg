use libisg::{Coord, Data, DataBounds};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::{CoordWrapper, CreationDateWrapper, DataWrapper, HeaderWrapper};

impl ToPyObject for HeaderWrapper {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);

        macro_rules! set_item {
            ($field:ident) => {
                dict.set_item(stringify!($field), &self.0.$field)
                    .expect(concat!("fail setting `", stringify!($field), "` to dict"));
            };
        }

        macro_rules! set_item_as_string {
            ($field:ident) => {
                dict.set_item(stringify!($field), &self.0.$field.to_string())
                    .expect(concat!("fail setting `", stringify!($field), "` to dict"));
            };
        }

        macro_rules! set_item_opt_as_string {
            ($field:ident) => {
                dict.set_item(
                    stringify!($field),
                    &self.0.$field.as_ref().map(|v| v.to_string()),
                )
                .expect(concat!("fail setting `", stringify!($field), "` to dict"));
            };
        }

        macro_rules! set_item_data_bounds {
            ($field:ident) => {
                dict.set_item(stringify!($field), CoordWrapper($field))
                    .expect(concat!("fail setting `", stringify!($field), "` to dict"));
            };
        }

        macro_rules! set_item_data_bounds_none {
            ($field:ident) => {
                dict.set_item(stringify!($field), None::<CoordWrapper>)
                    .expect(concat!("fail setting `", stringify!($field), "` to dict"));
            };
        }

        set_item!(model_name);
        set_item!(model_year);
        set_item_opt_as_string!(model_type);
        set_item_opt_as_string!(data_type);
        set_item_opt_as_string!(data_units);
        set_item_as_string!(data_format);
        set_item_opt_as_string!(data_ordering);
        set_item_opt_as_string!(ref_ellipsoid);
        set_item_opt_as_string!(ref_frame);
        set_item_opt_as_string!(height_datum);
        set_item_opt_as_string!(tide_system);
        set_item_as_string!(coord_type);
        set_item_as_string!(coord_units);
        set_item_opt_as_string!(map_projection);
        set_item_opt_as_string!(EPSG_code);
        match self.0.data_bounds {
            DataBounds::GridGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
                delta_lat,
                delta_lon,
            } => {
                set_item_data_bounds!(lat_min);
                set_item_data_bounds!(lat_max);
                set_item_data_bounds!(lon_min);
                set_item_data_bounds!(lon_max);
                set_item_data_bounds!(delta_lat);
                set_item_data_bounds!(delta_lon);
            }
            DataBounds::SparseGeodetic {
                lat_min,
                lat_max,
                lon_min,
                lon_max,
            } => {
                set_item_data_bounds!(lat_min);
                set_item_data_bounds!(lat_max);
                set_item_data_bounds!(lon_min);
                set_item_data_bounds!(lon_max);
                set_item_data_bounds_none!(delta_lat);
                set_item_data_bounds_none!(delta_lon);
            }
            DataBounds::GridProjected {
                north_min,
                north_max,
                east_min,
                east_max,
                delta_north,
                delta_east,
            } => {
                set_item_data_bounds!(north_min);
                set_item_data_bounds!(north_max);
                set_item_data_bounds!(east_min);
                set_item_data_bounds!(east_max);
                set_item_data_bounds!(delta_north);
                set_item_data_bounds!(delta_east);
            }
            DataBounds::SparseProjected {
                north_min,
                north_max,
                east_min,
                east_max,
            } => {
                set_item_data_bounds!(north_min);
                set_item_data_bounds!(north_max);
                set_item_data_bounds!(east_min);
                set_item_data_bounds!(east_max);
                set_item_data_bounds_none!(delta_north);
                set_item_data_bounds_none!(delta_east);
            }
        }
        set_item!(nrows);
        set_item!(ncols);
        set_item!(nodata);
        dict.set_item(
            "creation_date",
            self.0.creation_date.map(CreationDateWrapper),
        )
        .expect(concat!("fail setting `creation_date` to dict"));
        set_item!(ISG_format);
        dict.into_py(py)
    }
}

impl ToPyObject for DataWrapper {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match &self.0 {
            Data::Grid(data) => PyList::new_bound(py, data).into_py(py),
            Data::Sparse(data) => PyList::new_bound(
                py,
                data.iter()
                    .map(|row| (CoordWrapper(row.0), CoordWrapper(row.1), row.2)),
            )
            .into_py(py),
        }
    }
}

impl ToPyObject for CreationDateWrapper {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);

        dict.set_item("year", self.0.year)
            .expect("fail setting `year` to dict");
        dict.set_item("month", self.0.month)
            .expect("fail setting `month` to dict");
        dict.set_item("day", self.0.day)
            .expect("fail setting `day` to dict");

        dict.into_py(py)
    }
}

impl ToPyObject for CoordWrapper {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self.0 {
            Coord::DMS {
                degree,
                minutes,
                second,
            } => {
                let dict = PyDict::new_bound(py);

                dict.set_item("degree", degree)
                    .expect("fail setting `degree` to dict");
                dict.set_item("minutes", minutes)
                    .expect("fail setting `minutes` to dict");
                dict.set_item("second", second)
                    .expect("fail setting `second` to dict");

                dict.into_py(py)
            }
            Coord::Dec(v) => v.into_py(py),
        }
    }
}
