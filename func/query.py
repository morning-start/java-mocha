from pathlib import Path
from typing import List

from core import JSONDataHandler
from core.handler import ItemType, JSONType, get_version_part
from core.type import DataFile


def query_publisher_major(data_dir: Path, publisher: str):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PUBLISHERS.value)
