"""Provides serialize/deserialize API of ISG 2.0 format."""

from __future__ import annotations

from typing import Final, TextIO

from . import types
from .types import ISGFormatType

try:
    from . import pyisg as rsimpl
except ImportError:
    import warnings

    warnings.warn("pyisg: not supported platform", stacklevel=1)
    raise

__version__: Final = "0.1.0"

__all__ = [
    "types",
    #
    "ISGFormatType",
    #
    "loads",
    "load",
    "dumps",
    "dump",
    #
    "SerializeError",
    "DeserializeError",
]


def loads(s: str) -> ISGFormatType:
    """Deserialize ISG 2.0 format :obj:`str` to :obj:`dict`.

    Args:
        s: ISG 2.0 format :obj:`str`

    Returns:
        dict of ISG data

    Raises:
        DeserializeError: deserialization failed
    """
    try:
        return rsimpl.loads(s)
    except Exception as e:
        raise DeserializeError(*e.args) from None


def load(fp: TextIO) -> ISGFormatType:
    """Deserialize ISG 2.0 file-like obj to :obj:`dict`.

    Args:
        fp: file-like obj of ISG 2.0 format data

    Returns:
        dict of ISG data

    Raises:
        DeserializeError: deserialization failed
    """
    return loads(fp.read())


def dumps(obj: ISGFormatType) -> str:
    """Serialize ISG 2.0 formatted str into :obj:`str`.

    Args:
        obj: dict of ISG data

    Raises:
        SerializeError: serialization failed
    """
    try:
        return rsimpl.dumps(obj)
    except Exception as e:
        raise SerializeError(*e.args) from None


def dump(obj: ISGFormatType, fp: TextIO) -> int:
    """Serialize ISG 2.0 formatted str into file-like obj.

    Args:
        obj: dict of ISG data
        fp: output file-like obj

    Returns:
        return value of fp.write

    Raises:
        SerializeError: serialization failed
    """
    return fp.write(dumps(obj))


class SerializeError(ValueError):
    """Error of :func:`dump` and :func:`dumps`."""

    pass


class DeserializeError(ValueError):
    """Error of :func:`load` and :func:`loads`."""

    pass
