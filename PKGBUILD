pkgname="boj-git"
_pkgname="boj"
pkgver=0.2
pkgrel=1
pkgdesc="A Wonderful Discord Bot for a Wonderful Discord Server. Written in rust."
arch=("x86_64" "x86")
url="https://github.com/El-Wumbus/Bot-of-Justice"
license=("GPL2")
provides=("boj")
makedepends=("rust")
source=($_pkgname::"git+https://github.com/El-Wumbus/Bot-of-Justice.git")
sha256sums=("SKIP")

pkgver() {
  cd "$_pkgname"
  printf "r%s.$s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$_pkgname"
  cargo build --release
}

package() {
  cd "$_pkgname"
  install target/release/boj -Dm755 ${pkgdir}/usr/bin/boj
}