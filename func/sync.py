from pathlib import Path

from rich.progress import Progress, SpinnerColumn, TextColumn

from core import Foojay
from core.type import Architecture, DataFile, Distribution, OperatingSystem
from core.utils import save_json

SUPPORTED_PUBLISHER = [Distribution.ORACLE, Distribution.ZULU]


def sync_data(data_dir: Path, proxies: dict = None):
    foojay = Foojay(proxies)
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        transient=True,
    ) as progress:
        progress.add_task(description="Sync distributions...")
        distributions = foojay.search_distributions()
        save_json(data_dir / DataFile.PUBLISHERS.value, distributions)

        progress.add_task(description="Sync packages...")
        packages = foojay.search_packages(
            operating_system=OperatingSystem.get_local_os(),
            architecture=Architecture.get_local_architecture(),
            distribution=SUPPORTED_PUBLISHER,
            javafx_bundled=False,
            package_type="jdk",
        )
        save_json(data_dir / DataFile.PACKAGES.value, packages)

        progress.add_task(description="Sync versions...")
        versions = foojay.search_versions()
        save_json(data_dir / DataFile.VERSIONS.value, versions)
