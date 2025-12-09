#!/usr/bin/env python3

import os
import sys
from datetime import datetime
from pathlib import Path

from textual import on
# Workaround for certain Textual versions where TextArea's sub-component
# classes (e.g., 'text-area--gutter') may not be registered unless the
# private module is imported. This prevents KeyError on teardown.
import textual.widgets._text_area  # noqa: F401
from textual.app import App, ComposeResult
from textual.containers import Container, Horizontal, Vertical, VerticalScroll
from textual.screen import Screen
from textual.widgets import Button, DataTable, Footer, Header, Input, Label, Static, TabbedContent, TabPane, TextArea
from textual.reactive import reactive
from textual.timer import Timer
from typing import Optional, List, Tuple

# Import functions from lproc.py
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lproc import get_lprocs_data, get_lproc_file_path


class DetailView(Vertical):
    """Detail view widget embedded in a TabPane (no global IDs)."""

    def __init__(self, lproc_name: str, lproc_info: dict, is_active: bool = True) -> None:
        super().__init__()
        self.lproc_name = lproc_name
        self.lproc_info = lproc_info
        self._is_active = is_active
        self.refresh_timer: Optional[Timer] = None
        # Widget refs
        self.stdout_area: Optional[TextArea] = None
        self.stderr_area: Optional[TextArea] = None
        self.stdin_area: Optional[TextArea] = None
        self.stdin_input: Optional[Input] = None
        self.send_button: Optional[Button] = None
        self.cmd_label: Optional[Label] = None
        self._full_cmd_text: str = ""
        # Converter selections per stream (default: raw passthrough)
        self._converters = {"stdout": "un", "stderr": "un", "stdin": "un"}
        # Previous contents
        self._prev_stdout: Optional[str] = None
        self._prev_stderr: Optional[str] = None
        self._prev_stdin: Optional[str] = None

    def compose(self) -> ComposeResult:
        # Header row: title at left, inner command at right (ellipsized)
        with Horizontal(classes="detail-header-row"):
            yield Label(f"LProc Details: {self.lproc_name}", classes="detail-title")
            cmd_text = (
                (self.lproc_info.get('inner_bash_c') if isinstance(self.lproc_info, dict) else None)
                or (self.lproc_info.get('bash_c') if isinstance(self.lproc_info, dict) else None)
                or ""
            )
            self._full_cmd_text = cmd_text or ""
            self.cmd_label = Label(self._full_cmd_text, classes="detail-cmd")
            yield self.cmd_label

        with TabbedContent(initial="stdout", classes="file-tabs"):
            with TabPane("STDOUT", id="stdout"):
                self.stdout_area = TextArea("Loading STDOUT...", read_only=True, classes="content-area")
                yield self.stdout_area
            with TabPane("STDERR", id="stderr"):
                self.stderr_area = TextArea("Loading STDERR...", read_only=True, classes="content-area")
                yield self.stderr_area
            with TabPane("STDIN", id="stdin"):
                self.stdin_area = TextArea("Loading STDIN...", read_only=True, classes="content-area")
                yield self.stdin_area

        # One-line info about the pretty-print command
        self.conv_info_label = Label("", classes="conv-info")
        yield self.conv_info_label

        with Vertical(classes="detail-input-section"):
            with Horizontal(classes="detail-input-row"):
                self.stdin_input = Input(placeholder="Send to STDIN (Enter)", classes="detail-stdin-input")
                self.send_button = Button("Send", variant="primary", classes="detail-send-button")
                yield self.stdin_input
                yield self.send_button

    def on_mount(self) -> None:
        self.refresh_content()
        self.refresh_timer = self.set_interval(3, self.refresh_content)
        self.update_info_label()
        try:
            if self.send_button:
                self.send_button.disabled = not self._is_active
            if self.stdin_input:
                self.stdin_input.disabled = not self._is_active
        except Exception:
            pass

    def on_unmount(self) -> None:
        if self.refresh_timer:
            self.refresh_timer.stop()

    def pretty_tail(self, stream: str, n: int = 100) -> str:
        """Use lproc --pretty with the selected converter to fetch last N lines.

        Falls back to a short error note on failure; prints raw text on success.
        """
        try:
            import subprocess
            conv = self._converters.get(stream, "un")
            script_dir = os.path.dirname(os.path.abspath(__file__))
            lproc_path = os.path.join(script_dir, "lproc.py")
            if not os.path.exists(lproc_path):
                return "(lproc.py not found)"
            # Build command: python lproc.py -p <name> <stream> <N> <converter>
            argv = [sys.executable, lproc_path, "-p", self.lproc_name, stream, str(n), conv]
            result = subprocess.run(argv, capture_output=True, text=True)
            if result.returncode != 0:
                msg = (result.stdout or result.stderr or "converter error").strip()
                return f"(pretty error: {msg})"
            text = result.stdout or ""
            return text if text.strip() else "(empty file)"
        except Exception as e:
            return f"(pretty error: {e})"

    def refresh_content(self) -> None:
        try:
            try:
                running_lprocs, _inactive, _dir = get_lprocs_data()
                is_active_now = any(lp['name'] == self.lproc_name for lp in running_lprocs)
            except Exception:
                is_active_now = False
            if is_active_now != getattr(self, "_is_active", True):
                self._is_active = is_active_now
                try:
                    if self.send_button:
                        self.send_button.disabled = not self._is_active
                    if self.stdin_input:
                        self.stdin_input.disabled = not self._is_active
                    if not self._is_active:
                        self.app.notify("LProc became inactive; input disabled", severity="warning", timeout=3)
                except Exception:
                    pass

            # Update header command text from live data, if available
            try:
                live = None
                for lp in running_lprocs:
                    if lp['name'] == self.lproc_name:
                        live = lp
                        break
                new_cmd = ""
                if live:
                    # Always prefer the full inner bash command; fallback to outer bash -c
                    new_cmd = (live.get('inner_bash_c') or live.get('bash_c') or "")
                if new_cmd and new_cmd != self._full_cmd_text:
                    self._full_cmd_text = new_cmd
                    if self.cmd_label:
                        self.cmd_label.update(new_cmd)
            except Exception:
                pass

            # Update file contents
            stdout_content = self.pretty_tail("stdout", 100)
            if self.stdout_area and stdout_content != self._prev_stdout:
                self.stdout_area.text = stdout_content
                self.stdout_area.cursor_location = (len(self.stdout_area.text.split('\n')), 0)
                self._prev_stdout = stdout_content

            stderr_content = self.pretty_tail("stderr", 100)
            if self.stderr_area and stderr_content != self._prev_stderr:
                self.stderr_area.text = stderr_content
                self.stderr_area.cursor_location = (len(self.stderr_area.text.split('\n')), 0)
                self._prev_stderr = stderr_content

            stdin_content = self.pretty_tail("stdin", 100)
            if self.stdin_area and stdin_content != self._prev_stdin:
                self.stdin_area.text = stdin_content
                self.stdin_area.cursor_location = (len(self.stdin_area.text.split('\n')), 0)
                self._prev_stdin = stdin_content

            if hasattr(self.app, 'log_debug'):
                self.app.log_debug(
                    f"Refreshed {self.lproc_name}: out={len(stdout_content)} err={len(stderr_content)} in={len(stdin_content)}"
                )
            # Keep the info label in sync with active tab / converter
            self.update_info_label()
        except Exception as e:
            if hasattr(self.app, 'log_debug'):
                self.app.log_debug(f"ERROR refreshing content: {e}")

    # Click-to-copy removed per request; header is display-only now.

    def focus_input(self) -> None:
        try:
            if self.stdin_input and not getattr(self.stdin_input, "disabled", False):
                self.stdin_input.focus()
        except Exception:
            pass

    def list_converters(self, stream: str):
        """Return a list of (name, description) for converters supporting stream."""
        try:
            script_dir = os.path.dirname(os.path.abspath(__file__))
            conv_dir = os.path.join(script_dir, "converters")
            items = []
            for fname in os.listdir(conv_dir):
                if fname.endswith(f"__{stream}_r.py"):
                    name = fname.split("__", 1)[0]
                    desc = "Raw passthrough" if name == "un" else "Converter"
                    items.append((name, desc))
            # Deduplicate and sort by name
            items = sorted({k: v for k, v in items}.items())
            return items
        except Exception:
            return [("un", "Raw passthrough")]

    def open_converter_picker(self):
        """Open converter select screen for the currently active stream tab."""
        try:
            # The first TabbedContent inside DetailView is the file-tabs
            tabs = self.query(TabbedContent)
            target = None
            for t in tabs:
                # pick the one that has our three panes
                try:
                    ids = {p.id for p in t.query(TabPane)}
                    if {"stdout", "stderr", "stdin"}.issubset(ids):
                        target = t
                        break
                except Exception:
                    continue
            active_id = getattr(target, "active", "stdout") if target else "stdout"
            if not isinstance(active_id, str):
                active_id = "stdout"
            stream = active_id
        except Exception:
            stream = "stdout"
        # Build choices and push screen via app
        choices = self.list_converters(stream)
        self.app.push_screen(ConverterSelectScreen(self.lproc_name, stream, choices))

    def set_converter(self, stream: str, name: str) -> None:
        self._converters[stream] = name
        self.refresh_content()
        self.update_info_label()

    @on(TabbedContent.TabActivated)
    def _on_tab_activated(self, event: TabbedContent.TabActivated) -> None:  # type: ignore[attr-defined]
        # When the user switches among STDOUT/STDERR/STDIN, update label
        self.update_info_label()

    def _active_stream(self) -> str:
        try:
            tabs = self.query(TabbedContent)
            for t in tabs:
                ids = {p.id for p in t.query(TabPane)}
                if {"stdout", "stderr", "stdin"}.issubset(ids):
                    active = getattr(t, "active", "stdout")
                    return active if isinstance(active, str) else "stdout"
        except Exception:
            pass
        return "stdout"

    def update_info_label(self) -> None:
        try:
            stream = self._active_stream()
            conv = self._converters.get(stream, "un")
            script_dir = os.path.dirname(os.path.abspath(__file__))
            lproc_path = os.path.join(script_dir, "lproc.py")
            cmd = f"{os.path.basename(sys.executable)} {os.path.basename(lproc_path)} -p {self.lproc_name} {stream} 100 {conv}"
            text = f"Pretty: {stream.upper()} via {conv} — {cmd}"
            if self.conv_info_label:
                self.conv_info_label.update(text)
        except Exception:
            pass

    @on(Input.Submitted)
    def _on_input_submitted(self, event: Input.Submitted) -> None:
        if event.input is self.stdin_input:
            self.send_stdin_input()

    @on(Button.Pressed)
    def _on_send_pressed(self, event: Button.Pressed) -> None:
        if event.button is self.send_button:
            self.send_stdin_input()

    def send_stdin_input(self) -> None:
        if not self.stdin_input:
            return
        if getattr(self.stdin_input, "disabled", False):
            self.app.notify("LProc is inactive; sending disabled", severity="warning", timeout=2)
            return
        raw_value = self.stdin_input.value or ""
        text = raw_value.strip()
        if not text:
            return
        payload = None
        if text.startswith("$"):
            cmd = text[1:].strip()
            if not cmd:
                self.app.notify("Error: no command specified after '$'", severity="error", timeout=3)
                return
            try:
                import subprocess
                if hasattr(self.app, 'log_debug'):
                    self.app.log_debug(f"Executing stdin command for {self.lproc_name}: {cmd}")
                result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
            except Exception as e:
                self.app.notify(f"Command execution error: {e}", severity="error", timeout=3)
                return
            if result.returncode != 0:
                err_msg = (result.stderr or result.stdout or "command failed").strip()
                detail = f": {err_msg}" if err_msg else ""
                self.app.notify(
                    f"Command exited with {result.returncode}{detail}",
                    severity="error",
                    timeout=4,
                )
                return
            cmd_output = result.stdout or ""
            if cmd_output == "":
                self.app.notify("Command produced no stdout; nothing sent", severity="warning", timeout=3)
                return
            if not cmd_output.endswith("\n"):
                cmd_output += "\n"
            payload = cmd_output
        else:
            payload = text + "\n"
        try:
            with open(self.lproc_info['stdin_file'], 'a') as f:
                f.write(payload)
            self.stdin_input.value = ""
            self.refresh_content()
            self.app.notify(f"Sent to {self.lproc_name}.stdin", severity="success", timeout=2)
        except Exception as e:
            self.app.notify(f"Error: {e}", severity="error", timeout=3)

    


class HLProcTextual(App):
    """Main HLProc application using Textual"""

    @staticmethod
    def _sanitize_pane_id(name: str) -> str:
        """Sanitize lproc name to create a valid Textual DOM identifier.

        Textual IDs must contain only letters, numbers, underscores, or hyphens,
        and must not begin with a number.
        """
        # Replace invalid characters with underscores
        safe_name = "".join(c if c.isalnum() or c in "_-" else "_" for c in name)
        # Ensure it doesn't start with a digit
        if safe_name and safe_name[0].isdigit():
            safe_name = "d_" + safe_name
        return f"detail_{safe_name}"

    CSS = """
    /* List */
    DataTable { height: 100%; }

    /* Detail view styling using classes (works with multiple instances) */
    .detail-header-row { width: 100%; height: 1; }
    .detail-title {
        text-align: center;
        text-style: bold;
        color: cyan;
        margin: 0 1;
        height: 1;
    }
    .detail-cmd {
        color: #888888;
        margin: 0 1;
        width: 1fr;
        content-align: right middle;
        text-style: dim;
        height: 1;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    .file-tabs {
        margin: 0 1;
        border: none;
        height: 1fr;
    }
    .content-area {
        margin: 0 1;
        border: none;
        height: 1fr;
    }
    TabbedContent, TabPane {
        border: none;
        padding: 0;
        margin: 0;
    }
    .detail-input-section {
        dock: bottom;
        margin: 0 1;
        border: none;
        padding: 0;
        height: 4;
        min-height: 4;
    }
    .detail-input-row { height: 3; width: 100%; }
    .detail-stdin-input { width: 1fr; height: 3; }
    .detail-send-button { width: 10; height: 3; margin-left: 1; margin-right: 1; }
    """
    
    BINDINGS = [
        ("q", "quit", "Quit"),
        ("o", "open_detail", "Detail"),
        ("r", "refresh", "Refresh"),
        ("tab", "switch_tab", "Tabs"),
        ("s", "save_debug", "SaveDbg"),  # README/CLAUDE.md: 's' saves debug (DEBUG tab)
        ("c", "choose_converter", "Convert"),
        ("n", "create_lproc", "Create"),  # Rebound from 's' to avoid conflict
        ("k", "kill_selected", "Kill"),
        ("d", "delete_selected", "Delete"),
        ("w", "close_detail", "CloseDtl"),
        ("i", "detail_focus_input", "Focus"),
        ("t", "toggle_sort", "Sort"),
    ]
    
    def __init__(self):
        super().__init__()
        self.running_lprocs = []
        self.inactive_lprocs = []
        self.lproc_dir = ""
        self.list_refresh_timer = None
        self.debug_messages = []
        self._did_initial_focus = False
        # Sorting state (default: by Last, newest first)
        self._sort_by: str = "Last"
        self._sort_desc: bool = True
    
    def compose(self) -> ComposeResult:
        """Create the main layout"""
        yield Header()

        with TabbedContent(initial="list"):
            # LIST Tab
            with TabPane("LIST", id="list"):
                # Create table with cursor and zebra stripes configured
                with Container():
                    table = DataTable(id="lproc-table", cursor_type="row", zebra_stripes=True)
                    yield table

            # DEBUG Tab
            with TabPane("DEBUG", id="debug"):
                with VerticalScroll():
                    yield TextArea("Debug messages will appear here...\n", id="debug-text", read_only=True)

        yield Footer()
    
    def on_mount(self) -> None:
        """Set up the app when mounted"""
        try:
            # Initialize the table
            table = self.query_one("#lproc-table", DataTable)
            
            # Add columns - MUST be done before adding any rows
            table.add_columns("Name", "PGID", "STDIN", "STDOUT", "STDERR", "Status", "Last", "Command")
            
            # Make sure table can be focused
            table.can_focus = True
            
            # Schedule initial data load after a short delay to ensure proper mounting
            self.set_timer(0.1, self.initial_load)
            
            # Set up auto-refresh every 5 seconds
            self.list_refresh_timer = self.set_interval(5, self.refresh_list)
            
        except Exception as e:
            self.log_debug(f"ERROR in on_mount: {type(e).__name__}: {e}")
            import traceback
            self.log_debug(traceback.format_exc())
    
    def initial_load(self) -> None:
        """Load initial data after mount"""
        self.refresh_list()
        # Only on first load, auto-focus the table if LIST tab is active
        try:
            if not self._did_initial_focus and self.query_one(TabbedContent).active == "list":
                self.query_one("#lproc-table", DataTable).focus()
                self._did_initial_focus = True
        except Exception:
            pass
    
    def log_debug(self, message: str) -> None:
        """Add a debug message to the debug panel"""
        timestamp = datetime.now().strftime("%H:%M:%S")
        debug_msg = f"[{timestamp}] {message}\n"
        self.debug_messages.append(debug_msg)
        # Keep only the last 200 lines/messages
        if len(self.debug_messages) > 200:
            self.debug_messages = self.debug_messages[-200:]

        try:
            debug_area = self.query_one("#debug-text", TextArea)
            # Remember current cursor position to preserve scroll
            prev_row, prev_col = debug_area.cursor_location

            # Update text to the last 200 lines
            debug_area.text = "".join(self.debug_messages)

            # Restore cursor location (clamped to new content)
            lines = debug_area.text.split('\n')
            if not lines:
                debug_area.cursor_location = (0, 0)
            else:
                row = min(prev_row, max(0, len(lines) - 1))
                col = min(prev_col, len(lines[row]))
                debug_area.cursor_location = (row, col)
        except Exception:
            pass  # Debug panel might not be available yet
    
    def format_size(self, size: int) -> str:
        """Format file size in human-readable format"""
        for unit in ['B', 'KB', 'MB', 'GB']:
            if size < 1024.0:
                if unit == 'B':
                    return f"{size}B"
                return f"{size:.1f}{unit}"
            size /= 1024.0
        return f"{size:.1f}TB"

    def format_last_act(self, mtime: float) -> str:
        """Return human-friendly age (s/m/h) since mtime; '—' if unknown."""
        if not mtime or mtime <= 0:
            return "—"
        import time as _t
        delta = int(max(0, _t.time() - mtime))
        if delta < 60:
            return f"{delta}s"
        if delta < 3600:
            return f"{delta // 60}m"
        return f"{delta // 3600}h"
    
    def refresh_list(self) -> None:
        """Refresh the LProc list"""
        try:
            # Get table and remember current selection before we rebuild rows
            table = self.query_one("#lproc-table", DataTable)
            selected_name = None
            try:
                if table.row_count and table.cursor_row >= 0:
                    cur = table.get_row_at(table.cursor_row)
                    if cur:
                        selected_name = str(cur[0])
            except Exception:
                selected_name = None

            # Get data
            self.running_lprocs, self.inactive_lprocs, self.lproc_dir = get_lprocs_data()

            # Debug: Show count in title
            self.sub_title = f"Running: {len(self.running_lprocs)} | Inactive: {len(self.inactive_lprocs)} | Dir: {self.lproc_dir}"

            # Build desired rows (key, cells, mtime)
            desired: list[tuple[str, list[str], float]] = []

            def _row_for_running(lproc: dict):
                try:
                    stdin_size = os.path.getsize(lproc['stdin_file'])
                    stdout_size = os.path.getsize(lproc['stdout_file'])
                    stderr_size = os.path.getsize(lproc['stderr_file'])
                except Exception as e:
                    self.log_debug(f"ERROR getting file sizes for {lproc['name']}: {e}")
                    stdin_size = stdout_size = stderr_size = 0
                try:
                    mt = max(
                        os.path.getmtime(lproc['stdin_file']) if os.path.exists(lproc['stdin_file']) else 0.0,
                        os.path.getmtime(lproc['stdout_file']) if os.path.exists(lproc['stdout_file']) else 0.0,
                        os.path.getmtime(lproc['stderr_file']) if os.path.exists(lproc['stderr_file']) else 0.0,
                    )
                except Exception:
                    mt = 0.0
                last_act = self.format_last_act(mt)
                # Determine PGID (prefer value from lproc data; fallback to os.getpgid)
                if 'pgid' in lproc and lproc['pgid'] is not None:
                    pgid = str(lproc['pgid'])
                else:
                    try:
                        pgid_val = os.getpgid(lproc['bash_process']['pid']) if lproc.get('bash_process') else None
                        pgid = str(pgid_val) if pgid_val is not None else "N/A"
                    except Exception:
                        pgid = "N/A"
                # Prefer the actual running target command(s) when available
                if lproc['command_processes']:
                    parts = []
                    for proc in lproc['command_processes']:
                        pcmd = proc.get('cmd') or ''
                        first = pcmd.split()[0] if pcmd else ''
                        parts.append(os.path.basename(first) if first else '(cmd)')
                    display_cmd = " | ".join(parts)
                elif lproc.get('inner_bash_c'):
                    display_cmd = lproc['inner_bash_c']
                elif lproc.get('bash_c'):
                    display_cmd = lproc['bash_c']
                else:
                    bash_cmdline = (lproc.get('bash_process') or {}).get('cmd', '')
                    if bash_cmdline and " -c " in bash_cmdline:
                        display_cmd = bash_cmdline.split(" -c ", 1)[1].strip()
                    else:
                        display_cmd = "(exited)"
                try:
                    inner_bash_pid = (lproc.get('inner_bash') or {}).get('pid')
                except Exception:
                    inner_bash_pid = None
                status = "🟢 Running" if (lproc['command_processes'] or inner_bash_pid) else "🔴 Exited"
                cells = [
                    lproc['name'],
                    pgid,
                    self.format_size(stdin_size),
                    self.format_size(stdout_size),
                    self.format_size(stderr_size),
                    status,
                    last_act,
                    (display_cmd[:80] + ("…" if len(display_cmd) > 80 else "")),
                ]
                return (lproc['name'], cells, mt)

            def _row_for_inactive(name: str):
                try:
                    stdin_path = get_lproc_file_path(name, 'stdin')
                    stdout_path = get_lproc_file_path(name, 'stdout')
                    stderr_path = get_lproc_file_path(name, 'stderr')
                    stdin_size = os.path.getsize(stdin_path) if os.path.exists(stdin_path) else 0
                    stdout_size = os.path.getsize(stdout_path) if os.path.exists(stdout_path) else 0
                    stderr_size = os.path.getsize(stderr_path) if os.path.exists(stderr_path) else 0
                except Exception as e:
                    self.log_debug(f"ERROR getting inactive sizes for {name}: {e}")
                    stdin_size = stdout_size = stderr_size = 0
                try:
                    mt = max(
                        os.path.getmtime(stdin_path) if os.path.exists(stdin_path) else 0.0,
                        os.path.getmtime(stdout_path) if os.path.exists(stdout_path) else 0.0,
                        os.path.getmtime(stderr_path) if os.path.exists(stderr_path) else 0.0,
                    )
                except Exception:
                    mt = 0.0
                last_act = self.format_last_act(mt)
                cells = [
                    name,
                    "—",
                    self.format_size(stdin_size),
                    self.format_size(stdout_size),
                    self.format_size(stderr_size),
                    "⚫ Inactive",
                    last_act,
                    "—",
                ]
                return (f"inactive_{name}", cells, mt)

            for lp in self.running_lprocs:
                desired.append(_row_for_running(lp))
            for name in self.inactive_lprocs:
                desired.append(_row_for_inactive(name))

            # Sort according to current preference (supported: Last or Name)
            sort_col = getattr(self, "_sort_by", "Last")
            sort_desc = getattr(self, "_sort_desc", True)
            if sort_col == "Last":
                desired.sort(key=lambda x: x[2], reverse=sort_desc)
            else:  # Name
                desired.sort(key=lambda x: str(x[1][0]).lower(), reverse=sort_desc)
            desired_keys = [k for k, _, _ in desired]
            desired_set = set(desired_keys)
            existing_keys = getattr(self, "_table_row_keys", set())

            # Remove rows that disappeared
            for key in list(existing_keys - desired_set):
                try:
                    table.remove_row(key)
                except Exception:
                    pass

            # Reorder rows to match desired by remove-then-add sequence (prevents empty flash)
            # Build quick map for cells
            cells_map = {k: cells for (k, cells, _mt) in desired}
            for key in desired_keys:
                cells = cells_map[key]
                if key in existing_keys:
                    # Remove existing entry to reposition
                    try:
                        table.remove_row(key)
                    except Exception:
                        pass
                try:
                    table.add_row(*cells, key=key)
                except Exception as e:
                    self.log_debug(f"ERROR adding row {key}: {e}")

            # Track keys and order for next diff
            self._table_row_keys = set(desired_keys)
            self._row_order = list(desired_keys)

            # Restore cursor to previously selected row if possible, and keep it in view
            if selected_name:
                try:
                    target_row = None
                    for i in range(table.row_count):
                        row = table.get_row_at(i)
                        if row and str(row[0]) == selected_name:
                            target_row = i
                            break
                    if target_row is not None:
                        # Try several APIs depending on Textual version to restore cursor
                        restored = False
                        for setter in (
                            lambda: setattr(table, "cursor_coordinate", (target_row, 0)),
                            lambda: table.move_cursor(row=target_row, column=0),
                            lambda: table.focus_cell(target_row, 0),
                        ):
                            try:
                                setter()
                                restored = True
                                break
                            except Exception:
                                continue
                        # And attempt to restore scroll near the cursor
                        for scroller in (
                            lambda: table.scroll_to_row(target_row),
                            lambda: table.scroll_to_cell(target_row, 0),
                            lambda: table.scroll_to_coordinate(target_row, 0),
                        ):
                            try:
                                scroller()
                                break
                            except Exception:
                                continue
                except Exception:
                    pass

            # Do not force-focus the table on refresh; respect current focus
            
        except Exception as e:
            error_msg = f"ERROR in refresh_list: {type(e).__name__}: {e}"
            self.log_debug(error_msg)
            import traceback
            self.log_debug(traceback.format_exc())
            self.notify(error_msg, severity="error")
    
    def action_refresh(self) -> None:
        """Manual refresh action; context-aware for LIST vs DETAIL tabs."""
        try:
            tabbed_content = self.query_one(TabbedContent)
            active = tabbed_content.active
        except Exception:
            active = "list"
        if isinstance(active, str) and active.startswith("detail_"):
            # Refresh the active detail view
            try:
                pane = self.query_one(f"#{active}", TabPane)
                view = None
                for child in pane.children:
                    if isinstance(child, DetailView):
                        view = child
                        break
                if view:
                    view.refresh_content()
                    self.notify("Detail refreshed", severity="information", timeout=1)
                    return
            except Exception:
                pass
        # Default to list refresh
        self.refresh_list()
        self.notify("List refreshed", severity="information", timeout=1)

    def action_toggle_sort(self) -> None:
        """Toggle sorting between Last(desc) and Name(asc)."""
        try:
            if getattr(self, "_sort_by", "Last") == "Last":
                self._sort_by = "Name"
                self._sort_desc = False
            else:
                self._sort_by = "Last"
                self._sort_desc = True
            self.refresh_list()
            arrow = "↓" if self._sort_desc else "↑"
            self.notify(f"Sort: {self._sort_by} {arrow}", severity="information", timeout=1)
        except Exception:
            pass

    # Removed header-click sorting to keep UX keyboard-only (use 't').
    
    def action_open_detail(self) -> None:
        """Open or focus a detail TabPane for the selected LProc."""
        self.log_debug("'o' pressed - open_detail")
        try:
            tabbed_content = self.query_one(TabbedContent)
            if tabbed_content.active != "list":
                self.notify("Open details from LIST tab.", severity="warning", timeout=1)
                return
            table = self.query_one("#lproc-table", DataTable)
            if table.cursor_row < 0:
                self.notify("No selection.", severity="warning", timeout=1)
                return
            row_data = table.get_row_at(table.cursor_row)
            if not row_data:
                self.notify("Invalid selection.", severity="warning", timeout=1)
                return
            lproc_name = str(row_data[0])
            # Build info
            lproc_info = None
            is_active = False
            for lp in self.running_lprocs:
                if lp['name'] == lproc_name:
                    lproc_info = lp
                    is_active = True
                    break
            if lproc_info is None:
                lproc_info = {
                    'name': lproc_name,
                    'stdin_file': get_lproc_file_path(lproc_name, 'stdin'),
                    'stdout_file': get_lproc_file_path(lproc_name, 'stdout'),
                    'stderr_file': get_lproc_file_path(lproc_name, 'stderr'),
                    'command_processes': []
                }
            self.open_detail_tab(lproc_name, lproc_info, is_active)
        except Exception as e:
            self.log_debug(f"ERROR in action_open_detail: {e}")
            try:
                import traceback
                self.log_debug(traceback.format_exc())
            except Exception:
                pass
            self.notify(f"Error opening detail: {e}", severity="error", timeout=3)

    def open_detail_tab(self, lproc_name: str, lproc_info: dict, is_active: bool) -> None:
        """Create or focus a detail tab for the given LProc."""
        pane_id = self._sanitize_pane_id(lproc_name)
        tabbed = self.query_one(TabbedContent)
        # If pane exists, just focus it
        try:
            existing = self.query_one(f"#{pane_id}", TabPane)
            if existing:
                tabbed.active = pane_id
                return
        except Exception:
            pass
        # Create pane
        # Shorten the visible label to save space in the tab bar
        def _short_label(text: str, max_len: int = 18) -> str:
            return text if len(text) <= max_len else (text[: max_len - 1] + "…")
        pane = TabPane(_short_label(lproc_name), id=pane_id)
        view = DetailView(lproc_name, lproc_info, is_active)
        try:
            # Prefer API if available
            try:
                tabbed.add_pane(pane)
            except Exception:
                tabbed.mount(pane)
            pane.mount(view)
            tabbed.active = pane_id
            # Log process details once when opening the detail tab
            try:
                info = lproc_info or {}
                def _short(s: str, n: int = 160) -> str:
                    return s if len(s) <= n else s[: n - 1] + "…"
                self.log_debug(f"Detail opened for '{lproc_name}' (active={is_active})")
                self.log_debug(f"  PGID: {info.get('pgid')}")
                tail = info.get('tail_process') or {}
                self.log_debug(f"  LPTAIL: pid={tail.get('pid')} cmd={_short(tail.get('cmd',''))}")
                bashp = info.get('bash_process') or {}
                self.log_debug(f"  BASH:   pid={bashp.get('pid')} cmd={_short(bashp.get('cmd',''))}")
                inner = info.get('inner_bash') or {}
                if inner.get('pid') or info.get('inner_bash_c'):
                    self.log_debug(f"  INNER:  pid={inner.get('pid')} cmd={_short(info.get('inner_bash_c') or inner.get('cmd',''))}")
                cps = info.get('command_processes') or []
                if cps:
                    for cp in cps:
                        self.log_debug(f"  CMD:    pid={cp.get('pid')} name={cp.get('name')} cmd={_short(cp.get('cmd',''))}")
                self.log_debug(f"  Files: stdin={info.get('stdin_file')}\n         stdout={info.get('stdout_file')}\n         stderr={info.get('stderr_file')}")
            except Exception:
                pass
        except Exception as e:
            self.log_debug(f"ERROR creating detail tab: {e}")
            self.notify(f"Error creating tab: {e}", severity="error", timeout=3)

    def action_close_detail(self) -> None:
        """Close the active detail tab, if any."""
        try:
            tabbed = self.query_one(TabbedContent)
            active = tabbed.active
            if not (isinstance(active, str) and active.startswith("detail_")):
                return
            pane_id = active
            # First switch focus to LIST to avoid TabbedContent picking another tab
            tabbed.active = "list"
            # Then remove the previous active detail pane
            removed = False
            try:
                tabbed.remove_pane(pane_id)  # type: ignore[attr-defined]
                removed = True
            except Exception:
                try:
                    pane = self.query_one(f"#{pane_id}", TabPane)
                    pane.remove()
                    removed = True
                except Exception:
                    removed = False
            # Ensure LIST stays active even if TabbedContent changes it after removal
            self.call_after_refresh(lambda: setattr(tabbed, 'active', 'list'))
        except Exception as e:
            self.log_debug(f"ERROR closing detail tab: {e}")

    def action_detail_focus_input(self) -> None:
        """Focus the stdin input of the active detail tab."""
        try:
            tabbed = self.query_one(TabbedContent)
            active = tabbed.active
            if not (isinstance(active, str) and active.startswith("detail_")):
                return
            pane = self.query_one(f"#{active}", TabPane)
            for child in pane.children:
                if isinstance(child, DetailView):
                    child.focus_input()
                    break
        except Exception:
            pass

    def action_save_debug(self) -> None:
        """Save debug log to a file (only on DEBUG tab)."""
        try:
            if self.query_one(TabbedContent).active != "debug":
                # Only allow in DEBUG tab
                return
        except Exception:
            return
        try:
            debug_area = self.query_one("#debug-text", TextArea)
            debug_text = debug_area.text
            
            # Save to a timestamped file
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            filename = f"hlproc_debug_{timestamp}.log"
            
            with open(filename, 'w') as f:
                f.write(debug_text)
            
            self.notify(f"Debug log saved to {filename}", severity="success", timeout=3)
            self.log_debug(f"Debug log saved to {filename}")
        except Exception as e:
            self.notify(f"Error saving debug: {e}", severity="error")

    def action_choose_converter(self) -> None:
        """Open converter picker for the active detail view and stream."""
        try:
            tabbed = self.query_one(TabbedContent)
            active = tabbed.active
            if not (isinstance(active, str) and active.startswith("detail_")):
                self.notify("Open a detail tab first.", severity="warning", timeout=2)
                return
            pane = self.query_one(f"#{active}", TabPane)
            view = None
            for child in pane.children:
                if isinstance(child, DetailView):
                    view = child
                    break
            if view:
                view.open_converter_picker()
        except Exception as e:
            self.log_debug(f"ERROR opening converter picker: {e}")

    def action_create_lproc(self) -> None:
        """Open create LProc input (LIST tab only)."""
        try:
            if self.query_one(TabbedContent).active != "list":
                self.notify("Creation available in LIST tab.", severity="warning", timeout=2)
                return
            self.push_screen(CreateInputScreen())
        except Exception as e:
            self.log_debug(f"ERROR opening create input: {e}")
            import traceback
            self.log_debug(traceback.format_exc())
            self.notify(f"Error: {e}", severity="error", timeout=3)

    def handle_create_input(self, lproc_name: str, command: str) -> None:
        """Called by CreateInputScreen with user input; ask for confirmation."""
        # Close input screen if still open
        try:
            self.pop_screen()
        except Exception:
            pass
        self.push_screen(CreateConfirmScreen(lproc_name, command))

    def handle_create_confirm(self, lproc_name: str, command: str, confirmed: bool) -> None:
        """Handle confirmation and run lproc -s."""
        if not confirmed:
            try:
                self.pop_screen()  # close confirm
            except Exception:
                pass
            self.notify("Create cancelled.", severity="information", timeout=1)
            return
        # Close confirm first
        try:
            self.pop_screen()
        except Exception:
            pass

        # Run lproc.py -s <name> <command>
        try:
            import subprocess, shlex
            script_dir = os.path.dirname(os.path.abspath(__file__))
            lproc_path = os.path.join(script_dir, "lproc.py")
            if not os.path.exists(lproc_path):
                raise FileNotFoundError("lproc.py not found")
            result = subprocess.run(
                [sys.executable, lproc_path, "-s", lproc_name, command],
                capture_output=True,
                text=True,
            )
            out = result.stdout or ""
            err = result.stderr or ""
            summary = f"Exit code: {result.returncode}\n\n"
            content = summary
            if out:
                content += "[STDOUT]\n" + out
            if err:
                if content and not content.endswith("\n"):
                    content += "\n"
                content += "[STDERR]\n" + err
        except Exception as e:
            content = f"Error invoking lproc create: {e}"

        # Refresh list
        try:
            self.refresh_list()
        except Exception:
            pass

        # Show result
        self.push_screen(OutputScreen(title=f"Create output for '{lproc_name}'", text=content))

    def action_switch_tab(self) -> None:
        """Cycle to the next TabPane in the main TabbedContent (TAB)."""
        try:
            tabbed = self.query_one(TabbedContent)
            panes = [p.id for p in tabbed.query(TabPane)]
            panes = [pid for pid in panes if isinstance(pid, str) and pid]
            if not panes:
                return
            active = tabbed.active if isinstance(tabbed.active, str) else panes[0]
            try:
                idx = panes.index(active)
            except ValueError:
                idx = -1
            next_id = panes[(idx + 1) % len(panes)]
            tabbed.active = next_id
        except Exception:
            pass

    def action_kill_selected(self) -> None:
        """Prompt to kill the selected LProc from the LIST tab."""
        try:
            # Only valid on LIST tab
            tabbed_content = self.query_one(TabbedContent)
            if tabbed_content.active != "list":
                self.notify("Switch to LIST to kill.", severity="warning", timeout=2)
                return

            table = self.query_one("#lproc-table", DataTable)
            if table.cursor_row < 0 or table.row_count == 0:
                self.notify("No selection.", severity="warning", timeout=2)
                return
            row = table.get_row_at(table.cursor_row)
            if not row:
                self.notify("Invalid selection.", severity="warning", timeout=2)
                return
            name = str(row[0])
            # For inactive rows we prefixed key with inactive_, but first column is still the name
            self.push_screen(KillConfirmScreen(name))
        except Exception as e:
            self.log_debug(f"ERROR in action_kill_selected: {e}")
            import traceback
            self.log_debug(traceback.format_exc())
            self.notify(f"Error: {e}", severity="error", timeout=3)

    def action_delete_selected(self) -> None:
        """Prompt to delete the selected stopped LProc from the LIST tab."""
        try:
            # Only valid on LIST tab
            tabbed_content = self.query_one(TabbedContent)
            if tabbed_content.active != "list":
                self.notify("Switch to LIST to delete.", severity="warning", timeout=2)
                return

            table = self.query_one("#lproc-table", DataTable)
            if table.cursor_row < 0 or table.row_count == 0:
                self.notify("No selection.", severity="warning", timeout=2)
                return
            row = table.get_row_at(table.cursor_row)
            if not row:
                self.notify("Invalid selection.", severity="warning", timeout=2)
                return
            name = str(row[0])
            status = str(row[5]) if len(row) > 5 else ""

            # Check if LProc is stopped (inactive or exited)
            if "Inactive" not in status and "Exited" not in status:
                self.notify("Can only delete stopped LProcs. Kill it first.", severity="warning", timeout=3)
                return

            # Show delete confirmation screen
            self.push_screen(DeleteConfirmScreen(name))
        except Exception as e:
            self.log_debug(f"ERROR in action_delete_selected: {e}")
            import traceback
            self.log_debug(traceback.format_exc())
            self.notify(f"Error: {e}", severity="error", timeout=3)

    def handle_kill_confirm(self, lproc_name: str, confirmed: bool) -> None:
        """Callback from KillConfirmScreen with user's decision."""
        if not confirmed:
            # Close the confirm screen and return
            try:
                self.pop_screen()
            except Exception:
                pass
            self.notify("Kill cancelled.", severity="information", timeout=1)
            return
        # Close the confirm screen before running kill
        try:
            self.pop_screen()
        except Exception:
            pass
        # Run lproc.py -k <name> and capture output
        try:
            import subprocess
            script_dir = os.path.dirname(os.path.abspath(__file__))
            lproc_path = os.path.join(script_dir, "lproc.py")
            if not os.path.exists(lproc_path):
                raise FileNotFoundError("lproc.py not found")
            result = subprocess.run(
                [sys.executable, lproc_path, "-k", lproc_name],
                capture_output=True,
                text=True,
            )
            out = result.stdout or ""
            err = result.stderr or ""
            summary = f"Exit code: {result.returncode}\n\n"
            content = summary
            if out:
                content += "[STDOUT]\n" + out
            if err:
                if content and not content.endswith("\n"):
                    content += "\n"
                content += "[STDERR]\n" + err
        except Exception as e:
            content = f"Error invoking lproc kill: {e}"

        # Refresh list after attempting kill
        try:
            self.refresh_list()
        except Exception:
            pass

        # Show result in an overlay screen
        self.push_screen(OutputScreen(title=f"Kill output for '{lproc_name}'", text=content))

    def handle_delete_confirm(self, lproc_name: str, confirmed: bool) -> None:
        """Callback from DeleteConfirmScreen with user's decision."""
        if not confirmed:
            # Close the confirm screen and return
            try:
                self.pop_screen()
            except Exception:
                pass
            self.notify("Delete cancelled.", severity="information", timeout=1)
            return
        # Close the confirm screen before running delete
        try:
            self.pop_screen()
        except Exception:
            pass
        # Run lproc.py -d <name> and capture output
        try:
            import subprocess
            script_dir = os.path.dirname(os.path.abspath(__file__))
            lproc_path = os.path.join(script_dir, "lproc.py")
            if not os.path.exists(lproc_path):
                raise FileNotFoundError("lproc.py not found")
            result = subprocess.run(
                [sys.executable, lproc_path, "-d", lproc_name],
                capture_output=True,
                text=True,
            )
            out = result.stdout or ""
            err = result.stderr or ""
            summary = f"Exit code: {result.returncode}\n\n"
            content = summary
            if out:
                content += "[STDOUT]\n" + out
            if err:
                if content and not content.endswith("\n"):
                    content += "\n"
                content += "[STDERR]\n" + err
        except Exception as e:
            content = f"Error invoking lproc delete: {e}"

        # Refresh list after attempting delete
        try:
            self.refresh_list()
        except Exception:
            pass

        # Show result in an overlay screen
        self.push_screen(OutputScreen(title=f"Delete output for '{lproc_name}'", text=content))


class KillConfirmScreen(Screen):
    """Simple confirmation screen to kill a selected LProc."""

    BINDINGS = [
        ("y", "confirm", "Yes"),
        ("n", "cancel", "No"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self, lproc_name: str):
        super().__init__()
        self.lproc_name = lproc_name

    def compose(self) -> ComposeResult:
        with Vertical(id="confirm-container"):
            yield Label(f"Kill LProc '{self.lproc_name}'? Press Y to confirm, N to cancel.")
            with Horizontal():
                yield Button("Yes [Y]", id="yes", variant="error")
                yield Button("No [N]", id="no", variant="primary")
        yield Footer()

    def action_confirm(self) -> None:
        # Let the app handle closing this screen and showing results
        self.app.handle_kill_confirm(self.lproc_name, True)

    def action_cancel(self) -> None:
        self.app.handle_kill_confirm(self.lproc_name, False)

    @on(Button.Pressed, "#yes")
    def _on_yes(self) -> None:
        self.action_confirm()

    @on(Button.Pressed, "#no")
    def _on_no(self) -> None:
        self.action_cancel()


class DeleteConfirmScreen(Screen):
    """Confirmation screen to delete (archive) a stopped LProc."""

    BINDINGS = [
        ("y", "confirm", "Yes"),
        ("n", "cancel", "No"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self, lproc_name: str):
        super().__init__()
        self.lproc_name = lproc_name

    def compose(self) -> ComposeResult:
        with Vertical(id="delete-confirm-container"):
            yield Label(f"Delete (archive) LProc '{self.lproc_name}'?")
            yield Label("Files will be moved to .lproc_archive/")
            yield Label("Press Y to confirm, N to cancel.")
            with Horizontal():
                yield Button("Yes [Y]", id="yes", variant="error")
                yield Button("No [N]", id="no", variant="primary")
        yield Footer()

    def action_confirm(self) -> None:
        # Let the app handle closing this screen and showing results
        self.app.handle_delete_confirm(self.lproc_name, True)

    def action_cancel(self) -> None:
        self.app.handle_delete_confirm(self.lproc_name, False)

    @on(Button.Pressed, "#yes")
    def _on_yes(self) -> None:
        self.action_confirm()

    @on(Button.Pressed, "#no")
    def _on_no(self) -> None:
        self.action_cancel()


class OutputScreen(Screen):
    """Screen to display command output; close on any key."""

    def __init__(self, title: str, text: str):
        super().__init__()
        self._title = title
        self._text = text

    BINDINGS = [
        ("escape", "close", "Close"),
    ]

    def compose(self) -> ComposeResult:
        with Vertical():
            yield Label(self._title)
            yield TextArea(self._text, read_only=True)
            yield Label("Press any key or ESC to close.")
        yield Footer()

    def on_key(self, event) -> None:  # type: ignore[override]
        self.app.pop_screen()

    def action_close(self) -> None:
        self.app.pop_screen()


class CreateInputScreen(Screen):
    """Screen to input LProc name and command."""

    BINDINGS = [
        ("enter", "submit", "Submit"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self):
        super().__init__()
        from datetime import datetime
        self.default_name = f"lp_{datetime.now().strftime('%H%M%S')}"

    def compose(self) -> ComposeResult:
        with Vertical(id="create-container"):
            yield Label("Create LProc")
            yield Label("Name:")
            yield Input(self.default_name, id="create-name")
            yield Label("Command:")
            yield Input(placeholder="e.g. grep ERROR | awk '{print $1}'", id="create-command")
            with Horizontal():
                yield Button("Create [Enter]", id="create-btn", variant="primary")
                yield Button("Cancel [Esc]", id="cancel-btn")
        yield Footer()

    def on_mount(self) -> None:
        try:
            self.query_one("#create-name", Input).focus()
        except Exception:
            pass

    def action_submit(self) -> None:
        try:
            name = self.query_one("#create-name", Input).value.strip()
            cmd = self.query_one("#create-command", Input).value.strip()
            if not name or not cmd:
                self.app.notify("Name and command are required.", severity="warning", timeout=2)
                return
            self.app.handle_create_input(name, cmd)
        except Exception as e:
            self.app.notify(f"Error: {e}", severity="error", timeout=3)

    def action_cancel(self) -> None:
        try:
            self.app.pop_screen()
        except Exception:
            pass

    @on(Button.Pressed, "#create-btn")
    def _on_create(self) -> None:
        self.action_submit()

    @on(Button.Pressed, "#cancel-btn")
    def _on_cancel(self) -> None:
        self.action_cancel()


class CreateConfirmScreen(Screen):
    """Confirm creation with name and command."""

    BINDINGS = [
        ("y", "confirm", "Yes"),
        ("n", "cancel", "No"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self, lproc_name: str, command: str):
        super().__init__()
        self.lproc_name = lproc_name
        self.command = command

    def compose(self) -> ComposeResult:
        with Vertical(id="create-confirm-container"):
            yield Label("Create LProc with the following?")
            yield Label(f"Name: {self.lproc_name}")
            yield Label(f"Command: {self.command}")
            with Horizontal():
                yield Button("Yes [Y]", id="yes", variant="success")
                yield Button("No [N]", id="no", variant="primary")
        yield Footer()

    def action_confirm(self) -> None:
        self.app.handle_create_confirm(self.lproc_name, self.command, True)

    def action_cancel(self) -> None:
        self.app.handle_create_confirm(self.lproc_name, self.command, False)

    @on(Button.Pressed, "#yes")
    def _on_yes(self) -> None:
        self.action_confirm()

    @on(Button.Pressed, "#no")
    def _on_no(self) -> None:
        self.action_cancel()


class ConverterSelectScreen(Screen):
    """Screen to select a converter for a specific stream."""

    BINDINGS = [
        ("enter", "apply", "Apply"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self, lproc_name: str, stream: str, choices: List[Tuple[str, str]]):
        super().__init__()
        self.lproc_name = lproc_name
        self.stream = stream
        self.choices = choices

    def compose(self) -> ComposeResult:
        with Vertical(id="conv-select-container"):
            yield Label(f"Select converter for {self.lproc_name}:{self.stream}")
            table = DataTable(id="conv-table", cursor_type="row", zebra_stripes=True)
            yield table
            with Horizontal():
                yield Button("Apply [Enter]", id="apply", variant="primary")
                yield Button("Cancel [Esc]", id="cancel")
        yield Footer()

    def on_mount(self) -> None:
        try:
            table = self.query_one("#conv-table", DataTable)
            table.add_columns("Converter", "Description")
            for name, desc in self.choices:
                table.add_row(name, desc)
            table.cursor_coordinate = (0, 0)
            table.focus()
        except Exception:
            pass

    def action_apply(self) -> None:
        try:
            table = self.query_one("#conv-table", DataTable)
            if table.cursor_row < 0:
                return
            row = table.get_row_at(table.cursor_row)
            if not row:
                return
            name = str(row[0])
        except Exception:
            name = "un"
        # Apply to the active detail view
        try:
            tabbed = self.app.query_one(TabbedContent)
            active = tabbed.active
            pane = self.app.query_one(f"#{active}", TabPane)
            view = None
            for child in pane.children:
                if isinstance(child, DetailView):
                    view = child
                    break
            if view:
                view.set_converter(self.stream, name)
        except Exception:
            pass
        self.app.pop_screen()

    def action_cancel(self) -> None:
        self.app.pop_screen()

    @on(Button.Pressed, "#apply")
    def _on_apply(self) -> None:
        self.action_apply()

    @on(Button.Pressed, "#cancel")
    def _on_cancel(self) -> None:
        self.action_cancel()

    @on(DataTable.RowSelected, "#conv-table")
    def _on_row_selected(self, event: DataTable.RowSelected) -> None:
        # Hitting Enter on the table applies the current selection
        self.action_apply()
    


def main():
    """Main entry point"""
    import argparse

    # Parse command line arguments
    parser = argparse.ArgumentParser(
        description='HLProc - Textual UI for LProc Manager',
        add_help=False  # We'll handle --help manually
    )
    parser.add_argument('--help', action='store_true',
                       help='Show README documentation and exit')

    args = parser.parse_args()

    # Handle --help option
    if args.help:
        script_dir = os.path.dirname(os.path.abspath(__file__))
        readme_path = os.path.join(script_dir, "README.md")
        try:
            with open(readme_path, 'r', encoding='utf-8') as f:
                print(f.read())
            sys.exit(0)
        except Exception as e:
            print(f"Error: Could not read README.md: {e}", file=sys.stderr)
            sys.exit(1)

    # Check if lproc.py exists
    script_dir = os.path.dirname(os.path.abspath(__file__))
    lproc_path = os.path.join(script_dir, "lproc.py")

    if not os.path.exists(lproc_path):
        print("Error: lproc.py not found in the same directory", file=sys.stderr)
        sys.exit(1)

    # Run the app
    app = HLProcTextual()
    app.run()


if __name__ == "__main__":
    main()
