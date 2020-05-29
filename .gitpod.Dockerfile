FROM gitpod/workspace-full
                    
USER gitpod

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli