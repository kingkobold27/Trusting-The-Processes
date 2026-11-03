# Trusting-The-Processes

### cave_crawler — A Linux Process & Thread Monitor Written in Rust

**Trusting-The-Processes** is a lightweight Linux process-monitoring project developed in Rust. The application, `cave_crawler`, monitors running processes and their threads, detecting creation and termination events through periodic snapshots of the Linux `/proc` filesystem.

The project was developed to explore Linux process management, operating system internals, Rust programming, and fundamental concepts of host-based system monitoring.

## Features

- **Process Creation Detection:** Identifies new processes by comparing consecutive snapshots of active process IDs (PIDs).
- **Process Termination Detection:** Identifies processes that have disappeared between snapshots.
- **Process Metadata Collection:** Retrieves process names and parent process IDs (PPIDs).
- **Thread Monitoring:** Detects new and terminated threads within monitored processes using thread IDs (TIDs).
- **Process Information Caching:** Stores process metadata and thread snapshots to preserve information about processes after they terminate.
- **Continuous Monitoring:** Scans the system every two seconds until the user requests termination.
- **Session Summary:** Displays process creation and termination events collected during the monitoring session.

## Technologies Used

| Technology | Purpose |
|---|---|
| Rust | Primary programming language |
| Linux `/proc` | Source of process and thread information |
| HashSet | Stores and compares PID and TID snapshots |
| HashMap | Caches process metadata and thread information |
| crossbeam-channel | Communicates shutdown requests between threads |
| Cargo | Rust package management and build system |

## How It Works

### 1. Process Discovery

The application reads the `/proc` directory, identifying numeric directory names that correspond to active process IDs.

Each process ID is stored in a `HashSet<u32>`.

### 2. Process Metadata Collection

For each discovered process, `cave_crawler` attempts to retrieve:

- **PID:** Unique process identifier.
- **Process Name:** Retrieved from `/proc/<PID>/comm`.
- **PPID:** Parent process identifier retrieved from `/proc/<PID>/status`.
- **Thread IDs:** Retrieved from `/proc/<PID>/task`.

The information is cached in a `HashMap` for future comparisons.

### 3. Snapshot Comparison

Every two seconds, the application collects a new snapshot of running processes.

Rust's `HashSet::difference()` method identifies changes between snapshots.

**New processes:**

`Current PIDs - Previous PIDs`

**Terminated processes:**

`Previous PIDs - Current PIDs`

Newly discovered processes are added to the metadata cache, while terminated processes are removed after their cached information has been retrieved.

### 4. Thread Monitoring

For processes present in the current snapshot, the application compares previously recorded thread IDs against newly collected thread IDs.

This allows it to identify:

- Newly created threads.
- Threads that have terminated.

Thread information is updated after each successful, nonempty snapshot.

### 5. Graceful Shutdown

The application uses a separate Rust thread to listen for user input.

When the user enters `q`, a shutdown signal is sent through a `crossbeam-channel` to the monitoring loop.

The application then exits monitoring and displays a summary of the process events detected during the session.

### 6. Log Operation

The application will send the logged name, pids, and ppids to log.txt following shutdown.

- WARNING: This will override each session!

## Installation

### Requirements

- Linux operating system with the `/proc` filesystem mounted
- Rust and Cargo installed
- Access to the `/proc` filesystem

### Clone the Repository

```bash
git clone https://github.com/kingkobold27/Trusting-The-Processes.git
cd Trusting-The-Processes
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

Alternatively, run the compiled binary directly:

```bash
./target/release/cave_crawler
```

## Example Output

The following is illustrative output showing the format of events reported by the monitor.

```text
Press 'q' to end.

Snapshot A Processes Found: 288
Snapshot B Processes Found: 289

New processes (pids): 4521
Process name: firefox
Parent process id: 1382
Thread IDs: 4521
Thread IDs: 4522

New Threads detected: PID 4521 | TID 4523

Thread Terminated: PID 4521 | TID 4522

Snapshot A Processes Found: 289
Snapshot B Processes Found: 288

Missing processes (pids): 4521
Process Name: firefox
Parent process id: 1382
Thread IDs: 4521
Thread IDs: 4523
```

When monitoring ends, the application prints the process creation and termination records accumulated during the session.

## Limitations

`cave_crawler` is an educational monitoring tool rather than a comprehensive endpoint detection and response (EDR) system.

Current limitations include:

- **Polling Interval:** The application scans every two seconds. Processes or threads that start and terminate between scans may not be detected.
- **Linux Only:** The application relies on the Linux `/proc` filesystem and is not compatible with Windows or macOS.
- **Filesystem Access:** Process information may be unavailable because of permissions or process termination during a scan.
- **Thread Snapshot Errors:** Failed thread-directory reads return empty sets, so thread observations may be incomplete.
- **PID Reuse:** Rapid reuse of process IDs may cause separate process lifetimes to be treated as one.
- **No Resource Monitoring:** CPU usage, memory consumption, and network activity are not currently tracked.

## Potential Future Improvements

- Add timestamps to process and thread events.
- Improve error handling for inaccessible or changing `/proc` entries.
- Track CPU and memory usage per process.
- Support configurable monitoring intervals.
- Introduce optional process filtering.
- Improve detection of process identity changes and PID reuse.

## Project Goals

This project was created to strengthen practical understanding of:

- Rust ownership, borrowing, and collections.
- Linux process and thread management.
- Reading and interpreting the `/proc` filesystem.
- Multithreading and inter-thread communication.
- State tracking and snapshot comparison.
- Foundational host-based monitoring techniques.

## Disclaimer

This software is intended for educational purposes and system monitoring on Linux systems where the user is authorized to inspect process information.

It is not designed to replace professional security monitoring or incident response solutions.
