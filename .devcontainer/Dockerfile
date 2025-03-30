FROM public.ecr.aws/ubuntu/ubuntu:noble AS builder

ARG CMAKE_VERSION=4.0
ARG CMAKE_BUILD_VERSION=0

RUN apt update -y && \
	apt upgrade -y --no-install-recommends && \
	apt install -y --no-install-recommends \
		build-essential wget openssl libssl-dev

RUN cd /tmp && \
	wget https://cmake.org/files/v${CMAKE_VERSION}/cmake-${CMAKE_VERSION}.${CMAKE_BUILD_VERSION}.tar.gz --no-check-certificate && \
	tar -xzf cmake-${CMAKE_VERSION}.${CMAKE_BUILD_VERSION}.tar.gz && \
	cd cmake-${CMAKE_VERSION}.${CMAKE_BUILD_VERSION} && \
	./bootstrap && \
	make -j$(nproc) && \
	make install


FROM public.ecr.aws/ubuntu/ubuntu:noble

ARG CMAKE_VERSION=4.0

ENV CMAKE_ROOT=/usr/local/share/cmake-${CMAKE_VERSION}
COPY --from=builder /usr/local/share/cmake-${CMAKE_VERSION} /usr/local/share/cmake-${CMAKE_VERSION}
COPY --from=builder /usr/local/bin /usr/local/bin

RUN apt update -y && \
	apt upgrade -y --no-install-recommends && \
	apt install -y --no-install-recommends \
		bash-completion ssh git ninja-build \
		clang-19 lld-19 lldb-19 clang-format-19 clang-tidy-19 && \
	ln /usr/bin/clang-19 /usr/bin/clang && \
	ln /usr/bin/clang++-19 /usr/bin/clang++ && \
	ln /usr/bin/lld-19 /usr/bin/lld && \
	ln /usr/bin/lldb-19 /usr/bin/lldb && \
	ln /usr/bin/clang-format-19 /usr/bin/clang-format && \
	ln /usr/bin/clang-tidy-19 /usr/bin/clang-tidy

RUN mkdir -m 774 /workspaces && \
	chown -Rh ubuntu:ubuntu /workspaces