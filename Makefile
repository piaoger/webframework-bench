
# makefile
# borrowed from https://github.com/piaoger/golagger/Makefile

OS := $(shell uname -s)
THIS_FOLDER := $(shell cd ${0%/*} && echo ${PWD})
TIMESTAMP := $(shell date "+%G%m%d%H%M%S")
TODAY := $(shell date "+%G%m%d")

APP_NAME := uyun

APP_GLOBAL_BIN_DIR=${HOME}/.uyun/bin

export PATH := ${THIS_FOLDER}/bin:${PATH}

# --------------------------------------------
# node.js
# --------------------------------------------
NODEJS_VERSION := v14.15.1
NODEJS_ROOT_NAME := "unknown"
ifeq (${OS}, Linux)
	NODEJS_ROOT_NAME := node-${NODEJS_VERSION}-linux-x64
else
	NODEJS_ROOT_NAME := node-${NODEJS_VERSION}-darwin-x64
endif

APP_BIN_NODEJS_DIR := ${APP_GLOBAL_BIN_DIR}/apps/nodejs/${NODEJS_ROOT_NAME}
NODE := ${APP_BIN_NODEJS_DIR}/bin/node
NPM := ${APP_BIN_NODEJS_DIR}/bin/npm
#TSC := ${THIS_FOLDER}/node_modules/.bin/tsc
#WEBPACK := ${THIS_FOLDER}/node_modules/.bin/webpack
NCU := ${APP_BIN_NODEJS_DIR}/bin/ncu
export PATH := ${APP_BIN_NODEJS_DIR}/bin/:${THIS_FOLDER}/node_modules/.bin/:${PATH}
export NODE_PATH := ${NODE_PATH}:${APP_BIN_NODEJS_DIR}/lib/node_modules

# --------------------------------------------
# golang
# --------------------------------------------
GOLANG_VERSION=1.15.5
GOLANG_ROOT_NAME := "unknown"
ifeq (${OS}, Linux)
	GOLANG_ROOT_NAME := go${GOLANG_VERSION}.linux-amd64
else
	GOLANG_ROOT_NAME := go${GOLANG_VERSION}.darwin-amd64
endif
export GOROOT := ${APP_GLOBAL_BIN_DIR}/apps/golang/${GOLANG_ROOT_NAME}
export GOPATH := ${THIS_FOLDER}/vendor
export PATH := ${APP_GLOBAL_BIN_DIR}/apps/golang/${GOLANG_ROOT_NAME}/${OS}:${PATH}

GO_FILES :=${THIS_FOLDER}/go/*.go \
	${THIS_FOLDER}/go/**/*.go \
	${THIS_FOLDER}/go/**/**/*.go \
	${THIS_FOLDER}/go/**/**/**/*.go


GO := ${APP_GLOBAL_BIN_DIR}/apps/golang/${GOLANG_ROOT_NAME}/${OS}/go
GOFMT :=${APP_GLOBAL_BIN_DIR}/apps/golang/${GOLANG_ROOT_NAME}/${OS}/gofmt
GOFILES :=$(wildcard ${GO_FILES})

build:
	@${THIS_FOLDER}/bin/build.sh

run:
	@rm -rf result.md
	@${THIS_FOLDER}/bin/run.sh

clean:
	@cd ${THIS_FOLDER}/rust && cargo clean
	@rm -f ${THIS_FOLDER}/go/net_http/net_http


fmt:
	@echo 'rust fmt ...'
	@cd ${THIS_FOLDER}/rust && cargo fmt
	@echo 'go fmt ...'
	@${GOFMT} -s -w ${GOFILES}