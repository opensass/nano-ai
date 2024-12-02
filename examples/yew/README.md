# 🤖 **Nano AI Yew Component Example**

## ⚙️ **Pre-requisites**

Before running the project, make sure you have the necessary tools installed on your system.

### 🐧 **Linux Users**

1. **Install [`rustup`](https://www.rust-lang.org/tools/install)**:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **Install [`trunk`](https://trunkrs.dev/)**:

   ```sh
   cargo install --locked trunk
   ```

1. **Add the Wasm target**:

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

### 🪟 **Windows Users**

1. **Download and install `rustup`**: Follow the installation instructions [here](https://www.rust-lang.org/tools/install).

1. **Install [Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)**: Open PowerShell as administrator and run:

   ```sh
   wsl --install
   ```

1. **Reset Network Stack**: In PowerShell (administrator mode), run:

   ```sh
   netsh int ip reset all
   netsh winsock reset
   ```

1. **Install Linux packages in WSL**: Once inside your WSL terminal, update and install required dependencies:

   ```sh
   sudo apt update
   sudo apt install build-essential pkg-config libudev-dev
   ```

1. **Install `trunk`**:

   ```sh
   cargo install --locked trunk
   ```

1. **Add the Wasm target**:
s
   ```sh
   rustup target add wasm32-unknown-unknown
   ```

## 🏗️ **Building and Running**

1. **Clone the repository**:

   ```sh
   git clone https://github.com/opensass/nano-ai
   ```

1. **Navigate to the application directory**:

   ```sh
   cd nano-ai/examples/yew
   ```

1. **Run the application**:

   ```sh
   trunk serve --port 3000
   ```

1. **Open your browser**: Visit [http://localhost:3000](http://localhost:3000) to explore the landing page!
