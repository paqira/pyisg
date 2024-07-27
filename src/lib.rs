use libisg::{
    Coord, CoordType, CoordUnits, CreationDate, Data, DataFormat, DataOrdering, DataType,
    DataUnits, Header, ModelType, TideSystem, ISG,
};
use pyo3::create_exception;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

mod from_py_obj;
mod into;
mod to_py_obj;

create_exception!(pyisg, SerError, PyValueError);
create_exception!(pyisg, DeError, PyValueError);

pub(crate) struct HeaderWrapper(Header);
pub(crate) struct DataWrapper(Data);
pub(crate) struct ModelTypeWrapper(ModelType);
pub(crate) struct DataTypeWrapper(DataType);
pub(crate) struct DataUnitsWrapper(DataUnits);
pub(crate) struct DataFormatWrapper(DataFormat);
pub(crate) struct DataOrderingWrapper(DataOrdering);
pub(crate) struct TideSystemWrapper(TideSystem);
pub(crate) struct CoordTypeWrapper(CoordType);
pub(crate) struct CoordUnitsWrapper(CoordUnits);
pub(crate) struct CreationDateWrapper(CreationDate);
pub(crate) struct CoordWrapper(Coord);

#[pyfunction]
fn loads<'a>(py: Python<'a>, s: &'a str) -> PyResult<Bound<'a, PyDict>> {
    let isg = libisg::from_str(s).map_err(|e| DeError::new_err(e.to_string()))?;

    let dict = PyDict::new_bound(py);

    dict.set_item("comment", isg.comment)?;
    dict.set_item("header", HeaderWrapper(isg.header))?;
    dict.set_item("data", DataWrapper(isg.data))?;

    Ok(dict)
}

#[pyfunction]
fn dumps(obj: Bound<'_, PyDict>) -> PyResult<String> {
    let comment = obj
        .get_item("comment")?
        .map_or(Ok("".to_string()), |o| o.extract())
        .map_err(|_| PyTypeError::new_err("unexpected type on `comment`, expected str | None"))?;

    let header = obj
        .get_item("header")?
        .ok_or(SerError::new_err("missing `header`"))?
        .extract::<HeaderWrapper>()?
        .into();

    let data = obj
        .get_item("data")?
        .ok_or(SerError::new_err("missing `data`"))?
        .extract::<DataWrapper>()?
        .into();

    let isg = ISG {
        comment,
        header,
        data,
    };

    let s = isg.to_string();
    Ok(s)
}

#[pymodule]
fn pyisg(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;

    m.add("SerError", py.get_type_bound::<SerError>())?;
    m.add("DeError", py.get_type_bound::<DeError>())?;

    Ok(())
}
