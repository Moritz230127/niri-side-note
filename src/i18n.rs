use gtk::prelude::*;
use std::sync::Once;
use std::time::Duration;

/// Local build: Chinese is the default UI language. Set STICKIES_LANG=en to
/// keep the upstream English strings while debugging translations.
fn chinese() -> bool {
    match std::env::var("STICKIES_LANG") {
        Ok(language) => !language.eq_ignore_ascii_case("en"),
        Err(_) => true,
    }
}

fn tr(text: &str) -> Option<&'static str> {
    if !chinese() {
        return None;
    }
    Some(match text {
        "New note" => "新建便签",
        "Close" => "关闭",
        "Search" => "搜索",
        "Save" => "保存",
        "Search notes" => "搜索便签",
        "Search active notes" => "搜索现有便签",
        "All Notes" => "全部便签",
        "Untitled note" => "无标题便签",
        "Note deleted" => "便签已删除",
        "Undo" => "撤销",
        "All" => "全部",
        "Active" => "现有",
        "Archived" => "已归档",
        "Recently edited" => "最近编辑",
        "Title" => "标题",
        "Sort notes" => "排序方式",
        "Pin note" => "置顶便签",
        "Unpin note" => "取消置顶",
        "Pinned" => "已置顶",
        "Archive" => "归档",
        "Archive note" => "归档便签",
        "Restore" => "恢复",
        "Restore note" => "恢复便签",
        "Delete" => "删除",
        "Delete note" => "删除便签",
        "Copy" => "复制",
        "Copy text" => "复制文本",
        "Open away" => "独立窗口打开",
        "Open away from edge" => "从边缘独立打开",
        "Pop out" => "弹出为窗口",
        "Close note" => "关闭便签",
        "Close All Notes" => "关闭全部便签",
        "Drag to move note" => "拖动以移动便签",
        "Color" => "颜色",
        "Font" => "字体",
        "Checklist" => "清单",
        "Always open" => "常开",
        "More" => "更多",
        "Bold" => "粗体",
        "Italic" => "斜体",
        "Underline" => "下划线",
        "Strikethrough" => "删除线",
        "Bullet list" => "项目符号列表",
        "Numbered list" => "编号列表",
        "Plain" => "纯色",
        "Grid" => "网格",
        "Ruled" => "横线",
        "Dots" => "圆点",
        "Cancel" => "取消",
        "Done" => "完成",
        "Settings" => "设置",
        "Mode" => "模式",
        "System" => "跟随系统",
        "Light" => "浅色",
        "Dark" => "深色",
        "Minimal" => "极简",
        "Paper" => "纸张",
        "Playful" => "活泼",
        "Note edges" => "便签边缘",
        "Wavy" => "波浪",
        "Clean" => "平整",
        "Reduce motion" => "减少动画",
        "Edge" => "贴边位置",
        "Left" => "左",
        "Right" => "右",
        "Top" => "上",
        "Bottom" => "下",
        "Position" => "位置",
        "Center" => "居中",
        "Reposition" => "重新定位",
        "Visible notes" => "可见便签数",
        "Tabs" => "标签样式",
        "Classic" => "经典",
        "Stacked" => "叠放",
        "Marker" => "标记",
        "Bar" => "条状",
        "Pills" => "圆点",
        "Auto-close" => "自动收起",
        "Off" => "关闭",
        "1 min" => "1 分钟",
        "2 min" => "2 分钟",
        "5 min" => "5 分钟",
        "Close docked notes after inactivity" => "无操作后自动收起已展开的便签",
        "Hover to reveal" => "悬停展开",
        "Launch at login" => "开机自启",
        "Quit Stickies" => "退出 Stickies",
        "Star on GitHub" => "在 GitHub 上点星",
        "Drag the strip along the edge." => "沿屏幕边缘拖动便签条。",
        "Welcome to Stickies" => "欢迎使用 Stickies",
        "Choose a note" => "选择一条便签",
        "Back to notes" => "返回便签",
        "Clear search" => "清除搜索",
        "Create a note" => "新建便签",
        "Create your first note" => "创建第一条便签",
        "No active notes" => "没有现有便签",
        "No matching notes" => "没有匹配的便签",
        "Archive is empty" => "归档为空",
        "Notes you archive will appear here." => "归档的便签会显示在这里。",
        "Select a note from the list to view or edit it." => "从列表中选择一条便签查看或编辑。",
        "Create a note or switch to All to see your archived notes." => {
            "新建便签，或切换到“全部”查看归档。"
        }
        "Try another word or clear your search." => "换个关键词，或清除搜索。",
        "Archived · restore to edit" => "已归档 · 恢复后可编辑",
        "Restore this note to edit it" => "恢复此便签后才能编辑",
        "No matches. Create note" => "无匹配，创建便签",
        "Save failed" => "保存失败",
        "Saved" => "已保存",
        "Saving" => "保存中",
        "Unknown save error" => "未知保存错误",
        "Could not save note" => "无法保存便签",
        "Could not load note" => "无法加载便签",
        "Could not load notes" => "无法加载便签",
        "Could not create note" => "无法新建便签",
        "Could not create a note" => "无法新建便签",
        "Could not open note" => "无法打开便签",
        "Could not undo deletion" => "无法撤销删除",
        "Could not update note" => "无法更新便签",
        "Could not save position" => "无法保存位置",
        "Could not search notes" => "无法搜索便签",
        "Stickies is closing" => "Stickies 正在退出",
        "Stickies · Note" => "Stickies · 便签",
        "Stickies could not open your notes" => "Stickies 无法打开你的便签",
        "Stickies could not save your notes" => "Stickies 无法保存你的便签",
        "Choose Done or Cancel before changing Settings." => "更改设置前请先选择“完成”或“取消”。",
        "Correct the invalid configuration before repositioning." => {
            "配置无效，修正后才能重新定位。"
        }
        "The open note could not be saved. Retry saving it before repositioning." => {
            "当前便签未能保存，请重试后再重新定位。"
        }
        "Turn off Do Not Disturb before repositioning the strip." => {
            "请先关闭“勿扰模式”，再重新定位便签条。"
        }
        _ => return None,
    })
}

fn translate(text: &str) -> Option<String> {
    if let Some(chinese) = tr(text) {
        return Some(chinese.to_owned());
    }
    if let Some(rest) = text.strip_prefix("Created ")
        && let Some((created, updated)) = rest.split_once("   ·   Updated ")
    {
        return Some(format!("创建 {created} · 更新 {updated}"));
    }
    if let Some((prefix, rest)) = text.split_once(": ")
        && let Some(chinese) = tr(prefix)
    {
        return Some(format!("{chinese}：{rest}"));
    }
    None
}

fn skip_content(widget: &gtk::Widget) -> bool {
    // Note titles and snippets are user content, never UI copy.
    [
        "tab-title",
        "row-title",
        "row-snippet",
        "row-date",
        "edge-result-title",
    ]
    .iter()
    .any(|class| widget.has_css_class(class))
}

fn visit(widget: &gtk::Widget) {
    if widget.is::<gtk::TextView>() || skip_content(widget) {
        return;
    }
    if let Some(label) = widget.downcast_ref::<gtk::Label>() {
        let text = label.text();
        if let Some(translated) = translate(&text)
            && translated != text.as_str()
        {
            label.set_text(&translated);
        }
    }
    if let Some(entry) = widget.downcast_ref::<gtk::Entry>()
        && let Some(placeholder) = entry.placeholder_text()
        && let Some(translated) = translate(&placeholder)
    {
        entry.set_placeholder_text(Some(&translated));
    }
    if let Some(window) = widget.downcast_ref::<gtk::Window>()
        && let Some(title) = window.title()
        && let Some(translated) = translate(&title)
    {
        window.set_title(Some(&translated));
    }
    if let Some(tooltip) = widget.tooltip_text()
        && let Some(translated) = translate(&tooltip)
    {
        widget.set_tooltip_text(Some(&translated));
    }
    let mut child = widget.first_child();
    while let Some(child_widget) = child {
        visit(&child_widget);
        child = child_widget.next_sibling();
    }
}

/// Periodically localize every top-level window. This keeps strings created by
/// refreshes, popovers and toasts translated without touching user note text.
pub fn install() {
    static INSTALLED: Once = Once::new();
    INSTALLED.call_once(|| {
        gtk::glib::timeout_add_local(Duration::from_millis(1000), || {
            for window in gtk::Window::list_toplevels() {
                visit(window.upcast_ref());
            }
            gtk::glib::ControlFlow::Continue
        });
    });
}
