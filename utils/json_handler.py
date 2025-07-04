import json
from typing import Any, Callable, Dict, List, Optional


class JSONDataHandler:
    def __init__(self, file_path: str):
        self.file_path = file_path
        self.data = self._load_data()

    def _load_data(self) -> List[Dict[str, Any]]:
        """加载 JSON 文件数据"""
        with open(self.file_path, "r", encoding="utf-8") as f:
            return json.load(f)

    def query(
        self, key: Optional[str] = None, value: Optional[Any] = None
    ) -> List[Dict[str, Any]]:
        """
        基础查询功能，展示所有符合条件的数据
        """
        if key is None or value is None:
            return self.data
        return [item for item in self.data if item.get(key) == value]

    def filter(
        self, condition: Callable[[Dict[str, Any]], bool]
    ) -> List[Dict[str, Any]]:
        """过滤功能"""
        return [item for item in self.data if condition(item)]

    def group_by(self, key: str) -> Dict[Any, List[Dict[str, Any]]]:
        """分组汇总功能"""
        grouped_data = {}
        for item in self.data:
            group_key = item.get(key)
            if group_key not in grouped_data:
                grouped_data[group_key] = []
            grouped_data[group_key].append(item)
        return grouped_data
