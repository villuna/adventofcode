CXX=g++
RUSTC=rustc
RUSTCFLAGS=+nightly --crate-type=staticlib -C panic=abort -C opt-level=3
CXXFLAGS=-std=c++17 -Isrc -O2 -ltbb
OBJS=$(patsubst src/%.cpp,obj/%.o,$(wildcard src/*.cpp))
LIBS=lib/librust.a

build: $(OBJS) $(LIBS)
	$(CXX) $^ -L ./lib -lrust -larmadillo -o aocpp $(CXXFLAGS)

debug: CXXFLAGS += -g
debug: build

obj/%.o: src/%.cpp src/*.hpp | obj
	$(CXX) $< -c -o $@ $(CXXFLAGS)

lib/librust.a: src/*.rs | lib 
	$(RUSTC) $(RUSTCFLAGS) src/rust.rs -o $@

obj:
	mkdir -p obj

lib:
	mkdir -p lib

clean:
	rm -f obj/*.o
	rm -f lib/*.a
	rm -f aocpp
