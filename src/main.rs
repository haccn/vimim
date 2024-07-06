mod audioman;

use rand::Rng;

struct Vec2 {
	y: i32,
	x: i32,
}

fn main() {
	let aman = audioman::AudioMan::new();

	ncurses::initscr();
	ncurses::start_color();
	ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

	let mut max_y = 0;
	let mut max_x = 0;
	ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);

	let mut rng = rand::thread_rng();

	let mut player = Vec2 {
		y: max_y / 2,
		x: max_x / 2,
	};
	let mut enemy = Vec2 {
		y: 0,
		x: 0,
	};
	place_enemy(&mut rng, &mut enemy, max_y, max_x);

	let mut cmd_buf = String::new();

	loop {
		render(&player, &enemy, max_y, max_x);

		let ch = ncurses::getch();
		if ch == 'h' as i32 {
			move_player(&mut cmd_buf, 0, -1, &mut player, &aman);
		}
		else if ch == 'j' as i32 {
			move_player(&mut cmd_buf, 1, 0, &mut player, &aman);
		}
		else if ch == 'k' as i32 {
			move_player(&mut cmd_buf, -1, 0, &mut player, &aman);
		}
		else if ch == 'l' as i32 {
			move_player(&mut cmd_buf, 0, 1, &mut player, &aman);
		}
		else if ch == '0' as i32 {
			player.x = 0;
			cmd_buf.clear();
			aman.play(audioman::WOOSH_BYTES);
		}
		else if ch == '^' as i32 {
			if player.y == enemy.y {
				player.x = enemy.x;
				aman.play(audioman::WOOSH_BYTES);
			}
			else {
				player.x = 0;
				aman.play(audioman::WOOSH_BYTES);
			}
			cmd_buf.clear();
		}
		else if ch == '$' as i32 {
			if player.y == enemy.y {
				player.x = enemy.x + 1;
				aman.play(audioman::WOOSH_BYTES);
			}
			else {
				player.x = max_x - 1;
				aman.play(audioman::WOOSH_BYTES);
			}
			cmd_buf.clear();
		}
		else if ch == 'G' as i32 {
			if cmd_buf.is_empty() {
				player.y = max_y - 1;
				aman.play(audioman::WOOSH_BYTES);
			}
			else if let Ok(y) = cmd_buf.parse::<i32>() {
				player.y = y - 1;
				aman.play(audioman::WOOSH_BYTES);
			}
			cmd_buf.clear();
		}
		else if ch == 'g' as i32 {
			let chars = cmd_buf.chars();
			if let Some(lastch) = chars.last() {
				if lastch == 'g' {
					cmd_buf.pop();
					if cmd_buf.is_empty() {
						player.y = 0;
						aman.play(audioman::WOOSH_BYTES);
					}
					else if let Ok(y) = cmd_buf.parse::<i32>() {
						player.y = y - 1;
						aman.play(audioman::WOOSH_BYTES);
					}
					cmd_buf.clear();
				}
				else {
					cmd_buf.push(char::from_u32(ch as u32).unwrap());
				}
			}
			else {
				cmd_buf.push(char::from_u32(ch as u32).unwrap());
			}
		}
		else if ch == 'x' as i32 {
			if player.y == enemy.y && player.x == enemy.x {
				aman.play(audioman::EXPLOSION_BYTES);
				place_enemy(&mut rng, &mut enemy, max_y, max_x);
			}
			cmd_buf.clear();
		}
		else {
			cmd_buf.push(char::from_u32(ch as u32).unwrap());
		}

		ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);
		player.y = player.y.clamp(0, max_y - 1);
		player.x = player.x.clamp(3, max_x - 1);
	}
}

fn place_enemy(rng: &mut rand::rngs::ThreadRng, enemy: &mut Vec2, max_y: i32, max_x: i32) {
	enemy.y = rng.gen_range(0..max_y - 1);
	enemy.x = rng.gen_range(3..max_x - 1);
}

fn render(player: &Vec2, enemy: &Vec2, max_y: i32, max_x: i32) {
	ncurses::clear();

	// todo: add color to highlight current line
	//ncurses::init_color(0, 0, 43 * 4, 54 * 4);
	//ncurses::init_pair(2, 0, 0);
	//ncurses::attron(2);
	let mut i = 0;
	while i <= max_y {
		let n = if i == player.y { i + 1 } else { (player.y - i).abs() };
		ncurses::mvaddstr(i, 0, &n.to_string()).unwrap();
		i += 1;
	}

	ncurses::mvaddch(enemy.y, enemy.x, 'x' as u32);
	ncurses::mvaddch(player.y, player.x, 'o' as u32);
}

fn move_player(cmd_buf: &mut String, y: i32, x: i32, player: &mut Vec2, aman: &audioman::AudioMan) {
	if cmd_buf.is_empty() {
		player.y += y;
		player.x += x;
		aman.play_rand_footstep();
	}
	else if let Ok(count) = cmd_buf.parse::<i32>() {
		player.y += y * count;
		player.x += x * count;
		aman.play(audioman::WOOSH_BYTES);
	}
	cmd_buf.clear();
}
