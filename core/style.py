import re
from pathlib import Path
from typing import Dict, List, Optional

import requests
import typer.rich_utils
from rich.progress import (
    BarColumn,
    DownloadColumn,
    FileSizeColumn,
    Progress,
    TextColumn,
    TimeRemainingColumn,
    TransferSpeedColumn,
)
from rich.table import Table
from rich.tree import Tree
from typer.core import TyperGroup

EleType = str | int | bool | list | dict
ItemType = Dict[str, EleType]
JSONType = List[ItemType]
import typer

typer.rich_utils.rich_render_text


class AliasGroup(TyperGroup):

    _CMD_SPLIT_P = re.compile(r" ?[,|] ?")

    def get_command(self, ctx, cmd_name):
        cmd_name = self._group_cmd_name(cmd_name)
        return super().get_command(ctx, cmd_name)

    def _group_cmd_name(self, default_name):
        for cmd in self.commands.values():
            name = cmd.name
            if name and default_name in self._CMD_SPLIT_P.split(name):
                return name
        return default_name


def show_tree(versions: list[str], current: str = "", label: str = ""):

    tree = Tree(label, guide_style="none", style="grey50")

    # 添加每个版本到树中，当前版本用绿色高亮并添加星号标记
    for i, version in enumerate(versions, 1):
        is_current = version == current
        prefix = "[green]*[/green] " if is_current else "  "
        style = "green bold" if is_current else "default"
        tree.add(f"{prefix}{i}) {version}", style=style)

    return tree


def show_table(data: JSONType, title: str = ""):
    table = Table(title=title, show_lines=True, style="grey50")
    for key in data[0].keys():
        table.add_column(key)
    for item in data:
        table.add_row(*[str(item[key]) for key in item.keys()])
    return table


def download_package(uri: str, file_path: Path, proxies: Optional[dict] = None) -> None:
    """
    下载指定 URI 的文件到指定路径，使用 rich 进度条显示下载进度。

    :param uri: 文件的下载地址
    :param file_path: 文件保存的路径
    :param proxies: 代理配置，可选参数
    """
    with requests.get(uri, stream=True, proxies=proxies) as response:
        response.raise_for_status()
        total_size = int(response.headers.get("content-length", 0))

        progress_columns = [
            TextColumn("{task.fields[filename]}"),
            BarColumn(),
            FileSizeColumn(),
            DownloadColumn(),
            TransferSpeedColumn(),
            TimeRemainingColumn(),
        ]

        with Progress(*progress_columns) as progress:
            task = progress.add_task(
                "Downloading", total=total_size, filename=file_path.name
            )

            with open(file_path, "wb") as f:
                for chunk in response.iter_content(chunk_size=8192):
                    if chunk:
                        f.write(chunk)
                        progress.update(task, advance=len(chunk))
