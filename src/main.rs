use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::*;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::*;
use tui::text::*;

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tasks = &[
        ("   285", "connection-handler", "T", " 24.5", "  1.4", "  713", "remote-address=127.0.0.1:56723, request-id=dbabfa1a-f722-41c0-82dc-a02e88e55d2a"),
        ("   286", "connection-handler", "S", "  1.9", "  1.1", "  692", "remote-address=127.0.0.1:34135, request-id=2087d5f8-7275-4179-a0b4-5ed285b0d988"),
        ("     1", "public-accept",      "I", "  0.6", "  0.1", "  501", "local-address=127.0.0.1:8080"),
        ("     0", "main",               "I", "  0.0", "  0.0", "  106", ""),
    ];

    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
            ].as_ref())
            .split(size);

        // let system = Paragraph::new(vec![
        //     Spans::from(Span::raw("tokio-console - 1 runtime,  queue depth: 1.25, 1.12, 0.82    poll times (p99): 1.2ms, 0.9ms, 0.6ms")),
        //     Spans::from(Span::raw("Tasks: 4 total,    1 running, 366 sleeping,   0 deadlocked")),
        // ])
        // ;

        let system = Paragraph::new("\
Tokio - 1 runtime,  8 threads                                    Scheduler depth: 1.25, 1.12, 0.82
  Tasks:  4 total,  1 running,  366 sleeping,  0 deadlocked      Poll times (ms): 1.2,  0.9,  0.6
   %Run:  27.0,  22.1,  18.7                                    Wake to run (μs): 683,  502,  216"
        );

        f.render_widget(system, chunks[0]);

            /*
        let block = Block::default()
            .title("Tokio Runtime")
            .borders(Borders::NONE);

        f.render_widget(block, chunks[0]);
        */

        let rows: Vec<_> = tasks.iter().map(|c| {
            Row::new(vec![c.0, c.1, c.2, c.3, c.4, c.5, c.6])
        })
        .collect();

        let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["   TID", "NAME", "S", " %RUN", " POLL", " WAKE", "ATTRIBUTES"])
                .style(Style::default().fg(Color::Black).bg(Color::White))
        )
        // .block(Block::default().title("What"))
        .widths(&[
            Constraint::Length(6),
            Constraint::Percentage(20),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(30),
        ])
        .column_spacing(1)
        // .block(
        //     Block::default()
        //         .title("Tasks")
        //         .borders(Borders::ALL)
        // )
        ;

        f.render_widget(table, chunks[1]);

        /*
        let block = Block::default()
            .title("Tasks")
            .borders(Borders::NONE);

        f.render_widget(block, chunks[1]);
        */

    })?;

    std::thread::sleep(std::time::Duration::from_secs(3600));

    Ok(())
}
