#!gmake -f

SHELL := bash
GREP  := $(shell command -v grep)
PERL  := $(shell command -v perl)

THIS_MAKEFILE := $(realpath $(lastword $(MAKEFILE_LIST)))
PHONIES := $(shell ${PERL} -lane 'print $$1 if m{^([a-zA-Z][-a-zA-Z0-9_]*):[^=]*$$};' ${THIS_MAKEFILE})

#|------------------------------------------------------------------------------
#|
#| Description
#| -----------
#|
#| This top-level GNUmakefile provides support for developers to automate
#| simple tasks.
#|
#| Targets
#| -------
#|
#.______________________________________________________________________________
#| * help - display documentation (safe)
PERLHELP:='print unless m{ x } or ( m{ [*] } and not m{safe} )'
help: # default target
	@${GREP} '^#|' ${THIS_MAKEFILE} | ${GREP} -v ' x ' | cut -c 3-

.PHONY: ${PHONIES}

# Useful variables that may need updating:
export AUTH_SERVICE="127.0.0.0" # Aka localhost
export DIGITAL_IP := 147.182.164.45

#.______________________________________________________________________________
#| * build - create the services
build:
	cd app-service && cargo build
	cd auth-service && cargo build
#.______________________________________________________________________________
#| * run - run the app locally
run:
	cd app-service && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run &

#.______________________________________________________________________________
#| * view-app|auth - open respective web page
view-app:
	open http://localhost:8000
view-auth:
	open http://localhost:3000

#.______________________________________________________________________________
#| * dock-up - get the docker environment up and running
dock-up:
	cd app-service && cdocker compose build
	cd app-service && cdocker compose up &

#.______________________________________________________________________________
#| * dock-down - get the docker environment up and running
dock-down:
	cd app-service && docker compose down

#.______________________________________________________________________________
#| * open-local - Open page on local machine
open-local:
	open http://localhost:8000/ &
	open http://localhost:3000/ &

#.______________________________________________________________________________
#| * open-remote - Open page on digital ocean
open-remote:
	open http://${DIGITAL_IP}:8000/ &
	open http://${DIGITAL_IP}:3000/hello &

# Aliases
up: dock-up
down: dock-down
local: open-local
remote: open-remote

#|
#|------------------------------------------------------------------------------
# vim:syntax=make:nospell
# The end
