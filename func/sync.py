from pathlib import Path

from rich.progress import Progress, SpinnerColumn, TextColumn

from core import Foojay
from core.type import Architecture, OperatingSystem
from core.utils import save_json


def sync_data(jvm_root: Path):
    data_path = jvm_root / "data"
    foojay = Foojay()
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        transient=True,
    ) as progress:
        progress.add_task(description="Sync distributions...")
        distributions = foojay.search_distributions()
        save_json(data_path / "distributions.json", distributions)

        progress.add_task(description="Sync packages...")
        packages = foojay.search_packages(
            operating_system=OperatingSystem.get_local_os(),
            architecture=Architecture.get_local_architecture(),
            javafx_bundled=False,
            package_type="jdk",
        )
        save_json(data_path / "packages.json", packages)

        progress.add_task(description="Sync versions...")
        versions = foojay.search_versions()
        save_json(data_path / "versions.json", versions)
