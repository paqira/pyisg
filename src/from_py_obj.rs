use libisg::{Coord, Data, DataBounds};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::*;

impl<'a> FromPyObject<'a> for HeaderWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let obj = ob
            .downcast::<PyDict>()
            .map_err(|_| type_error!("header", "dict"))?;

        let model_name = obj
            .get_item("model_name")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("model_name", "str | None"))?;
        let model_year = obj
            .get_item("model_year")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("model_year", "str | `None`"))?;
        let model_type = obj
            .get_item("model_type")?
            .map_or(Ok(None), |obj| obj.extract::<Option<ModelTypeWrapper>>())
            .map_err(|_| {
                type_error!(
                    "model_type",
                    "'gravimetric' | 'geometric' | 'hybrid' | None"
                )
            })?
            .map(Into::into);
        let data_type = obj
            .get_item("data_type")?
            .map_or(Ok(None), |obj| obj.extract::<Option<DataTypeWrapper>>())
            .map_err(|_| type_error!("data_type", "'geoid' | 'quasi-geoid' | None"))?
            .map(Into::into);
        let data_units = obj
            .get_item("data_units")?
            .map_or(Ok(None), |obj| obj.extract::<Option<DataUnitsWrapper>>())
            .map_err(|_| type_error!("data_units", "'meters' | 'feet' | None"))?
            .map(Into::into);
        let data_format = obj
            .get_item("data_format")?
            .ok_or(missing_key!("data_format"))?
            .extract::<DataFormatWrapper>()
            .map_err(|_| type_error!("data_format", "'grid' | 'sparse'"))?
            .into();
        let data_ordering = obj
            .get_item("data_ordering")?
            .map(|obj| obj.extract::<DataOrderingWrapper>())
            .transpose()
            .map_err(|_| {
                type_error!(
                    "data_ordering",
                    "'N-to-S, W-to-E' | 'lat, lon, N' | 'east, north, N' | 'N' | 'zeta' | None"
                )
            })?
            .map(Into::into);
        let ref_ellipsoid = obj
            .get_item("ref_ellipsoid")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("ref_ellipsoid", "str | None"))?;
        let ref_frame = obj
            .get_item("ref_frame")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("ref_frame", "str | None"))?;
        let height_datum = obj
            .get_item("height_datum")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("height_datum", "str | None"))?;
        let tide_system = obj
            .get_item("tide_system")?
            .map_or(Ok(None), |obj| obj.extract::<Option<TideSystemWrapper>>())
            .map_err(|_| {
                type_error!(
                    "tide_system",
                    "'tide-free' | 'mean-tide' | 'zero-tide' | None"
                )
            })?
            .map(Into::into);
        let coord_type = obj
            .get_item("coord_type")?
            .ok_or(missing_key!("coord_type"))?
            .extract::<CoordTypeWrapper>()
            .map_err(|_| type_error!("coord_type", "'geodetic' | 'projected'"))?
            .into();
        let coord_units = obj
            .get_item("coord_units")?
            .ok_or(missing_key!("coord_units"))?
            .extract::<CoordUnitsWrapper>()
            .map_err(|_| type_error!("coord_units", "'dms' | 'deg' | 'meters' | 'feet'"))?
            .into();
        let map_projection = obj
            .get_item("map_projection")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("map_projection", "str | None"))?;
        #[allow(non_snake_case)]
        let EPSG_code = obj
            .get_item("EPSG_code")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| type_error!("EPSG_code", "str | None"))?;
        let nrows = obj
            .get_item("nrows")?
            .ok_or(missing_key!("nrows"))?
            .extract()
            .map_err(|_| type_error!("nrows", "int (usize)"))?;
        let ncols = obj
            .get_item("ncols")?
            .ok_or(missing_key!("ncols"))?
            .extract()
            .map_err(|_| type_error!("ncols", "int (usize)"))?;
        let nodata = obj
            .get_item("nodata")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| SerError::new_err("unexpected type on `nodata`, expected float | None"))?;
        let creation_date = obj
            .get_item("creation_date")?
            .map_or(Ok(None), |obj| obj.extract::<Option<CreationDateWrapper>>())
            .map_err(|_| {
                type_error!(
                    "creation_date",
                    "{ year: int (u16), month: int (u8), day: int (u8) } | None"
                )
            })?
            .map(Into::into);
        #[allow(non_snake_case)]
        let ISG_format = obj
            .get_item("ISG_format")?
            .ok_or(missing_key!("ISG_format"))?
            .extract()
            .map_err(|_| type_error!("ISG_format", "str | None"))?;

        let data_bounds = match coord_type {
            CoordType::Geodetic => {
                let lat_min = obj
                    .get_item("lat_min")?
                    .ok_or(missing_key!("lat_min"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "lat_min",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let lat_max = obj
                    .get_item("lat_max")?
                    .ok_or(missing_key!("lat_max"))
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "lat_max",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let lon_min = obj
                    .get_item("lon_min")?
                    .ok_or(missing_key!("lon_min"))
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "lon_min",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let lon_max = obj
                    .get_item("lon_max")?
                    .ok_or(missing_key!("lon_max"))
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "lon_max",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();

                match data_format {
                    DataFormat::Grid => {
                        let delta_lat = obj
                            .get_item("delta_lat")?
                            .ok_or(missing_key!("delta_lat"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| type_error!("delta_lat", "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();
                        let delta_lon = obj
                            .get_item("delta_lon")?
                            .ok_or(missing_key!("delta_lon"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| type_error!("delta_lon", "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();

                        DataBounds::GridGeodetic {
                            lat_min,
                            lat_max,
                            lon_min,
                            lon_max,
                            delta_lat,
                            delta_lon,
                        }
                    }
                    DataFormat::Sparse => DataBounds::SparseGeodetic {
                        lat_min,
                        lat_max,
                        lon_min,
                        lon_max,
                    },
                }
            }
            CoordType::Projected => {
                let north_min = obj
                    .get_item("north_min")?
                    .ok_or(missing_key!("north_min"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "north_min",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let north_max = obj
                    .get_item("north_max")?
                    .ok_or(missing_key!("north_max"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "north_max",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let east_min = obj
                    .get_item("east_min")?
                    .ok_or(missing_key!("east_min"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "east_min",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();
                let east_max = obj
                    .get_item("east_max")?
                    .ok_or(missing_key!("east_max"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| {
                        type_error!(
                            "east_max",
                            "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"
                        )
                    })?
                    .into();

                match data_format {
                    DataFormat::Grid => {
                        let delta_north = obj
                            .get_item("delta_north")?
                            .ok_or(missing_key!("delta_north"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| type_error!("delta_north", "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();
                        let delta_east = obj
                            .get_item("delta_east")?
                            .ok_or(missing_key!("delta_east"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| type_error!("delta_east", "float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();

                        DataBounds::GridProjected {
                            north_min,
                            north_max,
                            east_min,
                            east_max,
                            delta_north,
                            delta_east,
                        }
                    }
                    DataFormat::Sparse => DataBounds::SparseProjected {
                        north_min,
                        north_max,
                        east_min,
                        east_max,
                    },
                }
            }
        };

        Ok(Self(Header {
            model_name,
            model_year,
            model_type,
            data_type,
            data_units,
            data_format,
            data_ordering,
            ref_ellipsoid,
            ref_frame,
            height_datum,
            tide_system,
            coord_type,
            coord_units,
            map_projection,
            EPSG_code,
            data_bounds,
            nrows,
            ncols,
            nodata,
            creation_date,
            ISG_format,
        }))
    }
}

impl<'a> FromPyObject<'a> for DataWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        if let Ok(data) = ob.extract() {
            Ok(DataWrapper(Data::Grid(data)))
        } else if let Ok(data) = ob.extract::<Vec<(CoordWrapper, CoordWrapper, f64)>>() {
            Ok(DataWrapper(Data::Sparse(
                data.into_iter()
                    .map(|(a, b, c)| (a.into(), b.into(), c))
                    .collect(),
            )))
        } else {
            Err(type_error!(
                "data",
                concat!(
                    "list[list[float | None]] | list[tuple[float | { degree: int (i16), minutes: int (u8), second: int (u8) }, float | { degree: int (i16), minutes: int (u8), second: int (u8) }, float]]"
                )
            ))
        }
    }
}

macro_rules! impl_from_py_object {
    ($type:tt) => {
        impl<'a> FromPyObject<'a> for $type {
            fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
                let r = ob
                    .extract::<String>()?
                    .parse()
                    .map_err(|_| PyValueError::new_err("unexpected value"))?;

                Ok(Self(r))
            }
        }
    };
}

impl_from_py_object!(ModelTypeWrapper);
impl_from_py_object!(DataTypeWrapper);
impl_from_py_object!(DataUnitsWrapper);
impl_from_py_object!(DataFormatWrapper);
impl_from_py_object!(DataOrderingWrapper);
impl_from_py_object!(TideSystemWrapper);
impl_from_py_object!(CoordTypeWrapper);
impl_from_py_object!(CoordUnitsWrapper);

impl<'a> FromPyObject<'a> for CreationDateWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;

        let year = dict
            .get_item("year")?
            .ok_or(PyKeyError::new_err("year"))?
            .extract()?;
        let month = dict
            .get_item("month")?
            .ok_or(PyKeyError::new_err("month"))?
            .extract()?;
        let day = dict
            .get_item("day")?
            .ok_or(PyKeyError::new_err("day"))?
            .extract()?;

        Ok(Self(CreationDate { year, month, day }))
    }
}

impl<'a> FromPyObject<'a> for CoordWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        if let Ok(v) = ob.extract() {
            Ok(Self(Coord::Dec(v)))
        } else if let Ok(dict) = ob.downcast::<PyDict>() {
            let deg = dict
                .get_item("degree")?
                .ok_or(PyKeyError::new_err("degree"))?
                .extract()?;
            let min = dict
                .get_item("minutes")?
                .ok_or(PyKeyError::new_err("minutes"))?
                .extract()?;
            let sec = dict
                .get_item("second")?
                .ok_or(PyKeyError::new_err("second"))?
                .extract()?;

            Ok(Self(Coord::with_dms(deg, min, sec)))
        } else {
            Err(PyTypeError::new_err("unexpected type"))
        }
    }
}
