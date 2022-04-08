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
REQUIREMENTS=$(IMPOSCDIR)/requirements
REQUIREMENTS_DEV=$(IMPOSCDIR)/requirements-dev
TOUCH=touch
PORT?=8000


venv: $(VENV)/$(MARKER)

$(VENV):
	$(PY) -m venv $(VENVDIR)
	$(VENV)/python -m pip install --upgrade pip setuptools wheel

$(REQUIREMENTS).out: $(VENV) $(REQUIREMENTS).txt
	$(VENV)/python -m pip install -r $(REQUIREMENTS).txt
	$(VENV)/python -m pip freeze -r $(REQUIREMENTS).txt > $(REQUIREMENTS).out

$(REQUIREMENTS_DEV).out: $(VENV) $(REQUIREMENTS_DEV).txt
	$(VENV)/python -m pip install -r $(REQUIREMENTS_DEV).txt
	$(VENV)/python -m pip freeze -r $(REQUIREMENTS_DEV).txt > $(REQUIREMENTS_DEV).out
        
.PHONY: clean-out
clean-out:
	-$(RM) $(REQUIREMENTS).out $(REQUIREMENTS_DEV).out

$(VENV)/$(MARKER):
	$(TOUCH) $(VENV)/$(MARKER)

.PHONY: clean-venv
clean-venv:
	-$(RM) -r "$(VENVDIR)"
        
.PHONY: clean-img
clean-img:
	-$(RM) ./*.png imposc/*.png

.PHONY: clean-cargo
clean-cargo:
	-$(RM) -r "$(TARGET)"
	-$(RM) $(DEVELOPDIR)/$(MARKER)

.PHONY: clean
clean: clean-venv clean-cargo clean-out clean-img

.PHONY: develop $(REQUIREMENTS).out
develop: $(DEVELOPDIR)/$(MARKER) $(TARGET)

$(DEVELOPDIR)/$(MARKER): $(REQUIREMENTS_DEV).out
	source $(VENV)/activate && cd imposclib && maturin develop

.PHONY: npm-install
npm-install:
	cd imposc/static && npm install

.PHONY: cargo-test pytest test jstest
cargo-test:
	cd imposclib && RUST_LOG=debug cargo test

pytest: develop $(REQUIREMENTS).out
	$(VENV)/python -m pip install pytest pytest-cov
	cd imposc && $(VENV)/python -m pytest

jstest: npm-install
	cd imposc/static && npx browserslist@latest --update-db && npm test

test: cargo-test pytest jstest

.PHONY: cargo-doc
cargo-doc:
	cd imposclib && cargo doc --no-deps

run: develop clean-img $(REQUIREMENTS).out
	source $(VENV)/activate && cd $(IMPOSCDIR) && uvicorn main:app --host 0.0.0.0 --port $(PORT) --reload

run-debug: develop $(REQUIREMENTS).out
	source $(VENV)/activate && cd $(IMPOSCDIR) && RUST_LOG=debug uvicorn main:app --host 0.0.0.0 --port $(PORT) --reload