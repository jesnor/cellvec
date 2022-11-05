use cellvec::{
    cell_set::{ArrayCellSet, CellSet, VecCellSet},
    mcell::MCell,
    ptr::Ptr,
};
use std::rc::Rc;

struct Player {
    game:    GameRef,
    name:    MCell<String>,
    friends: ArrayCellSet<PlayerRef, 10>,
}

type PlayerRef = Rc<Player>;

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self as *const Self, other as *const Self) }
}

#[derive(Debug)]
struct Game {
    players: VecCellSet<PlayerRef>,
}

type GameRef = Ptr<&'static Game>;

impl Game {
    fn new(player_cap: usize) -> Self {
        Self {
            players: CellSet::new_vec(player_cap),
        }
    }

    fn add_player(&'static self, name: &str) -> PlayerRef {
        let p = Rc::new(Player {
            game:    Ptr::new(self),
            name:    name.to_owned().into(),
            friends: Default::default(),
        });

        self.players.insert(p.clone()).unwrap();
        p
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self as *const Self, other as *const Self) }
}

fn main() {
    let game = Box::leak(Game::new(100).into());
    let p1 = game.add_player("Sune");
    let p2 = game.add_player("Berra");
    let _ = p1.friends.insert(p2.clone()).unwrap();
    p2.friends.insert(p1.clone()).unwrap();
    assert_eq!(p1.game, p2.game);
    //    assert_eq!(p1.friends.get_clone(index), Some(p2));

    for _ in 0..2 {
        for p in game.players.iter_clone() {
            println!("{}", p.name);

            for f in p.friends.iter_clone() {
                println!("  {}", f.name);
            }

            println!();
        }
    }
}
