## Web Markdown

### Instructions for Running the Project

To set up and run the project, follow these steps:

1. **Install Rust and Crate:**

   - Ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Rust is required for building and running the project.
   - Install the necessary crates using `cargo` (the Rust package manager) as specified in your `Cargo.toml` file.

2. **Install Node.js and npm:**

   - Install [Node.js](https://nodejs.org/) and npm (Node Package Manager) to manage JavaScript dependencies.

3. **Run the following commands:**

   ```sh
   # Install pnpm globally
   npm install -g pnpm

   # Install project dependencies using pnpm
   pnpm install

   # Build the WebAssembly package
   pnpm run wasm-build

   # Start the development server
   pnpm run dev
   ```

### TODO

Here are the key areas that need further development and improvement:

- **Enhance Editor Styles and Functionality.**
- **Optimize Diff Algorithm (Node Comparison).**
- **Support for Multiple Large Models.**
- **Integrate [WebGPU](https://www.w3.org/TR/webgpu/) for improved performance in graphics and compute ([Example](https://github.com/cryscan/web-rwkv)).**
- **Deploy the Application Online.**
