FROM python:3.9-slim-buster
SHELL ["/bin/bash", "-c"]
ENV HOME /root

RUN apt-get update && \
    apt-get install -y --no-install-recommends git vim curl\
    pkg-config libssl-dev\
    make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev\
    libsqlite3-dev wget llvm libncurses5-dev xz-utils

# rye install
# # RUN curl -sSf https://rye-up.com/get | bash
# # RUN echo 'source "$HOME/.rye/env"' >> ~/.bashrc

# # Rust 1.71.1 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.71.1
# # Rust env var
ENV PATH="/root/.cargo/bin:${PATH}"
#
# # pyenv install
RUN git clone https://github.com/pyenv/pyenv.git ~/.pyenv
ENV PYENV_ROOT $HOME/.pyenv
ENV PATH $PYENV_ROOT/bin:$PATH
RUN echo 'eval "$(pyenv init -)"' >> ~/.bashrc && \
    eval "$(pyenv init -)"
RUN pyenv install anaconda3-2021.05 && pyenv global anaconda3-2021.05

WORKDIR /home

# entrypoint
CMD ["/bin/bash"]
