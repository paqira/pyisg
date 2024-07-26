use libisg::{Coord, Data, DataBounds};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::*;

impl<'a> FromPyObject<'a> for HeaderWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let obj = ob
            .downcast::<PyDict>()
            .map_err(|_| PyTypeError::new_err("unexpected type on `header`, expected dict"))?;

        let model_name = obj
            .get_item("model_name")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `model_name`, expected str | None")
            })?;
        let model_year = obj
            .get_item("model_year")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `model_year`, expected str | `None`")
            })?;
        let model_type = obj
            .get_item("model_type")?
            .map_or(Ok(None), |obj| obj.extract::<Option<ModelTypeWrapper>>())
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `model_type`, expected 'gravimetric' | 'geometric' | 'hybrid' | None",
                )
            })?
            .map(Into::into);
        let data_type = obj
            .get_item("data_type")?
            .map_or(Ok(None), |obj| obj.extract::<Option<DataTypeWrapper>>())
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `data_type`, expected 'geoid' | 'quasi-geoid' | None",
                )
            })?
            .map(Into::into);
        let data_units = obj
            .get_item("data_units")?
            .map_or(Ok(None), |obj| obj.extract::<Option<DataUnitsWrapper>>())
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `data_units`, expected 'meters' | 'feet' | None",
                )
            })?
            .map(Into::into);
        let data_format = obj
            .get_item("data_format")?
            .ok_or(SerError::new_err("missing `data_format`"))?
            .extract::<DataFormatWrapper>()
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `data_format`, expected 'grid' | 'sparse'")
            })?
            .into();
        let data_ordering = obj
            .get_item("data_ordering")?
            .map(|obj| obj.extract::<DataOrderingWrapper>())
            .transpose()
            .map_err(|_| PyTypeError::new_err("unexpected type on `data_ordering`, expected 'N-to-S, W-to-E' | 'lat, lon, N' | 'east, north, N' | 'N' | 'zeta' | None"))?
            .map(Into::into);
        let ref_ellipsoid = obj
            .get_item("ref_ellipsoid")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `ref_ellipsoid`, expected str | None")
            })?;
        let ref_frame = obj
            .get_item("ref_frame")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `ref_frame`, expected str | None")
            })?;
        let height_datum = obj
            .get_item("height_datum")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `height_datum`, expected str | None")
            })?;
        let tide_system = obj
            .get_item("tide_system")?
            .map_or(Ok(None), |obj| obj.extract::<Option<TideSystemWrapper>>())
            .map_err(|_| PyTypeError::new_err("unexpected type on `tide_system`, expected 'tide-free' | 'mean-tide' | 'zero-tide' | None"))?
            .map(Into::into);
        let coord_type = obj
            .get_item("coord_type")?
            .ok_or(SerError::new_err("missing `coord_type`"))?
            .extract::<CoordTypeWrapper>()
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `coord_type`, expected 'geodetic' | 'projected'",
                )
            })?
            .into();
        let coord_units = obj
            .get_item("coord_units")?
            .ok_or(SerError::new_err("missing `coord_units`"))?
            .extract::<CoordUnitsWrapper>()
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `coord_units`, expected 'dms' | 'deg' | 'meters' | 'feet'",
                )
            })?
            .into();
        let map_projection = obj
            .get_item("map_projection")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `map_projection`, expected str | None")
            })?;
        #[allow(non_snake_case)]
        let EPSG_code = obj
            .get_item("EPSG_code")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `EPSG_code`, expected str | None")
            })?;
        let nrows = obj
            .get_item("nrows")?
            .ok_or(SerError::new_err("missing `nrows`"))?
            .extract()
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `nrows`, expected int (usize)")
            })?;
        let ncols = obj
            .get_item("ncols")?
            .ok_or(SerError::new_err("missing `ncols`"))?
            .extract()
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `ncols`, expected int (usize)")
            })?;
        let nodata = obj
            .get_item("nodata")?
            .map_or(Ok(None), |obj| obj.extract())
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `nodata`, expected float | None")
            })?;
        let creation_date = obj
            .get_item("creation_date")?
            .map_or(Ok(None), |obj| obj.extract::<Option<CreationDateWrapper>>())
            .map_err(|_| {
                PyTypeError::new_err(
                    "unexpected type on `creation_date`, expected { year: int (u16), month: int (u8), day: int (u8) } | None",
                )
            })?
            .map(Into::into);
        #[allow(non_snake_case)]
        let ISG_format = obj
            .get_item("ISG_format")?
            .ok_or(SerError::new_err("missing `ISG_format`"))?
            .extract()
            .map_err(|_| {
                PyTypeError::new_err("unexpected type on `ISG_format`, expected str | None")
            })?;

        let data_bounds = match coord_type {
            CoordType::Geodetic => {
                let lat_min = obj
                    .get_item("lat_min")?
                    .ok_or(SerError::new_err("missing `lat_min`"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| PyTypeError::new_err("unexpected type on `lat_min`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .into();
                let lat_max = obj
                    .get_item("lat_max")?
                    .ok_or(SerError::new_err("missing `lat_max`"))
                    .map_err(|_| PyTypeError::new_err("unexpected type on `lat_max`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .extract::<CoordWrapper>()?
                    .into();
                let lon_min = obj
                    .get_item("lon_min")?
                    .ok_or(SerError::new_err("missing `lon_min`"))
                    .map_err(|_| PyTypeError::new_err("unexpected type on `lon_min`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .extract::<CoordWrapper>()?
                    .into();
                let lon_max = obj
                    .get_item("lon_max")?
                    .ok_or(SerError::new_err("missing `lon_max`"))
                    .map_err(|_| PyTypeError::new_err("unexpected type on `lon_max`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .extract::<CoordWrapper>()?
                    .into();

                match data_format {
                    DataFormat::Grid => {
                        let delta_lat = obj
                            .get_item("delta_lat")?
                            .ok_or(SerError::new_err("missing `delta_lat`"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| PyTypeError::new_err("unexpected type on `delta_lat`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();
                        let delta_lon = obj
                            .get_item("delta_lon")?
                            .ok_or(SerError::new_err("missing `delta_lon`"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| PyTypeError::new_err("unexpected type on `delta_lon`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
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
                    .ok_or(SerError::new_err("missing `north_min`"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| PyTypeError::new_err("unexpected type on `north_min`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .into();
                let north_max = obj
                    .get_item("north_max")?
                    .ok_or(SerError::new_err("missing `north_max`"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| PyTypeError::new_err("unexpected type on `north_max`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .into();
                let east_min = obj
                    .get_item("east_min")?
                    .ok_or(SerError::new_err("missing `east_min`"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| PyTypeError::new_err("unexpected type on `east_min`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .into();
                let east_max = obj
                    .get_item("east_max")?
                    .ok_or(SerError::new_err("missing `east_max`"))?
                    .extract::<CoordWrapper>()
                    .map_err(|_| PyTypeError::new_err("unexpected type on `east_max`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                    .into();

                match data_format {
                    DataFormat::Grid => {
                        let delta_north = obj
                            .get_item("delta_north")?
                            .ok_or(SerError::new_err("missing `delta_north`"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| PyTypeError::new_err("unexpected type on `delta_north`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
                            .into();
                        let delta_east = obj
                            .get_item("delta_east")?
                            .ok_or(SerError::new_err("missing `delta_east`"))?
                            .extract::<CoordWrapper>()
                            .map_err(|_| PyTypeError::new_err("unexpected type on `delta_east`, expected float | { degree: int (i16), minutes: int (u8), second: int (u8) }"))?
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
            Err(PyTypeError::new_err("unexpected type on `data`, expected list[list[float | None]] | list[tuple[float | { degree: int (i16), minutes: int (u8), second: int (u8) }, float | { degree: int (i16), minutes: int (u8), second: int (u8) }, float]]"))
        }
    }
}

impl<'a> FromPyObject<'a> for ModelTypeWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for DataTypeWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for DataUnitsWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for DataFormatWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for DataOrderingWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for TideSystemWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for CoordTypeWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for CoordUnitsWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let r = ob
            .extract::<String>()?
            .parse()
            .map_err(|_| SerError::new_err("unexpected value"))?;
        Ok(Self(r))
    }
}

impl<'a> FromPyObject<'a> for CreationDateWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        let year = dict
            .get_item("year")?
            .ok_or(SerError::new_err("missing `year`"))?
            .extract()?;
        let month = dict
            .get_item("month")?
            .ok_or(SerError::new_err("missing `month`"))?
            .extract()?;
        let day = dict
            .get_item("day")?
            .ok_or(SerError::new_err("missing `day`"))?
            .extract()?;
        Ok(Self(CreationDate { year, month, day }))
    }
}

impl<'a> FromPyObject<'a> for CoordWrapper {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<f64>() {
            Ok(Self(Coord::Dec(v)))
        } else if let Ok(dict) = ob.downcast::<PyDict>() {
            let deg = dict
                .get_item("degree")?
                .ok_or(SerError::new_err("missing `degree`"))?
                .extract()?;
            let min = dict
                .get_item("minutes")?
                .ok_or(SerError::new_err("missing `minutes`"))?
                .extract()?;
            let sec = dict
                .get_item("second")?
                .ok_or(SerError::new_err("missing `second`"))?
                .extract()?;
            Ok(Self(Coord::with_dms(deg, min, sec)))
        } else {
            Err(PyTypeError::new_err("unexpected type"))
        }
    }
}
