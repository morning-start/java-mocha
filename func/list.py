import platform
from pathlib import Path
from typing import Optional

import typer
from typing_extensions import Annotated

from api import Distribution, Foojay, OperatingSystem
from api.type import Architecture, SupportTerm, enum2val
from func.config import set_config
from func.sync import sync_data
from utils.file_utils import load_json, mk_sure, save_json
from utils.json_handler import JSONDataHandler
