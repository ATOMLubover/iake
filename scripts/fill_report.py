"""填充实习报告封面模板。

用法：
    uv run python3 scripts/fill_report.py

配置在脚本末尾的 `__main__` 块中修改。
"""

from __future__ import annotations

import copy
import shutil
from dataclasses import dataclass
from datetime import date
from pathlib import Path

from docx import Document
from docx.oxml.ns import qn


# ── 段落索引常量（基于原始 .doc 转 .docx 后的结构分析） ──────────────────

# 封面填充段落
P_IDX_TOP_NUMBER = 0       # "编号：" + 空格 → 填入右上角编号
P_IDX_CLASS_NUMBER = 9     # "编    号：  班序号 CP" + 空格 → 填入班序号
P_IDX_TOPIC = 10           # "实习题目：" + 空格 → 填入题目
P_IDX_MAJOR = 11           # "专业（班）：" + 空格 + 空格 → 填入专业班级
P_IDX_STUDENT_ID = 12      # "学生学号：" + 空格 + 空格 → 填入学号
P_IDX_STUDENT_NAME = 13    # "学生姓名：" + 空格 → 填入姓名
P_IDX_TEACHER = 14         # "任课教师：" + 空格（含"杜卓敏"） → 调整
P_IDX_DATE = 20            # "２０２６  年      月       日"

# 目录段落
P_IDX_TOC_TITLE = 23       # "目    录"
P_IDX_TOC_START = 25       # 第一部分（含页码 1）
P_IDX_TOC_END = 29         # 第五部分

# TOC 条目与 page 位置映射：段落索引 → 页码所在 run 中的偏移描述
# P[25] 格式: "第一部分\t\t语言语法规则（自然语言描述）………………\t1"
# TOC 页码在文本末尾 \t 之后；run 的 .text 整体包含页码
# 替换策略：修改 run.text 末尾的页码数字

# 日期段落 run 分布
# P[20] run[0]='２０２' run[1]='６' run[2]='  年  ' run[3]='    '
#       run[4]='月 ' run[5]='     ' run[6]=' 日'
# 简化为：只改年份为当年（保留全角数字），月份和日期填入 run[3] 和 run[5]


@dataclass
class FillConfig:
    """模板填充配置。"""

    # ── 封面信息 ──
    top_number: str = ""                 # 右上角编号（留空则不填）
    class_number: str = "CP"             # 班序号
    topic: str = "Mini 语言语法分析器"    # 实习题目
    major: str = ""                      # 专业（班）
    student_id: str = ""                 # 学生学号
    student_name: str = ""               # 学生姓名
    teacher: str = "杜卓敏"              # 任课教师（模板已预填）

    # ── 日期 ──
    year: int | None = None              # 年份（None=当前年）
    month: int | None = None             # 月份（None=当前月）
    day: int | None = None               # 日（None=当前日）

    # ── 目录页码 ──
    toc_pages: tuple[int, int, int, int, int] = (1, 0, 0, 0, 0)
    # 第一部分~第五部分的页码，0 表示不修改


# ── 辅助函数 ──────────────────────────────────────────────────────────────


def to_fullwidth_digit(n: int) -> str:
    """将阿拉伯数字转为全角数字。"""
    return "".join(chr(ord("０") + int(d)) for d in str(n))


def replace_run_text(paragraph, run_index: int, new_text: str):
    """替换段落中指定 run 的文本（保留格式）。"""
    runs = paragraph.runs
    if run_index >= len(runs):
        raise IndexError(f"run_index {run_index} out of range (paragraph has {len(runs)} runs)")
    runs[run_index].text = new_text


def set_run_text_if_empty(paragraph, run_index: int, text: str):
    """若 run 文本全是空白，则替换为目标文本。"""
    runs = paragraph.runs
    if run_index < len(runs) and runs[run_index].text.strip() == "":
        runs[run_index].text = text


def fill_number_line(paragraph, value: str):
    """
    填充 "标签：  值" 格式的段落。
    段落通常为 label_run[0] + padding_run[1..n]。
    将 padding_run 合并为单个值 + 适当空格。
    """
    runs = paragraph.runs
    if len(runs) < 2:
        return

    # 保留第一个 run（标签），把后续所有 run 的文本替换为值
    # 计算原本填充区的总字符宽以粗略对齐
    original_width = sum(len(r.text) for r in runs[1:])
    if value:
        # 值 + 尾部空格填充到原始总宽
        padded = value + " " * max(0, original_width - len(value))
    else:
        padded = " " * original_width

    # 写入第一个 padding run，清空其余
    runs[1].text = padded
    for r in runs[2:]:
        r.text = ""


def fill_toc_page(paragraph, page: int):
    """
    替换目录条目的页码。
    目录格式如 "第一部分\t\t语言语法规则……\t1"
    策略：找到文本末尾的数字并替换。
    """
    runs = paragraph.runs
    if not runs:
        return

    # 在最后一个 run 中找末尾的数字
    last_run = runs[-1]
    text = last_run.text

    # 从末尾找连续数字
    i = len(text) - 1
    while i >= 0 and text[i].isdigit():
        i -= 1
    start = i + 1

    if start < len(text):
        # 替换数字部分
        new_page = str(page)
        last_run.text = text[:start] + new_page


def fill_date_paragraph(paragraph, year: int, month: int, day: int):
    """
    填充日期段落 P[20]。
    原格式 run 分布复杂（中英文字体混排），简化为：
    保留第一个 run 的全角 "２０２"，修改年份末位 → 统一为单一 run 写全角日期。
    """
    fullwidth_year = to_fullwidth_digit(year)
    fullwidth_month = to_fullwidth_digit(month)
    fullwidth_day = to_fullwidth_digit(day)

    runs = paragraph.runs
    # 简化：清空所有 run，只留一个 run 写完整日期
    # 格式："YYYY  年  MM  月  DD  日"
    date_text = f"{fullwidth_year}  年  {fullwidth_month}  月  {fullwidth_day}  日"

    # 保留第一个 run 的格式，写入完整日期
    runs[0].text = date_text
    for r in runs[1:]:
        r.text = ""


# ── 主填充流程 ──────────────────────────────────────────────────────────────


def fill(config: FillConfig, template_path: str | Path, output_path: str | Path):
    """按配置填充模板并保存。"""
    doc = Document(str(template_path))
    paragraphs = doc.paragraphs

    # ── 1. 右上角编号 ──
    if config.top_number:
        fill_number_line(paragraphs[P_IDX_TOP_NUMBER], config.top_number)

    # ── 2. 班序号 ──
    fill_number_line(paragraphs[P_IDX_CLASS_NUMBER], config.class_number)

    # ── 3. 实习题目 ──
    fill_number_line(paragraphs[P_IDX_TOPIC], config.topic)

    # ── 4. 专业（班） ──
    fill_number_line(paragraphs[P_IDX_MAJOR], config.major)

    # ── 5. 学生学号 ──
    fill_number_line(paragraphs[P_IDX_STUDENT_ID], config.student_id)

    # ── 6. 学生姓名 ──
    fill_number_line(paragraphs[P_IDX_STUDENT_NAME], config.student_name)

    # ── 7. 任课教师 ──
    fill_number_line(paragraphs[P_IDX_TEACHER], config.teacher)

    # ── 8. 日期 ──
    now = date.today()
    year = config.year or now.year
    month = config.month or now.month
    day_value = config.day or now.day
    fill_date_paragraph(paragraphs[P_IDX_DATE], year, month, day_value)

    # ── 9. 目录页码 ──
    for i, page in enumerate(config.toc_pages):
        if page > 0:
            para_idx = P_IDX_TOC_START + i
            if para_idx < len(paragraphs):
                fill_toc_page(paragraphs[para_idx], page)

    # ── 保存 ──
    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    doc.save(str(output_path))
    print(f"✅ 已写入: {output_path}")


# ── CLI ────────────────────────────────────────────────────────────────────


if __name__ == "__main__":
    PROJECT = Path(__file__).resolve().parent.parent

    TEMPLATE_SRC = PROJECT / "docs" / "cover-template.docx"
    OUTPUT = PROJECT / "output" / "语法分析实习报告.docx"

    # ── ══════════════ 修改此处配置 ══════════════ ──
    cfg = FillConfig(
        top_number="",
        class_number="软工23-CP",
        topic="Mini 语言语法分析器",
        major="软件工程",
        student_id="2023302110xxx",
        student_name="张三",
        teacher="杜卓敏",
        # 日期留空 = 使用当前日期
        year=None,
        month=None,
        day=None,
        # 目录页码（第 2~5 部分先留 0，排版后回填）
        toc_pages=(1, 0, 0, 0, 0),
    )
    # ── ═════════════════════════════════════════ ──

    fill(cfg, TEMPLATE_SRC, OUTPUT)
