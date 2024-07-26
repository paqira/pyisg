import unittest

import pyisg

TEMPLATE = {
    "comment": "",
    "header": {
        "model_name": "EXAMPLE",
        "model_year": "2020",
        "model_type": "gravimetric",
        "data_type": "geoid",
        "data_units": "meters",
        "data_format": "grid",
        "data_ordering": "N-to-S, W-to-E",
        "ref_ellipsoid": "GRS80",
        "ref_frame": "ITRF2014",
        "height_datum": None,
        "tide_system": "mean-tide",
        "coord_type": "geodetic",
        "coord_units": "dms",
        "map_projection": None,
        "EPSG_code": "7912",
        "lat_min": {"degree": 39, "minutes": 50, "second": 0},
        "lat_max": {"degree": 41, "minutes": 10, "second": 0},
        "lon_min": {"degree": 119, "minutes": 50, "second": 0},
        "lon_max": {"degree": 121, "minutes": 50, "second": 0},
        "delta_lat": {"degree": 0, "minutes": 20, "second": 0},
        "delta_lon": {"degree": 0, "minutes": 20, "second": 0},
        "nrows": 4,
        "ncols": 6,
        "nodata": -9999.0,
        "creation_date": {"year": 2020, "month": 5, "day": 31},
        "ISG_format": "2.0",
    },
    "data": [
        [30.1234, 31.2222, 32.3456, 33.4444, 34.5678, 36.6666],
        [41.1111, 42.2345, 43.3333, 44.4567, 45.5555, 46.6789],
        [51.4321, 52.9753, 53.6543, 54.8642, None, None],
        [61.9999, 62.8888, 63.7777, 64.6666, None, None],
    ],
}


class TestTypeError(unittest.TestCase):
    def test_comment(self):
        obj = {
            "comment": 1,
            "header": {
                "model_name": "EXAMPLE",
                "model_year": "2020",
                "model_type": "gravimetric",
                "data_type": "geoid",
                "data_units": "meters",
                "data_format": "grid",
                "data_ordering": "N-to-S, W-to-E",
                "ref_ellipsoid": "GRS80",
                "ref_frame": "ITRF2014",
                "height_datum": None,
                "tide_system": "mean-tide",
                "coord_type": "geodetic",
                "coord_units": "dms",
                "map_projection": None,
                "EPSG_code": "7912",
                "lat_min": {"degree": 39, "minutes": 50, "second": 0},
                "lat_max": {"degree": 41, "minutes": 10, "second": 0},
                "lon_min": {"degree": 119, "minutes": 50, "second": 0},
                "lon_max": {"degree": 121, "minutes": 50, "second": 0},
                "delta_lat": {"degree": 0, "minutes": 20, "second": 0},
                "delta_lon": {"degree": 0, "minutes": 20, "second": 0},
                "nrows": 4,
                "ncols": 6,
                "nodata": -9999.0,
                "creation_date": {"year": 2020, "month": 5, "day": 31},
                "ISG_format": "2.0",
            },
            "data": [
                [30.1234, 31.2222, 32.3456, 33.4444, 34.5678, 36.6666],
                [41.1111, 42.2345, 43.3333, 44.4567, 45.5555, 46.6789],
                [51.4321, 52.9753, 53.6543, 54.8642, None, None],
                [61.9999, 62.8888, 63.7777, 64.6666, None, None],
            ],
        }

        with self.assertRaises(pyisg.SerializeError) as cm:
            pyisg.dumps(obj)
        self.assertEqual(cm.exception.args, ("unexpected type on `comment`, expected str | None",))

    def test_creation_date(self):
        obj = {
            "header": {
                "model_name": "EXAMPLE",
                "model_year": "2020",
                "model_type": "gravimetric",
                "data_type": "geoid",
                "data_units": "meters",
                "data_format": "grid",
                "data_ordering": "N-to-S, W-to-E",
                "ref_ellipsoid": "GRS80",
                "ref_frame": "ITRF2014",
                "height_datum": None,
                "tide_system": "mean-tide",
                "coord_type": "geodetic",
                "coord_units": "dms",
                "map_projection": None,
                "EPSG_code": "7912",
                "lat_min": {"degree": 39, "minutes": 50, "second": 0},
                "lat_max": {"degree": 41, "minutes": 10, "second": 0},
                "lon_min": {"degree": 119, "minutes": 50, "second": 0},
                "lon_max": {"degree": 121, "minutes": 50, "second": 0},
                "delta_lat": {"degree": 0, "minutes": 20, "second": 0},
                "delta_lon": {"degree": 0, "minutes": 20, "second": 0},
                "nrows": 4,
                "ncols": 6,
                "nodata": -9999.0,
                "creation_date": {"year": 100000, "month": 5, "day": 31},
                "ISG_format": "2.0",
            },
            "data": [
                [30.1234, 31.2222, 32.3456, 33.4444, 34.5678, 36.6666],
                [41.1111, 42.2345, 43.3333, 44.4567, 45.5555, 46.6789],
                [51.4321, 52.9753, 53.6543, 54.8642, None, None],
                [61.9999, 62.8888, 63.7777, 64.6666, None, None],
            ],
        }

        with self.assertRaises(pyisg.SerializeError) as cm:
            pyisg.dumps(obj)
        self.assertEqual(
            cm.exception.args,
            (
                "unexpected type on `creation_date`, expected { year: int (u16), month: int (u8), day: int (u8) } | None",
            ),
        )


class TestDeError(unittest.TestCase):
    def test_header(self):
        obj = {
            "comment": "",
            "data": [
                [30.1234, 31.2222, 32.3456, 33.4444, 34.5678, 36.6666],
                [41.1111, 42.2345, 43.3333, 44.4567, 45.5555, 46.6789],
                [51.4321, 52.9753, 53.6543, 54.8642, None, None],
                [61.9999, 62.8888, 63.7777, 64.6666, None, None],
            ],
        }

        with self.assertRaises(pyisg.SerializeError) as cm:
            pyisg.dumps(obj)
        self.assertEqual(cm.exception.args, ("missing `header`",))


if __name__ == "__main__":
    unittest.main()
