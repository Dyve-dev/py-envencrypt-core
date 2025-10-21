#!/usr/bin/env python3
import subprocess
import json
import os
import sys
from typing import Any, Union
import re
import shutil


def main():
    # Get the directory where this script is located
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    try:
        # Run cargo build command in the script's directory
        result = subprocess.run(
            ["cargo", "build", "-r", "--message-format=json"],
            cwd=script_dir,
            capture_output=True,
            text=True,
            check=True
        )
        
        # Parse JSON output line by line (cargo outputs one JSON object per line)
        json_objects: list[dict[str, Any]] = []
        for line in result.stdout.strip().split('\n'):
            if line.strip():
                try:
                    json_obj = json.loads(line)
                    json_objects.append(json_obj)
                except json.JSONDecodeError:
                    continue
        
        # Print parsed JSON for analysis
        # print(json.dumps(json_objects, indent=2))
        # Filter for cdylib target with name 'dpapi'
        dpapi_lib = None
        for json_obj in json_objects:
            if (json_obj.get('target', {}).get('kind') == ['cdylib'] and 
                json_obj.get('target', {}).get('name') == 'dpapi'):
                dpapi_lib = json_obj
                break
        if not dpapi_lib:
            print("No cdylib target named 'dpapi' found in cargo output", file=sys.stderr)
            sys.exit(1)
        
        # Extract the .dll file path

        dll_path: Union[str, None] = None
        for filename in dpapi_lib.get('filenames', []):
            if re.search(r'.*dpapi\.dll$', filename):
                dll_path = filename
            break
        
        if not dll_path:
            print("No dpapi.dll found in build artifacts", file=sys.stderr)
            sys.exit(1)
        
        # Create target directory if it doesn't exist
        target_dir = os.path.join(script_dir, "python", "envencrypt_core", "dpapi")
        os.makedirs(target_dir, exist_ok=True)
        
        # Copy dll to .pyd file
        target_path = os.path.join(target_dir, "dpapi.pyd")
        shutil.copy2(dll_path, target_path)
        print(f"Copied {dll_path} to {target_path}")
        
    except subprocess.CalledProcessError as e:
        print(f"Error running cargo build: {e}", file=sys.stderr)
        print(f"stderr: {e.stderr}", file=sys.stderr)
        sys.exit(1)
    except FileNotFoundError:
        print("Error: cargo command not found. Make sure Rust is installed.", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()