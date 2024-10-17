# CMD DC-1 Rotary to Fader

## Features
- Listens to rotary input and maps it to fader level changes.

## Requirements
- **Operating System**: Windows, macOS, Linux, and other Unix Systems
- **Virtual MIDI Port**: Requires [**LoopBe1**](https://www.nerds.de/en/loopbe1.html) to run the program on Windows. Unix systems use built-in virtual MIDI ports.
- **Rust**: (Optional) Install [Rust](https://www.rust-lang.org/) for building from source.

## Installation

### Using Pre-Compiled Binaries
If you prefer not to build the project from source, you can download pre-compiled binaries from the [Releases](https://github.com/stygigoth/cmddc1rtf/releases) page.

1. Download the appropriate binary for your operating system.
2. Extract the files to a directory of your choice.
3. (Windows only) Install **LoopBe1** from [here](https://www.nerds.de/en/download.html) (a restart may be required after installation, as it installs a kernel-level driver).
4. Run the executable directly:
   - On Windows: `Windows.cmddc1rtf.exe`
   - On Linix: `./Linux.cmddc1rtf`

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/stygigoth/cmddc1rtf.git
   cd cmddc1rtf
   ```
2. Build the project using Cargo
   ```bash
   cargo build --release
   ```

## Usage
1. Run the system-appropriate executable:
   - On Windows: `Windows.cmddc1rtf.exe` (Ensure **LoopBe1** is installed)
   - On Linux: `./Linux.cmddc1rtf`
2. Select the MIDI input port for the CMD DC-1 controller. If on Windows, also select your **LoopBe1** port as an output.

## Contributing

Contributions are welcome! Fork the project, make your changes, and open a pull request.

