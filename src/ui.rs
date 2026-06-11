use crate::state::AppState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, List, ListItem, Paragraph},
};

pub fn draw(f: &mut Frame, state: &AppState) {
    let area = f.area();

    let title_style = Style::default().fg(Color::Cyan);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(16),
            Constraint::Min(0),
        ])
        .split(area);

    let title_text = if state.paused {
        "mousse [PAUSED — p: resume, c: clear, q: quit]"
    } else {
        "mousse [p: pause, c: clear, q: quit]"
    };
    let title = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(title_text, title_style));
    f.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(7),
            Constraint::Min(0),
        ])
        .split(body_chunks[0]);

    f.render_widget(render_position(state, title_style), left_chunks[0]);
    f.render_widget(render_buttons(state, title_style), left_chunks[1]);
    f.render_widget(render_scroll_stats(state, title_style), left_chunks[2]);

    let bars: Vec<(&str, u64)> = state
        .scroll_dy_history
        .iter()
        .map(|v| ("", v.abs() as u64))
        .collect();
    let scroll_max = state
        .scroll_dy_history
        .iter()
        .fold(0.0_f64, |a, b| a.max(b.abs()))
        .max(1.0) as u64;
    f.render_widget(render_scroll_chart(&bars, scroll_max, title_style), body_chunks[1]);

    f.render_widget(render_event_log(state, title_style), chunks[2]);
}

fn render_position(state: &AppState, title_style: Style) -> Paragraph<'static> {
    let value_style = Style::default().fg(Color::Green);
    let text = Text::from(vec![
        Line::from(vec![
            Span::raw("x: "),
            Span::styled(state.mouse_x.to_string(), value_style),
        ]),
        Line::from(vec![
            Span::raw("y: "),
            Span::styled(state.mouse_y.to_string(), value_style),
        ]),
    ]);
    Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("position", title_style)),
    )
}

fn render_buttons(state: &AppState, title_style: Style) -> Paragraph<'static> {
    let btn_style = |pressed: bool| {
        if pressed {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };
    let text = Text::from(vec![
        Line::from(vec![
            Span::raw("left:    "),
            Span::styled("●", btn_style(state.btn_left)),
        ]),
        Line::from(vec![
            Span::raw("right:   "),
            Span::styled("●", btn_style(state.btn_right)),
        ]),
        Line::from(vec![
            Span::raw("middle:  "),
            Span::styled("●", btn_style(state.btn_middle)),
        ]),
        Line::from(vec![
            Span::raw("back:    "),
            Span::styled("●", btn_style(state.btn_side_back)),
        ]),
        Line::from(vec![
            Span::raw("forward: "),
            Span::styled("●", btn_style(state.btn_side_forward)),
        ]),
    ]);
    Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("buttons", title_style)),
    )
}

fn history_stats(history: &std::collections::VecDeque<f64>) -> (f64, f64, f64) {
    if history.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let min = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg = history.iter().sum::<f64>() / history.len() as f64;
    (min, max, avg)
}

fn render_scroll_stats(state: &AppState, title_style: Style) -> Paragraph<'static> {
    let (dy_min, dy_max, dy_avg) = history_stats(&state.scroll_dy_history);
    let (dx_min, dx_max, dx_avg) = history_stats(&state.scroll_dx_history);

    let dim_style = Style::default().fg(Color::DarkGray);
    let yellow = Style::default().fg(Color::Yellow);
    let cyan = Style::default().fg(Color::Cyan);
    let green = Style::default().fg(Color::Green);

    let text = Text::from(vec![
        Line::from(vec![
            Span::raw("dy: "),
            Span::styled(format!("{:.0}", state.scroll_dy), yellow),
            Span::styled("  min:", dim_style),
            Span::styled(format!("{:.0}", dy_min), cyan),
            Span::styled("  max:", dim_style),
            Span::styled(format!("{:.0}", dy_max), cyan),
            Span::styled("  avg:", dim_style),
            Span::styled(format!("{:.1}", dy_avg), green),
        ]),
        Line::from(vec![
            Span::raw("dx: "),
            Span::styled(format!("{:.0}", state.scroll_dx), yellow),
            Span::styled("  min:", dim_style),
            Span::styled(format!("{:.0}", dx_min), cyan),
            Span::styled("  max:", dim_style),
            Span::styled(format!("{:.0}", dx_max), cyan),
            Span::styled("  avg:", dim_style),
            Span::styled(format!("{:.1}", dx_avg), green),
        ]),
        Line::from(vec![
            Span::raw("events: "),
            Span::styled(state.scroll_dy_history.len().to_string(), dim_style),
        ]),
    ]);
    Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("scroll stats", title_style)),
    )
}

fn render_scroll_chart<'a>(bars: &'a [(&'a str, u64)], max: u64, title_style: Style) -> BarChart<'a> {
    BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("scroll history (dy)", title_style)),
        )
        .data(bars)
        .bar_width(1)
        .bar_gap(0)
        .max(max)
        .bar_style(Style::default().fg(Color::Cyan))
        .value_style(Style::default().fg(Color::Black))
}

fn render_event_log<'a>(state: &'a AppState, title_style: Style) -> List<'a> {
    let items: Vec<ListItem> = state
        .event_log
        .iter()
        .rev()
        .map(|e| {
            let style = if e.contains("SCROLL") {
                Style::default().fg(Color::Cyan)
            } else if e.contains("DOWN") {
                Style::default().fg(Color::Green)
            } else if e.contains("UP") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Span::styled(e.as_str(), style))
        })
        .collect();

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("event log", title_style)),
    )
}
