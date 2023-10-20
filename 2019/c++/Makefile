CXX=g++
RUSTC=rustc
RUSTCFLAGS=--crate-type=cdylib -C opt-level=3
CXXFLAGS=-std=c++17 -Isrc
OBJS=$(patsubst src/%.cpp,obj/%.o,$(wildcard src/*.cpp))
LIBS=lib/libday3.so

build: $(OBJS) $(LIBS)
	$(CXX) $^ -o aocpp $(CXXFLAGS)

debug: CXXFLAGS += -g
debug: build

obj/%.o: src/%.cpp src/*.hpp | obj
	$(CXX) $< -c -o $@ $(CXXFLAGS)

lib/libday3.so: src/day3.rs | lib 
	$(RUSTC) $< -o $@ $(RUSTCFLAGS)

obj:
	mkdir -p obj

lib:
	mkdir -p lib

clean:
	rm -f obj/*.o
	rm -f lib/*.so
	rm -f aocpp
