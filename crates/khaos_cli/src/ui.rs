use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(7),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new(Line::from(vec![
        Span::styled(" khaos ", Style::default().fg(Color::Black).bg(Color::Red)),
        Span::raw(" physical entropy workbench"),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    let report = Paragraph::new(report_lines(app))
        .block(Block::default().title(" Sample ").borders(Borders::ALL));

    let output = Paragraph::new(output_lines(app))
        .block(Block::default().title(" Output ").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    let help = Paragraph::new(Line::from(vec![
        Span::styled(" g ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("generate 32 bytes   "),
        Span::styled(" q ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("quit"),
    ]))
    .block(Block::default().borders(Borders::TOP));

    frame.render_widget(title, areas[0]);
    frame.render_widget(report, areas[1]);
    frame.render_widget(output, areas[2]);
    frame.render_widget(help, areas[3]);
}

fn report_lines(app: &App) -> Vec<Line<'static>> {
    let Some(report) = app.report() else {
        return vec![
            Line::from("No sample loaded."),
            Line::from("Run: cargo run -p khaos_cli -- path/to/sample.bin"),
        ];
    };

    vec![
        Line::from(format!("Input bytes: {}", report.input_bytes)),
        Line::from(format!("Unbiased bits: {}", report.unbiased_bits)),
        Line::from(format!("Raw ones ratio: {:.4}", report.ones_ratio)),
    ]
}

fn output_lines(app: &App) -> Vec<Line<'static>> {
    if let Some(error) = app.error() {
        return vec![Line::styled(
            error.to_owned(),
            Style::default().fg(Color::Red),
        )];
    }

    if let Some(output) = app.output() {
        return vec![Line::from(to_hex(output))];
    }

    if app.has_sample() {
        vec![Line::from("Press g to mix the sample and generate bytes.")]
    } else {
        vec![Line::from("Load a sample file to enable generation.")]
    }
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
