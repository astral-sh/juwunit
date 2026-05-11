# juwunit

_juwunit_ is a small JUnit XML serializer and deserializer for Python.

It's a small Python layer on top of the excellent [quick-junit].

[quick-junit]: https://crates.io/crates/quick-junit

## Usage

Install it:

```console
uv add juwunit
```

Loading a report:

```python
import juwunit

report = juwunit.Report.from_xml("...")
for suite in report.test_suites:
    for case in suite.test_cases:
        print(f"{case.name}: {case.status}")
```

Building a report:

```python
import juwunit

case = juwunit.TestCase("my-test", status=juwunit.Success())
suite = juwunit.TestSuite("my-suite")
suite.add_test_case(case)
report = juwunit.Report("my-report")
report.add_test_suite(suite)

print(report.to_xml())
```

The Python API follows `quick-junit` closely while using Python-native types where that is more
natural:

- `uuid.UUID` for report UUIDs
- timezone-aware `datetime.datetime` for timestamps
- `datetime.timedelta` for durations
- `Success`, `Failure`, `Error`, and `Skipped` status objects
- `dict[str, str]`-style `extra` attributes and ordered `Property` objects

## Development

Install the editable extension:

```shell
uv run --dev maturin develop
```

Run the unit tests:

```shell
# rust side
cargo test

# python side
uv run --dev pytest
```
