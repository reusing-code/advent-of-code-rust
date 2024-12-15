use std::{time, usize};

use advent_of_code::{split_by_empt_line, template::Day, Coord2D, Field, DIRECTIONS};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct Warehouse {
    field: Field<char>,
    current: Coord2D,
    steps: Vec<char>,
    current_step: i64,
    exit: bool,
    score: i64,
    manual: bool,
}

impl Warehouse {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));
            self.handle_events();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) {
        let ev = event::poll(time::Duration::from_millis(100));
        let mut dir = ' ';

        if ev.is_ok() && ev.unwrap() {
            match event::read().unwrap() {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Left => {
                            self.manual = true;
                            dir = '<'
                        }
                        KeyCode::Right => {
                            self.manual = true;
                            dir = '>'
                        }
                        KeyCode::Up => {
                            self.manual = true;
                            dir = '^'
                        }
                        KeyCode::Down => {
                            self.manual = true;
                            dir = 'v'
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        self.do_step(dir);
    }

    fn do_step(&mut self, dir: char) {
        if self.manual {
            if dir == ' ' {
                return;
            }
            do_move(&mut self.field, &mut self.current, dir);
        } else {
            if self.current_step >= self.steps.len() as i64 {
                return;
            }
            do_move(
                &mut self.field,
                &mut self.current,
                self.steps[self.current_step as usize],
            );
            self.current_step += 1;
        }
        self.score = calc_score(&self.field);
    }
}

impl Widget for &Warehouse {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![];
        for y in 0..self.field.h {
            lines.push(Line::from(
                self.field.data[y * self.field.w..(y + 1) * self.field.w]
                    .iter()
                    .map(|x| match *x {
                        '.' => " ".black(),
                        '#' => "#".gray().bold(),
                        '@' => "@".yellow().bold(),
                        'O' => "O".green().bold(),
                        '[' => "[".green().bold(),
                        ']' => "]".green().bold(),
                        _ => "".red(),
                    })
                    .collect::<Vec<_>>(),
            ));
        }
        let title = Line::from(" Warehouse robot ".bold());
        let score = Line::from(vec!["Score: ".into(), format!("{}", self.score).blue()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(score.centered())
            .border_set(border::THICK);

        let field_text = Text::from(lines);

        Paragraph::new(field_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() {
    let input = advent_of_code::template::read_file("inputs", Day::new(15).unwrap());
    run(&input);
}

fn run(input: &str) {
    let mut terminal = ratatui::init();
    let split_empyline = split_by_empt_line(input);
    let field = Field::<char>::parse_vec(&split_empyline[0]);
    let current = field.coord_from_index(field.data.iter().position(|x| *x == '@').unwrap() as i64);

    let mut steps = vec![];
    for move_line in &split_empyline[1] {
        steps.extend(move_line.chars());
    }

    let mut wh = Warehouse {
        field,
        steps,
        current,
        current_step: 0,
        exit: false,
        score: 0,
        manual: false,
    };
    wh.run(&mut terminal);
    ratatui::restore();
}

fn do_move(field: &mut Field<char>, current: &mut Coord2D, c: char) {
    let dir = char_to_direction(c);
    let mut move_stack = vec![];
    let mut next = current.add(dir);
    while field.get_coord(&next).unwrap() == 'O' {
        move_stack.push(next.clone());
        next = next.add(dir);
    }
    if field.get_coord(&next).unwrap() == '#' {
        return;
    }

    let _ = move_stack
        .iter()
        .rev()
        .map(|x| {
            *field.get_coord_mut(&x.add(dir)).unwrap() = 'O';
        })
        .collect::<Vec<_>>();
    *field.get_coord_mut(current).unwrap() = '.';
    *current = current.add(dir);
    *field.get_coord_mut(current).unwrap() = '@';
}

fn char_to_direction(c: char) -> &'static Coord2D {
    match c {
        '>' => &DIRECTIONS[0],
        '<' => &DIRECTIONS[1],
        'v' => &DIRECTIONS[2],
        '^' => &DIRECTIONS[3],
        _ => &DIRECTIONS[0],
    }
}

fn calc_score(field: &Field<char>) -> i64 {
    let mut result = 0;
    for (i, c) in field.data.iter().enumerate() {
        if *c == 'O' || *c == '[' {
            let coord = field.coord_from_index(i as i64);
            result += coord.x + 100 * coord.y;
        }
    }
    result
}
