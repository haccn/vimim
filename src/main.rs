mod audioman;

use rand::Rng;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;

struct Vec2 {
	x: i32,
	y: i32,
}

fn main() {
	let aman = audioman::AudioMan::new();
	let mut rng = rand::thread_rng();
	let mut stdin = std::io::stdin().keys();
	let mut stdout = std::io::stdout().lock().into_raw_mode().unwrap();

	write!(stdout, "{}", termion::cursor::SteadyBlock).unwrap();
	stdout.flush().unwrap();

	let (mut max_x_u16, mut max_y_u16) = termion::terminal_size().unwrap();
	let mut max_x = max_x_u16 as i32;
	let mut max_y = max_y_u16 as i32;

	let mut player = Vec2 {
		x: max_x / 2,
		y: max_y / 2,
	};
	let mut enemy = Vec2 {
		x: 1,
		y: 1,
	};
	place_enemy(&mut rng, &mut enemy, max_x, max_y);

	let mut motion_buf = String::new();

	loop {
		render(&mut stdout, &player, &enemy, max_x, max_y, &motion_buf);

		let evt = stdin.next().unwrap().unwrap();
		use termion::event::Key::*;
		match evt {
			Char('h') => {
				move_player(&mut motion_buf, -1, 0, &mut player, &aman);
			}
			Char('j') => {
				move_player(&mut motion_buf, 0, 1, &mut player, &aman);
			}
			Char('k') => {
				move_player(&mut motion_buf, 0, -1, &mut player, &aman);
			}
			Char('l') => {
				move_player(&mut motion_buf, 1, 0, &mut player, &aman);
			}
			Char('0') => {
				player.x = 1;
				motion_buf.clear();
				aman.play(audioman::WOOSH_BYTES);
			}
			Char('^') => {
				if player.y == enemy.y {
					player.x = enemy.x;
					aman.play(audioman::WOOSH_BYTES);
				}
				else {
					player.x = 1;
					aman.play(audioman::WOOSH_BYTES);
				}
				motion_buf.clear();
			}
			Char('$') => {
				if player.y == enemy.y {
					player.x = enemy.x + 1;
					aman.play(audioman::WOOSH_BYTES);
				}
				else {
					player.x = max_x - 1;
					aman.play(audioman::WOOSH_BYTES);
				}
				motion_buf.clear();
			}
			Char('G') => {
				if motion_buf.is_empty() {
					player.y = max_y - 1;
					aman.play(audioman::WOOSH_BYTES);
				}
				else if let Ok(y) = motion_buf.parse::<i32>() {
					player.y = y - 1;
					aman.play(audioman::WOOSH_BYTES);
				}
				motion_buf.clear();
			}
			Char('g') => {
				let chars = motion_buf.chars();
				if let Some(lastch) = chars.last() {
					if lastch == 'g' {
						motion_buf.pop();
						if motion_buf.is_empty() {
							player.y = 1;
							aman.play(audioman::WOOSH_BYTES);
						}
						else if let Ok(y) = motion_buf.parse::<i32>() {
							player.y = y - 1;
							aman.play(audioman::WOOSH_BYTES);
						}
						motion_buf.clear();
					}
					else {
						motion_buf.push('g');
					}
				}
				else {
					motion_buf.push('g');
				}
			}
			Char('x') => {
				if player.x == enemy.x && player.y == enemy.y {
					aman.play(audioman::EXPLOSION_BYTES);
					place_enemy(&mut rng, &mut enemy, max_x, max_y);
				}
				motion_buf.clear();
			}
			Char('\n') => {
				if motion_buf == ":q" {
					// todo: return terminal to prev state (alt terminals?)
					std::process::exit(0);
				}
				else {
					motion_buf.push('q');
				}
			}
			Char(ch) => {
				motion_buf.push(ch);
			}
			Esc => {
				motion_buf.clear();
			}
			_ => {}
		}

		(max_x_u16, max_y_u16) = termion::terminal_size().unwrap();
		max_x = max_x_u16 as i32;
		max_y = max_y_u16 as i32;
		player.x = player.x.clamp(3, max_x - 1);
		player.y = player.y.clamp(1, max_y - 1);
	}
}

fn place_enemy(rng: &mut rand::rngs::ThreadRng, enemy: &mut Vec2, max_x: i32, max_y: i32) {
	enemy.x = rng.gen_range(3..max_x - 1);
	enemy.y = rng.gen_range(1..max_y - 1);
}

fn render(stdout: &mut std::io::StdoutLock, player: &Vec2, enemy: &Vec2, max_x: i32, max_y: i32, motion_buf: &String) {
	// todo: add color to highlight current line
	let mut numbers = String::new();
	let mut i = 1;
	while i <= max_y {
		if i == player.y {
			numbers.push_str(&termion::color::Bg(termion::color::Rgb(64, 64, 64)).to_string());
			numbers.push_str(&termion::cursor::Goto(1, i as u16).to_string());
			numbers.push_str(&i.to_string());
			numbers.push_str(&termion::color::Bg(termion::color::Black).to_string());
		}
		else {
			numbers.push_str(&termion::cursor::Goto(1, i as u16).to_string());
			numbers.push_str(&(player.y - i).abs().to_string());
		};
		i += 1;
	}

	write!(stdout,
		"{}{}{}{}{}{}{}x{}",
		termion::clear::All,
		termion::color::Fg(termion::color::Rgb(128, 128, 128)),
		numbers,
		termion::style::Reset,
		termion::cursor::Goto(1, max_y as u16),
		motion_buf,
		termion::cursor::Goto(enemy.x as u16, enemy.y as u16),
		termion::cursor::Goto(player.x as u16, player.y as u16),
	).unwrap();
	stdout.flush().unwrap();
}

fn move_player(motion_buf: &mut String, x: i32, y: i32, player: &mut Vec2, aman: &audioman::AudioMan) {
	if motion_buf.is_empty() {
		player.x = player.x.saturating_add(x);
		player.y = player.y.saturating_add(y);
		aman.play_rand_footstep();
	}
	else if let Ok(count) = motion_buf.parse::<i32>() {
		player.x = player.x.saturating_add(x * count);
		player.y = player.y.saturating_add(y * count);
		aman.play(audioman::WOOSH_BYTES);
	}
	motion_buf.clear();
}
