FROM rust:1.66-slim
RUN apt update && apt install -y clang git
RUN cd /usr/local/src/ && \
    git clone https://github.com/rui314/mold.git && \
    mkdir mold/build && \
    cd mold/build && \
    git checkout v1.8.0 && \
    ../install-build-deps.sh && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. && \
    cmake --build . -j $(nproc) && \
    cmake --install .
RUN cargo install cargo-watch
