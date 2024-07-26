"""Provides definition of types."""

from __future__ import annotations

from typing import Literal, TypeAlias, TypedDict

__all__ = [
    "ISGFormatType",
    #
    "HeaderType",
    #
    "ModelTypeType",
    "DataTypeType",
    "DataUnitsType",
    "DataFormatType",
    "DataOrderingType",
    "TideSystemType",
    "CoordTypeType",
    "CoordUnitsType",
    "CreationDateType",
    "DmsCoordType",
    #
    "SparseData",
    "GridData",
]

ModelTypeType: TypeAlias = Literal["gravimetric", "geometric", "hybrid"]
DataTypeType: TypeAlias = Literal["geoid", "quasi-geoid"]
DataUnitsType: TypeAlias = Literal["meters", "feet"]
DataFormatType: TypeAlias = Literal["grid", "sparse"]
DataOrderingType: TypeAlias = Literal["N-to-S, W-to-E", "lat, lon, N", "east, north, N", "N", "zeta"]
TideSystemType: TypeAlias = Literal["tide-free", "mean-tide", "zero-tide"]
CoordTypeType: TypeAlias = Literal["geodetic", "projected"]
CoordUnitsType: TypeAlias = Literal["dms", "deg", "meters", "feet"]


class CreationDateType(TypedDict):
    """Type of creation date."""

    year: int
    month: int
    day: int


class DmsCoordType(TypedDict):
    """Type of DMS coordinate."""

    degree: int
    minutes: int
    second: int


class HeaderType(TypedDict):
    """Type of Header dict."""

    model_name: str | None
    model_year: str | None
    model_type: ModelTypeType | None
    data_type: DataTypeType | None
    data_units: DataUnitsType | None
    data_format: DataFormatType | None
    data_ordering: DataOrderingType | None
    ref_ellipsoid: str | None
    ref_frame: str | None
    height_datum: str | None
    tide_system: TideSystemType | None
    coord_type: CoordTypeType
    coord_units: CoordUnitsType
    map_projection: str | None
    EPSG_code: str | None
    lat_min: float | DmsCoordType | None
    lat_max: float | DmsCoordType | None
    north_min: float | DmsCoordType | None
    north_max: float | DmsCoordType | None
    lon_min: float | DmsCoordType | None
    lon_max: float | DmsCoordType | None
    east_min: float | DmsCoordType | None
    east_max: float | DmsCoordType | None
    delta_lat: float | DmsCoordType | None
    delta_lon: float | DmsCoordType | None
    delta_north: float | DmsCoordType | None
    delta_east: float | DmsCoordType | None
    nrows: int
    ncols: int
    nodata: float | None
    creation_date: CreationDateType | None
    ISG_format: str


SparseData: TypeAlias = list[tuple[float | DmsCoordType, float | DmsCoordType, float]]
GridData: TypeAlias = list[list[float | None]]


class ISGFormatType(TypedDict):
    """Type of ISG data dict."""

    comment: str
    header: HeaderType
    data: GridData | SparseData
