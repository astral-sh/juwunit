from __future__ import annotations

from datetime import datetime, timedelta, timezone
from io import BytesIO, StringIO
from pathlib import Path
from uuid import UUID

from inline_snapshot import snapshot

import pytest

import juwunit


def test_basic_serialization_and_counts() -> None:
    suite = juwunit.TestSuite("suite")
    suite.add_test_cases(
        [
            juwunit.TestCase("passes", juwunit.Success()),
            juwunit.TestCase("fails", juwunit.Failure("boom")),
            juwunit.TestCase("errors", juwunit.Error("kaboom")),
            juwunit.TestCase("skips", juwunit.Skipped("later")),
        ]
    )
    report = juwunit.Report("run")
    report.add_test_suite(suite)

    assert report.to_xml() == snapshot("""\
<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="run" tests="4" failures="1" errors="1">
    <testsuite name="suite" tests="4" disabled="1" errors="1" failures="1">
        <testcase name="passes">
        </testcase>
        <testcase name="fails">
            <failure message="boom"/>
        </testcase>
        <testcase name="errors">
            <error message="kaboom"/>
        </testcase>
        <testcase name="skips">
            <skipped message="later"/>
        </testcase>
    </testsuite>
</testsuites>
""")


def test_full_feature_roundtrip() -> None:
    started = datetime(2026, 5, 8, 20, 53, 15, 186000, tzinfo=timezone.utc)
    flaky = juwunit.TestRerun(
        juwunit.NonSuccessKind.FAILURE,
        timestamp=started,
        time=timedelta(milliseconds=5),
        message="first try",
        type="AssertionError",
        stack_trace="stack",
        system_out="out",
        system_err="err",
        description="flaky description",
    )
    rerun = juwunit.TestRerun(
        juwunit.NonSuccessKind.ERROR,
        timestamp=started,
        time=timedelta(milliseconds=7),
        message="retry",
    )
    passing = juwunit.TestCase(
        "passing",
        juwunit.Success(flaky_runs=[flaky]),
        classname="pkg.tests",
        assertions=3,
        timestamp=started,
        time=timedelta(milliseconds=12),
        system_out="stdout",
        system_err="stderr",
        extra={"file": "tests/test_pkg.py"},
        properties=[juwunit.Property("step", "one")],
    )
    failing = juwunit.TestCase(
        "failing",
        juwunit.Failure(
            "failed",
            type="AssertionError",
            description="expected true",
            reruns=[rerun],
            rerun_kind=juwunit.RerunKind.FLAKY,
        ),
    )
    suite = juwunit.TestSuite(
        "suite",
        timestamp=started,
        time=timedelta(seconds=1),
        test_cases=[passing, failing],
        properties=[juwunit.Property("env", "test")],
        system_out="suite out",
        system_err="suite err",
        extra={"hostname": "ci"},
    )
    report = juwunit.Report(
        "run",
        uuid=UUID("bfdac0d9-1740-4af6-bfac-126438be6a1f"),
        timestamp=started,
        time=timedelta(seconds=2),
    )
    report.add_test_suite(suite)

    parsed = juwunit.Report.from_xml(report.to_xml())
    parsed_suite = parsed.test_suites[0]
    parsed_passing = parsed_suite.test_cases[0]
    parsed_failing = parsed_suite.test_cases[1]
    passing_status = parsed_passing.status
    failing_status = parsed_failing.status

    assert isinstance(passing_status, juwunit.Success)
    assert isinstance(failing_status, juwunit.Failure)

    assert parsed.to_xml() == snapshot("""\
<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="run" tests="2" failures="1" errors="0" uuid="bfdac0d9-1740-4af6-bfac-126438be6a1f" timestamp="2026-05-08T20:53:15.186+00:00" time="2.000">
    <testsuite name="suite" tests="2" disabled="0" errors="0" failures="1" timestamp="2026-05-08T20:53:15.186+00:00" time="1.000" hostname="ci">
        <properties>
            <property name="env" value="test"/>
        </properties>
        <testcase name="passing" classname="pkg.tests" assertions="3" timestamp="2026-05-08T20:53:15.186+00:00" time="0.012" file="tests/test_pkg.py">
            <properties>
                <property name="step" value="one"/>
            </properties>
            <flakyFailure timestamp="2026-05-08T20:53:15.186+00:00" time="0.005" message="first try" type="AssertionError">flaky description
                <stackTrace>stack</stackTrace>
                <system-out>out</system-out>
                <system-err>err</system-err>
            </flakyFailure>
            <system-out>stdout</system-out>
            <system-err>stderr</system-err>
        </testcase>
        <testcase name="failing">
            <failure message="failed" type="AssertionError">expected true</failure>
            <flakyError timestamp="2026-05-08T20:53:15.186+00:00" time="0.007" message="retry">
            </flakyError>
        </testcase>
        <system-out>suite out</system-out>
        <system-err>suite err</system-err>
    </testsuite>
</testsuites>
""")
    assert {
        "report": {
            "uuid": str(parsed.uuid),
            "timestamp": parsed.timestamp,
            "time": parsed.time,
        },
        "suite": {
            "extra": parsed_suite.extra,
            "properties": [
                (property.name, property.value) for property in parsed_suite.properties
            ],
        },
        "passing": {
            "extra": parsed_passing.extra,
            "properties": [
                (property.name, property.value)
                for property in parsed_passing.properties
            ],
            "status_type": type(passing_status).__name__,
            "flaky_stack_trace": passing_status.flaky_runs[0].stack_trace,
        },
        "failing": {
            "status_type": type(failing_status).__name__,
            "rerun_kind": repr(failing_status.rerun_kind),
            "rerun_kinds": [repr(rerun.kind) for rerun in failing_status.reruns],
        },
    } == snapshot(
        {
            "report": {
                "uuid": "bfdac0d9-1740-4af6-bfac-126438be6a1f",
                "timestamp": datetime(
                    2026, 5, 8, 20, 53, 15, 186000, tzinfo=timezone.utc
                ),
                "time": timedelta(seconds=2),
            },
            "suite": {"extra": {"hostname": "ci"}, "properties": [("env", "test")]},
            "passing": {
                "extra": {"file": "tests/test_pkg.py"},
                "properties": [("step", "one")],
                "status_type": "Success",
                "flaky_stack_trace": "stack",
            },
            "failing": {
                "status_type": "Failure",
                "rerun_kind": "RerunKind.FLAKY",
                "rerun_kinds": ["NonSuccessKind.ERROR"],
            },
        }
    )


def test_deserialization_uses_derived_counts() -> None:
    xml = """<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="run" tests="99" failures="99" errors="99">
    <testsuite name="suite" tests="99" disabled="99" errors="99" failures="99">
        <testcase name="passes"/>
        <testcase name="fails"><failure/></testcase>
    </testsuite>
</testsuites>
"""
    report = juwunit.Report.from_xml(xml)
    suite = report.test_suites[0]

    assert {
        "suite": {
            "tests": suite.tests,
            "disabled": suite.disabled,
            "failures": suite.failures,
            "errors": suite.errors,
        },
        "report": {
            "tests": report.tests,
            "failures": report.failures,
            "errors": report.errors,
        },
    } == snapshot(
        {
            "suite": {"tests": 2, "disabled": 0, "failures": 1, "errors": 0},
            "report": {"tests": 2, "failures": 1, "errors": 0},
        }
    )


def test_xml_sanitization() -> None:
    case = juwunit.TestCase("na\x00me", juwunit.Success(), system_out="ok\x1b[31mred")
    suite = juwunit.TestSuite("suite", test_cases=[case])
    report = juwunit.Report("run")
    report.add_test_suite(suite)

    assert {
        "name": case.name,
        "system_out": case.system_out,
        "xml": report.to_xml(),
    } == snapshot(
        {
            "name": "name",
            "system_out": "okred",
            "xml": '<?xml version="1.0" encoding="UTF-8"?>\n<testsuites name="run" tests="1" failures="0" errors="0">\n    <testsuite name="suite" tests="1" disabled="0" errors="0" failures="0">\n        <testcase name="name">\n            <system-out>okred</system-out>\n        </testcase>\n    </testsuite>\n</testsuites>\n',
        }
    )


def test_file_helpers() -> None:
    report = juwunit.Report("run")
    report.add_test_suite(juwunit.TestSuite("suite"))

    writer = StringIO()
    report.write_xml(writer)
    text = writer.getvalue()

    assert [
        juwunit.Report.read_xml(StringIO(text)).name,
        juwunit.Report.read_xml(BytesIO(text.encode())).name,
    ] == snapshot(["run", "run"])


def test_bad_xml_raises_custom_error() -> None:
    with pytest.raises(juwunit.DeserializationError):
        juwunit.Report.from_xml("<broken/>")


def test_rejects_naive_datetime_and_negative_timedelta() -> None:
    with pytest.raises(ValueError, match="timezone-aware"):
        juwunit.Report("run", timestamp=datetime(2026, 5, 8))

    with pytest.raises(ValueError, match="negative timedelta"):
        juwunit.Report("run", time=timedelta(seconds=-1))


def test_typing_artifacts_are_packaged() -> None:
    package_dir = Path(juwunit.__file__).parent
    assert (package_dir / "__init__.pyi").is_file()
    assert (package_dir / "py.typed").is_file()
