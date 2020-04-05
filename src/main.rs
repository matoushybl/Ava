use crate::utils::event::{Event, Events};
use crate::utils::stateful_list::StatefulList;
use huelib::{Group, Scene};
use std::io;
use std::net::IpAddr;
use std::str::FromStr;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph, Text};
use tui::Terminal;

use crate::services::hue_service::HueService;
use crate::state::{AppState, SelectedList};

mod services;
mod state;
mod utils;

fn main() {
    // let addresses = huelib::bridge::discover().unwrap();
    // let a: Vec<String> = addresses.into_iter().map(|addr| addr.to_string()).collect();

    let user = "pt3ugNVaTX5AzEJrJL8HBAX9BHJ3yTgBIspgeRbN";
    let service = HueService::new(IpAddr::from_str("10.15.0.218").unwrap(), user);

    let stdout = io::stdout()
        .into_raw_mode()
        .expect("Failed to put stdout into raw mode.");
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal.");
    terminal.hide_cursor().expect("Failed to hide cursor.");

    let mut app = AppState::new(service.get_all_groups(), service.get_all_scenes());
    let events = Events::new();

    loop {
        terminal
            .draw(|mut frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                    .split(frame.size());

                let inner_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[0]);

                let style = Style::default().fg(Color::Black);

                let items_view = List::new(app.groups.items.iter().map(|item| {
                    Text::raw(format!(
                        "{} {}",
                        item.name,
                        if item.state.map(|s| s.any_on).unwrap_or(false) {
                            "*"
                        } else {
                            " "
                        }
                    ))
                }))
                .block(Block::default().borders(Borders::ALL).title("Lights"))
                .highlight_style(style.fg(Color::Red).modifier(Modifier::BOLD))
                .highlight_symbol(if app.selected_list == SelectedList::Lights {
                    ">"
                } else {
                    ""
                });

                frame.render_stateful_widget(items_view, inner_chunks[0], &mut app.groups.state);

                let scenes_view = List::new(
                    app.scenes
                        .items
                        .iter()
                        .map(|item| Text::raw(item.name.as_str())),
                )
                .block(Block::default().borders(Borders::ALL).title("Scenes"))
                .highlight_style(style.fg(Color::Red).modifier(Modifier::BOLD))
                .highlight_symbol(if app.selected_list == SelectedList::Scenes {
                    ">"
                } else {
                    ""
                });

                frame.render_stateful_widget(scenes_view, inner_chunks[1], &mut app.scenes.state);

                let legend_text = vec![Text::raw("q - Quit, hjkl - Navigation, space - On/Off")];
                let legend = Paragraph::new(legend_text.iter());

                frame.render_widget(legend, chunks[1]);
            })
            .expect("Failed to draw frame.");

        match events.next().expect("Failed to get next event.") {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left | Key::Char('h') => {
                    if app.selected_list == SelectedList::Scenes {
                        app.selected_list = SelectedList::Lights;
                        app.scenes.state.select(None);
                    }
                }
                Key::Right | Key::Char('l') => {
                    if app.selected_list == SelectedList::Lights {
                        app.selected_list = SelectedList::Scenes;
                        app.scenes.state.select(Some(0usize));
                    }
                }
                Key::Down | Key::Char('j') => match app.selected_list {
                    SelectedList::Lights => app.groups.next(),
                    SelectedList::Scenes => app.scenes.next(),
                },
                Key::Up | Key::Char('k') => match app.selected_list {
                    SelectedList::Lights => app.groups.previous(),
                    SelectedList::Scenes => app.scenes.previous(),
                },
                Key::Char(' ') => match app.selected_list {
                    SelectedList::Lights => {
                        if let Some(index) = app.groups.state.selected() {
                            let light = app.groups.items[index].clone();
                            service.toggle_group(&light);

                            app.groups.replace(service.get_all_groups());
                            app.scenes.replace(service.get_all_scenes());
                        }
                    }
                    SelectedList::Scenes => {
                        if let (Some(light_index), Some(scene_index)) =
                            (app.groups.state.selected(), app.scenes.state.selected())
                        {
                            let group = app.groups.items[light_index].clone();
                            let scene = app.scenes.items[scene_index].clone();

                            service.set_scene_to_group(&group, &scene);

                            app.groups.replace(service.get_all_groups());
                            app.scenes.replace(service.get_all_scenes());
                        }
                    }
                },
                _ => {}
            },
            Event::Tick => {}
        }
    }
}
