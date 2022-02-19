FROM frolvlad/alpine-glibc:alpine-3.8

RUN apk update && \
    apk add \
    --upgrade \
    bash \
    ca-certificates \
    gcc \
    python3 \
    wget \
    xdg-utils \
    xvfb \
    xz

# Unfortunately it seems like using ebook-convert from calibre 4+ doesn't work due to some problem with glibc++ when converting PDF:s, so we  manually install an older version
RUN wget --no-check-certificate -nv https://download.calibre-ebook.com/3.48.0/calibre-3.48.0-x86_64.txz -O calibre-tarball.txz && \
    mkdir -p /opt/calibre && \
    rm -rf /opt/calibre/* && \
    tar xvf /calibre-tarball.txz -C /opt/calibre && \
    /opt/calibre/calibre_postinstall

COPY target/x86_64-unknown-linux-gnu/debug/kindlegen-as-a-service .
CMD ["./kindlegen-as-a-service"]
