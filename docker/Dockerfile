from ubuntu:18.04
RUN apt-get update
RUN apt-get install -y curl git zip build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN chmod +x rustup.sh && ./rustup.sh -y
RUN bash -lc "source $HOME/.cargo/env && rustup default nightly"
RUN mkdir -p /build-arena
COPY ./ubuntu_builder.sh /
RUN chmod +x /ubuntu_builder.sh
CMD /ubuntu_builder.sh
