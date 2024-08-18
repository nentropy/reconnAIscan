# Stage 1: Base Image with Kali Tools
FROM debian:11 as kali-base

# Set environment variables for non-interactive apt installs
ENV DEBIAN_FRONTEND=noninteractive

# Update and install basic dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release \
    sudo \
    wget \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Add Kali Linux repository and its key
RUN echo "deb http://http.kali.org/kali kali-rolling main contrib non-free" > /etc/apt/sources.list.d/kali.list \
    && wget -q -O - https://archive.kali.org/archive-key.asc | apt-key add - \
    && apt-get update

# Install Kali tools (add or remove tools as needed)
RUN apt-get install -y --no-install-recommends \
    aircrack-ng \
    airmon-ng \
    tcpdump \
    iw \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Stage 2: Install airgorah
FROM kali-base as airgorah-install

# Install airgorah
RUN wget https://github.com/martin-olivier/airgorah/releases/download/v0.7.3/airgorah_0.7.3_`dpkg --print-architecture`.deb && \
    apt-get update && \
    apt-get install -y ./airgorah_0.7.3_`dpkg --print-architecture`.deb && \
    rm -f airgorah_0.7.3_`dpkg --print-architecture`.deb

# Stage 3: Final Image
FROM debian:11

# Copy the installed tools from the previous stages
COPY --from=kali-base /usr/bin/aircrack-ng /usr/bin/aircrack-ng
COPY --from=kali-base /usr/bin/airmon-ng /usr/bin/airmon-ng
COPY --from=kali-base /usr/sbin/tcpdump /usr/sbin/tcpdump
COPY --from=kali-base /usr/sbin/iw /usr/sbin/iw
COPY --from=airgorah-install /usr/bin/airgorah /usr/bin/airgorah

# Set up entrypoint or default command
CMD ["bash"]