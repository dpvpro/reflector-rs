# Maintainer: James Liu <contact@no-bull.sh>

pkgname=reflector
pkgver=0.1.0
pkgrel=1
pkgdesc='Retrieve and filter the latest Arch Linux mirror list (Rust implementation)'
arch=('x86_64')
url='https://github.com/james7132/reflector-rs'
license=('GPL-2.0-or-later')
depends=('gcc-libs')
makedepends=('cargo' 'git')
backup=('etc/xdg/reflector/reflector.conf')
source=(
  "$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz"
  'reflector.service'
  'reflector.timer'
  'reflector.conf'
)
sha256sums=('SKIP'
            'SKIP'
            'SKIP'
            'SKIP')

prepare() {
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --target "$CARCH-unknown-linux-gnu"
}

check() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --target "$CARCH-unknown-linux-gnu"
}

package() {
  cd "$pkgname-$pkgver"

  # Install binary
  install -Dm755 "target/$CARCH-unknown-linux-gnu/release/$pkgname" \
    "$pkgdir/usr/bin/$pkgname"

  # Install systemd service and timer
  install -Dm644 "$srcdir/reflector.service" \
    "$pkgdir/usr/lib/systemd/system/reflector.service"
  install -Dm644 "$srcdir/reflector.timer" \
    "$pkgdir/usr/lib/systemd/system/reflector.timer"

  # Install default configuration
  install -Dm644 "$srcdir/reflector.conf" \
    "$pkgdir/etc/xdg/reflector/reflector.conf"

  # Install man page (if you create one)
  # install -Dm644 reflector.1 "$pkgdir/usr/share/man/man1/reflector.1"

  # Install license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
