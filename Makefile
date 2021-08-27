# look here https://github.com/sio/Makefile.venv

.PHONY: refresh venv

MARKER=.initialized-with-Makefile.venv
VENVDIR?=$(realpath ./imposc/.venv)
VENV=$(VENVDIR)/bin
TARGET?=$(realpath ./imposclib/target)

venv: $(VENV)/$(MARKER)

$(VENV):
	python3 -m venv $(VENVDIR)
	$(VENV)/python -m pip install --upgrade pip setuptools wheel

$(VENV)/$(MARKER): $(VENV)
	$(VENV)/python -m pip install -r ./imposc/requirements-dev.txt
	touch $(VENV)/$(MARKER)

.PHONY: clean-venv
clean-venv:
	-$(RM) -r "$(VENVDIR)"

.PHONY: clean-cargo
clean-cargo:
	-$(RM) -r "$(TARGET)"

.PHONY: clean
clean: clean-venv clean-cargo

.PHONY: develop
develop: venv $(VENVDIR)/lib/python3.9/site-packages/imposclib
	source $(VENV)/activate && cd imposclib && maturin develop

.PHONY: cargo-test pytest test
cargo-test:
	cd imposclib && cargo test

pytest: develop
	cd imposc && $(VENV)/python -m pytest

test: cargo-test pytest