FROM noodle/rust

RUN	curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash && \
	apt install -y --no-install-recommends \
		git-lfs \
		pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 \
		libwayland-dev libxkbcommon-dev \
		mesa-vulkan-drivers
