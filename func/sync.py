from pathlib import Path

from rich.progress import Progress, SpinnerColumn, TextColumn

from core import Foojay
from core.type import Architecture, ArchiveType, DataFile, Distribution, OperatingSystem
from core.utils import save_json

SUPPORTED_PUBLISHER = [Distribution.ORACLE]
SUPPORTED_ARCHIVE_TYPE = [ArchiveType.TAR_GZ, ArchiveType.ZIP]


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
            archive_type=SUPPORTED_ARCHIVE_TYPE,
            distribution=SUPPORTED_PUBLISHER,
            javafx_bundled=False,
            package_type="jdk",
        )
        save_json(data_dir / DataFile.PACKAGES.value, packages)

        progress.add_task(description="Sync versions...")
        versions = foojay.search_versions()
        save_json(data_dir / DataFile.VERSIONS.value, versions)
