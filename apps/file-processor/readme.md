# File Processor

File Processor is a versatile command-line tool built in Rust, designed for performing a multitude of operations on files. Whether you need to search and replace text, convert file formats, or simply summarize file contents, File Processor has got you covered.

## Features

### Search and Replace
- Search for specific strings or regular expression patterns within files.
- Replace found occurrences with a specified replacement string.
- Option to apply changes across multiple files within a directory.

### File Format Conversion
- Convert files between various formats (e.g., CSV to JSON, Markdown to HTML).
- Customizable options for specific formats (e.g., CSV delimiter selection).

### Content Summarization
- Generate summaries for text files, including word and character counts.
- Summarize the structure of structured files (CSV, JSON) like the number of records or keys.

### Batch Processing
- Process multiple files simultaneously with the same operation.
- Support for recursive directory traversal and file extension filtering.

### Interactive Mode
- Preview changes or conversion outcomes before applying.
- Accept or reject modifications individually or collectively.

### Logging and Undo Operations
- Maintain logs of all modifications, including original and changed states.
- Undo feature to revert files back to their original state based on logs.

## Installation

To use File Processor, clone this repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/file-processor.git
cd file-processor
cargo build --release
```

The executable will be located in ./target/release/.

## Usage

A quick overview of how to use File Processor. For detailed information, run `file-processor --help`.

### Search and Replace

```bash
file-processor search-replace --file path/to/file.txt --search "old text" --replace "new text"
```

### Convert File Format

```bash
file-processor convert --from csv --to json --file path/to/file.csv
```

### Summarize File Content

```bash
file-processor summarize --file path/to/file.txt
```


