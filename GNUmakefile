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
export GIT_WORK_DIR=$(shell git rev-parse --show-toplevel)
export PROJ=live-bootcamp-project
export SOLN=live-bootcamp-solution
export APP_DIR="${GIT_WORK_DIR}/app-service"
export AUTH_DIR="${GIT_WORK_DIR}/auth-service"

ifdef remote
TCP:=${DIGITAL_IP}
else
TCP:=localhost
endif

#.______________________________________________________________________________
#| * build-app - create app service
build-app:
	cd "${APP_DIR}" && cargo build
#.______________________________________________________________________________
#| * build-auth - create auth services
build-auth:
	cd "${AUTH_DIR}" && cargo build
#.______________________________________________________________________________
#| * build - create both services
build-all: build-app build-auth

#.______________________________________________________________________________
#| * run-app - run the app locally
run-app:
	cd "${APP_DIR}" && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run &

#.______________________________________________________________________________
#| * run-auth - run the auth locally
run-auth:
	cd "${AUTH_DIR}" && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run &

#.______________________________________________________________________________
#| * dock-up - get the docker environment up and running
dock-up:
	cd "${APP_DIR}" && docker compose build
	cd "${APP_DIR}" && docker compose up &

#.______________________________________________________________________________
#| * dock-ps - show docker status
dock-ps:
	cd "${APP_DIR}" && docker ps

#.______________________________________________________________________________
#| * dock-down - get the docker environment up and running
dock-down:
	cd "${APP_DIR}" && docker compose down

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
#| * compare-solution - use BeyondCompare to compare against Bogdan's solution
compare-solution:
	cd $(dir ${GIT_WORK_DIR})\
	&& byc ${PROJ} ${SOLN}

#.______________________________________________________________________________
#| * Aliases: up, down, local, remote
byc: compare-solution
up: dock-up
down: dock-down
local: view-all
remote:
	$(MAKE) view-all remote=1

#|
#|------------------------------------------------------------------------------
# vim:syntax=make:nospell
# The end
