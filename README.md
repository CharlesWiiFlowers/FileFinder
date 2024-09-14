# File Finder CLI 📂🔍

This simple CLI application written in Rust can search for files with a specified name in a given directory (Or the root system if you wanna). Below are several practical use cases where this tool can come in handy!

## Use Cases 💡

### 1. 🧹 **Cleaning up duplicate files**

   You can use this tool to locate files with the same name across different folders. For example, you may have `duplicate.txt` files scattered across your workspace. Use this tool to find them all and clean up your directories by removing or consolidating them.

### 2. 💾 **Searching for specific project files**

   If you're working on a large project or repository with many subfolders, you can quickly locate configuration files like `README.md` or `Cargo.toml` without having to manually navigate through all the directories.

### 3. 🕵️‍♂️ **File auditing and tracking**

   In case of audits or compliance checks, you might want to locate all the instances of sensitive files (e.g., `config.json`, `.env`) in your system. This tool allows you to do just that, preventing possible security breaches.

### 4. 💻 **System-wide file search**

   You can use this application to search for essential files (like `logs.txt`) across the entire system, whether it's for debugging, file recovery, or general system analysis.

### 5. 📂 **Organizing media files**

   Whether it's music (`.mp3`), photos (`.jpg`, `.png`), or documents (`.pdf`), this tool can help you search and organize files based on their name. For example, quickly finding all your `family_photo_2023.jpg` files scattered across different folders.

## How to Use 🚀

### First way: **Download the Executable**

First, head over to the [File Finder repository](https://github.com/CharlesWiiFlowers/FileFinder) and download the `NameSearch.exe` file.

1. Go to [File Finder repository](https://github.com/CharlesWiiFlowers/FileFinder).
2. Download `NameSearch.exe` to your desired folder on your system.

### **Open CMD or PowerShell**

Next, open **CMD** or **PowerShell**.

1. Press `Win + R`, type `cmd` or `powershell`, and press `Enter`.
2. Navigate to the folder where you saved the `NameSearch.exe` file.  
   Example:

   ```bash
   cd C:\path\to\your\folder
   ```

### **Run the Application**

Once you're in the correct directory, you can now use the application!

- **Basic Search Command:**
   To search for a file by name, use the following command:

   ```bash
   .\NameSearch.exe --filename [file_name]
   ```

   Example:

   ```bash
   .\NameSearch.exe --filename report.txt
   ```

   This will search for `report.txt` in the current directory and all its subfolders.

- **Help Command:**
   If you need help or want to see all available options, you can run:

   ```bash
   .\NameSearch.exe --help
   ```

### **Wait for the Result**

Once the search begins, just sit tight! The application will display the full paths of any matching files it finds. If no files are found, it will notify you that no matches were located. 🕵️‍♂️

### Troubleshooting ⚠️

- If the application doesn't seem to work, make sure you're in the correct directory where `NameSearch.exe` is located.
- You can use the `--help` option for more guidance.

### Second Way: Build by yourself!! 🦈

1. **Installation**  
   To use the `search` command in this application, ensure you have Rust installed in your system. You can install Rust via [rustup](https://rustup.rs/).

2. **Compile the application**
   Navigate to your project folder and run the following:

   ```bash
   cargo build --release
   ```

3. **Run the application**
   Once compiled, you can run the application from the terminal:

   ```bash
   ./target/release/search --root <directory> <filename>
   ```

   - **directory**: The directory where you want to start the search.
   - **filename**: The name of the file you're looking for.

4. **Example Usage**

   ```bash
   ./target/release/search --root /home/user/Documents report.txt
   ```

   This will search for `report.txt` in the `Documents` folder and all its subfolders.

### Contributions & Feedback 🛠️

Feel free to fork the project or submit feedback! This is an ongoing project, and contributions are always welcome. 😄
