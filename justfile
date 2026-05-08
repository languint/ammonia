set shell := ["bash", "-cu"]

build_dir := "build"
build_type := "Debug"

configure:
    cmake -B {{build_dir}} -S . -DCMAKE_BUILD_TYPE={{build_type}}

build: configure
    cmake --build {{build_dir}}

run: build
    {{build_dir}}/ammonia

test: build
    ctest --test-dir {{build_dir}} --output-on-failure

clean:
    rm -rf {{build_dir}}

rebuild: clean build

watch:
    find src tests -name '*.c' -o -name '*.h' | entr -c just test
