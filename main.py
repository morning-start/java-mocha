from api import Foojay
from utils import load_json, mk_sure, save_json

mk_sure("data")
foojay = Foojay()
distributions = foojay.search_distributions()
save_json("data/distributions.json", distributions)
