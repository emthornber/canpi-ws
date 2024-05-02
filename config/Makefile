################################################################################
#
#	Derive canpi-ws.cfg from canpi-ws.json current values
#
#   2 May, 2024 - E M Thornber
#   Created
#
################################################################################

FNAME := canpi-ws
PYTHON := /usr/bin/python3
SCRIPT := ../scripts/create_canpiws_cfg.py

all: $(FNAME).cfg

.PHONY: all

$(FNAME).cfg: $(FNAME).json
	$(PYTHON) $(SCRIPT) < $< > $@