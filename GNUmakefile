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
export TCP
ifdef remote
TCP:=${DIGITAL_IP}
else
TCP:=localhost
endif

#.______________________________________________________________________________
#| * build-app - create app service
build-app:
	cd app-service && cargo build
#.______________________________________________________________________________
#| * build-auth - create auth services
build-auth:
	cd app-service && cargo build
#.______________________________________________________________________________
#| * build - create both services
build-all: build-app build-auth

#.______________________________________________________________________________
#| * run-app - run the app locally
run-app:
	cd app-service && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run &

#.______________________________________________________________________________
#| * run-auth - run the auth locally
run-auth:
	cd auth-service && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run &

#.______________________________________________________________________________
#| * dock-up - get the docker environment up and running
dock-up:
	cd app-service && docker compose build
	cd app-service && docker compose up &

#.______________________________________________________________________________
#| * dock-ps - show docker status
dock-ps:
	cd app-service && docker ps

#.______________________________________________________________________________
#| * dock-down - get the docker environment up and running
dock-down:
	cd app-service && docker compose down

#.______________________________________________________________________________
#| * view-app - open respective web page
view-app:
	open http://${TCP}:8000 &

#.______________________________________________________________________________
#| * view-auth - open respective web page
view-auth:
	open http://${TCP}:3000/hello &

#.______________________________________________________________________________
#| * view-all - open respective web page
view-all: view-app view-auth

#.______________________________________________________________________________
#| * Aliases: up, down, local, remote
up: dock-up
down: dock-down
local: view-all
remote:
	$(MAKE) view-all remote=1

#|
#|------------------------------------------------------------------------------
# vim:syntax=make:nospell
# The end
