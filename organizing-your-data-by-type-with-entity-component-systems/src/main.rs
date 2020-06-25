mod data;
mod gen;
mod store;
mod systems;

use std::io::Write;
use store::EcsStore;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // cargo run --bin organizing-your-data-by-type-with-entity-component-systems
    // get keyboard input in a thread
    let (chan_send, chan_read) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        // keys depends on the TermRead Trait
        for key in stdin.keys() {
            chan_send.send(key).ok();
        }
    });

    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = gen::GenerationManager::new();
    let mut strengths = store::VecStore::new();
    let mut dirs = store::VecStore::new();
    let mut poss = store::VecStore::new();
    let mut screen_refresh_count = 0;

    loop {
        // create one element per loop (choice not requirement)
        let g = gen.next();
        strengths.add(
            g,
            data::Strength {
                strength: 1,
                health: 5,
            },
        );
        dirs.add(
            g,
            data::Direction {
                velocity_x: 0,
                velocity_y: 0,
            },
        );
        poss.add(
            g,
            data::Position {
                x: rand::random::<i32>() % w,
                y: rand::random::<i32>() % h,
            },
        );

        systems::dir_sys(&mut dirs, &poss);
        systems::move_sys(&dirs, &mut poss);
        systems::collision_sys(&poss, &mut strengths);
        systems::death_sys(&mut gen, &mut strengths, &mut poss, &mut dirs);
        systems::render_sys(&mut screen, &poss, &strengths);
        // print pass too
        write!(
            &mut screen,
            "{}Refresh:{}",
            termion::cursor::Goto(1, 1),
            screen_refresh_count
        )
        .ok();
        screen_refresh_count += 1;
        screen.flush().ok();

        while let Ok(Ok(key)) = chan_read.try_recv() {
            match key {
                Key::Char('q') => return,
                // Here handle any key presses to make the game do stuff
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
