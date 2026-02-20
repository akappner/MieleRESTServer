.PHONY: blake pylint

PY_FILES := $(shell find . -name "*.py" -not -path "./.git/*" -not -path "./.github/*" -not -path "./.venv/*")

blake:
	black $(PY_FILES)

pylint:
	pylint $(PY_FILES)
