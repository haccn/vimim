use rand::Rng;

struct Vec2 {
	y: i32,
	x: i32,
}

fn main() {
	// todo: sound

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

	loop {
		render(&player, &enemy, max_y, max_x);

		let ch = ncurses::getch();
		if ch == 'h' as i32 { player.x -= 1; }
		else if ch == 'j' as i32 { player.y += 1; }
		else if ch == 'k' as i32 { player.y -= 1; }
		else if ch == 'l' as i32 { player.x += 1; }
		else if ch == 'x' as i32 {
			if player.y == enemy.y && player.x == enemy.x {
				place_enemy(&mut rng, &mut enemy, max_y, max_x);
			}
		}
	}
}

fn place_enemy(rng: &mut rand::rngs::ThreadRng, enemy: &mut Vec2, max_y: i32, max_x: i32) {
	enemy.y = rng.gen_range(0..max_y);
	enemy.x = rng.gen_range(0..max_x);
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
