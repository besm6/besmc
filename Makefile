#
# make          -- build besmc
# make test     -- run tests (must be sequential; share cwd)
# make install  -- install to ~/.local/bin
# make clean    -- remove build artifacts
#

CXX      ?= g++
CXXFLAGS ?= -std=c++20 -O2 -Wall -Wextra -Wpedantic
PREFIX   ?= $(HOME)/.local
BINDDIR  := $(PREFIX)/bin

SRC      := src/besmc.cpp src/main.cpp
OBJ      := $(SRC:src/%.cpp=build/%.o)
TEST_OBJ := build/besmc.o build/tests.o

.PHONY: all test install clean

all: build/besmc

build:
	mkdir -p build

build/%.o: src/%.cpp src/besmc.hpp | build
	$(CXX) $(CXXFLAGS) -Isrc -c -o $@ $<

build/tests.o: tests/tests.cpp src/besmc.hpp | build
	$(CXX) $(CXXFLAGS) -Isrc -c -o $@ $<

build/besmc: $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $(OBJ)

build/tests: $(TEST_OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $(TEST_OBJ)

test: build/tests
	./build/tests

install: build/besmc
	install -d $(BINDDIR)
	install -m 755 build/besmc $(BINDDIR)/besmc

clean:
	rm -rf build
	rm -f *.bin target/*.exe target/*.obj target/*.lst target/*.dub
