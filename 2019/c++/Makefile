CC=g++
CFLAGS=-std=c++17 -Isrc
OBJS=$(patsubst src/%.cpp,obj/%.o,$(wildcard src/*.cpp))

build: $(OBJS)
	$(CC) $^ -o aocpp $(CFLAGS)

obj/%.o: src/%.cpp | obj
	$(CC) $^ -c -o $@ $(CFLAGS)

obj:
	mkdir -p obj

clean:
	rm -f obj/*.o
	rm -f aocpp
