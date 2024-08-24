FROM rust:1.75.0-bookworm as builder

RUN mkdir /data
RUN mkdir -p /data/.cargo
RUN mkdir -p /data/deploy
RUN mkdir -p /data/dump
RUN mkdir -p /data/src
RUN mkdir -p /data/target
RUN mkdir -p /data/tests
RUN mkdir -p /data/vendor


COPY  .cargo /data/.cargo
COPY  deploy /data/deploy
COPY  dump /data/dumps
COPY  src /data/src
COPY  tests /data/tests
COPY  src /data/src
COPY  vendor /data/vendor
COPY  Cargo.lock /data
COPY  Cargo.toml /data
COPY  makefile /data
COPY  readme.md /data

# RUN echo $PWD
# RUN cd ./deploy
RUN cd /data/deploy \
    && tar -zxvf cmake-3.12.2-Linux-x86_64.tar.gz \
    && mv cmake-3.12.2-Linux-x86_64 cmake-3.12 \
    && cp -r cmake-3.12 /usr/share/

ENV PATH=$PATH:/usr/share/cmake-3.12/bin 
RUN cd /data && make build

FROM ubuntu
ENV TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN mkdir -p /data/deploy
COPY --from=builder /data/deploy /data/deploy





CMD ["/bin/echo", "Hello world"]