from collections.abc import Sequence
from datetime import datetime, timedelta
from typing import BinaryIO, ClassVar, TextIO, TypeAlias
from uuid import UUID

class JuwunitError(Exception): ...
class DeserializationError(JuwunitError): ...
class SerializationError(JuwunitError): ...

class NonSuccessKind:
    FAILURE: ClassVar[NonSuccessKind]
    ERROR: ClassVar[NonSuccessKind]

class RerunKind:
    FLAKY: ClassVar[RerunKind]
    RERUN: ClassVar[RerunKind]

class Property:
    name: str
    value: str
    def __init__(self, name: str, value: str) -> None: ...

class TestRerun:
    kind: NonSuccessKind
    timestamp: datetime | None
    time: timedelta | None
    message: str | None
    type: str | None
    stack_trace: str | None
    system_out: str | None
    system_err: str | None
    description: str | None
    def __init__(
        self,
        kind: NonSuccessKind,
        *,
        timestamp: datetime | None = ...,
        time: timedelta | None = ...,
        message: str | None = ...,
        type: str | None = ...,
        stack_trace: str | None = ...,
        system_out: str | None = ...,
        system_err: str | None = ...,
        description: str | None = ...,
    ) -> None: ...

class Success:
    flaky_runs: Sequence[TestRerun]
    def __init__(self, *, flaky_runs: Sequence[TestRerun] = ...) -> None: ...
    def add_rerun(self, rerun: TestRerun) -> None: ...
    def add_reruns(self, reruns: Sequence[TestRerun]) -> None: ...

class Failure:
    kind: NonSuccessKind
    message: str | None
    type: str | None
    description: str | None
    reruns: Sequence[TestRerun]
    rerun_kind: RerunKind
    def __init__(
        self,
        message: str | None = ...,
        *,
        type: str | None = ...,
        description: str | None = ...,
        reruns: Sequence[TestRerun] = ...,
        rerun_kind: RerunKind = ...,
    ) -> None: ...
    def add_rerun(self, rerun: TestRerun) -> None: ...
    def add_reruns(self, reruns: Sequence[TestRerun]) -> None: ...

class Error:
    kind: NonSuccessKind
    message: str | None
    type: str | None
    description: str | None
    reruns: Sequence[TestRerun]
    rerun_kind: RerunKind
    def __init__(
        self,
        message: str | None = ...,
        *,
        type: str | None = ...,
        description: str | None = ...,
        reruns: Sequence[TestRerun] = ...,
        rerun_kind: RerunKind = ...,
    ) -> None: ...
    def add_rerun(self, rerun: TestRerun) -> None: ...
    def add_reruns(self, reruns: Sequence[TestRerun]) -> None: ...

class Skipped:
    message: str | None
    type: str | None
    description: str | None
    def __init__(
        self,
        message: str | None = ...,
        *,
        type: str | None = ...,
        description: str | None = ...,
    ) -> None: ...

Status: TypeAlias = Success | Failure | Error | Skipped

class TestCase:
    name: str
    classname: str | None
    assertions: int | None
    timestamp: datetime | None
    time: timedelta | None
    status: Status
    system_out: str | None
    system_err: str | None
    extra: dict[str, str]
    properties: Sequence[Property]
    def __init__(
        self,
        name: str,
        status: Status,
        *,
        classname: str | None = ...,
        assertions: int | None = ...,
        timestamp: datetime | None = ...,
        time: timedelta | None = ...,
        system_out: str | None = ...,
        system_err: str | None = ...,
        extra: dict[str, str] | None = ...,
        properties: Sequence[Property] = ...,
    ) -> None: ...
    def add_property(self, property: Property) -> None: ...
    def add_properties(self, properties: Sequence[Property]) -> None: ...

class TestSuite:
    name: str
    tests: int
    disabled: int
    failures: int
    errors: int
    timestamp: datetime | None
    time: timedelta | None
    test_cases: Sequence[TestCase]
    properties: Sequence[Property]
    system_out: str | None
    system_err: str | None
    extra: dict[str, str]
    def __init__(
        self,
        name: str,
        *,
        timestamp: datetime | None = ...,
        time: timedelta | None = ...,
        test_cases: Sequence[TestCase] = ...,
        properties: Sequence[Property] = ...,
        system_out: str | None = ...,
        system_err: str | None = ...,
        extra: dict[str, str] | None = ...,
    ) -> None: ...
    def add_test_case(self, test_case: TestCase) -> None: ...
    def add_test_cases(self, test_cases: Sequence[TestCase]) -> None: ...
    def add_property(self, property: Property) -> None: ...
    def add_properties(self, properties: Sequence[Property]) -> None: ...

class Report:
    name: str
    uuid: UUID | None
    timestamp: datetime | None
    time: timedelta | None
    tests: int
    failures: int
    errors: int
    test_suites: Sequence[TestSuite]
    def __init__(
        self,
        name: str,
        *,
        uuid: UUID | None = ...,
        timestamp: datetime | None = ...,
        time: timedelta | None = ...,
    ) -> None: ...
    @classmethod
    def from_xml(cls, xml: str | bytes) -> Report: ...
    @classmethod
    def read_xml(cls, reader: TextIO | BinaryIO) -> Report: ...
    def to_xml(self) -> str: ...
    def write_xml(self, writer: TextIO) -> None: ...
    def add_test_suite(self, test_suite: TestSuite) -> None: ...
    def add_test_suites(self, test_suites: Sequence[TestSuite]) -> None: ...
