FROM rust:1.70.0

# cargo-competeをインストール
RUN cargo install cargo-compete

RUN apt-get update && apt-get install -y git

WORKDIR /tmp
RUN git clone https://github.com/githubnext/monaspace
RUN cp ./monaspace/util/install_linux.sh ./monaspace/fonts/
RUN sh ./monaspace/fonts/install_linux.sh

RUN echo 'alias actest="cargo compete test"' >> ~/.bashrc
RUN echo 'alias acsub="cargo compete submit"' >> ~/.bashrc
