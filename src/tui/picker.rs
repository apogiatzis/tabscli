use std::collections::HashSet;
use std::io;

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::model::tab::Tab;

struct PickerState {
    all_tabs: Vec<Tab>,
    filtered_indices: Vec<usize>,
    query: String,
    cursor: usize,
    selected: HashSet<usize>, // indices into all_tabs
    multi: bool,
}

impl PickerState {
    fn new(tabs: Vec<Tab>, multi: bool) -> Self {
        let filtered_indices: Vec<usize> = (0..tabs.len()).collect();
        Self {
            all_tabs: tabs,
            filtered_indices,
            query: String::new(),
            cursor: 0,
            selected: HashSet::new(),
            multi,
        }
    }

    fn update_filter(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_indices = (0..self.all_tabs.len()).collect();
        } else {
            self.filtered_indices = self
                .all_tabs
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    t.title.to_lowercase().contains(&q) || t.url.to_lowercase().contains(&q)
                })
                .map(|(i, _)| i)
                .collect();
        }
        // Keep cursor in bounds
        if self.cursor >= self.filtered_indices.len() {
            self.cursor = self.filtered_indices.len().saturating_sub(1);
        }
    }

    fn current_tab_index(&self) -> Option<usize> {
        self.filtered_indices.get(self.cursor).copied()
    }

    fn toggle_selection(&mut self) {
        if let Some(idx) = self.current_tab_index() {
            if self.selected.contains(&idx) {
                self.selected.remove(&idx);
            } else {
                self.selected.insert(idx);
            }
        }
    }

    fn get_result(self) -> Vec<Tab> {
        if self.multi {
            self.selected
                .into_iter()
                .filter_map(|i| self.all_tabs.get(i).cloned())
                .collect()
        } else if let Some(idx) = self.filtered_indices.get(self.cursor) {
            self.all_tabs.get(*idx).cloned().into_iter().collect()
        } else {
            vec![]
        }
    }
}

pub fn run_picker(tabs: Vec<Tab>, multi: bool) -> anyhow::Result<Vec<Tab>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

    let mut state = PickerState::new(tabs, multi);
    let mut cancelled = false;

    loop {
        terminal.draw(|f| draw(f, &state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Esc => {
                    cancelled = true;
                    break;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    cancelled = true;
                    break;
                }
                KeyCode::Enter => break,
                KeyCode::Up => {
                    state.cursor = state.cursor.saturating_sub(1);
                }
                KeyCode::Down => {
                    if state.cursor + 1 < state.filtered_indices.len() {
                        state.cursor += 1;
                    }
                }
                KeyCode::Tab if multi => {
                    state.toggle_selection();
                    // Move cursor down after toggle
                    if state.cursor + 1 < state.filtered_indices.len() {
                        state.cursor += 1;
                    }
                }
                KeyCode::Backspace => {
                    state.query.pop();
                    state.update_filter();
                }
                KeyCode::Char(c) => {
                    state.query.push(c);
                    state.update_filter();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    if cancelled {
        Ok(vec![])
    } else {
        Ok(state.get_result())
    }
}

fn draw(f: &mut Frame, state: &PickerState) {
    let area = f.area();
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(1),    // Tab list
            Constraint::Length(1), // Help line
        ])
        .split(area);

    // Search input
    let counter = format!("{}/{}", state.filtered_indices.len(), state.all_tabs.len());
    let input_text = format!("> {}", state.query);
    let input = Paragraph::new(Line::from(vec![
        Span::styled(&input_text, Style::default().fg(Color::Yellow)),
        Span::styled(
            "_",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
        Span::raw(" ".repeat(chunks[0].width as usize)),
        Span::styled(&counter, Style::default().fg(Color::DarkGray)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, chunks[0]);

    // Tab list
    let visible_height = chunks[1].height as usize;
    let scroll_offset = if state.cursor >= visible_height {
        state.cursor - visible_height + 1
    } else {
        0
    };

    let items: Vec<ListItem> = state
        .filtered_indices
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_height)
        .map(|(i, &tab_idx)| {
            let tab = &state.all_tabs[tab_idx];
            let is_selected = state.selected.contains(&tab_idx);
            let is_cursor = i == state.cursor;

            let marker = if state.multi {
                if is_selected { "[x] " } else { "[ ] " }
            } else {
                ""
            };

            let prefix = if is_cursor { "> " } else { "  " };
            let domain = tab.domain();
            let title = truncate(
                &tab.title,
                (chunks[1].width as usize)
                    .saturating_sub(domain.len() + marker.len() + prefix.len() + 4),
            );

            let style = if is_cursor {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(marker, Style::default().fg(Color::Cyan)),
                Span::styled(title, style),
                Span::raw("  "),
                Span::styled(domain, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::NONE));
    f.render_widget(list, chunks[1]);

    // Help line
    let help = if state.multi {
        " ↑↓ navigate  enter confirm  tab toggle  esc quit"
    } else {
        " ↑↓ navigate  enter select  esc quit"
    };
    let help_line = Paragraph::new(Span::styled(help, Style::default().fg(Color::DarkGray)));
    f.render_widget(help_line, chunks[2]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max.saturating_sub(3)).collect();
        format!("{}...", truncated)
    }
}
