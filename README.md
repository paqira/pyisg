# pyisg

Library reading/writing the [ISG 2.0 format][SPEC].

This provides APIs, such as `load`, `loads`, `dump` and `dumps`.

```python
import pyisg

# serialize to ISG 2.0 format str to dict
with open("file.isg") as fs:
    obj = pyisg.load(fs)

# deserialize to ISG 2.0 format str
s = pyisg.dumps(obj)
```

## Licence

MIT or Apache-2.0

## Reference

Specification: https://www.isgeoid.polimi.it/Geoid/format_specs.html


[SPEC]: https://www.isgeoid.polimi.it/Geoid/format_specs.html
