FROM docker.io/library/rust:bullseye
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN echo 'PS1="\[\e[32m\][\[\e[m\]\[\e[34m\]\u\[\e[m\]\[\e[31m\]@\[\e[m\]\[\e[33m\]\h\[\e[m\] \[\e[35m\]\w\[\e[m\]\[\e[37m\] \[\e[m\]\[\e[36m\]\\$\[\e[m\]\[\e[32m\]]\[\e[m\]: "' >> ~/.bashrc
