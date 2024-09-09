#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use semver::Version;
    use toml_edit::DocumentMut;

    #[test]
    fn test_convert() -> anyhow::Result<()> {
        let uv_version = Version::parse("0.4.0")?;
        let mut rye: DocumentMut = r#"
[project]
name = "test-project"
version = "0.1.1"
description = "This is a test pyproject.toml"
authors = [
    { name = "John Smith", email = "john.smith@example.com" },
]
dependencies = ["narwhals"]
readme = "README.md"
requires-python = ">= 3.8"
classifiers = [
    "Programming Language :: Python :: 3",
    "License :: OSI Approved :: MIT License",
    "Operating System :: OS Independent",
]

[project.urls]
Homepage = "https://github.com/lucianosrp"

[project.optional-dependencies]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.rye]
managed = true
universal = true
virtual = true
generate-hashes = true
dev-dependencies = [
    "polars",
    "pandas",
    "pytest",
    "pytest-cov",
    "typing-extensions",
]
lock-with-sources = true

[tool.ruff.format]
docstring-code-format = true

[tool.ruff.lint]
ignore = ["F401"]

[tool.hatch.build]
exclude = ["/.*", "/docs", "/tests", "/examples"]

[tool.hatch.metadata]
allow-direct-references = true

[tool.hatch.build.targets.wheel]
packages = ["test-project"]

[tool.mypy]
strict = true

[[tool.mypy.overrides]]
module = ["narwhals.*"]
ignore_missing_imports = true

[tool.pytest.ini_options]
addopts = "--cov=dapter --cov-fail-under=100"
"#
        .parse()?;

        rye_uv::convert(&mut rye, uv_version)?;

        assert_eq!(
            rye.to_string(),
            r#"
[project]
name = "test-project"
version = "0.1.1"
description = "This is a test pyproject.toml"
authors = [
    { name = "John Smith", email = "john.smith@example.com" },
]
dependencies = ["narwhals"]
readme = "README.md"
requires-python = ">= 3.8"
classifiers = [
    "Programming Language :: Python :: 3",
    "License :: OSI Approved :: MIT License",
    "Operating System :: OS Independent",
]

[project.urls]
Homepage = "https://github.com/lucianosrp"

[project.optional-dependencies]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv]
managed = true
dev-dependencies = [
    "polars",
    "pandas",
    "pytest",
    "pytest-cov",
    "typing-extensions",
]
no-sources = false
package = false

[tool.uv.pip]
universal = true
generate-hashes = true

[tool.ruff.format]
docstring-code-format = true

[tool.ruff.lint]
ignore = ["F401"]

[tool.hatch.build]
exclude = ["/.*", "/docs", "/tests", "/examples"]

[tool.hatch.metadata]
allow-direct-references = true

[tool.hatch.build.targets.wheel]
packages = ["test-project"]

[tool.mypy]
strict = true

[[tool.mypy.overrides]]
module = ["narwhals.*"]
ignore_missing_imports = true

[tool.pytest.ini_options]
addopts = "--cov=dapter --cov-fail-under=100"
"#
        );

        Ok(())
    }
}
