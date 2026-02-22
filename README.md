# ProxPatch - An automated rolling patch manager for Proxmox clusters
<img align="left" src="https://cdn.gyptazy.com/img/ProxPatch_github.jpg"/>

<br clear="left">

<p float="center">
  <img src="https://img.shields.io/github/license/gyptazy/ProxPatch"/>
  <img src="https://img.shields.io/github/contributors/gyptazy/ProxPatch"/>
  <img src="https://img.shields.io/github/last-commit/gyptazy/ProxPatch/main"/>
  <img src="https://img.shields.io/github/issues-raw/gyptazy/ProxPatch"/>
  <img src="https://img.shields.io/github/issues-pr/gyptazy/ProxPatch"/>
</p>

# Table of Contents

- [Overview](#overview)
- [Requirements](#requirements)
- [Installation](#installation)
  - [Debian Repository](#debian-repository)
  - [Debian Packages](#debian-packages)
- [Configuration](#configuration)
- [Usage](#usage)
- [Community & Support](#community--support)
- [Author](#author)
- [Disclaimer](#disclaimer)

---

**ProxPatch** (written by [gyptazy](https://gyptazy.com/proxpatch/)) is a lightweight, automation-first patch orchestration tool for Proxmox VE clusters. It performs **rolling security updates** across nodes, safely migrates running VMs, reboots when required, and keeps cluster downtime to a minimum.

The design goal is simple:
> [!TIP]
> Patch every node automatically without disrupting running workloads.

## Overview
ProxPatch is a rolling patch orchestration tool for Proxmox VE clusters that automates one of the most repetitive and risk-prone operational tasks: keeping cluster nodes updated without interrupting running workloads.

Instead of manually draining nodes, migrating VMs, applying updates, and rebooting one host at a time, ProxPatch coordinates this process automatically. It inspects the cluster state, upgrades nodes via SSH, determines whether a reboot is required, migrates running guests away from affected nodes, and performs controlled reboots while keeping the cluster operational.

The tool is intentionally minimal and transparent. It does not rely on external orchestration frameworks, databases, or API tokens. By using native Proxmox tooling (`pvesh`, `qm`, SSH) and a clear execution flow, ProxPatch remains easy to audit, predictable in behavior, and suitable for both homelabs and production environments. The initial idea was to implement this into my already exisiting [ProxLB](https://github.com/gyptazy/ProxLB) project which handles DRS alike load balancing of VMs across Proxmox clusters. However, missing API endpoints for patching and rebooting nodes in a rolling fashion made it necessary to implement this as a separate tool (or to always patch the Proxmox API which could lead into additional issues in long-term).

At its core, ProxPatch follows a simple philosophy:

- Prefer safety over speed  
- Avoid unnecessary downtime  
- Keep the cluster running at all times  
- Make automation observable and debuggable  
- Stay lightweight and dependency-free  

ProxPatch is not a full lifecycle manager or HA replacement. Instead, it focuses on one job and does it well: **unattended and fully automated rolling patching of Proxmox nodes with minimal service disruption**.

## Requirements 
ProxPatch is designed to run inside or alongside a Proxmox VE cluster and relies only on native tooling already present on most installations.

* Proxmox VE cluster (tested on 8.x and 9.x)
    * Minimum of three Nodes
    * Cluster must maintain quorum during patching
    * Shared storage (e.g. Ceph, NFS) for live migration
* SSH access to all cluster nodes (passwordless key-based authentication recommended)
* `jq` on the machine running ProxPatch for JSON parsing

## Installation
To quickly get started with ProxPatch, you can install it directly from the official Debian repository. This is the recommended method for most users as it ensures you receive updates and security patches automatically.

> [!CAUTION]
> ProxPatch must run on exactly one node per cluster.
Do not enable or start the proxpatch service on multiple nodes simultaneously.

### Debian Repository

```
# Add the official gyptazy.com repository
curl https://git.gyptazy.com/api/packages/gyptazy/debian/repository.key -o /etc/apt/keyrings/gyptazy.asc
echo "deb [signed-by=/etc/apt/keyrings/gyptazy.asc] https://packages.gyptazy.com/api/packages/gyptazy/debian trixie main" | sudo tee -a /etc/apt/sources.list.d/gyptazy.list
apt-get update

# Install ProxPatch
apt-get install -y proxpatch
```

### Debian Packages

You can also download and install the latest Debian package directly from the gyptazy CDN:

* https://cdn.gyptazy.com/debian/proxpatch/

## Configuration
ProxPatch is designed to work out of the box with minimal setup. In most environments, **no configuration is required**.

However, if you need to customize ProxPatch's behavior, you can create a configuration file at `/etc/proxpatch/config.yaml`. This file allows you to adjust several settings. Please see the [configuration documentation](https://github.com/gyptazy/ProxPatch/tree/main/docs) for detailed information on available options and their effects.

## Usage
ProxPatch is designed to run fully automated rolling updates across your Proxmox VE cluster. To begin the rolling upgrade process, simply enable and start the provided systemd unit:

```bash
systemctl enable proxpatch
systemctl start proxpatch
```

## Community & Support

Have questions, ideas, or need help with ProxPatch?  
There are multiple ways to get support and connect with the community.

[Join the ProxPatch Discord server](https://discord.gg/p9UxdMnx) for real-time discussions, help, and exchange with other users.

If you found a bug, want to request a feature, or suggest an improvement, please [create](https://github.com/gyptazy/ProxPatch/issues) an issue on GitHub. Your feedback is invaluable in making ProxPatch better for everyone!

## Author
* [Florian Paul Azim Hoberg (gyptazy)](https://gyptazy.com/proxpatch/)
* [proxpatch.de](https://proxpatch.de)

## Disclaimer
This software is provided “as is”, without warranty of any kind. Use it at your own risk. The authors and contributors are not liable for any damages resulting from its use.
