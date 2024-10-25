### **User Guide for Command-Line Tool by using Rust**

---

#### **1. Overview**

This Rust program is a simple command-line calculator that helps you compute exponentiation (a raised to the power of b).

---

#### **2. Installation Instructions**

**Prerequisites:**
- Rust installed on your system (use `rustup` if needed).

**Steps to install:**

1. Clone the repository:
   ```bash
   git clone <repository_url>
   ```

2. Move into the project directory:
   ```bash
   cd <project_directory>
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

This will create a binary file in `target/release/`.

---

#### **3. Running the Tool**

To run the tool, use the following command:

```bash
./target/release/<binary_name> [options]
```

**Common options**:
- `--help`: View help and available options.

Example:
```bash
./target/release/my_tool
```

---

#### **4. CI/CD Pipeline**

The project has an automated CI/CD pipeline that:
- Builds the tool
- Runs tests
- Provides the binary as an artifact

The pipeline runs automatically when you push code to the main branch.

---

#### **5. README.md**

Make sure your `README.md` includes:
- How to install the tool
- Basic usage instructions
- Contribution guidelines

---

### **Deliverables**
- **Packaged Tool**: The binary file will be in `target/release/`.
- **CI/CD**: A pipeline automatically tests and builds this rust template project.

