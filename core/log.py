from rich.console import Console, JustifyMethod, OverflowMethod
from rich.style import Style
from rich.theme import Theme

custom_theme = Theme(
    {
        "debug": "bright_black",
        "info": "",
        "warning": "orange1",
        "error": "bold red",
        "critical": "bold white on red",
    }
)

console = Console(theme=custom_theme)


def info(*msg, sep: str = " ", end: str = "\n"):
    console.print(*msg, sep=sep, end=end, style="info")


def debug(*msg, sep: str = " ", end: str = "\n"):
    console.print(*msg, sep=sep, end=end, style="debug")


def warning(*msg, sep: str = " ", end: str = "\n"):
    console.print(*msg, sep=sep, end=end, style="warning")


def error(*msg, sep: str = " ", end: str = "\n"):
    console.print(*msg, sep=sep, end=end, style="error")


def critical(*msg, sep: str = " ", end: str = "\n"):
    console.print(*msg, sep=sep, end=end, style="critical")
