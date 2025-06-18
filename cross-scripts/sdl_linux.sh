dpkg --add-architecture $CROSS_DEB_ARCH
apt-get update
apt-get install --assume-yes libjpeg-dev:$CROSS_DEB_ARCH libwebp-dev:$CROSS_DEB_ARCH libtiff5-dev:$CROSS_DEB_ARCH libsdl2-image-dev:$CROSS_DEB_ARCH libsdl2-image-2.0-0:$CROSS_DEB_ARCH libsdl2-dev:$CROSS_DEB_ARCH libsdl2-2.0-0:$CROSS_DEB_ARCH
