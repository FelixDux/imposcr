# look here https://github.com/sio/Makefile.venv
SHELL := /bin/bash
.PHONY: refresh venv pyenv

PY?=python3.9
MARKER=.initialized-with-Makefile.venv
IMPOSCDIR=$(realpath ./imposc)
VENVDIR?=$(IMPOSCDIR)/.venv
VENV=$(VENVDIR)/bin
LIBDIR=$(realpath ./imposclib)
TARGET?=$(LIBDIR)/target
DEVELOPDIR=$(VENVDIR)/lib/$(PY)/site-packages/imposclib
REQUIREMENTS_DEV=$(IMPOSCDIR)/requirements-dev.txt
TOUCH=touch


venv: $(VENV)/$(MARKER)

$(VENV):
	$(PY) -m venv $(VENVDIR)
	$(VENV)/python -m pip install --upgrade pip setuptools wheel

$(REQUIREMENTS_DEV): $(VENV)
	$(VENV)/python -m pip install -r $(REQUIREMENTS_DEV)

$(VENV)/$(MARKER): $(REQUIREMENTS_DEV)
	$(TOUCH) $(VENV)/$(MARKER)

.PHONY: clean-venv
clean-venv:
	-$(RM) -r "$(VENVDIR)"

.PHONY: clean-cargo
clean-cargo:
	-$(RM) -r "$(TARGET)"

.PHONY: clean
clean: clean-venv clean-cargo

.PHONY: develop
develop: $(DEVELOPDIR)/$(MARKER)

$(DEVELOPDIR)/$(MARKER): venv
	source $(VENV)/activate && cd imposclib && maturin develop
	$(TOUCH) $(DEVELOPDIR)/$(MARKER)

.PHONY: npm-install
npm-install:
	cd imposc/static && npm install

.PHONY: cargo-test pytest test jstest
cargo-test:
	cd imposclib && RUST_LOG=debug cargo test

pytest: develop
	cd imposc && $(VENV)/python -m pytest

jstest: npm-install
	cd imposc/static && npm test

test: cargo-test pytest jstest

.PHONY: cargo-doc
cargo-doc:
	cd imposclib && cargo doc --no-deps

run: develop
	source $(VENV)/activate && cd $(IMPOSCDIR) && uvicorn main:app --host 0.0.0.0 --port 8000 --reload

run-debug: develop
	source $(VENV)/activate && cd $(IMPOSCDIR) && RUST_LOG=debug uvicorn main:app --host 0.0.0.0 --port 8000 --reload