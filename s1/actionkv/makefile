PROJECT=akv

SVR_IMAGE=
SVR_BIN=akv
MODE=   #如果是release使用--release

GIT_VERSION ?= $(shell git describe --tags --always --dirty)
BUILD_TIME ?= $(shell date '+%Y-%m-%d__%H:%M:%S%p')
.PHONY: all clean build  docker-build

all: build

build:
	cargo build --target-dir ./target
	cp -f ./target/debug/${SVR_BIN} ./deploy

clean:
	-rm ./deploy/${SVR_BIN}