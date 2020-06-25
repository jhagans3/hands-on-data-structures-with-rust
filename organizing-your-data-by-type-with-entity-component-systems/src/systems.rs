use crate::data::*;
use crate::gen::GenerationManager;
use crate::store::EcsStore;
use termion::raw::RawTerminal;
use termion::{color, cursor};

// this can also be impl with structures instead of functions
// directions do not change, but the  positions do
pub fn move_sys<D: EcsStore<Direction>, P: EcsStore<Position>>(dir_list: &D, pos_list: &mut P) {
    pos_list.for_each_mut(|g, p| {
        if let Some(d) = dir_list.get(g) {
            p.x += d.velocity_x;
            p.y += d.velocity_y;
        }
    })
}

// positions do not change, but the directions do
pub fn dir_sys<D: EcsStore<Direction>, P: EcsStore<Position>>(dir_list: &mut D, pos_list: &P) {
    // better ways to handle unwrap, not sure if termion works 100% for windows
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    dir_list.for_each_mut(|g, dr| {
        match rand::random::<u8>() % 5 {
            0 => dr.velocity_x += 1,
            1 => dr.velocity_x -= 1,
            2 => dr.velocity_y += 1,
            3 => dr.velocity_y += 1,
            _ => {}
        }

        dr.velocity_x = std::cmp::min(3, dr.velocity_x);
        dr.velocity_y = std::cmp::min(3, dr.velocity_y);
        dr.velocity_x = std::cmp::max(-3, dr.velocity_x);
        dr.velocity_y = std::cmp::max(-3, dr.velocity_y);
        if let Some(p) = pos_list.get(g) {
            if p.x < 4 {
                dr.velocity_x = 1
            }
            if p.y < 4 {
                dr.velocity_y = 1
            }
            if p.x + 4 > w {
                dr.velocity_x = -1
            }
            if p.y + 4 > h {
                dr.velocity_x = -1
            }
        }
    });
}

pub fn collision_sys<P: EcsStore<Position>, S: EcsStore<Strength>>(
    pos_list: &P,
    strength_list: &mut S,
) {
    let mut collisions = Vec::new();
    pos_list.for_each(|outer_gen, outer_pos| {
        pos_list.for_each(|inner_gen, inner_pos| {
            if (inner_pos == outer_pos) && (inner_gen != outer_gen) {
                collisions.push((outer_gen, inner_gen));
            }
        }); // O(n^2)
    });

    for (outer_gen, inner_gen) in collisions {
        let damage = match strength_list.get(outer_gen) {
            Some(b) => b.strength,
            None => continue,
        };

        let health_up = if let Some(bumpee) = strength_list.get_mut(inner_gen) {
            let n = bumpee.strength + 1;
            bumpee.health -= damage;
            if bumpee.health <= 0 {
                n
            } else {
                0
            };

            if health_up > 0 {
                if let Some(bumper) = strength_list.get_mut(outer_gen) {
                    bumper.health += health_up;
                    bumper.strength += 1;
                }
            }
        };
    }
}

pub fn render_sys<T: std::io::Write, P: EcsStore<Position>, S: EcsStore<Strength>>(
    t: &mut RawTerminal<T>,
    pos_list: &P,
    strength_list: &S,
) {
    write!(t, "{}", termion::clear::All).ok();
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    pos_list.for_each(|g, p| {
        if let Some(st) = strength_list.get(g) {
            let col = match st.health {
                0 => color::Fg(color::Black).to_string(),
                1 => color::Fg(color::Red).to_string(),
                2 => color::Fg(color::Yellow).to_string(),
                3 => color::Fg(color::Green).to_string(),
                _ => color::Fg(color::Blue).to_string(),
            };

            // term positions start at 1
            let x = (p.x % w) + 1;
            let y = (p.y % h) + 1;
            write!(
                t,
                "{}{}{}",
                cursor::Goto(x as u16, y as u16),
                col,
                st.strength,
            )
            .ok();
        }
    });
}

pub fn death_sys<S: EcsStore<Strength>, P: EcsStore<Position>, D: EcsStore<Direction>>(
    gen_man: &mut GenerationManager,
    strength_list: &mut S,
    pos_list: &mut P,
    dir_list: &mut D,
) {
    let mut to_kill = Vec::new();
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    pos_list.for_each(|g, p| {
        if p.x > w || p.x < 0 || p.y > h || p.y < 0 {
            to_kill.push(g);
        }
    });
    strength_list.for_each(|g, s| {
        if s.health <= 0 {
            to_kill.push(g);
        }
    });
    for tk in to_kill {
        gen_man.drop(tk);
        pos_list.drop(tk);
        strength_list.drop(tk);
        dir_list.drop(tk);
    }
}
