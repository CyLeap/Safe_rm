#!/bin/bash

LOG_FILE="/tmp/trash/trash.log"

if [[ $# -lt 1 ]]; then
    echo "Usage: rs <partial_filename>"
    exit 1
fi

search_term="$1"

# Read matching entries from trash.log using jq and grep fuzzy matching
mapfile -t matches < <(jq -c '. | select(.original | test("'"$search_term"'"; "i"))' "$LOG_FILE")

if [[ ${#matches[@]} -eq 0 ]]; then
    echo "No matches found for '$search_term'"
    exit 1
fi

echo "Found matches:"
declare -a original_paths
declare -a trash_paths

for i in "${!matches[@]}"; do
    original=$(echo "${matches[$i]}" | jq -r '.original')
    trash=$(echo "${matches[$i]}" | jq -r '.trash')
    original_paths+=("$original")
    trash_paths+=("$trash")
    echo "[$i] $original"
done

read -p "Enter the number of the file/folder to restore (or 'all' to restore all): " choice

if [[ "$choice" == "all" ]]; then
    for i in "${!original_paths[@]}"; do
        if [[ -e "${trash_paths[$i]}" ]]; then
            mv "${trash_paths[$i]}" "${original_paths[$i]}"
            echo "Restored: ${original_paths[$i]}"
        else
            echo "Warning: ${trash_paths[$i]} does not exist"
fi
done

elif [[ "$choice" =~ ^[0-9]+$ ]] && (( choice < ${#original_paths[@]} )); then
    if [[ -e "${trash_paths[$choice]}" ]]; then
        mv "${trash_paths[$choice]}" "${original_paths[$choice]}"
        echo "Restored: ${original_paths[$choice]}"
    else
        echo "Warning: ${trash_paths[$choice]} does not exist"
    fi
else
    echo "Invalid choice."
fi