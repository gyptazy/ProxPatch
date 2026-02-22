# Architecture

## Flow Chart
```mermaid
flowchart TD

A[Start ProxPatch Execution]

%% --- Phase 1 ---
A --> B[Cluster State Assessment]

B --> B1[Query all cluster nodes]
B1 --> B2[Collect node resource metrics]
B2 --> B3[Inventory running VMs + memory usage]
B3 --> B4[Verify cluster quorum]
B4 --> B5[Select safe patching candidates]

%% --- Phase 2 ---
B5 --> C[Rolling Patch Execution]

C --> D{More nodes to process?}

D -->|Yes| E[Select next node]

E --> F[Check for updates via SSH]
F --> G[Apply updates if available]
G --> H[Check reboot-required flag]

H -->|No reboot needed| I[Mark node healthy]
I --> D

H -->|Reboot required| J[Evacuate node]

J --> J1[Live-migrate running VMs]
J1 --> J2[Verify node has no running VMs]
J2 --> K[Perform controlled reboot]

K --> L[Wait for node to return]
L --> M[Verify node rejoined cluster]
M --> N[Confirm node fully operational]
N --> D

D -->|No nodes left| O[Patch cycle complete]

%% --- Safety ---
subgraph "Safety Guarantees"
S1[Maintain cluster quorum]
S2[No VM downtime via live migration]
S3[Validate node health before continuing]
S4[Abort if cluster stability compromised]
end

B4 --> S1
J1 --> S2
M --> S3
C --> S4
```

## Execution Flow
ProxPatch follows a systematic approach to ensure safe and efficient cluster patching:

1. **Cluster State Assessment**
    - Queries all nodes in the cluster and gathers current resource metrics
    - Inventories running VMs and their memory requirements on each node
    - Verifies cluster quorum and identifies safe patching candidates

2. **Rolling Patch Execution**
    - Processes nodes sequentially to maintain cluster stability
    - For each node:
      - Checks for available security and system updates via SSH
      - Applies updates if available
      - Detects if a node reboot is required post-patching
      - If reboot needed:
         - Live-migrates all running VMs to other cluster nodes
         - Performs controlled node reboot
         - Monitors node recovery and cluster rejoin
      - Confirms node is fully operational before proceeding to next node

3. **Safety Guarantees**
    - Maintains cluster quorum throughout the patching cycle
    - Ensures no workload interruption through pre-reboot VM migration
    - Validates node health before processing subsequent nodes
    - Aborts patching if cluster stability is compromised