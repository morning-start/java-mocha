import platform

from api import Distribution, Foojay, OperatingSystem
from api.type import Architecture, SupportTerm, enum2val
from utils import load_json, mk_sure, save_json


def sync_data():
    data = mk_sure("data")
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


if __name__ == "__main__":
    # sync_data()
    data = mk_sure("data")
