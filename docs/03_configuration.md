# Configuration
ProxPatch is designed to work out of the box with minimal setup. In most environments, **no configuration is required**.

- ProxPatch only needs to be installed on **one** Proxmox VE node per cluster.
- From that node, it automatically discovers all other nodes in the cluster and orchestrates patching, migrations, and reboots remotely.

By default, ProxPatch uses the authentication and SSH trust that already exists in a standard Proxmox cluster setup:

- Uses the cluster’s existing SSH keys and trust relationships  
- Connects to other nodes as **`root`**  
- Executes:
  - package updates  
  - system upgrades  
  - reboots  
- Automatically gathers all nodes in the cluster  
- No configuration file is required for standard Proxmox installations  

If your cluster was created normally, you can run ProxPatch immediately after installing it.

You can optionally create a configuration file to use a different SSH user instead of `root`:

```yaml /etc/proxpatch/proxpatch.yaml
ssh_user: proxpatch
deactivate_proxlb: true
```

If ssh_user is defined:
* ProxPatch will SSH into other nodes using that user
* All commands will be executed via sudo

The user must have:
* Passwordless sudo privileges
* Be able to SSH to all cluster node

Example suodoers entry for a custom user named `proxpatch`:

```
# Allow proxpatch to run required Proxmox patch commands without password

User_Alias PROXPATCH = proxpatch

Cmnd_Alias PROXPATCH_CMDS = \
    /usr/bin/pvesh create *, \
    /usr/bin/apt-get update, \
    /usr/bin/apt-get dist-upgrade, \
    /usr/bin/apt-get -y dist-upgrade, \
    /usr/sbin/reboot, \
    /sbin/reboot

PROXPATCH ALL=(root) NOPASSWD: PROXPATCH_CMDS
```

## CLI Options
| Setting      | Default  | Required | Description                      |
| ------------ | -------- | -------- | -------------------------------- |
| -d  | None     | No       | Run in debug mode      |
| -c    | None     | No       | Define custom configuration file path    |


## Enable Debug Mode
To enable debug mode, run ProxPatch with the `-d` flag. This will provide more detailed output for troubleshooting and monitoring the patching process. When using systemd, you need to adjust the unit file `/var/lib/systemd/system/proxpatch.service` to:

```
[Service]
ExecStart=/usr/bin/proxpatch -d
```

