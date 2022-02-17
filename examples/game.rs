use cellvec::{
    cell_vec::{CellVec, CellVecRef},
    clear::Clear,
    fixed_cell_set::FixedCellSet,
    ptr::Ptr,
};
use std::cell::RefCell;

struct Player<'t> {
    game:    GameRef<'t>,
    name:    RefCell<String>,
    friends: FixedCellSet<PlayerRef<'t>, 10>,
}

type PlayerRef<'t> = CellVecRef<'t, Player<'t>>;

impl<'t> Clear for Player<'t> {
    fn clear(&self) {
        self.name.borrow_mut().clear();
        self.friends.clear();
    }
}

struct Game<'t> {
    players: CellVec<Player<'t>>,
}

type GameRef<'t> = Ptr<&'t Game<'t>>;

impl<'t> Game<'t> {
    fn new(player_cap: usize) -> Self {
        Self {
            players: CellVec::with_capacity(player_cap),
        }
    }

    fn add_player(&'t self, name: &str) -> PlayerRef<'t> {
        let p = self
            .players
            .alloc(|| Player {
                game:    self.into(),
                name:    Default::default(),
                friends: Default::default(),
            })
            .unwrap();

        *(p.name.borrow_mut()) = name.to_owned();
        p
    }
}

fn main() {
    let game = Game::new(100);
    let p1 = game.add_player("Sune");
    let p2 = game.add_player("Berra");
    let index = p1.friends.insert(p2).unwrap();
    p2.friends.insert(p1).unwrap();
    assert_eq!(p1.game, p2.game);
    assert_eq!(p1.friends.get(index), Some(p2));

    for p in game.players.iter() {
        println!("{}", p.name.borrow());

        for f in p.friends.iter() {
            println!("  {}", f.name.borrow());
        }

        println!();
    }
}
