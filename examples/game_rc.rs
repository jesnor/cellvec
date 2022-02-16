use cellvec::fixed_cell_set::FixedCellSet;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Player {
    game:    GameRef,
    name:    RefCell<String>,
    friends: FixedCellSet<PlayerRef, 10>,
}

type PlayerRef = Rc<Player>;

struct Game {
    players: Vec<Cell<Option<PlayerRef>>>,
}

type GameRef = Rc<Game>;

impl Game {
    fn new(cap: usize) -> Self {
        let mut s = Self {
            players: Vec::with_capacity(cap),
        };

        s.players.resize_with(s.players.capacity(), Default::default);
        s
    }

    fn add_player(self: &Rc<Self>, name: &str) -> PlayerRef {
        let p = Rc::new(Player {
            game:    self.clone(),
            name:    name.to_owned().into(),
            friends: Default::default(),
        });

        for s in self.players.iter() {
            let r = s.take();

            if r.is_none() {
                s.set(Some(p.clone()));
                return p;
            } else {
                s.set(r);
            }
        }

        panic!();
    }

    fn players(&self) -> impl Iterator<Item = PlayerRef> + '_ {
        self.players.iter().filter_map(|c| {
            let p1 = c.take();
            let p2 = p1.clone();
            c.set(p1);
            p2
        })
    }
}

fn main() {
    let game = Rc::new(Game::new(100));
    let p1 = game.add_player("Sune");
    let p2 = game.add_player("Berra");
    let index = p1.friends.insert(p2.clone()).unwrap();
    p2.friends.insert(p1).unwrap();
    //assert_eq!(p1.game, p2.game);
    //assert_eq!(p1.friends.get_clone(index), Some(p2));

    for p in game.players() {
        println!("{}", p.name.borrow());

        for f in p.friends.iter_clone() {
            println!("  {}", f.name.borrow());
        }

        println!();
    }
}
