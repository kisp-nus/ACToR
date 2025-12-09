#!/usr/bin/env python3
import sys
import json
import argparse
from typing import Any, Dict, List


def colorize(s: str, role_key: str, use_color: bool) -> str:
    if not use_color:
        return s
    # Foreground bright white on role-colored background
    bg_colors = {
        "system": "46",       # cyan background
        "assistant": "42",    # green background
        "tool": "43",         # yellow background
        "tool_result": "45",  # magenta background
        "user": "44",         # blue background
        "result": "104",      # bright blue background
        "error": "41",        # red background
        "default": "47",      # white/grey background
    }
    bg = bg_colors.get(role_key, bg_colors["default"])
    return f"\033[1;97;{bg}m{s}\033[0m"


def fmt_label(label: str, role: str, use_color: bool) -> str:
    # Apply reverse-style coloring: white text on colored background per role
    return colorize(label, role, use_color)


def print_code_block(text: str, use_color: bool) -> None:
    # Use 4-backtick fences at column 0 so inner content can contain ``` safely
    fence = "````"
    print(fence)
    # Keep content raw without indentation
    print(text.rstrip("\n"))
    print(fence)


def pretty_tool_input(obj: Any) -> str:
    try:
        return json.dumps(obj, indent=2, ensure_ascii=False)
    except Exception:
        return str(obj)


def handle_system(obj: Dict[str, Any], use_color: bool) -> None:
    subtype = obj.get("subtype")
    title = f"[{subtype}]" if subtype else ""
    header("system", title, use_color)
    # Show a couple of key fields if present
    details = []
    if "cwd" in obj:
        details.append(f"  cwd: {obj['cwd']}")
    if "model" in obj:
        details.append(f"  model: {obj['model']}")
    if obj.get("tools"):
        tools = obj.get("tools")
        if isinstance(tools, list):
            details.append(f"  tools: {', '.join(map(str, tools))}")
    if details:
        for line in details:
            print(line)


def header(role_key: str, title: str = "", use_color: bool = False) -> None:
    # role_key in {system, assistant, tool, tool_result, user, result, error, default}
    role_display = {
        "system": "System",
        "assistant": "Assistant",
        "tool": "Tool",
        "tool_result": "Tool",
        "user": "User",
        "result": "Result",
        "error": "Error",
        "default": "Entry",
    }.get(role_key, "Entry")

    # Unified header format in both modes, with preceding blank line
    base = f"### =====[{role_display}]" + (f" {title}" if title else "") + "====="
    print()
    print(fmt_label(base, role_key, use_color))


def handle_assistant(obj: Dict[str, Any], use_color: bool, tool_id_to_name: Dict[str, str]) -> None:
    msg = obj.get("message", {})
    contents: List[Dict[str, Any]] = msg.get("content", []) or []

    # Collect text parts first
    text_parts: List[str] = []
    for item in contents:
        if not isinstance(item, dict):
            continue
        if item.get("type") == "text":
            text_parts.append(item.get("text", ""))

    if text_parts:
        body = "\n".join([p for p in text_parts if p]).strip()
        if body:
            # Always use a header; print multi-line body in a fenced block
            header("assistant", "" if "\n" in body else body, use_color)
            if "\n" in body:
                print_code_block(body, use_color)

    # Print tool uses
    for item in contents:
        if not isinstance(item, dict):
            continue
        if item.get("type") == "tool_use":
            tool_id = item.get("id")
            tool_name = item.get("name") or "<tool>"
            if tool_id:
                tool_id_to_name[tool_id] = tool_name
            header("tool", f"uses {tool_name}", use_color)
            tool_in = pretty_tool_input(item.get("input"))
            print_code_block(tool_in, use_color)


def handle_user(obj: Dict[str, Any], use_color: bool, tool_id_to_name: Dict[str, str]) -> None:
    msg = obj.get("message", {})
    contents: List[Dict[str, Any]] = msg.get("content", []) or []

    # Print any plain text content if present
    for item in contents:
        if item.get("type") == "text":
            text = item.get("text", "").strip()
            if text:
                header("user", text, use_color)

    # Handle tool results
    for item in contents:
        if item.get("type") == "tool_result":
            tool_use_id = item.get("tool_use_id")
            is_error = bool(item.get("is_error"))
            name = tool_id_to_name.get(tool_use_id) if tool_use_id else None
            base = (f"result from {name}" if name else "result").strip()
            role = "error" if is_error else "tool_result"
            header(role, base, use_color)
            content = item.get("content", "")
            print_code_block(str(content), use_color)


def main() -> None:
    parser = argparse.ArgumentParser(description="Convert Claude-like JSONL console output to readable text/markdown, line-by-line.")
    parser.add_argument("--color", "-c", action="store_true", help="Enable ANSI-colored output for terminals")
    args = parser.parse_args()

    use_color = bool(args.color)

    # Map tool_use id -> name for linking results
    tool_id_to_name: Dict[str, str] = {}

    # Ensure line-buffered stdout for streaming usage
    try:
        sys.stdout.reconfigure(line_buffering=True)
    except Exception:
        pass

    for raw in sys.stdin:
        if not raw:
            continue
        line = raw.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except Exception:
            # Not JSON; print as-is
            print(line)
            continue

        typ = obj.get("type")
        if typ == "system":
            handle_system(obj, use_color)
        elif typ == "assistant":
            handle_assistant(obj, use_color, tool_id_to_name)
        elif typ == "user":
            handle_user(obj, use_color, tool_id_to_name)
        elif typ == "result":
            # Summarize final result objects if present
            subtype = obj.get("subtype")
            is_error = bool(obj.get("is_error"))
            status = subtype or ("error" if is_error else "ok")
            header("result" if not is_error else "error", status, use_color)
            # Show concise metadata if available
            meta_bits = []
            if "duration_ms" in obj:
                meta_bits.append(f"duration_ms={obj['duration_ms']}")
            if "num_turns" in obj:
                meta_bits.append(f"num_turns={obj['num_turns']}")
            if meta_bits:
                print(", ".join(meta_bits))
            # Show human-readable result content if present
            if isinstance(obj.get("result"), str):
                print_code_block(obj.get("result"), use_color)
            else:
                # Fallback to structured dump
                print_code_block(pretty_tool_input(obj), use_color)
        else:
            # Fallback for unknown entries
            header("default", typ or "entry", use_color)
            print_code_block(pretty_tool_input(obj), use_color)

        # Flush promptly for streaming
        try:
            sys.stdout.flush()
        except Exception:
            pass


if __name__ == "__main__":
    main()
