use crate::{
    app::NoteColor,
    appearance,
    config::{Appearance, Config, PlayfulEdges, ScreenEdge, Shape, TabLayout, Theme},
};
use gtk::prelude::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct Panel {
    pub root: gtk::ScrolledWindow,
    position: Rc<PositionControls>,
    timeouts: Rc<Vec<gtk::ToggleButton>>,
    syncing: Rc<Cell<bool>>,
    appearance: Rc<AppearanceControls>,
    extras: Vec<Box<dyn Fn()>>,
}
impl Panel {
    pub fn refresh(&self) {
        self.appearance.sync();
        self.position.sync();
        for sync in &self.extras {
            sync();
        }
        self.syncing.set(true);
        let seconds = self.position.current.borrow().inactivity_seconds;
        for (button, timeout) in self.timeouts.iter().zip([0, 60, 120, 300]) {
            button.set_active(seconds == timeout);
        }
        self.syncing.set(false);
    }
}

struct AppearanceControls {
    current: Rc<RefCell<Config>>,
    syncing: Rc<Cell<bool>>,
    themes: Vec<(gtk::ToggleButton, Theme)>,
    mode: gtk::Box,
    modes: Vec<gtk::ToggleButton>,
    playful_row: gtk::Box,
    playful_edges: Vec<gtk::ToggleButton>,
    motion: gtk::Switch,
    samples: Vec<(Rc<RefCell<Config>>, gtk::DrawingArea)>,
}
impl AppearanceControls {
    fn sync(&self) {
        let config = self.current.borrow().clone();
        let was_syncing = self.syncing.replace(true);
        for (button, theme) in &self.themes {
            button.set_active(config.theme == *theme);
        }
        self.mode.set_visible(config.theme == Theme::Minimal);
        self.playful_row.set_visible(config.theme == Theme::Playful);
        for (button, edges) in self
            .playful_edges
            .iter()
            .zip([PlayfulEdges::Wavy, PlayfulEdges::Clean])
        {
            button.set_active(config.playful_edges == edges);
        }
        for (index, button) in self.modes.iter().enumerate() {
            button.set_active(config.appearance as usize == index);
        }
        self.motion.set_active(config.reduced_motion);
        for (sample, drawing) in &self.samples {
            let mut sample = sample.borrow_mut();
            sample.appearance = config.appearance;
            sample.playful_edges = config.playful_edges;
            drawing.queue_draw();
        }
        self.syncing.set(was_syncing);
    }
}

struct PositionControls {
    root: gtk::Box,
    edges: Vec<gtk::ToggleButton>,
    presets: Vec<gtk::ToggleButton>,
    layouts: Vec<gtk::ToggleButton>,
    screens: Vec<(gtk::ToggleButton, Option<String>)>,
    error: gtk::Label,
    current: Rc<RefCell<Config>>,
    syncing: Cell<bool>,
}
impl PositionControls {
    fn sync(&self) {
        self.syncing.set(true);
        let config = self.current.borrow();
        for (button, edge) in self.edges.iter().zip([
            ScreenEdge::Left,
            ScreenEdge::Right,
            ScreenEdge::Top,
            ScreenEdge::Bottom,
        ]) {
            button.set_active(config.edge == edge);
        }
        let labels = if matches!(config.edge, ScreenEdge::Top | ScreenEdge::Bottom) {
            ["左", "居中", "右"]
        } else {
            ["上", "居中", "下"]
        };
        for (index, button) in self.presets.iter().enumerate() {
            button.set_label(labels[index]);
            button.set_active((config.marker_offset - index as f64 / 2.).abs() < 0.000001);
        }
        for (button, layout) in self
            .layouts
            .iter()
            .zip([TabLayout::Classic, TabLayout::Stacked])
        {
            button.set_active(config.tab_layout == layout);
        }
        for (button, monitor) in &self.screens {
            button.set_active(config.monitor.as_deref() == monitor.as_deref());
        }
        self.syncing.set(false);
    }
    fn result(&self, result: Result<(), String>) {
        if let Err(message) = result {
            self.error.set_text(&message);
            self.error.set_visible(true);
        } else {
            self.error.set_visible(false);
        }
        self.sync();
    }
    fn new(
        current: Rc<RefCell<Config>>,
        change: Rc<dyn Fn(Config) -> Result<(), String>>,
        reposition: impl Fn() -> Result<(), String> + 'static,
    ) -> Rc<Self> {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 16);
        let screens = screen_choices(current.borrow().monitor.as_deref());
        section(&root, "屏幕", &screens.0);
        let edges = choices(&["左", "右", "上", "下"], 0);
        let presets = choices(&["上", "居中", "下"], 1);
        section(&root, "贴边位置", &edges.0);
        let position = gtk::Box::new(gtk::Orientation::Vertical, 10);
        position.append(&presets.0);
        let move_button = gtk::Button::with_label("重新定位");
        move_button.set_halign(gtk::Align::Start);
        move_button.add_css_class("reposition-button");
        position.append(&move_button);
        section(&root, "位置", &position);
        let layouts = choices(&["经典", "叠放"], 1);
        section(&root, "标签样式", &layouts.0);
        let error = gtk::Label::new(None);
        error.set_wrap(true);
        error.set_max_width_chars(28);
        error.add_css_class("save-error");
        error.set_visible(false);
        root.append(&error);
        let controls = Rc::new(Self {
            root,
            edges: edges.1,
            presets: presets.1,
            layouts: layouts.1,
            screens: screens.1,
            error,
            current,
            syncing: Cell::new(false),
        });
        controls.sync();
        for (index, button) in controls.edges.iter().enumerate() {
            let weak = Rc::downgrade(&controls);
            let change = change.clone();
            button.connect_toggled(move |button| {
                if let Some(controls) = weak.upgrade()
                    && !controls.syncing.get()
                    && button.is_active()
                {
                    let mut config = controls.current.borrow().clone();
                    config.edge = [
                        ScreenEdge::Left,
                        ScreenEdge::Right,
                        ScreenEdge::Top,
                        ScreenEdge::Bottom,
                    ][index];
                    controls.result(change(config));
                }
            });
        }
        for (index, button) in controls.presets.iter().enumerate() {
            let weak = Rc::downgrade(&controls);
            let change = change.clone();
            button.connect_toggled(move |button| {
                if let Some(controls) = weak.upgrade()
                    && !controls.syncing.get()
                    && button.is_active()
                {
                    let mut config = controls.current.borrow().clone();
                    config.marker_offset = index as f64 / 2.;
                    controls.result(change(config));
                }
            });
        }
        for (button, monitor) in controls.screens.clone() {
            let weak = Rc::downgrade(&controls);
            let change = change.clone();
            button.connect_toggled(move |button| {
                if let Some(controls) = weak.upgrade()
                    && !controls.syncing.get()
                    && button.is_active()
                {
                    let mut config = controls.current.borrow().clone();
                    config.monitor = monitor.clone();
                    controls.result(change(config));
                }
            });
        }
        for (buttons, write) in [(
            &controls.layouts,
            (|c: &mut Config, index: usize| {
                c.tab_layout = [TabLayout::Classic, TabLayout::Stacked][index]
            }) as fn(&mut Config, usize),
        )] {
            for (index, button) in buttons.iter().enumerate() {
                let weak = Rc::downgrade(&controls);
                let change = change.clone();
                button.connect_toggled(move |button| {
                    if let Some(controls) = weak.upgrade()
                        && !controls.syncing.get()
                        && button.is_active()
                    {
                        let mut config = controls.current.borrow().clone();
                        write(&mut config, index);
                        controls.result(change(config));
                    }
                });
            }
        }
        let weak = Rc::downgrade(&controls);
        move_button.connect_clicked(move |_| {
            if let Some(controls) = weak.upgrade() {
                controls.result(reposition());
            }
        });
        controls
    }
}

pub fn build(
    current: Rc<RefCell<Config>>,
    on_change: impl Fn(Config) -> Result<(), String> + 'static,
    on_reposition: impl Fn() -> Result<(), String> + 'static,
) -> Panel {
    let on_change: Rc<dyn Fn(Config) -> Result<(), String>> = Rc::new(on_change);
    let position = PositionControls::new(current.clone(), on_change.clone(), on_reposition);
    let root = gtk::Box::new(gtk::Orientation::Vertical, 18);
    root.add_css_class("appearance-panel");
    let error = gtk::Label::new(None);
    error.set_wrap(true);
    error.add_css_class("save-error");
    error.set_visible(false);
    root.append(&error);
    let mut samples = Vec::new();
    // Build controls once; changing a theme must not rebuild the focused drawer.
    let config = current.borrow().clone();
    let syncing = Rc::new(Cell::new(false));
    let themes = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    root.append(&themes);
    let mut theme_buttons = Vec::new();
    let mut group: Option<gtk::ToggleButton> = None;
    for (theme, name) in [
        (Theme::Minimal, "极简"),
        (Theme::Paper, "纸张"),
        (Theme::Playful, "活泼"),
    ] {
        let button = gtk::ToggleButton::new();
        button.add_css_class("theme-choice");
        button.set_hexpand(true);
        button.set_group(group.as_ref());
        button.set_active(config.theme == theme);
        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let overlay = gtk::Overlay::new();
        overlay.add_css_class("theme-sample");
        let sample = Config {
            theme,
            shape: if theme == Theme::Paper {
                Shape::Ticket
            } else {
                Shape::Square
            },
            ..config.clone()
        };
        let sample = Rc::new(RefCell::new(sample));
        let paper = appearance::paper_background(
            sample.clone(),
            Rc::new(Cell::new(NoteColor::Green)),
            Rc::new(Cell::new(1)),
        );
        samples.push((sample, paper.clone()));
        paper.set_size_request(
            crate::edge_surface::scale_px(50),
            crate::edge_surface::scale_px(48),
        );
        overlay.set_child(Some(&paper));
        let badge = gtk::Label::new(Some("✓"));
        badge.add_css_class("selection-badge");
        badge.set_halign(gtk::Align::End);
        badge.set_valign(gtk::Align::Start);
        badge.set_visible(button.is_active());
        overlay.add_overlay(&badge);
        let visible = badge.clone();
        button.connect_toggled(move |b| visible.set_visible(b.is_active()));
        content.append(&overlay);
        let caption = gtk::Label::new(Some(name));
        caption.add_css_class("theme-caption");
        content.append(&caption);
        button.set_child(Some(&content));
        themes.append(&button);
        if group.is_none() {
            group = Some(button.clone());
        }
        theme_buttons.push((button, theme));
    }
    let mode = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let modes = choices(&["跟随系统", "浅色", "深色"], config.appearance as usize);
    section(&mode, "模式", &modes.0);
    mode.set_visible(config.theme == Theme::Minimal);
    root.append(&mode);
    let playful_row = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let playful_edges = choices(&["波浪", "平整"], config.playful_edges as usize);
    section(&playful_row, "便签边缘", &playful_edges.0);
    playful_row.set_visible(config.theme == Theme::Playful);
    root.append(&playful_row);
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let label = gtk::Label::new(Some("减少动画"));
    label.set_xalign(0.);
    label.set_hexpand(true);
    let switch = gtk::Switch::new();
    switch.set_valign(gtk::Align::Center);
    switch.set_active(config.reduced_motion);
    row.append(&label);
    row.append(&switch);
    let motion_row = row;
    let appearance = Rc::new(AppearanceControls {
        current: current.clone(),
        syncing: syncing.clone(),
        themes: theme_buttons,
        mode,
        modes: modes.1,
        playful_row,
        playful_edges: playful_edges.1,
        motion: switch,
        samples,
    });
    let weak = Rc::downgrade(&appearance);
    let update: Rc<dyn Fn(Config)> = Rc::new(move |config| {
        match on_change(config) {
            Ok(()) => error.set_visible(false),
            Err(message) => {
                error.set_text(&message);
                error.set_visible(true);
            }
        }
        if let Some(controls) = weak.upgrade() {
            controls.sync();
        }
    });
    for (button, theme) in &appearance.themes {
        let theme = *theme;
        let weak = Rc::downgrade(&appearance);
        let update = update.clone();
        button.connect_toggled(move |button| {
            if let Some(controls) = weak.upgrade()
                && button.is_active()
                && !controls.syncing.get()
            {
                let mut config = controls.current.borrow().clone();
                config.theme = theme;
                update(config);
            }
        });
    }
    for (index, button) in appearance.modes.iter().enumerate() {
        let weak = Rc::downgrade(&appearance);
        let update = update.clone();
        button.connect_toggled(move |button| {
            if let Some(controls) = weak.upgrade()
                && button.is_active()
                && !controls.syncing.get()
            {
                let mut config = controls.current.borrow().clone();
                config.appearance =
                    [Appearance::System, Appearance::Light, Appearance::Dark][index];
                update(config);
            }
        });
    }
    for (index, button) in appearance.playful_edges.iter().enumerate() {
        let weak = Rc::downgrade(&appearance);
        let update = update.clone();
        button.connect_toggled(move |button| {
            if let Some(controls) = weak.upgrade()
                && button.is_active()
                && !controls.syncing.get()
            {
                let mut config = controls.current.borrow().clone();
                config.playful_edges = [PlayfulEdges::Wavy, PlayfulEdges::Clean][index];
                update(config);
            }
        });
    }
    {
        let weak = Rc::downgrade(&appearance);
        let update = update.clone();
        appearance.motion.connect_active_notify(move |switch| {
            if let Some(controls) = weak.upgrade()
                && !controls.syncing.get()
                && controls.current.borrow().reduced_motion != switch.is_active()
            {
                let mut config = controls.current.borrow().clone();
                config.reduced_motion = switch.is_active();
                update(config);
            }
        });
    }
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    root.append(&position.root);
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    let selected = [0, 60, 120, 300]
        .iter()
        .position(|seconds| *seconds == config.inactivity_seconds)
        .unwrap_or(2);
    let timeouts = choices(&["关闭", "1 分钟", "2 分钟", "5 分钟"], selected);
    section(&root, "自动收起", &timeouts.0);
    timeouts
        .0
        .set_tooltip_text(Some("无操作后自动收起已展开的便签"));
    let timeouts = Rc::new(timeouts.1);
    for (index, button) in timeouts.iter().enumerate() {
        let current = current.clone();
        let update = update.clone();
        let syncing = syncing.clone();
        let buttons = Rc::downgrade(&timeouts);
        button.connect_toggled(move |button| {
            if button.is_active() && !syncing.get() {
                let mut config = current.borrow().clone();
                config.inactivity_seconds = [0, 60, 120, 300][index];
                update(config);
                syncing.set(true);
                let seconds = current.borrow().inactivity_seconds;
                if let Some(buttons) = buttons.upgrade() {
                    for (button, timeout) in buttons.iter().zip([0, 60, 120, 300]) {
                        button.set_active(seconds == timeout);
                    }
                }
                syncing.set(false);
            }
        });
    }
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    root.append(&motion_row);
    let mut extras: Vec<Box<dyn Fn()>> = Vec::new();
    // 行为区目前只有一个开关；保留统一注册逻辑，后续增加开关时无需改刷新路径。
    for (label, read, write) in std::iter::once((
        "悬停展开",
        (|c: &Config| c.edge_hover_enabled) as fn(&Config) -> bool,
        (|c: &mut Config, v| c.edge_hover_enabled = v) as fn(&mut Config, bool),
    )) {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let text = gtk::Label::new(Some(label));
        text.set_hexpand(true);
        text.set_xalign(0.);
        let switch = gtk::Switch::new();
        switch.set_valign(gtk::Align::Center);
        switch.update_property(&[gtk::accessible::Property::Label(label)]);
        switch.set_active(read(&current.borrow()));
        row.append(&text);
        row.append(&switch);
        root.append(&row);
        let current = current.clone();
        let update = update.clone();
        let guard = Rc::new(Cell::new(false));
        let sync: Rc<dyn Fn()> = Rc::new({
            let current = current.clone();
            let weak = switch.downgrade();
            let guard = guard.clone();
            move || {
                guard.set(true);
                if let Some(switch) = weak.upgrade() {
                    switch.set_active(read(&current.borrow()));
                }
                guard.set(false);
            }
        });
        let sync_after = sync.clone();
        switch.connect_active_notify(move |switch| {
            if guard.get() || read(&current.borrow()) == switch.is_active() {
                return;
            }
            let mut config = current.borrow().clone();
            write(&mut config, switch.is_active());
            update(config);
            sync_after();
        });
        extras.push(Box::new(move || sync()));
    }
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    let quit = gtk::Button::with_label("退出 Stickies");
    quit.add_css_class("reposition-button");
    quit.set_action_name(Some("app.quit"));
    root.append(&quit);

    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    let github =
        gtk::LinkButton::with_label("https://github.com/0atxl/stickies", "在 GitHub 上点星");
    github.set_halign(gtk::Align::Center);
    github.add_css_class("github-footer");
    let link_content = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    link_content.append(&gtk::Label::new(Some("在 GitHub 上点星")));
    link_content.append(&crate::ui_assets::image("popout"));
    github.set_child(Some(&link_content));
    root.append(&github);
    Panel {
        root: gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .child(&root)
            .build(),
        position,
        timeouts,
        syncing,
        appearance,
        extras,
    }
}

/// 屏幕选择按钮同时提供可见标题、tooltip 和 Accessibility Label，
/// 这样鼠标、键盘和 AT-SPI 测试都能识别同一个目标。
fn screen_button(label: &str, group: Option<&gtk::ToggleButton>) -> gtk::ToggleButton {
    let button = gtk::ToggleButton::new();
    let text = gtk::Label::new(Some(label));
    text.set_xalign(0.);
    button.set_child(Some(&text));
    button.set_halign(gtk::Align::Fill);
    button.set_hexpand(true);
    button.set_group(group);
    button.update_property(&[gtk::accessible::Property::Label(label)]);
    button.set_tooltip_text(Some(label));
    button
}

/// 枚举当前 GDK 显示器；connector 作为持久配置值，None 表示自动选择第一块。
/// 配置里的屏幕已断开时保留一个「未连接」选项，避免用户丢失选择。
fn screen_choices(current: Option<&str>) -> (gtk::Box, Vec<(gtk::ToggleButton, Option<String>)>) {
    let list = gtk::Box::new(gtk::Orientation::Vertical, 4);
    list.add_css_class("segments");
    let mut buttons: Vec<(gtk::ToggleButton, Option<String>)> = Vec::new();
    let automatic = screen_button("自动（第一块屏幕）", None);
    list.append(&automatic);
    buttons.push((automatic.clone(), None));
    if let Some(display) = gtk::gdk::Display::default() {
        for monitor in display
            .monitors()
            .iter::<gtk::gdk::Monitor>()
            .filter_map(Result::ok)
            .filter(|monitor| {
                let geometry = monitor.geometry();
                monitor.is_valid() && geometry.width() > 0 && geometry.height() > 0
            })
        {
            let connector = monitor.connector().map(|value| value.to_string());
            let model = monitor
                .model()
                .map(|value| value.to_string())
                .unwrap_or_default();
            let geometry = monitor.geometry();
            let name = connector.filter(|value| !value.is_empty()).unwrap_or(model);
            if name.is_empty() {
                continue;
            }
            let label = format!("{name} · {}×{}", geometry.width(), geometry.height());
            let button = screen_button(&label, Some(&automatic));
            list.append(&button);
            buttons.push((button, Some(name)));
        }
    }
    if let Some(current) = current
        && !buttons
            .iter()
            .any(|(_, monitor)| monitor.as_deref() == Some(current))
    {
        let button = screen_button(&format!("{current}（未连接）"), Some(&automatic));
        list.append(&button);
        buttons.push((button, Some(current.to_owned())));
    }
    (list, buttons)
}

fn section(root: &gtk::Box, name: &str, control: &impl IsA<gtk::Widget>) {
    // Give the four-way choices and position presets the drawer's full width.
    let stacked = matches!(name, "屏幕" | "贴边位置" | "位置" | "自动收起");
    let box_ = gtk::Box::new(
        if stacked {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        },
        if stacked { 8 } else { 14 },
    );
    box_.add_css_class("setting-row");
    let label = gtk::Label::new(Some(name));
    label.set_xalign(0.);
    label.add_css_class("setting-label");
    label.set_valign(gtk::Align::Center);
    control.as_ref().set_hexpand(true);
    box_.append(&label);
    box_.append(control);
    root.append(&box_);
}
fn choices(labels: &[&str], selected: usize) -> (gtk::Box, Vec<gtk::ToggleButton>) {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    row.add_css_class("segments");
    row.set_homogeneous(true);
    let mut buttons = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let b = gtk::ToggleButton::with_label(label);
        b.set_group(buttons.first());
        b.set_active(i == selected);
        row.append(&b);
        buttons.push(b);
    }
    (row, buttons)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires a GTK display"]
    fn appearance_rejections_restore_controls_without_submitting_another_update() {
        adw::init().unwrap();
        let current = Rc::new(RefCell::new(Config {
            theme: Theme::Paper,
            appearance: Appearance::Light,
            ..Config::default()
        }));
        let attempts = Rc::new(Cell::new(0));
        let reject = Rc::new(Cell::new(true));
        let panel = build(
            current.clone(),
            {
                let current = current.clone();
                let attempts = attempts.clone();
                let reject = reject.clone();
                move |config| {
                    attempts.set(attempts.get() + 1);
                    if reject.get() {
                        Err("Rejected for regression coverage".into())
                    } else {
                        current.replace(config);
                        Ok(())
                    }
                }
            },
            || Ok(()),
        );
        let controls = &panel.appearance;
        controls.themes[0].0.set_active(true);
        assert_eq!(attempts.get(), 1);
        assert!(controls.themes[1].0.is_active());
        assert!(!controls.mode.is_visible());
        controls.motion.set_active(true);
        assert_eq!(attempts.get(), 2);
        assert!(!controls.motion.is_active());

        current.borrow_mut().theme = Theme::Minimal;
        panel.refresh();
        controls.modes[2].set_active(true);
        assert_eq!(attempts.get(), 3);
        assert!(controls.mode.is_visible());
        assert!(controls.modes[1].is_active());

        reject.set(false);
        controls.themes[2].0.set_active(true);
        assert_eq!(attempts.get(), 4);
        assert_eq!(current.borrow().theme, Theme::Playful);
        assert!(controls.themes[2].0.is_active());
        assert!(!controls.mode.is_visible());
        reject.set(true);
        fn descendants(widget: &gtk::Widget, result: &mut Vec<gtk::Widget>) {
            result.push(widget.clone());
            let mut child = widget.first_child();
            while let Some(widget) = child {
                descendants(&widget, result);
                child = widget.next_sibling();
            }
        }
        let mut widgets = Vec::new();
        descendants(panel.root.upcast_ref(), &mut widgets);
        for widget in widgets {
            if let Some(switch) = widget.downcast_ref::<gtk::Switch>() {
                let before = switch.is_active();
                let count = attempts.get();
                switch.set_active(!before);
                assert_eq!(switch.is_active(), before, "rejected switch must revert");
                assert_eq!(
                    attempts.get(),
                    count + 1,
                    "reverting must not submit another write"
                );
            }
        }
    }
}
