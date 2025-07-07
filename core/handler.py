from typing import Callable, Dict, List, Optional

from .utils import load_json

EleType = str | int | bool | list | dict
ItemType = Dict[str, EleType]
JSONType = List[ItemType]


class DocumentHandler:
    def __init__(self, doc: JSONType):
        self.document = doc

    @classmethod
    def load_data(cls, file_path: str):
        """加载 JSON 文件数据"""
        doc = load_json(file_name=file_path)
        return cls(doc)

    def query(self, key: Optional[str] = None, value: Optional[EleType] = None):
        """
        基础查询功能，展示所有符合条件的数据
        """
        if key is None or value is None:
            return self
        return DocumentHandler(
            [item for item in self.document if item.get(key) == value]
        )

    def rename(self, name_map: dict):
        """
        重命名字段
        """
        for item in self.document:
            for old_name, new_name in name_map.items():
                if old_name in item:
                    item[new_name] = item.pop(old_name)

    def orderby(self, levels: list[str]):
        # self.document 是一个list，其中每一dict为Item
        # 每个item的字段，按照levles排序
        new_doc = []
        for item in self.document:
            new_item = {}
            for level in levels:
                if level in item:
                    new_item[level] = item[level]
            new_doc.append(new_item)
        self.document = new_doc

    def sort(self, key: str, reverse: bool = False):
        """排序功能"""
        self.document.sort(key=lambda x: x[key], reverse=reverse)

    def map(self, func: Callable[[ItemType], ItemType]):
        """
        对每个Item应用函数
        """
        self.document = [func(item) for item in self.document]

    def apply(self, key: str, func: Callable[[EleType], EleType]):
        """
        对指定ele应用函数
        """
        for item in self.document:
            item[key] = func(item[key])

    def filter(self, condition: Callable[[ItemType], bool]):
        """过滤功能"""
        return DocumentHandler([item for item in self.document if condition(item)])

    def get_specific_fields(self, fields: List[str]):
        """
        获取数据中指定字段的信息
        """

        result = []
        for item in self.document:
            new_item = {k: v for k, v in item.items() if k in fields}
            result.append(new_item)
        return DocumentHandler(result)

    def group_by(
        self,
        key: str,
        agg_map: (
            Dict[str, Callable[[list[EleType]], list[EleType]]]
            | Callable[[list[EleType]], list[EleType]]
        ) = None,
    ):
        """
        按照指定 key 筛选，将所有该 key 的 value 一样的放到一起，
        除了该 key 之外的所有键的值通过 list 组合到一起。
        可传入聚合函数对组合后的列表进行处理。

        Args:
            key (str): 用于分组的键名。
            agg_map (Dict[str, Callable[ItemType, ItemType]] | Callable[ItemType, ItemType], optional): 聚合函数，用于处理组合后的列表。默认为 None。

        Returns:
            JSONDataHandler: 分组后的结果。
        """
        grouped_data: dict[str, dict[str, EleType | list[EleType]]] = {}
        for item in self.document:
            key_value = item.get(key)
            if key_value not in grouped_data:
                grouped_data[key_value] = {key: key_value}
            for k, v in item.items():
                if k != key:
                    if k not in grouped_data[key_value]:
                        grouped_data[key_value][k] = []
                    grouped_data[key_value][k].append(v)

        if isinstance(agg_map, dict):
            for group in grouped_data.values():
                for k, v in group.items():
                    if k in agg_map:
                        group[k] = agg_map[k](v)
        elif isinstance(agg_map, Callable):
            for group in grouped_data.values():
                for k, v in group.items():
                    if k != key:
                        group[k] = agg_map(v)

        return DocumentHandler(list(grouped_data.values()))
