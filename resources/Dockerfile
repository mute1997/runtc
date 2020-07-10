FROM ubuntu

ENV LOG4RS_CONFIG /runtc/resources/log4rs.yaml
ENV PATH $PATH:/usr/local/go/bin/
ENV PATH $PATH:/runtc/target/debug

RUN apt update -y && \
      apt install -y npm git wget node-tap && \
      wget https://dl.google.com/go/go1.14.1.linux-amd64.tar.gz && \
      tar -xvf go1.14.1.linux-amd64.tar.gz && \
      mv go /usr/local && \
      git clone https://github.com/opencontainers/runtime-tools.git /runtime-tools && \
      cd /runtime-tools && \
      git checkout v0.9.0 -b v0.9.0 && \
      go mod init runtime-tools && \
      go mod vendor && \
      make runtimetest validation-executables
