# Safe_rm Script

## Overview
This project contains a custom shell script named `rm` that acts as a safer alternative to the standard `rm` command. Instead of permanently deleting files or directories, it moves them to a trash folder (`/tmp/trash`) and logs the deletion details.

## Script Functionality

- **Trash Directory:** Files and directories are moved to `/tmp/trash` instead of being permanently deleted.
- **Logging:** Each deletion is logged in a JSON format in `/tmp/trash/trash.log` with the original path, trash path, and timestamp.
- **Recursive Deletion:** Supports a `-r` flag to allow recursive deletion of directories.
- **Error Handling:** 
  - Unsupported flags produce an error message.
  - Attempting to delete directories without `-r` produces an error.
  - Missing files produce an error message.
- **Usage:** 
  ```
  rm [-r] file1 [file2 ...]
  ```

## Script Details

- The script creates the trash directory and log file if they do not exist.
- It parses command-line arguments to detect the recursive flag and target files/directories.
- For each target:
  - Checks if it exists.
  - If a directory and recursive flag is not set, it errors out.
  - Moves the file/directory to the trash folder with a timestamp appended to avoid name clashes.
  - Logs the deletion in JSON format.
  - Prints a confirmation message.

## Known Issue and Fix

- The script file may contain Windows-style line endings (`CRLF`), which cause syntax errors when run in a Unix-like shell.
- To fix this, convert the file to Unix-style line endings (`LF`) using a tool like `dos2unix` or the following command:
  ```
  sed -i 's/\r$//' rm
  ```

## Testing

- Critical areas to test:
  - File and directory deletion behavior.
  - Recursive deletion with `-r` flag.
  - Proper logging of deletions.
  - Error handling for unsupported flags and missing files.
- Testing can be done by running the script with various files and directories and verifying the trash folder and log file.

## Usage Example

```bash
# Move a file to trash
bash rm file.txt

# Recursively move a directory to trash
bash rm -r myfolder
```

## Notes

- This script is intended as a safer alternative to `rm` by preventing permanent deletion.
- The trash folder is located in `/tmp/trash`, which may be cleared on system reboot depending on your OS.