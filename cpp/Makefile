dev:
	@make build
	mv ./build/compile_commands.json .
	./build/wordle

build: FORCE
	cmake -S . -B build
	cmake --build build --parallel

FORCE: ;
