from typing import Any, Callable, Dict, List, Literal, Optional

from .file_utils import load_json


def get_version_part(
    version_str: str, part: Literal["major", "minor", "patch"] = "major"
) -> str:
    """
    从版本号字符串中提取 major 级别版本号
    """
    # 移除可能的后缀如 -ea, + 等
    version_str = version_str.split("-")[0].split("+")[0]

    version_levels = ["major", "minor", "patch"]

    idx = version_levels.index(part)
    return version_str.split(".")[idx]


class JSONDataHandler:
    def __init__(self, data: list):
        self.data = data

    @classmethod
    def load_data(cls, file_path: str):
        """加载 JSON 文件数据"""
        data = load_json(file_name=file_path)
        return cls(data)

    def query(self, key: Optional[str] = None, value: Optional[Any] = None):
        """
        基础查询功能，展示所有符合条件的数据
        """
        if key is None or value is None:
            return self
        return JSONDataHandler([item for item in self.data if item.get(key) == value])

    def filter(self, condition: Callable[[Dict[str, Any]], bool]):
        """过滤功能"""
        return JSONDataHandler([item for item in self.data if condition(item)])

    def group_by(self, key: str):
        """分组汇总功能"""
        grouped_data = {}
        for item in self.data:
            group_key = item.get(key)
            if group_key not in grouped_data:
                grouped_data[group_key] = []
            grouped_data[group_key].append(item)
        return JSONDataHandler(grouped_data)

    def get_specific_fields(self, fields: List[str]):
        """
        获取数据中指定字段的信息
        """

        result = []
        for item in self.data:
            new_item = {k: v for k, v in item.items() if k in fields}
            result.append(new_item)
        return JSONDataHandler(result)
