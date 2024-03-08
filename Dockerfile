FROM rust:1.70.0

# cargo-competeをインストール
RUN cargo install cargo-compete

RUN echo 'alias actest="cargo compete test"' >> ~/.bashrc
RUN echo 'alias acsub="cargo compete submit"' >> ~/.bashrc
