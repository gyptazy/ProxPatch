# Installation
To quickly get started with ProxPatch, you can install it directly from the official Debian repository. This is the recommended method for most users as it ensures you receive updates and security patches automatically.

## Debian Repository

```
# Add the official gyptazy.com repository
curl https://git.gyptazy.com/api/packages/gyptazy/debian/repository.key -o /etc/apt/keyrings/gyptazy.asc
echo "deb [signed-by=/etc/apt/keyrings/gyptazy.asc] https://packages.gyptazy.com/api/packages/gyptazy/debian trixie main" | sudo tee -a /etc/apt/sources.list.d/gyptazy.list
apt-get update

# Install ProxPatch
apt-get install -y proxpatch
```

## Debian Packages

You can also download and install the latest Debian package directly from the gyptazy CDN:

* https://cdn.gyptazy.com/debian/proxpatch/

## Build from Source

If you prefer to compile ProxPatch from source, you can clone the Git repository and build it using Rust/Cargo:

### Install Build Dependencies
```
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  pkg-config \
  musl-tools \
  debhelper-compat

curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustup toolchain install stable
rustup complete stable
```

### Clone Repository
```
git clone https://github.com/gyptazy/ProxPatch.git
cd ProxPatch
```

### Build ProxPatch
```
cd proxpatch
cargo build --release
```

Afterwards, you can find the compiled binary at `target/release/proxpatch`. You can copy this binary to your Proxmox VE node and run it directly, or create a Debian package for easier installation.

### Create Debian Package
```
# Create Debian Package from root of repository
cd ..
dpkg-buildpackge -us -uc
```

Afterwards, you can find the generated `.deb` package in the parent directory. You can install it using `dpkg -i proxpatch_*.deb`.
