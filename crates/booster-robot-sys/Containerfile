FROM ubuntu:22.04

RUN apt update -y && \
 	apt install -y \
 	git \
 	lsb-release \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable
RUN git clone https://github.com/BoosterRobotics/booster_robotics_sdk.git
WORKDIR booster_robotics_sdk
RUN yes | ./install.sh
RUN mkdir build && cd build && cmake .. && make -j$(nproc)
