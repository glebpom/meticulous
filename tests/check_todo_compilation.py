#!/usr/bin/env python3
"""Check the public API from a consumer crate, without extra dependencies.

Run all four combinations with `python3 tests/check_todo_compilation.py`.
CI selects one combination with --profile and --features. All generated Cargo
files and build artifacts live in a temporary directory, outside the repository.
"""

import argparse
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


def check_fixture(manifest, profile, features, fixture, missing_type=None):
    command = [
        "cargo",
        "check",
        "--offline",
        "--color=never",
        "--message-format=json",
        "--manifest-path",
        str(manifest),
        "--target-dir",
        str(manifest.parent / "target"),
        "--bin",
        fixture,
    ]
    if profile == "release":
        command.append("--release")
    if features == "all":
        command.append("--all-features")
    result = subprocess.run(command, capture_output=True, text=True)
    context = f"{profile}, {features} features, {fixture}"

    if missing_type is None:
        if result.returncode != 0:
            raise AssertionError(f"Expected compilation to succeed: {context}\n"
                                 f"{result.stdout}\n{result.stderr}")
    else:
        errors = []
        for line in result.stdout.splitlines():
            diagnostic = json.loads(line)
            if diagnostic.get("reason") == "compiler-message":
                message = diagnostic["message"]
                if message["level"] == "error":
                    errors.append(message)
        expected_error = (
            len(errors) == 1
            and (errors[0].get("code") or {}).get("code") == "E0599"
            and "no method named `todo`" in errors[0]["message"]
            and f"enum `{missing_type}" in errors[0]["message"]
            and any(
                span["is_primary"]
                and Path(span["file_name"]).name == f"{fixture}.rs"
                for span in errors[0]["spans"]
            )
        )
        if result.returncode == 0 or not expected_error:
            raise AssertionError(f"Expected only E0599 for missing {missing_type}.todo: "
                                 f"{context}\n{result.stdout}\n{result.stderr}")

    outcome = "rejected missing todo as expected" if missing_type else "compiled"
    print(f"{context}: {outcome}", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", choices=("debug", "release"))
    parser.add_argument("--features", choices=("default", "all"))
    args = parser.parse_args()
    profiles = (args.profile,) if args.profile else ("debug", "release")
    feature_modes = (args.features,) if args.features else ("default", "all")
    root = Path(__file__).resolve().parent.parent

    with tempfile.TemporaryDirectory(prefix="meticulous-consumer-") as temporary:
        consumer = Path(temporary)
        shutil.copytree(root / "tests" / "fixtures", consumer / "src" / "bin")
        manifest = consumer / "Cargo.toml"
        manifest.write_text(
            '[package]\nname = "meticulous-consumer-tests"\n'
            'version = "0.0.0"\nedition = "2024"\npublish = false\n\n'
            '[dependencies]\nmeticulous = { path = '
            + json.dumps(str(root))
            + ' }\n\n[features]\n'
            'disallow-todo-on-release = ["meticulous/disallow-todo-on-release"]\n',
            encoding="utf-8",
        )
        for profile in profiles:
            for features in feature_modes:
                forbidden = profile == "release" and features == "all"
                check_fixture(manifest, profile, features, "safe_methods")
                check_fixture(manifest, profile, features, "result_todo",
                              "Result" if forbidden else None)
                check_fixture(manifest, profile, features, "option_todo",
                              "Option" if forbidden else None)


if __name__ == "__main__":
    main()
