#!/usr/bin/env python3
"""
ACToR - Adversarial Agent Collaboration for C to Rust Translation
With Parallel Processing and Live Monitoring
"""

import os
import json
import shutil
import argparse
import threading
import time
from datetime import datetime
from pathlib import Path
from typing import Optional, Dict, List
from enum import Enum

from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.prompt import Prompt, Confirm
from rich import box

import importlib.util

from utils import white_list_for_copy_c, white_list_for_copy_rs, white_list_for_copy_test_cases, white_list_for_copy_log_files, _copy_directory

# Helper to import modules with dots in filename (e.g., "CC-Sonnet-4.5.py")
def _import_module_from_file(directory: str, filename: str):
    """Import a module from a file path, handling filenames with dots."""
    script_dir = Path(__file__).parent
    file_path = script_dir / directory / f"{filename}.py"
    spec = importlib.util.spec_from_file_location(filename, file_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

# Import translators (file names contain hyphens and dots)
translator_cc_sonnet_45 = _import_module_from_file("translators", "CC-Sonnet-4.5").translator
translator_swe_sonnet_45 = _import_module_from_file("translators", "SWE-Sonnet-4.5").translator
translator_swe_sonnet_4 = _import_module_from_file("translators", "SWE-Sonnet-4").translator
translator_swe_gpt_5mini = _import_module_from_file("translators", "SWE-GPT-5mini").translator

# Import discriminators (file names contain hyphens and dots)
discriminator_cc_sonnet_45_actor = _import_module_from_file("discriminators", "CC-Sonnet-4.5-ACToR").discriminator
discriminator_cc_sonnet_45_actor_1_3 = _import_module_from_file("discriminators", "CC-Sonnet-4.5-ACToR-1_3").discriminator
discriminator_cc_sonnet_45_actor_15_1 = _import_module_from_file("discriminators", "CC-Sonnet-4.5-ACToR-15_1").discriminator
discriminator_cc_sonnet_45_actor_15_5 = _import_module_from_file("discriminators", "CC-Sonnet-4.5-ACToR-15_5").discriminator
discriminator_cc_sonnet_45_actor_nofuzz = _import_module_from_file("discriminators", "CC-Sonnet-4.5-ACToR-noFuzz").discriminator
discriminator_cc_sonnet_45_coverage = _import_module_from_file("discriminators", "CC-Sonnet-4.5-Coverage").discriminator
discriminator_swe_sonnet_45_actor = _import_module_from_file("discriminators", "SWE-Sonnet-4.5-ACToR").discriminator
discriminator_swe_sonnet_4_actor = _import_module_from_file("discriminators", "SWE-Sonnet-4-ACToR").discriminator
discriminator_swe_gpt_5mini_actor = _import_module_from_file("discriminators", "SWE-GPT-5mini-ACToR").discriminator

"""
Default configuration for ACToR system.
"""

# Version
VERSION = "1.0.0"

# ASCII Logo
ACTOR_LOGO = r"""
 ________  ________ _________            ________     
|\   __  \|\   ____\\___   ___\ _______ |\   __  \    
\ \  \|\  \ \  \___\|___ \  \_||\   __  \ \  \|\  \   
 \ \   __  \ \  \       \ \  \ \ \  \\\  \ \   _  _\  
  \ \  \ \  \ \  \____   \ \  \ \ \  \\\  \ \  \\  \| 
   \ \__\ \__\ \_______\  \ \__\ \ \_______\ \__\\ _\ 
    \|__|\|__|\|_______|   \|__|  \|_______|\|__|\|__|
"""

# Default configuration values
DEFAULT_CONFIG = {
    "max_parallel": 5,
    "input_directory": "projects_input",
    "working_directory": ".working",
    "backups_directory": ".backups",
    "output_directory": "projects_output"
}

# Configuration descriptions
CONFIG_DESCRIPTIONS = {
    "max_parallel": "Maximum number of parallel translation tasks",
    "input_directory": "Directory containing input projects",
    "working_directory": "Directory for working files during translation",
    "backups_directory": "Directory for iteration backups",
    "output_directory": "Directory for final translation output"
}

console = Console()


class ProjectStatus(Enum):
    """Project execution status."""
    QUEUED = "queued"
    INITIALIZING = "initializing"
    TRANSLATING = "translating"
    DISCRIMINATING = "discriminating"
    PAUSED = "paused"
    COMPLETED = "completed"
    STOPPED = "stopped"
    ERROR = "error"


class WorkManager:
    """
    Manages a single translation project workflow.
    """
    
    def __init__(self, project_name: str, config: Dict, session_id: str = None):
        self.project_name = project_name
        self.config = config
        self.input_dir = Path(config['input_dir'])
        
        # Generate session ID for this translation run if no session ID is provided
        if session_id is None:
            import hashlib
            session_data = f"{project_name}_{datetime.now().isoformat()}".encode()
            session_id = hashlib.sha256(session_data).hexdigest()[:6]
        self.session_id = session_id
        
        # Merge project name with hash for unique directory names
        self.project_instance = f"{project_name}_{self.session_id}"
        
        self.working_dir = Path(config['working_dir']) / self.project_instance
        self.output_dir = Path(config['output_dir']) / self.project_instance
        self.backup_dir = Path(config.get('backup_dir')) / self.project_instance
        self.state_file = self.working_dir / '.translation_state.json'
        
        self.max_iterations = config.get('max_iterations', 10)


        self.translator = config.get('translator', 'CC-Sonnet-4.5')
        if self.translator == 'CC-Sonnet-4.5':
            self.translator = translator_cc_sonnet_45
        elif self.translator == 'SWE-Sonnet-4.5':
            self.translator = translator_swe_sonnet_45
        elif self.translator == 'SWE-Sonnet-4':
            self.translator = translator_swe_sonnet_4
        elif self.translator == 'SWE-GPT-5mini':
            self.translator = translator_swe_gpt_5mini
        else:
            assert False, f"[red]Invalid translator: {self.translator}[/red]"

        discriminator = config.get('discriminator', 'CC-Sonnet-4.5-ACToR')
        if discriminator == 'CC-Sonnet-4.5-ACToR':
            self.discriminator = discriminator_cc_sonnet_45_actor
        elif discriminator == 'CC-Sonnet-4.5-ACToR-1_3':
            self.discriminator = discriminator_cc_sonnet_45_actor_1_3
        elif discriminator == 'CC-Sonnet-4.5-ACToR-15_1':
            self.discriminator = discriminator_cc_sonnet_45_actor_15_1
        elif discriminator == 'CC-Sonnet-4.5-ACToR-15_5':
            self.discriminator = discriminator_cc_sonnet_45_actor_15_5
        elif discriminator == 'CC-Sonnet-4.5-ACToR-noFuzz':
            self.discriminator = discriminator_cc_sonnet_45_actor_nofuzz
        elif discriminator == 'CC-Sonnet-4.5-Coverage':
            self.discriminator = discriminator_cc_sonnet_45_coverage
        elif discriminator == 'SWE-Sonnet-4.5-ACToR':
            self.discriminator = discriminator_swe_sonnet_45_actor
        elif discriminator == 'SWE-Sonnet-4-ACToR':
            self.discriminator = discriminator_swe_sonnet_4_actor
        elif discriminator == 'SWE-GPT-5mini-ACToR':
            self.discriminator = discriminator_swe_gpt_5mini_actor
        else:
            assert False, f"[red]Invalid discriminator: {discriminator}[/red]"

        assert self.translator is not None and self.discriminator is not None, "Translator and discriminator must be set"

        # Runtime state
        self.state = {
            'project_name': project_name,
            'project_instance': self.project_instance,
            'session_id': self.session_id,
            'translator': config.get('translator', 'CC-Sonnet-4.5'),
            'discriminator': config.get('discriminator', 'CC-Sonnet-4.5-ACToR'),
            'status': ProjectStatus.QUEUED.value,
            'current_iteration': 0,
            'current_phase': None,
            'history': [],
            'last_updated': None,
            'backups': [],
            'errors': [],
            'start_time': None,
            'end_time': None
        }
        
        # Control flags
        self.should_stop = False
        self.should_pause = False
        
        # Don't load old state - each session is fresh
    
    def initialize_working_directory(self):
        """Initialize or restore working directory from input."""
        self.state['status'] = ProjectStatus.INITIALIZING.value
        self.state['start_time'] = datetime.now().isoformat()
        
        # Create directories
        self.working_dir.mkdir(parents=True, exist_ok=True)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.backup_dir.mkdir(parents=True, exist_ok=True)
        
        assert self.input_dir.exists()

        ### copy all files to sandbox
        work_dir_sandbox = self.working_dir / "sandbox"
        work_dir_sandbox.mkdir(parents=True, exist_ok=True)
        whitelist = white_list_for_copy_c + white_list_for_copy_rs + white_list_for_copy_test_cases
        _copy_directory(self.input_dir, work_dir_sandbox, whitelist=whitelist)
        
        ### copy only c files to c_files
        work_dir_c_files = self.working_dir / "c_files"
        work_dir_c_files.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.input_dir, work_dir_c_files, whitelist=white_list_for_copy_c)

        ### copy only rs files to rs_files
        work_dir_rs_files = self.working_dir / "rs_files"
        work_dir_rs_files.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.input_dir, work_dir_rs_files, whitelist=white_list_for_copy_rs)

        ### copy only test cases to test_cases
        work_dir_test_cases = self.working_dir / "test_cases"
        work_dir_test_cases.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.input_dir, work_dir_test_cases, whitelist=white_list_for_copy_test_cases)

        ### copy log files to log_files
        work_dir_log_files = self.working_dir / "log_files"
        work_dir_log_files.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.input_dir, work_dir_log_files, whitelist=white_list_for_copy_log_files)
        
        self.save_state()
    
    def create_translation_storage(self):
        """Create temporary storage for the translation result."""
        _copy_directory(self.working_dir / "sandbox", self.working_dir / "rs_files", whitelist=white_list_for_copy_rs)
    
    def create_discrimination_storage(self):
        """Create temporary storage for the discrimination result."""
        _copy_directory(self.working_dir / "sandbox", self.working_dir / "test_cases", whitelist=white_list_for_copy_test_cases)
    
    def create_backup(self, iteration_num: int) -> str:
        """
        Create a backup of the current working directory after iteration completes.
        backup/iteration_k saves the state AFTER iteration k finishes.
        """
        backup_name = f"iteration_{iteration_num}"
        backup_path = self.backup_dir / backup_name
        backup_path.mkdir(parents=True, exist_ok=True)

        backup_rs_files = backup_path / "rs_files"
        backup_rs_files.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.working_dir / "rs_files", backup_rs_files, whitelist=white_list_for_copy_rs)

        backup_test_cases = backup_path / "test_cases"
        backup_test_cases.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.working_dir / "test_cases", backup_test_cases, whitelist=white_list_for_copy_test_cases)
        
        backup_log_files = backup_path / "log_files"
        backup_log_files.mkdir(parents=True, exist_ok=True)
        _copy_directory(self.working_dir / "log_files", backup_log_files, whitelist=white_list_for_copy_log_files)
        
        # Record backup
        self.state['backups'].append({
            'name': backup_name,
            'path': str(backup_path),
            'timestamp': datetime.now().isoformat(),
            'iteration': iteration_num,
            'session_id': self.session_id
        })
        self.save_state()
        
        return backup_name
    
    def save_state(self):
        """Save current state to JSON file."""
        self.state['last_updated'] = datetime.now().isoformat()
        self.state_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self.state_file, 'w') as f:
            json.dump(self.state, f, indent=2)
    
    def load_state(self):
        """Load state from JSON file if exists."""
        if self.state_file.exists():
            with open(self.state_file, 'r') as f:
                loaded_state = json.load(f)
                self.state.update(loaded_state)
    
    def update_history(self, event_type: str, details: Dict):
        """Add an event to the history."""
        self.state['history'].append({
            'timestamp': datetime.now().isoformat(),
            'iteration': self.state['current_iteration'],
            'event_type': event_type,
            'details': details
        })
        self.save_state()
    
    def finalize(self):
        """Copy working directory to output directory."""
        whitelist = white_list_for_copy_c + white_list_for_copy_rs + white_list_for_copy_test_cases
        _copy_directory(self.working_dir, self.output_dir, whitelist=whitelist)
        self.state['status'] = ProjectStatus.COMPLETED.value
        self.state['end_time'] = datetime.now().isoformat()
        self.save_state()
    
    def get_elapsed_time(self) -> str:
        """Get elapsed time for the project."""
        if not self.state.get('start_time'):
            return "N/A"
        
        start = datetime.fromisoformat(self.state['start_time'])
        
        if self.state.get('end_time'):
            end = datetime.fromisoformat(self.state['end_time'])
        else:
            end = datetime.now()
        
        elapsed = end - start
        hours, remainder = divmod(int(elapsed.total_seconds()), 3600)
        minutes, seconds = divmod(remainder, 60)
        
        if hours > 0:
            return f"{hours}h {minutes}m {seconds}s"
        elif minutes > 0:
            return f"{minutes}m {seconds}s"
        else:
            return f"{seconds}s"


def discover_projects(input_base_dir: str) -> List[Dict]:
    """
    Discover all projects in the input directory.
    Each subdirectory is considered a project.
    """
    base_path = Path(input_base_dir)
    
    if not base_path.exists():
        return []
    
    ### check if `dangerous_list.json` exists
    dangerous_list_file = './scripts/dangerous.json'
    ignore_list = []
    assert os.path.exists(dangerous_list_file), f"Dangerous list file {dangerous_list_file} does not exist"
    with open(dangerous_list_file, 'r') as f:
        ignore_list = json.load(f).get('ignore_list', [])

    projects = []
    for item in base_path.iterdir():
        if item.is_dir() and not item.name.startswith('.'):
            if item.name in ignore_list:
                continue
            # Count files in project
            file_count = sum(1 for _ in item.rglob('*') if _.is_file())
            
            # Check for README
            has_readme = (item / 'README.md').exists() or (item / 'readme.md').exists()
            
            projects.append({
                'name': item.name,
                'path': str(item),
                'file_count': file_count,
                'has_readme': has_readme
            })
    
    return sorted(projects, key=lambda p: p['name'])


def display_projects_table(projects: List[Dict]) -> Table:
    """Display discovered projects in a rich table."""
    table = Table(title="Discovered Projects", box=box.ROUNDED, show_header=True, header_style="bold magenta")
    
    table.add_column("#", style="cyan", no_wrap=True, justify="right")
    table.add_column("Project Name", style="green")
    table.add_column("Location", style="white")
    table.add_column("Files", justify="right", style="yellow")
    table.add_column("README", justify="center", style="blue")
    
    for i, proj in enumerate(projects, 1):
        table.add_row(
            str(i),
            proj['name'],
            proj['path'],
            str(proj['file_count']),
            "✓" if proj['has_readme'] else "-"
        )
    
    return table


def select_projects_interactive(projects: List[Dict]) -> List[str]:
    """
    Let user interactively select projects to translate.
    Returns list of selected project names.
    """
    console.print("\n[bold cyan]Select Projects to Translate[/bold cyan]\n")
    console.print("Options:")
    console.print("  • Enter project numbers (e.g., '1,3,4' or '1 3 4')")
    console.print("  • Enter project names (e.g., 'project1,project2' or 'project1 project2')")
    console.print("  • Mix numbers and names (e.g., '1 project2 3')")
    console.print("  • Enter 'all' to select all projects")
    console.print("  • Enter 'q' to quit\n")
    
    # Create a mapping of lowercase project names to actual names for case-insensitive matching
    project_name_map = {proj['name'].lower(): proj['name'] for proj in projects}
    
    while True:
        selection = Prompt.ask("Your selection")
        
        if selection.lower() == 'q':
            return []
        
        if selection.lower() == 'all':
            return [proj['name'] for proj in projects]
        
        # Parse selection - handle both comma and space separated
        tokens = selection.replace(',', ' ').split()
        selected = []
        invalid_tokens = []
        
        for token in tokens:
            # Try to parse as number (1-based index)
            try:
                idx = int(token) - 1
                if 0 <= idx < len(projects):
                    selected.append(projects[idx]['name'])
                else:
                    invalid_tokens.append(token)
            except ValueError:
                # Not a number, try to match as project name (case-insensitive)
                token_lower = token.lower()
                if token_lower in project_name_map:
                    selected.append(project_name_map[token_lower])
                else:
                    invalid_tokens.append(token)
        
        if invalid_tokens:
            console.print(f"[red]Invalid selections: {', '.join(invalid_tokens)}[/red]")
            console.print("[yellow]Please check project numbers/names and try again.[/yellow]")
        elif selected:
            # Remove duplicates while preserving order
            selected = list(dict.fromkeys(selected))
            
            # Show confirmation
            console.print(f"\n[green]✓ Selected {len(selected)} project(s):[/green]")
            for name in selected:
                console.print(f"  • {name}")
            
            if Confirm.ask("\nProceed with these projects?", default=True):
                return selected
        else:
            console.print("[red]No valid selections. Please try again.[/red]")


def configure_execution() -> tuple:
    """
    Configure execution parameters (parallel/sequential, max_parallel, etc.).
    Returns (parallel: bool, max_parallel: int, max_iterations: int)
    """
    console.print("\n[bold cyan]Execution Configuration[/bold cyan]\n")

    # Translator selection with table
    translators = [
        ("CC-Sonnet-4.5", "Claude Code + Sonnet-4.5 (default)"),
        ("SWE-Sonnet-4.5", "SWE Agent + Sonnet-4.5"),
        ("SWE-Sonnet-4", "SWE Agent + Sonnet-4"),
        ("SWE-GPT-5mini", "SWE Agent + GPT-5mini"),
    ]
    
    translator_table = Table(title="Available Translators", box=box.ROUNDED)
    translator_table.add_column("#", style="cyan", justify="right", width=3)
    translator_table.add_column("Name", style="green")
    translator_table.add_column("Description", style="white")
    
    for i, (name, desc) in enumerate(translators, 1):
        translator_table.add_row(str(i), name, desc)
    
    console.print(translator_table)
    translator_idx = Prompt.ask("Select translator", default="1")
    try:
        translator_choice = translators[int(translator_idx) - 1][0]
    except (ValueError, IndexError):
        console.print("[yellow]Invalid selection, using default: CC-Sonnet-4.5[/yellow]")
        translator_choice = "CC-Sonnet-4.5"
    
    console.print()
    
    # Discriminator selection with table
    discriminators = [
        ("CC-Sonnet-4.5-ACToR", "Claude Code + ACToR (default, 15 init + 3 new/iter)"),
        ("CC-Sonnet-4.5-ACToR-noFuzz", "Claude Code + ACToR without fuzzing"),
        ("CC-Sonnet-4.5-Coverage", "Claude Code + Coverage baseline"),
        ("CC-Sonnet-4.5-ACToR-1_3", "Claude Code + ACToR (1 init + 3 new/iter)"),
        ("CC-Sonnet-4.5-ACToR-15_1", "Claude Code + ACToR (15 init + 1 new/iter)"),
        ("CC-Sonnet-4.5-ACToR-15_5", "Claude Code + ACToR (15 init + 5 new/iter)"),
        ("SWE-Sonnet-4.5-ACToR", "SWE Agent + ACToR (Sonnet-4.5)"),
        ("SWE-Sonnet-4-ACToR", "SWE Agent + ACToR (Sonnet-4)"),
        ("SWE-GPT-5mini-ACToR", "SWE Agent + ACToR (GPT-5mini)"),
    ]
    
    discriminator_table = Table(title="Available Discriminators", box=box.ROUNDED)
    discriminator_table.add_column("#", style="cyan", justify="right", width=3)
    discriminator_table.add_column("Name", style="green")
    discriminator_table.add_column("Description", style="white")
    
    for i, (name, desc) in enumerate(discriminators, 1):
        discriminator_table.add_row(str(i), name, desc)
    
    console.print(discriminator_table)
    discriminator_idx = Prompt.ask("Select discriminator", default="1")
    try:
        discriminator_choice = discriminators[int(discriminator_idx) - 1][0]
    except (ValueError, IndexError):
        console.print("[yellow]Invalid selection, using default: CC-Sonnet-4.5-ACToR[/yellow]")
        discriminator_choice = "CC-Sonnet-4.5-ACToR"


    # Max iterations
    max_iterations_str = Prompt.ask(
        "Maximum iterations per project",
        default="10"
    )
    try:
        max_iterations = int(max_iterations_str)
        if max_iterations < 1:
            console.print("[yellow]Invalid value, using default: 10[/yellow]")
            max_iterations = 10
    except ValueError:
        console.print("[yellow]Invalid value, using default: 10[/yellow]")
        max_iterations = 10
    
    return translator_choice, discriminator_choice, max_iterations


def show_help():
    """Display help information."""
    help_panel = Panel.fit(
        "[bold cyan]ACToR - C to Rust Translation[/bold cyan]\n\n"
        "[white]Adversarial Agent Collaboration for automated C to Rust translation:[/white]\n"
        "  • Automatic project discovery\n"
        "  • Parallel task execution\n"
        "  • Live monitoring dashboard\n"
        "  • Interactive task control (stop, continue, fork)\n"
        "  • Automatic backups at each iteration\n\n"
        "[yellow]Usage:[/yellow]\n"
        "  [cyan]python scripts/actor.py[/cyan]              # Interactive mode\n"
        "  [cyan]python scripts/actor.py --help[/cyan]       # Show this help\n"
        "  [cyan]python scripts/actor.py -c config.json[/cyan]  # Use config file\n\n"
        "[yellow]Interactive Mode:[/yellow]\n"
        "  1. Configure directories and parallel settings\n"
        "  2. Discovers projects in input directory\n"
        "  3. Select projects to translate with 'add' command\n"
        "  4. Monitor progress in live dashboard\n"
        "  5. Use 'stop', 'continue', 'fork' for task control\n\n"
        "[yellow]Commands:[/yellow]\n"
        "  • [bold]add[/bold]       - Add new translation tasks\n"
        "  • [bold]stop[/bold]      - Stop running tasks\n"
        "  • [bold]continue[/bold]  - Resume stopped tasks\n"
        "  • [bold]fork[/bold]      - Fork tasks with new settings",
        border_style="blue",
        padding=(1, 2)
    )
    console.print(help_panel)


class TranslationServer:
    """
    Unified translation server that manages all translation projects.
    Handles project creation, execution, monitoring, and lifecycle management.
    """
    
    def __init__(self, input_base_dir: str, working_dir: str, backup_dir: str, output_dir: str, max_parallel: int = 10):
        self.input_base_dir = input_base_dir
        self.working_dir = working_dir
        self.backup_dir = backup_dir
        self.output_dir = output_dir

        self.max_parallel = max_parallel
        self.running = True
        
        # Direct project management - no separate manager class needed
        self.managers: Dict[str, WorkManager] = {}
        self.threads: Dict[str, threading.Thread] = {}
        self.lock = threading.Lock()
        
        # Execution control
        self.execution_mode_parallel = True
        self.worker_thread: Optional[threading.Thread] = None
    
    def run_project(self, project_instance: str):
        """
        Run a single project in a thread.
        
        Args:
            project_instance: The unique project instance ID (e.g., "myproject_abc123")
        
        Iteration logic:
        - Iteration 0: Initial translation (no discrimination before)
        - Iteration k (k>0): Discriminate first (find bugs), then translate (fix bugs)
        """
        manager = self.managers.get(project_instance)
        if not manager:
            return
        
        try:
            # Check if this is a restored session (current_iteration > 0 and working_dir already exists with files)
            is_restored = (manager.state['current_iteration'] > 0 and 
                          manager.working_dir.exists() and 
                          any(manager.working_dir.iterdir()))
            
            # Initialize only if not restored
            if not is_restored:
                manager.initialize_working_directory()
            else:
                # For restored sessions, ensure start_time is set if not already
                if not manager.state.get('start_time'):
                    manager.state['start_time'] = datetime.now().isoformat()
                    manager.save_state()
            
            # Iteration 0: Initial translation (only if starting from scratch)
            if manager.state['current_iteration'] == 0:
                # Check control flags
                if manager.should_stop:
                    manager.state['status'] = ProjectStatus.STOPPED.value
                    manager.save_state()
                    return
                
                while manager.should_pause:
                    manager.state['status'] = ProjectStatus.PAUSED.value
                    manager.save_state()
                    time.sleep(1)
                    if manager.should_stop:
                        manager.state['status'] = ProjectStatus.STOPPED.value
                        manager.save_state()
                        return
                
                # Initial translation
                manager.state['status'] = ProjectStatus.TRANSLATING.value
                manager.state['current_phase'] = 'initial_translation'
                manager.save_state()
                
                translation_result = manager.translator(manager.project_name, manager.project_instance, str(manager.working_dir), 0)
                manager.update_history('translate', translation_result)
                
                # Create temporary storage for the translation result
                manager.create_translation_storage()

                # Save backup after iteration 0 completes
                manager.create_backup(0)
                
                # Move to next iteration
                manager.state['current_iteration'] = 1
                manager.save_state()
            
            # Main refinement loop (iterations 1 through max_iterations)
            while manager.state['current_iteration'] <= manager.max_iterations:
                # Check control flags
                if manager.should_stop:
                    manager.state['status'] = ProjectStatus.STOPPED.value
                    manager.save_state()
                    return
                
                while manager.should_pause:
                    manager.state['status'] = ProjectStatus.PAUSED.value
                    manager.save_state()
                    time.sleep(1)
                    if manager.should_stop:
                        manager.state['status'] = ProjectStatus.STOPPED.value
                        manager.save_state()
                        return
                
                iteration = manager.state['current_iteration']
                
                # Step 1: Discrimination (find bugs)
                manager.state['status'] = ProjectStatus.DISCRIMINATING.value
                manager.state['current_phase'] = 'discrimination'
                manager.save_state()
                
                is_bsd = 'BSD' in self.input_base_dir
                discrimination_result = manager.discriminator(manager.project_name, manager.project_instance, str(manager.working_dir), iteration, is_bsd)
                manager.update_history('discriminate', discrimination_result)

                # Create temporary storage for the discrimination result
                manager.create_discrimination_storage()
                
                # Step 2: Translation (fix bugs found in discrimination)
                manager.state['status'] = ProjectStatus.TRANSLATING.value
                manager.state['current_phase'] = 'translation'
                manager.save_state()
                
                translation_result = manager.translator(manager.project_name, manager.project_instance, str(manager.working_dir), iteration)
                manager.update_history('translate', translation_result)

                # Create temporary storage for the translation result
                manager.create_translation_storage()
                
                # Save backup after iteration completes
                manager.create_backup(iteration)
                
                # Move to next iteration
                manager.state['current_iteration'] += 1
                manager.save_state()
            
            # Finalize
            manager.finalize()
            
        except Exception as e:
            manager.state['status'] = ProjectStatus.ERROR.value
            print(f"[ERROR] Failed to run project {project_instance}: {e}")
            manager.state['errors'].append(str(e))
            manager.save_state()
    
    def _run_worker_loop(self):
        """Background worker that continuously manages project execution."""
        while self.running:
            with self.lock:
                # Find projects that need to be started
                projects_to_start = []
                for project_instance, manager in self.managers.items():
                    if manager.state['status'] == ProjectStatus.QUEUED.value:
                        if project_instance not in self.threads or not self.threads[project_instance].is_alive():
                            projects_to_start.append(project_instance)
                
                # Start new projects up to max_parallel
                active_count = sum(1 for t in self.threads.values() if t.is_alive())
                
                for project_instance in projects_to_start:
                    if active_count >= self.max_parallel:
                        break
                    
                    thread = threading.Thread(target=self.run_project, args=(project_instance,))
                    thread.daemon = True
                    thread.start()
                    self.threads[project_instance] = thread
                    active_count += 1
                
                # Clean up finished threads
                finished = [pid for pid, t in self.threads.items() if not t.is_alive()]
                for pid in finished:
                    del self.threads[pid]
            
            time.sleep(0.5)
    
    def start_worker(self):
        """Start the background worker thread if not already running."""
        if not self.worker_thread or not self.worker_thread.is_alive():
            self.worker_thread = threading.Thread(target=self._run_worker_loop)
            self.worker_thread.daemon = True
            self.worker_thread.start()
    
    def stop_project(self, identifier: str):
        """Stop a specific project by instance ID or project name."""
        manager = self._find_project(identifier)
        if manager:
            manager.should_stop = True
            return True
        return False
    
    def _find_project(self, identifier: str) -> Optional[WorkManager]:
        """
        Find a project by instance ID or project name.
        Returns the first matching WorkManager or None.
        """
        # First try exact match on project_instance
        if identifier in self.managers:
            return self.managers[identifier]
        
        # Then try matching by project_name
        for project_instance, manager in self.managers.items():
            if manager.project_name == identifier:
                return manager
        
        return None
    
    def stop_all(self):
        """Stop all projects."""
        for manager in self.managers.values():
            manager.should_stop = True
    
    def get_status_table(self) -> Table:
        """Generate a rich table showing status of all projects with numbered indices."""
        table = Table(title="Translation Projects Status", box=box.ROUNDED, show_header=True, header_style="bold magenta")
        
        table.add_column("#", style="bold white", no_wrap=True, justify="right", width=3)
        table.add_column("Project Name", style="cyan", no_wrap=True)
        table.add_column("Instance ID", style="dim cyan", no_wrap=True)
        table.add_column("Setting", style="white")
        table.add_column("Status", style="yellow")
        table.add_column("Iter", justify="right", style="green")
        table.add_column("Phase", style="blue")
        table.add_column("Elapsed", justify="right", style="white")
        
        for idx, (project_instance, manager) in enumerate(self.managers.items(), 1):
            status = manager.state['status']
            iteration = manager.state['current_iteration']
            phase = manager.state.get('current_phase', 'N/A')
            setting_translator = manager.state.get('translator', 'N/A')
            setting_discriminator = manager.state.get('discriminator', 'N/A')
            elapsed = manager.get_elapsed_time()
            instance_id = manager.session_id
            
            # Color status
            status_style = {
                ProjectStatus.QUEUED.value: "white",
                ProjectStatus.INITIALIZING.value: "cyan",
                ProjectStatus.TRANSLATING.value: "blue",
                ProjectStatus.DISCRIMINATING.value: "magenta",
                ProjectStatus.PAUSED.value: "yellow",
                ProjectStatus.COMPLETED.value: "green",
                ProjectStatus.STOPPED.value: "red",
                ProjectStatus.ERROR.value: "bold red"
            }.get(status, "white")

            # INSERT_YOUR_CODE
            # Change status to 'running' with 'cyan' color if in certain phases
            running_statuses = [
                ProjectStatus.INITIALIZING.value,
                ProjectStatus.TRANSLATING.value,
                ProjectStatus.DISCRIMINATING.value,
            ]
            display_status = status
            display_status_style = status_style
            if status in running_statuses:
                display_status = "running"
                display_status_style = "cyan"

            # # Summarize translator/discriminator to short names
            # translator_short_names = {
            #     'CC-Sonnet-4.5': 'CC-Sonnet-4.5',
            #     'SWE-Sonnet-4.5': 'SWE-Sonnet-4.5',
            #     'SWE-Sonnet-4': 'SWE-Sonnet-4',
            #     'SWE-GPT-5mini': 'SWE-GPT-5mini'
            # }
            # discriminator_short_names = {
            #     'CC-Sonnet-4.5-ACToR': 'CC-Sonnet-4.5-ACToR',
            #     'CC-Sonnet-4.5-ACToR-1_3': 'CC-Sonnet-4.5-ACToR-1_3',
            #     'CC-Sonnet-4.5-ACToR-15_1': 'CC-Sonnet-4.5-ACToR-15_1',
            #     'CC-Sonnet-4.5-ACToR-15_5': 'CC-Sonnet-4.5-ACToR-15_5',
            #     'CC-Sonnet-4.5-ACToR-noFuzz': 'CC-Sonnet-4.5-ACToR-noFuzz',
            #     'CC-Sonnet-4.5-Coverage': 'CC-Sonnet-4.5-Coverage',
            #     'SWE-Sonnet-4.5-ACToR': 'SWE-Sonnet-4.5-ACToR',
            #     'SWE-Sonnet-4-ACToR': 'SWE-Sonnet-4-ACToR',
            #     'SWE-GPT-5mini-ACToR': 'SWE-GPT-5mini-ACToR'
            # }
            # setting_translator = translator_short_names.get(setting_translator, setting_translator)
            # setting_discriminator = discriminator_short_names.get(setting_discriminator, setting_discriminator)
            
            table.add_row(
                str(idx),
                manager.project_name,
                instance_id,
                f"{setting_translator} / {setting_discriminator}",
                f"[{display_status_style}]{display_status}[/{display_status_style}]",
                f"{iteration}/{manager.max_iterations}",
                phase,
                elapsed
            )
        
        return table
    
    def get_project_by_index(self, index: int) -> Optional[WorkManager]:
        """Get project by its display index (1-based)."""
        if index < 1 or index > len(self.managers):
            return None
        project_instance = list(self.managers.keys())[index - 1]
        return self.managers[project_instance]
    
    def get_project_identifier(self, user_input: str) -> Optional[str]:
        """
        Parse user input to get project identifier.
        Accepts: index number, project name, or instance ID.
        Returns the project_instance key or None.
        """
        # Try as index number
        try:
            index = int(user_input)
            manager = self.get_project_by_index(index)
            if manager:
                return manager.project_instance
        except ValueError:
            pass
        
        # Try as exact project_instance match
        if user_input in self.managers:
            return user_input
        
        # Try as project_name match
        for project_instance, manager in self.managers.items():
            if manager.project_name == user_input or manager.session_id == user_input:
                return project_instance
        
        return None
    
    def show_help_screen(self):
        """Display help screen with all available commands."""
        console.clear()
        
        # Header
        console.print(Panel.fit(
            "[bold cyan]ACToR - Help[/bold cyan]\n"
            "[white]Adversarial Agent Collaboration for C to Rust Translation[/white]",
            border_style="blue"
        ))
        console.print()
        
        # Command menu with descriptions
        console.print(Panel(
            "[bold yellow]Available Commands:[/bold yellow]\n\n"
            "[cyan]add[/cyan]              - Add new translation project(s)\n"
            "                   Discovers projects, lets you select and configure execution\n\n"
            "[cyan]stop <#|name>[/cyan]    - Stop a project\n"
            "                   Permanently stops a project\n\n"
            "[cyan]stopall[/cyan]          - Stop all projects\n"
            "                   Stops all running/queued projects\n\n"
            "[cyan]continue[/cyan]         - Continue from a previous session at a specific iteration\n"
            "                   Restore and continue from any iteration backup\n\n"
            "[cyan]fork[/cyan]             - Fork a project\n"
            "                   Continue from a previous session using a new session ID\n\n"
            "[cyan]help[/cyan]             - Show this help\n"
            "                   Display this help screen\n\n"
            "[cyan]exit[/cyan]             - Exit server\n"
            "                   Stop all projects and exit the server",
            title="Commands",
            border_style="green"
        ))
        console.print()
        Prompt.ask("\nPress Enter to continue")
    
    def create_new_project(self):
        """Create and start new translation projects."""
        console.clear()
        console.print("[bold yellow]Create New Translation Project[/bold yellow]\n")
        
        # Discover projects
        discovered_projects = discover_projects(self.input_base_dir)
        
        if not discovered_projects:
            console.print(f"[red]No projects found in '{self.input_base_dir}'![/red]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Display projects
        projects_table = display_projects_table(discovered_projects)
        console.print(projects_table)
        console.print()
        
        # Select projects
        # console.print("[bold cyan]Select Projects[/bold cyan]\n")
        selected_projects = select_projects_interactive(discovered_projects)
        
        if not selected_projects:
            console.print("[yellow]No projects selected.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Configure execution
        console.print("\n[bold cyan]Configure Execution[/bold cyan]")
        translator, discriminator, max_iterations = configure_execution()
        
        ### Add a confirmation prompt here
        console.print("\n[bold cyan]Task Configuration Summary[/bold cyan]")
        console.print(f"\n[bold]Selected Projects:[/bold] {len(selected_projects)}")
        for proj_name in selected_projects:
            console.print(f"  • {proj_name}")
        console.print(f"\n[bold]Translator:[/bold] {translator}")
        console.print(f"[bold]Discriminator:[/bold] {discriminator}")
        console.print(f"[bold]Max Iterations:[/bold] {max_iterations}")
        
        console.print("\n[yellow] This will create new project instances and start processing.[/yellow]")
        confirm = Prompt.ask("\n[bold]Confirm and proceed?[/bold]", choices=["y", "n"], default="y")
        
        if confirm.lower() != 'y':
            console.print("[yellow]Operation cancelled.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
            return

        # Build configurations and add new projects
        new_count = 0
        added_instances = []
        with self.lock:
            for proj_name in selected_projects:
                proj_info = next((p for p in discovered_projects if p['name'] == proj_name), None)
                if proj_info:
                    config = {
                        'project_name': proj_name,
                        'input_dir': proj_info['path'],
                        'output_dir': self.output_dir,
                        'working_dir': self.working_dir,
                        'backup_dir': self.backup_dir,
                        'max_iterations': max_iterations,
                        'translator': translator,
                        'discriminator': discriminator
                    }
                    
                    # Create WorkManager - each gets a unique session_id/hash
                    work_manager = WorkManager(proj_name, config)
                    
                    # Use the unique project_instance as the key (project_name_hash)
                    # This allows multiple instances of the same project
                    self.managers[work_manager.project_instance] = work_manager
                    added_instances.append((work_manager.project_name, work_manager.session_id))
                    new_count += 1
        
        if new_count == 0:
            console.print("[yellow]No new projects added.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Start the background worker if not running
        self.start_worker()
        
        console.print(f"\n[green]✓ {new_count} project(s) added and queued![/green]")
        console.print("\n[cyan]Created instances:[/cyan]")
        for project_name, session_id in added_instances:
            console.print(f"  • {project_name} [dim](Instance: {session_id})[/dim]")
        console.print("\n[dim]Tip: You can create multiple instances of the same project.[/dim]")
        Prompt.ask("\nPress Enter to continue")
    
    
    def list_previous_sessions(self):
        """List all previous translation sessions."""
        console.clear()
        console.print("[bold cyan]Previous Translation Sessions[/bold cyan]\n")
        
        sessions = self._discover_sessions()
        
        if not sessions:
            console.print("[yellow]No previous sessions found.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        table = Table(title="Available Sessions", box=box.ROUNDED)
        table.add_column("#", style="cyan", justify="right")
        table.add_column("Project", style="green")
        table.add_column("Instance", style="yellow")
        table.add_column("Iterations", style="magenta", justify="right")
        table.add_column("Status", style="white")
        table.add_column("Last Updated", style="dim white")
        
        for i, session in enumerate(sessions, 1):
            table.add_row(
                str(i),
                session['project_name'],
                session['session_id'],
                str(session['max_iteration']),
                session['status'],
                session['last_updated']
            )
        
        console.print(table)
        Prompt.ask("\nPress Enter to continue")
    
    
    def clean_old_sessions(self):
        """Clean up old session directories."""
        console.clear()
        console.print("[bold yellow]Clean Old Sessions[/bold yellow]\n")
        
        # Count sessions
        working_sessions = list(Path(self.working_dir).glob('*_*')) if Path(self.working_dir).exists() else []
        backup_sessions = list(Path(self.backup_dir).glob('*_*')) if Path(self.backup_dir).exists() else []
        output_sessions = list(Path(self.output_dir).glob('*_*')) if Path(self.output_dir).exists() else []
        
        console.print(f"Working directories: {len(working_sessions)}")
        console.print(f"Backup directories: {len(backup_sessions)}")
        console.print(f"Output directories: {len(output_sessions)}")
        console.print()
        
        if Confirm.ask("Clean all old sessions? [bold red]This cannot be undone![/bold red]"):
            for path in working_sessions + backup_sessions + output_sessions:
                if path.is_dir():
                    shutil.rmtree(path)
            console.print("[green]✓ Cleaned[/green]")
        else:
            console.print("[yellow]Cancelled[/yellow]")
        
        Prompt.ask("\nPress Enter to continue")
    
    def _discover_sessions(self) -> List[Dict]:
        """Discover all previous sessions from working directories."""
        sessions = []
        
        working_dir = Path(self.working_dir)
        if not working_dir.exists():
            return sessions
        
        for project_dir in working_dir.iterdir():
            if not project_dir.is_dir():
                continue
            
            state_file = project_dir / '.translation_state.json'
            if not state_file.exists():
                continue
            
            try:
                with open(state_file, 'r') as f:
                    state = json.load(f)
                    
                    # Find max iteration from backups
                    backup_dir = Path(self.backup_dir) / project_dir.name
                    max_iter = -1
                    available_backups = []
                    if backup_dir.exists():
                        for backup in backup_dir.iterdir():
                            if backup.name.startswith('iteration_'):
                                try:
                                    iter_num = int(backup.name.split('_')[1])
                                    max_iter = max(max_iter, iter_num)
                                    available_backups.append(iter_num)
                                except:
                                    pass
                    
                    sessions.append({
                        'project_name': state.get('project_name', 'unknown'),
                        'session_id': state.get('session_id', 'unknown'),
                        'translator': state.get('translator', 'unknown'),
                        'discriminator': state.get('discriminator', 'unknown'),
                        'instance_name': project_dir.name,
                        'status': state.get('status', 'unknown'),
                        'current_iteration': state.get('current_iteration', 0),
                        'max_iteration': max_iter,
                        'available_backups': sorted(available_backups),
                        'last_updated': state.get('last_updated', 'unknown')[:19] if state.get('last_updated') else 'N/A',
                        'backup_dir': backup_dir,
                        'working_dir': project_dir,
                        'max_iterations': state.get('max_iterations', 10)
                    })
            except:
                continue
        
        return sorted(sessions, key=lambda s: s['last_updated'], reverse=True)
    
    def _get_session_backups(self, session: Dict) -> List[int]:
        """Get list of available backup iterations for a session."""
        return session.get('available_backups', [])
    
    def continue_from_iteration(self, cmd: str):
        """Continue a previous task from a specific iteration."""
        console.clear()
        console.print("[bold cyan]Continue from Previous Session[/bold cyan]\n")
        
        # Step 1: Discover all sessions
        sessions = self._discover_sessions()
        
        if not sessions:
            console.print("[yellow]No previous sessions found.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Step 2: Display sessions with their available iterations
        table = Table(title="Available Sessions to Continue", box=box.ROUNDED)
        table.add_column("#", style="cyan", justify="right", width=3)
        table.add_column("Project", style="green")
        table.add_column("Instance", style="yellow")
        table.add_column("Setting", style="blue")
        table.add_column("Status", style="white")
        table.add_column("Max Iteration", style="magenta", justify="right")
        table.add_column("Available Backups", style="dim white")
        table.add_column("Last Updated", style="dim white")
        
        for i, session in enumerate(sessions, 1):
            backups_str = ', '.join(str(b) for b in session.get('available_backups', []))
            if not backups_str:
                backups_str = "None"
            
            table.add_row(
                str(i),
                session['project_name'],
                session['session_id'],
                f"Trans: {session['translator']} Disc: {session['discriminator']}",
                session['status'],
                str(session['max_iteration']) if session['max_iteration'] >= 0 else "N/A",
                backups_str,
                session['last_updated']
            )
        
        console.print(table)
        console.print()
        
        # Step 3: Select a session
        console.print("[white]Select a session to continue from (or 'q' to cancel):[/white]")
        session_choice = Prompt.ask("Session number")
        
        if session_choice.lower() == 'q':
            return
        
        try:
            session_idx = int(session_choice) - 1
            if session_idx < 0 or session_idx >= len(sessions):
                console.print("[red]Invalid session number[/red]")
                Prompt.ask("\nPress Enter to continue")
                return
        except ValueError:
            console.print("[red]Invalid input[/red]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        selected_session = sessions[session_idx]
        available_backups = selected_session.get('available_backups', [])
        
        if not available_backups:
            console.print("[red]No backups available for this session![/red]")
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Step 4: Select iteration to continue from
        console.print(f"\n[cyan]Selected: {selected_session['project_name']} ({selected_session['session_id']})[/cyan]")
        console.print(f"[white]Available iteration backups: {', '.join(str(b) for b in available_backups)}[/white]\n")
        console.print("[yellow]Note:[/yellow] If you select iteration N, the new task will restore the state")
        console.print("       after iteration N - 1 and continue from iteration N.\n")

        max_next_iteration = max(available_backups) + 1
        available_backups.append(max_next_iteration)
        iteration_choice = Prompt.ask(f"Continue from iteration", choices=[str(b) for b in available_backups])
        
        try:
            continue_from_iter = int(iteration_choice) - 1
            assert continue_from_iter in available_backups, "[red]Invalid iteration number[/red]"
        except Exception as e:
            console.print("[red]Invalid iteration number[/red]", available_backups)
            Prompt.ask("\nPress Enter to continue")
            return
        
        # Step 5: Configure the new task
        console.print(f"\n[bold green]Creating new task to continue from iteration {continue_from_iter}[/bold green]")
        
        # Step 6: Create new project from backup
        try:
            # Find the original input directory
            original_input_path = Path(self.input_base_dir) / selected_session['project_name']
            if not original_input_path.exists():
                console.print(f"[red]Error: Original input directory not found: {original_input_path}[/red]")
                Prompt.ask("\nPress Enter to continue")
                return
            
            # Create new WorkManager
            if cmd == 'continue':
                # Continue: keep original settings, only ask for max_iterations
                console.print(f"\n[cyan]Original translator: {selected_session['translator']}[/cyan]")
                console.print(f"[cyan]Original discriminator: {selected_session['discriminator']}[/cyan]\n")
                
                max_iterations_str = Prompt.ask(
                    "Maximum iterations for new task",
                    default=str(selected_session.get('max_iterations', 10))
                )
                try:
                    max_iterations = int(max_iterations_str)
                    if max_iterations < 1:
                        max_iterations = 10
                except ValueError:
                    max_iterations = 10
                
                # Create config with original translator/discriminator
                config = {
                    'project_name': selected_session['project_name'],
                    'input_dir': str(original_input_path),
                    'output_dir': self.output_dir,
                    'working_dir': self.working_dir,
                    'backup_dir': self.backup_dir,
                    'max_iterations': max_iterations,
                    'translator': selected_session['translator'],
                    'discriminator': selected_session['discriminator']
                }

                # Confirm
                console.print(f"\n[yellow]Summary:[/yellow]")
                console.print(f"  Project: {selected_session['project_name']}")
                console.print(f"  Restore from: Iteration {continue_from_iter}")
                console.print(f"  Next iteration: Iteration {continue_from_iter + 1}")
                console.print(f"  Max iterations: {max_iterations}")
                console.print(f"  Translator: {selected_session['translator']}")
                console.print(f"  Discriminator: {selected_session['discriminator']}")
                console.print()
                
                if not Confirm.ask("Create this task?", default=True):
                    console.print("[yellow]Cancelled[/yellow]")
                    Prompt.ask("\nPress Enter to continue")
                    return
                # Reuse the session ID of the selected session
                new_manager = WorkManager(selected_session['project_name'], config, session_id=selected_session['session_id'])
            else:
                assert cmd == 'fork', "[red]Invalid command[/red]"

                # Fork: get new settings via configure_execution
                console.print(f"\n[cyan]Original translator: {selected_session['translator']}[/cyan]")
                console.print(f"[cyan]Original discriminator: {selected_session['discriminator']}[/cyan]\n")
                console.print("[white]Configure new settings for the forked task:[/white]\n")
                
                fork_translator, fork_discriminator, max_iterations = configure_execution()

                # Create config for new project
                config = {
                    'project_name': selected_session['project_name'],
                    'input_dir': str(original_input_path),
                    'output_dir': self.output_dir,
                    'working_dir': self.working_dir,
                    'backup_dir': self.backup_dir,
                    'max_iterations': max_iterations,
                    'translator': fork_translator,
                    'discriminator': fork_discriminator
                }

                # Confirm
                console.print(f"\n[yellow]Summary:[/yellow]")
                console.print(f"  Project: {selected_session['project_name']}")
                console.print(f"  Restore from: Iteration {continue_from_iter}")
                console.print(f"  Next iteration: Iteration {continue_from_iter + 1}")
                console.print(f"  Max iterations: {max_iterations}")
                console.print(f"  Translator: {fork_translator}")
                console.print(f"  Discriminator: {fork_discriminator}")
                console.print()
                
                if not Confirm.ask("Create this task?", default=True):
                    console.print("[yellow]Cancelled[/yellow]")
                    Prompt.ask("\nPress Enter to continue")
                    return

                # by using a new session ID
                new_manager = WorkManager(selected_session['project_name'], config)
            
            # Initialize directories (creates working_dir, output_dir, backup_dir)
            new_manager.working_dir.mkdir(parents=True, exist_ok=True)
            new_manager.output_dir.mkdir(parents=True, exist_ok=True)
            new_manager.backup_dir.mkdir(parents=True, exist_ok=True)
            
            # Restore from selected backup
            backup_source = selected_session['backup_dir'] / f"iteration_{continue_from_iter}"
            if not backup_source.exists():
                console.print(f"[red]Error: Backup not found: {backup_source}[/red]")
                Prompt.ask("\nPress Enter to continue")
                return
            
            console.print(f"[cyan]Restoring from backup: {backup_source}[/cyan]")

            _copy_directory(original_input_path, new_manager.working_dir / "c_files", whitelist=white_list_for_copy_c)
            _copy_directory(backup_source / "rs_files", new_manager.working_dir / "rs_files", whitelist=white_list_for_copy_rs)
            _copy_directory(backup_source / "test_cases", new_manager.working_dir / "test_cases", whitelist=white_list_for_copy_test_cases)
            _copy_directory(backup_source / "log_files", new_manager.working_dir / "log_files", whitelist=white_list_for_copy_log_files)

            # copy all to the sandbox directory
            _copy_directory(new_manager.working_dir / "rs_files", new_manager.working_dir / "sandbox", whitelist=white_list_for_copy_rs)
            _copy_directory(new_manager.working_dir / "test_cases", new_manager.working_dir / "sandbox", whitelist=white_list_for_copy_test_cases)
            _copy_directory(new_manager.working_dir / "c_files", new_manager.working_dir / "sandbox", whitelist=white_list_for_copy_c)
            
            # Set initial state to continue from the next iteration
            new_manager.state['status'] = ProjectStatus.QUEUED.value
            new_manager.state['current_iteration'] = continue_from_iter + 1
            new_manager.state['current_phase'] = None
            new_manager.state['start_time'] = datetime.now().isoformat()
            new_manager.state['history'].append({
                'timestamp': datetime.now().isoformat(),
                'iteration': continue_from_iter,
                'event_type': 'restored_from_backup',
                'details': {
                    'message': f'Restored from session {selected_session["session_id"]} iteration {continue_from_iter}',
                    'source_session': selected_session['session_id'],
                    'source_iteration': continue_from_iter
                }
            })
            new_manager.save_state()
            
            # Add to server's managed projects
            with self.lock:
                self.managers[new_manager.project_instance] = new_manager
            
            # Start worker if not running
            self.start_worker()
            
            console.print(f"\n[green]✓ Task created successfully![/green]")
            console.print(f"[cyan]New instance: {new_manager.project_name} ({new_manager.session_id})[/cyan]")
            console.print(f"[white]Will continue from iteration {continue_from_iter + 1}[/white]")
            Prompt.ask("\nPress Enter to continue")
            
        except Exception as e:
            console.print(f"[red]Error creating task: {e}[/red]")
            import traceback
            console.print(f"[dim]{traceback.format_exc()}[/dim]")
            Prompt.ask("\nPress Enter to continue")
    
    def handle_command(self, command: str):
        """Handle user command in unified UI."""
        parts = command.strip().split(maxsplit=1)
        if not parts:
            return
        
        cmd = parts[0].lower()
        arg = parts[1] if len(parts) > 1 else None
        
        if cmd == 'add':
            self.create_new_project()
        
        elif cmd == 'stop':
            if not arg:
                console.print("[yellow]Usage: stop <#|name>[/yellow]")
                Prompt.ask("\nPress Enter to continue")
                return
            
            project_instance = self.get_project_identifier(arg)
            if project_instance and project_instance in self.managers:
                self.managers[project_instance].should_stop = True
                manager = self.managers[project_instance]
                console.print(f"[green]✓ Stopped: {manager.project_name} ({manager.session_id})[/green]")
            else:
                console.print(f"[red]✗ Project '{arg}' not found[/red]")
        
        elif cmd == 'stopall':
            if self.managers and Confirm.ask("[bold red]Stop all projects?[/bold red]"):
                self.stop_all()
                console.print("[green]✓ All projects stopped[/green]")
            elif not self.managers:
                console.print("[yellow]No projects to stop[/yellow]")
        
        elif cmd == 'continue' or cmd == 'fork':
            self.continue_from_iteration(cmd)
        
        elif cmd == 'help':
            self.show_help_screen()
        
        elif cmd == 'exit':
            if Confirm.ask("[bold yellow]Exit server (All the tasks will be stopped)?[/bold yellow]"):
                if self.managers:
                    self.stop_all()
                console.print("\n[cyan]Translation Server stopped.[/cyan]")
                self.running = False
        
        else:
            console.print(f"[yellow]Unknown command: '{cmd}'. Type 'help' for available commands.[/yellow]")
            Prompt.ask("\nPress Enter to continue")
    
    def run(self):
        """Main server loop with improved UI."""
        # Show initial UI
        console.clear()
        console.print(Panel.fit(
            "[bold cyan]ACToR - C to Rust Translation[/bold cyan]\n"
            "[white]Adversarial Agent Collaboration System[/white]",
            border_style="blue"
        ))
        console.print()
        
        # Show command menu once
        console.print(Panel(
            "[bold yellow]Available Commands:[/bold yellow]\n\n"
            "[cyan]add[/cyan]              - Add new translation project(s)\n"
            "[cyan]stop <#|name>[/cyan]    - Stop a project\n"
            "[cyan]stopall[/cyan]          - Stop all projects\n"
            "[cyan]continue[/cyan]         - Continue from a previous session at a specific iteration with same session ID\n"
            "[cyan]fork[/cyan]             - Continue from a previous session using a new session ID\n"
            "[cyan]help[/cyan]             - Show this help\n"
            "[cyan]exit[/cyan]             - Exit server",
            title="Commands",
            border_style="green"
        ))
        console.print()
        
        last_refresh = time.time()
        refresh_interval = 3  # Refresh status every 3 seconds
        
        while self.running:
            # Show current status
            current_time = time.time()
            if current_time - last_refresh >= refresh_interval:
                self._display_status()
                last_refresh = current_time
            else:
                # First display
                if last_refresh == current_time:
                    self._display_status()
            
            # Get user input
            try:
                user_input = Prompt.ask("\n[dim]Command[/dim]").strip()
                
                if user_input:
                    self.handle_command(user_input)
                    last_refresh = 0  # Force refresh after command
                    
            except KeyboardInterrupt:
                console.print("\n[yellow]Use 'exit' command to quit[/yellow]")
                continue
            except EOFError:
                break
    
    def _display_status(self):
        """Display current status of all projects."""
        console.print()
        console.rule("[cyan]Current Status[/cyan]")
        
        if self.managers:
            console.print(self.get_status_table())
        else:
            console.print("[dim]No projects running. Use 'add' or 'continue' to start.[/dim]")
        
        console.print()



def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description='ACToR - Adversarial Agent Collaboration for C to Rust Translation',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        add_help=False
    )
    
    parser.add_argument('--help', '-h', action='store_true',
                       help='Show help message')
    
    parser.add_argument('--version', '-v', action='store_true',
                       help='Show version information')
    
    parser.add_argument('--config', '-c',
                       help='JSON configuration file')
    
    return parser.parse_args()


def show_opening_screen():
    """Display the opening screen with logo and configuration prompts."""
    # Show ASCII logo
    console.print(f"[bold cyan]{ACTOR_LOGO}[/bold cyan]")
    console.print()
    console.print(f"[bold white]Adversarial Agent Collaboration for C to Rust Translation[/bold white]  [dim]v{VERSION}[/dim]")
    console.print("[dim]Automated program translation with parallel processing and live monitoring[/dim]")
    console.print()
    console.print("─" * 60)
    console.print()
    
    # Prompt for configuration
    console.print("[bold yellow]Configuration Setup[/bold yellow]")
    console.print("[dim]Press Enter to use default values shown in brackets[/dim]")
    console.print()
    
    config = {}
    
    # Max parallel
    default_val = DEFAULT_CONFIG["max_parallel"]
    value = Prompt.ask(
        f"  [cyan]Max parallel tasks[/cyan]",
        default=str(default_val)
    )
    config["max_parallel"] = int(value)
    
    # Input directory
    default_val = DEFAULT_CONFIG["input_directory"]
    config["input_directory"] = Prompt.ask(
        f"  [cyan]Input directory[/cyan]",
        default=default_val
    )
    
    # Working directory
    default_val = DEFAULT_CONFIG["working_directory"]
    config["working_directory"] = Prompt.ask(
        f"  [cyan]Working directory[/cyan]",
        default=default_val
    )
    
    # Backups directory
    default_val = DEFAULT_CONFIG["backups_directory"]
    config["backups_directory"] = Prompt.ask(
        f"  [cyan]Backups directory[/cyan]",
        default=default_val
    )
    
    # Output directory
    default_val = DEFAULT_CONFIG["output_directory"]
    config["output_directory"] = Prompt.ask(
        f"  [cyan]Output directory[/cyan]",
        default=default_val
    )
    
    console.print()
    
    # Show configuration summary
    config_table = Table(title="Configuration Summary", box=box.ROUNDED)
    config_table.add_column("Setting", style="cyan")
    config_table.add_column("Value", style="white")
    
    config_table.add_row("Max Parallel", str(config["max_parallel"]))
    config_table.add_row("Input Directory", config["input_directory"])
    config_table.add_row("Working Directory", config["working_directory"])
    config_table.add_row("Backups Directory", config["backups_directory"])
    config_table.add_row("Output Directory", config["output_directory"])
    
    console.print(config_table)
    console.print()
    
    if not Confirm.ask("Start ACToR with this configuration?", default=True):
        console.print("[yellow]Cancelled.[/yellow]")
        return None
    
    return config
    
def main():
    """Main entry point - starts the translation server."""
    args = parse_args()
    
    # Show help if requested
    if args.help:
        show_help()
        return
    
    # Show version if requested
    if args.version:
        console.print(f"[bold cyan]ACToR[/bold cyan] version [bold white]{VERSION}[/bold white]")
        console.print("[dim]Adversarial Agent Collaboration for C to Rust Translation[/dim]")
        return
    
    # Config file mode
    if args.config:
        console.print(f"[bold cyan]{ACTOR_LOGO}[/bold cyan]")
        console.print(f"[dim]v{VERSION}[/dim]")
        console.print()
        console.print(f"[cyan]Loading configuration from {args.config}...[/cyan]\n")
        try:
            with open(args.config, 'r') as f:
                config_data = json.load(f)
            
            # Extract configuration
            max_parallel = config_data.get('max_parallel', DEFAULT_CONFIG['max_parallel'])
            input_dir = config_data.get('input_directory', DEFAULT_CONFIG['input_directory'])
            working_dir = config_data.get('working_directory', DEFAULT_CONFIG['working_directory'])
            backup_dir = config_data.get('backups_directory', DEFAULT_CONFIG['backups_directory'])
            output_dir = config_data.get('output_directory', DEFAULT_CONFIG['output_directory'])
            
            # Show configuration summary
            config_table = Table(title="Configuration from File", box=box.ROUNDED)
            config_table.add_column("Setting", style="cyan")
            config_table.add_column("Value", style="white")
            
            config_table.add_row("Max Parallel", str(max_parallel))
            config_table.add_row("Input Directory", input_dir)
            config_table.add_row("Working Directory", working_dir)
            config_table.add_row("Backups Directory", backup_dir)
            config_table.add_row("Output Directory", output_dir)
            
            console.print(config_table)
            console.print()
            
            if not Confirm.ask("Start ACToR with this configuration?", default=True):
                console.print("[yellow]Cancelled.[/yellow]")
                return
            
            # Start translation server with config file settings
            server = TranslationServer(input_dir, working_dir, backup_dir, output_dir, max_parallel)
            server.run()
            
        except FileNotFoundError:
            console.print(f"[red]Error: Config file '{args.config}' not found![/red]")
        except json.JSONDecodeError as e:
            console.print(f"[red]Error: Invalid JSON in config file: {e}[/red]")
        except Exception as e:
            console.print(f"[red]Error loading config: {e}[/red]")
        return
    
    # Interactive mode - show opening screen and prompt for configuration
    config = show_opening_screen()
    if config is None:
        return
    
    # Start translation server with user configuration
    server = TranslationServer(
        config["input_directory"],
        config["working_directory"],
        config["backups_directory"],
        config["output_directory"],
        config["max_parallel"]
    )
    server.run()


if __name__ == "__main__":
    main()
