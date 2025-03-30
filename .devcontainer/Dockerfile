FROM debian:trixie

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Paris

RUN	apt update -y \
&&	apt upgrade -y --no-install-recommends \
&&	apt install -y --no-install-recommends \
		sudo ca-certificates \
		ssh git bash-completion pkg-config \
		cmake clang lld rustup libclang-dev \
		libasound2-dev pipewire-alsa pipewire  \
		libx11-dev libudev-dev \
		libwayland-dev libxkbcommon-dev mesa-vulkan-drivers \
&&	groupadd --gid 1000 devgroup \
&&	useradd --uid 1000 --gid devgroup --shell /bin/bash --create-home devuser \
&&	mkdir -p /etc/sudoers.d \
&&	echo "devuser ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers.d/user \
&&	ln -sf /usr/share/zoneinfo/$TZ /etc/localtime \
&&	echo $TZ > /etc/timezone

USER devuser
RUN	rustup default stable \
&&	rustup toolchain add nightly-x86_64-unknown-linux-gnu \
&&	rustup target add x86_64-pc-windows-gnu
