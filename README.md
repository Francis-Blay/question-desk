# Question Desk

Question Desk is a desktop tool for pastors, ministers, and church leaders to manage questions, drafts, and summaries. It includes a Tauri desktop app and a reporting script that can export question activity to a CSV file.

## Features

- Track questions and drafts
- View question metadata and tags
- Generate a CSV report from saved question data
- Run as a desktop app on Windows, macOS, and Linux

## Requirements

Before running the project, install these tools:

- Node.js 18 or newer
- npm (usually included with Node.js)
- Rust and Cargo
- Python 3

### Install prerequisites

#### Windows

- Install Node.js from https://nodejs.org/
- Install Rust from https://rustup.rs/
- Install Python from https://www.python.org/downloads/
- Make sure `python` or `py` is available in your terminal

#### macOS

```bash
# Install Homebrew if needed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Node.js, Rust, and Python
brew install node rust python
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y nodejs npm rustc python3 python3-pip
```

## Clone and install

```bash
git clone <your-repository-url>
cd question-desk
npm install
```

If the Tauri app needs additional setup, you may also need to install the Tauri CLI:

```bash
npm install -D @tauri-apps/cli
```

## Run the app

Start the desktop app in development mode:

### Windows

```powershell
npm run tauri dev
```

### macOS / Linux

```bash
npm run tauri dev
```

This launches the Tauri application using Vite and the Rust backend.

## Anthropic API key setup

Some features of Question Desk rely on Anthropic's AI model. You will need an API key before using the drafting or AI-assisted question features.

1. Sign in or create an account at https://console.anthropic.com/
2. Create a new API key in your Anthropic dashboard.
3. Copy the key and store it in a safe environment variable or config file used by the app.
4. Restart the app after adding the key.

### Example environment variable setup

#### Windows PowerShell

```powershell
$env:ANTHROPIC_API_KEY="your-api-key-here"
```

#### macOS / Linux

```bash
export ANTHROPIC_API_KEY="your-api-key-here"
```

> If your app expects the key in a different env var name, check the app's configuration or environment-loading code before running it.

## Report script and questions data path

The report script reads a saved questions file named `questions.json`. The script is currently configured to look for it in this location:

```text
C:/Users/AKOSUA/AppData/Roaming/africa.apakan.question-desk/questions.json
```

This path is specific to the Windows desktop app installation. If you are running the app on macOS or Linux, the saved location may be different depending on the app's data directory and OS conventions.

### Check the data file location

If the report script fails with a file-not-found error, confirm that `questions.json` exists in the app's data directory and then update the `QUESTIONS_PATH` value in [scripts/report.py](scripts/report.py) to match the correct location.

Example:

```python
QUESTIONS_PATH = "C:/Users/AKOSUA/AppData/Roaming/africa.apakan.question-desk/questions.json"
```

On macOS/Linux, it may look more like:

```python
QUESTIONS_PATH = "/Users/your-user/Library/Application Support/africa.apakan.question-desk/questions.json"
```

or

```python
QUESTIONS_PATH = "/home/your-user/.config/africa.apakan.question-desk/questions.json"
```

## Generate the report

The project includes a Python report script at `scripts/report.py`.

### Windows

```powershell
python scripts/report.py
```

If `python` does not work, try:

```powershell
py scripts/report.py
```

### macOS / Linux

```bash
python3 scripts/report.py
```

The script reads question data from the app's saved data directory and writes a CSV report named using the current date, such as `09-24-2026.csv`.

## Build the app for production

```bash
npm run build
```

To build the desktop app bundle:

```bash
npm run tauri build
```

## Troubleshooting

### Python command not found

- Windows: use `py` or `python`
- macOS/Linux: use `python3`

### Tauri build errors

- Make sure Rust is installed and the toolchain is up to date:

```bash
rustup update
```

- Reinstall dependencies if needed:

```bash
rm -rf node_modules
npm install
```

### Report script fails

- Confirm the app has created `questions.json` in its app data folder.
- Confirm Python is installed and available in your terminal.
- Make sure you are running the command from the project root.

## Project structure

```text
question-desk/
├── scripts/
│   └── report.py
├── src/
├── src-tauri/
├── package.json
├── README.md
├── index.html
├── vite.config.ts
└── tsconfig.json
```

## Notes

This project is intended to be run locally on your machine. The exact location of the saved question data depends on the operating system and the app's runtime configuration.


