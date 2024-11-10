# Fast File Explorer

We all know Windows File Explorer is trash, why not fix it?

Introducing Fast File Explorer, a lighting-fast file explorer with filters to make it easy to browse through huge amount of files.

## Usage

1. Download the latest release from the [releases page](https://github.com/L1shed/FastFileExplorer/releases)
2. Run `file-explorer.exe`
3. Type your file or folder name you are searching
4. (Optional) Use filters like:
    - `@dir` for Directories only (also works for `@file`)
    - `@ext:pdf` for PDF files
    - `@size>1mb` or `@size<9kb` for specifying file size
    - `@date:2023` for files with a specific last modified date

5. Press Enter to search and scroll to navigate results.

## How it works

When executed, the executable caches all the file indexes using a Trie Data Structure. </br>
Then it Searches for the given files with filters if specified.
