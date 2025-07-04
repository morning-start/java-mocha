import json
from pathlib import Path

from api import Foojay


def save_json(file_name, json_data):
    with open(file_name, "w") as f:
        json.dump(json_data, f, indent=4)


def load_json(file_name) -> list:
    return json.load(open(file_name))


Path("data").mkdir(parents=True, exist_ok=True)
foojay = Foojay()
distributions = foojay.search_distributions()
save_json("data/distributions.json", distributions)
