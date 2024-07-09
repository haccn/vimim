mod audioman;

use crossterm::{QueueableCommand, cursor, style, terminal};
use rand::Rng;
use std::io::Write;

struct Vec2 {
	x: i32,
	y: i32,
}

fn main() {
	let aman = audioman::AudioMan::new();
	let mut rng = rand::thread_rng();
	let mut stdout = std::io::stdout();

	terminal::enable_raw_mode().unwrap();
	stdout
		.queue(cursor::SetCursorStyle::SteadyBlock).unwrap()
		.flush().unwrap();

	let (mut max_x_u16, mut max_y_u16) = terminal::size().unwrap();
	let mut max_x = max_x_u16 as i32;
	let mut max_y = max_y_u16 as i32;

	let mut player = Vec2 {
		x: max_x / 2,
		y: max_y / 2,
	};
	let mut enemy = Vec2 {
		x: 0,
		y: 0,
	};
	place_enemy(&mut rng, &mut enemy, max_x, max_y);

	let mut motion_buf = String::new();

	loop {
		render(&mut stdout, &player, &enemy, max_x, max_y, &motion_buf);

		use crossterm::event::Event::*;
		match crossterm::event::read().unwrap() {
			Key(key_evt) => {
				use crossterm::event::KeyCode::*;
				match key_evt.code {
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
						if motion_buf.is_empty() {
							player.x = 0;
							motion_buf.clear();
							aman.play(audioman::WOOSH_BYTES);
						}
						else {
							motion_buf.push('0');
						}
					}
					Char('^') => {
						if player.y == enemy.y {
							player.x = enemy.x;
							aman.play(audioman::WOOSH_BYTES);
						}
						else {
							player.x = 0;
							aman.play(audioman::WOOSH_BYTES);
						}
						motion_buf.clear();
					}
					Char('$') => {
						if player.y == enemy.y {
							player.x = enemy.x;
							aman.play(audioman::WOOSH_BYTES);
						}
						else {
							player.x = max_x;
							aman.play(audioman::WOOSH_BYTES);
						}
						motion_buf.clear();
					}
					Char('G') => {
						if motion_buf.is_empty() {
							player.y = max_y;
							aman.play(audioman::WOOSH_BYTES);
						}
						else if let Ok(y) = motion_buf.parse::<i32>() {
							player.y = y;
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
									player.y = 0;
									aman.play(audioman::WOOSH_BYTES);
								}
								else if let Ok(y) = motion_buf.parse::<i32>() {
									player.y = y;
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
					Char(ch) => {
						motion_buf.push(ch);
					}
					Enter => {
						if motion_buf == ":q" {
							// todo: return terminal to prev state (alt terminals?)
							stdout
								.queue(terminal::Clear(terminal::ClearType::All)).unwrap()
								.flush().unwrap();
							std::process::exit(0);
						}
						else {
							motion_buf.push('q');
						}
					}
					Esc => {
						motion_buf.clear();
					}
					_ => {}
				}
			}
			_ => {}
		}

		(max_x_u16, max_y_u16) = terminal::size().unwrap();
		max_x = max_x_u16 as i32;
		max_y = max_y_u16 as i32;
		player.x = player.x.clamp(2, max_x - 1);
		player.y = player.y.clamp(0, max_y - 2);
	}
}

fn place_enemy(rng: &mut rand::rngs::ThreadRng, enemy: &mut Vec2, max_x: i32, max_y: i32) {
	enemy.x = rng.gen_range(2..max_x - 1);
	enemy.y = rng.gen_range(0..max_y - 2);
}

fn render(stdout: &mut std::io::Stdout, player: &Vec2, enemy: &Vec2, max_x: i32, max_y: i32, motion_buf: &String) {
	stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
	// numbers
	stdout.queue(style::SetForegroundColor(style::Color::DarkGrey)).unwrap();
	let mut i = 0;
	while i <= max_y - 2 {
		stdout.queue(cursor::MoveTo(0, i as u16)).unwrap();
		if i == player.y {
			stdout
				.queue(style::SetColors(style::Colors::new(style::Color::Reset, style::Color::DarkGrey))).unwrap()
				.queue(style::Print(format!("{: <1$}", i + 1, max_x as usize))).unwrap()
				.queue(style::SetForegroundColor(style::Color::DarkGrey)).unwrap()
				.queue(style::SetBackgroundColor(style::Color::Reset)).unwrap();
		}
		else {
			stdout.queue(style::Print((player.y - i).abs().to_string())).unwrap();
		}
		i += 1;
	}
	stdout
		.queue(style::ResetColor).unwrap()
		// motion buf
		.queue(cursor::MoveTo(0, max_y as u16)).unwrap()
		.queue(style::Print(motion_buf)).unwrap()
		// enemy
		.queue(cursor::MoveTo(enemy.x as u16, enemy.y as u16)).unwrap()
		.queue(style::Print('x')).unwrap()
		// player
		.queue(cursor::MoveTo(player.x as u16, player.y as u16)).unwrap()
		.flush().unwrap();
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
