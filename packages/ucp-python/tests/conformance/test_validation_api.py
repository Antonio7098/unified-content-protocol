"""
Conformance tests for ValidationResult API.

These tests verify API consistency for ValidationResult, ValidationIssue,
and related validation types.
"""

from ucp import (
    ValidationResult,
    ValidationIssue,
    ValidationSeverity,
)


class TestValidationIssueFactory:
    """Test ValidationIssue factory methods."""

    def test_error_factory(self):
        """error() creates ERROR severity issue."""
        issue = ValidationIssue.error("E001", "Error message")

        assert issue.severity == ValidationSeverity.ERROR
        assert issue.code == "E001"
        assert issue.message == "Error message"
        assert issue.block_id is None

    def test_error_with_block_id(self):
        """error() can include block_id."""
        issue = ValidationIssue.error("E001", "Error", block_id="blk_123")

        assert issue.block_id == "blk_123"

    def test_warning_factory(self):
        """warning() creates WARNING severity issue."""
        issue = ValidationIssue.warning("W001", "Warning message")

        assert issue.severity == ValidationSeverity.WARNING
        assert issue.code == "W001"
        assert issue.message == "Warning message"

    def test_warning_with_block_id(self):
        """warning() can include block_id."""
        issue = ValidationIssue.warning("W001", "Warning", block_id="blk_456")

        assert issue.block_id == "blk_456"

    def test_info_factory(self):
        """info() creates INFO severity issue."""
        issue = ValidationIssue.info("I001", "Info message")

        assert issue.severity == ValidationSeverity.INFO
        assert issue.code == "I001"
        assert issue.message == "Info message"

    def test_info_with_block_id(self):
        """info() can include block_id."""
        issue = ValidationIssue.info("I001", "Info", block_id="blk_789")

        assert issue.block_id == "blk_789"


class TestValidationResultMethods:
    """Test ValidationResult filtering methods."""

    def test_errors_returns_only_errors(self):
        """errors() returns only ERROR severity issues."""
        result = ValidationResult(
            valid=False,
            issues=[
                ValidationIssue.error("E001", "Error 1"),
                ValidationIssue.warning("W001", "Warning 1"),
                ValidationIssue.info("I001", "Info 1"),
                ValidationIssue.error("E002", "Error 2"),
            ]
        )

        errors = result.errors()

        assert len(errors) == 2
        assert all(e.severity == ValidationSeverity.ERROR for e in errors)
        assert errors[0].code == "E001"
        assert errors[1].code == "E002"

    def test_warnings_returns_only_warnings(self):
        """warnings() returns only WARNING severity issues."""
        result = ValidationResult(
            valid=True,
            issues=[
                ValidationIssue.error("E001", "Error 1"),
                ValidationIssue.warning("W001", "Warning 1"),
                ValidationIssue.warning("W002", "Warning 2"),
                ValidationIssue.info("I001", "Info 1"),
            ]
        )

        warnings = result.warnings()

        assert len(warnings) == 2
        assert all(w.severity == ValidationSeverity.WARNING for w in warnings)
        assert warnings[0].code == "W001"
        assert warnings[1].code == "W002"

    def test_infos_returns_only_infos(self):
        """infos() returns only INFO severity issues."""
        result = ValidationResult(
            valid=True,
            issues=[
                ValidationIssue.error("E001", "Error 1"),
                ValidationIssue.warning("W001", "Warning 1"),
                ValidationIssue.info("I001", "Info 1"),
                ValidationIssue.info("I002", "Info 2"),
                ValidationIssue.info("I003", "Info 3"),
            ]
        )

        infos = result.infos()

        assert len(infos) == 3
        assert all(i.severity == ValidationSeverity.INFO for i in infos)
        assert infos[0].code == "I001"
        assert infos[1].code == "I002"
        assert infos[2].code == "I003"

    def test_empty_result_returns_empty_lists(self):
        """Empty result returns empty lists for all methods."""
        result = ValidationResult.success()

        assert result.errors() == []
        assert result.warnings() == []
        assert result.infos() == []

    def test_no_errors_means_empty_list(self):
        """Result with no errors returns empty errors list."""
        result = ValidationResult(
            valid=True,
            issues=[
                ValidationIssue.warning("W001", "Warning"),
                ValidationIssue.info("I001", "Info"),
            ]
        )

        assert result.errors() == []

    def test_no_warnings_means_empty_list(self):
        """Result with no warnings returns empty warnings list."""
        result = ValidationResult(
            valid=False,
            issues=[
                ValidationIssue.error("E001", "Error"),
                ValidationIssue.info("I001", "Info"),
            ]
        )

        assert result.warnings() == []

    def test_no_infos_means_empty_list(self):
        """Result with no infos returns empty infos list."""
        result = ValidationResult(
            valid=False,
            issues=[
                ValidationIssue.error("E001", "Error"),
                ValidationIssue.warning("W001", "Warning"),
            ]
        )

        assert result.infos() == []


class TestValidationResultCreation:
    """Test ValidationResult creation methods."""

    def test_success_creates_valid_result(self):
        """success() creates valid result with no issues."""
        result = ValidationResult.success()

        assert result.valid is True
        assert result.issues == []

    def test_failure_with_errors_creates_invalid(self):
        """failure() with errors creates invalid result."""
        issues = [
            ValidationIssue.error("E001", "Error")
        ]
        result = ValidationResult.failure(issues)

        assert result.valid is False
        assert len(result.issues) == 1

    def test_failure_with_only_warnings_creates_valid(self):
        """failure() with only warnings creates valid result."""
        issues = [
            ValidationIssue.warning("W001", "Warning")
        ]
        result = ValidationResult.failure(issues)

        assert result.valid is True
        assert len(result.issues) == 1


class TestValidationResultMerge:
    """Test ValidationResult merge functionality."""

    def test_merge_combines_issues(self):
        """merge() combines issues from both results."""
        result1 = ValidationResult(
            valid=True,
            issues=[ValidationIssue.warning("W001", "Warning")]
        )
        result2 = ValidationResult(
            valid=True,
            issues=[ValidationIssue.info("I001", "Info")]
        )

        result1.merge(result2)

        assert len(result1.issues) == 2
        assert result1.warnings()[0].code == "W001"
        assert result1.infos()[0].code == "I001"

    def test_merge_invalidates_if_other_invalid(self):
        """merge() sets valid=False if other is invalid."""
        result1 = ValidationResult.success()
        result2 = ValidationResult(
            valid=False,
            issues=[ValidationIssue.error("E001", "Error")]
        )

        result1.merge(result2)

        assert result1.valid is False

    def test_merge_preserves_invalid_if_self_invalid(self):
        """merge() keeps valid=False if self was already invalid."""
        result1 = ValidationResult(
            valid=False,
            issues=[ValidationIssue.error("E001", "Error")]
        )
        result2 = ValidationResult.success()

        result1.merge(result2)

        assert result1.valid is False


class TestValidationSeverity:
    """Test ValidationSeverity enum."""

    def test_severity_values(self):
        """Severity enum has correct values."""
        assert ValidationSeverity.ERROR.value == "error"
        assert ValidationSeverity.WARNING.value == "warning"
        assert ValidationSeverity.INFO.value == "info"

    def test_severity_comparison(self):
        """Severity values can be compared."""
        error = ValidationSeverity.ERROR
        warning = ValidationSeverity.WARNING
        info = ValidationSeverity.INFO

        # Each is distinct
        assert error != warning
        assert warning != info
        assert error != info
