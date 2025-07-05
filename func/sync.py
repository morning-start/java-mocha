from pathlib import Path

from core import Foojay
from core.type import Architecture, OperatingSystem
from core.utils import save_json


def sync_data(data: Path):

    foojay = Foojay()
    distributions = foojay.search_distributions()
    save_json(data / "distributions.json", distributions)

    #  javafx_bundled=True,
    packages = foojay.search_packages(
        operating_system=OperatingSystem.get_local_os(),
        architecture=Architecture.get_local_architecture(),
        javafx_bundled=False,
        package_type="jdk",
    )
    save_json(data / "packages.json", packages)

    versions = foojay.search_versions()
    save_json(data / "versions.json", versions)
