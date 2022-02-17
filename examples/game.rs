use cellvec::{
    cell_set::ArrayCellSet,
    clear::Clear,
    ptr::Ptr,
    slot_pool::{CellVecRef, SlotPool, VecSlotPool},
    string_cell::StringCell,
};

struct Player<'t> {
    game:    GameRef<'t>,
    name:    StringCell,
    friends: ArrayCellSet<PlayerRef<'t>, 10>,
}

type PlayerRef<'t> = CellVecRef<'t, Player<'t>>;

impl<'t> Clear for Player<'t> {
    fn clear(&self) {
        self.name.clear();
        self.friends.clear();
    }
}

struct Game<'t> {
    players: VecSlotPool<Player<'t>>,
}

type GameRef<'t> = Ptr<&'t Game<'t>>;

impl<'t> Game<'t> {
    fn new(player_cap: usize) -> Self {
        Self {
            players: SlotPool::new_vec(player_cap),
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

        p.name.set(name);
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
        println!("{}", p.name);

        for f in p.friends.iter() {
            println!("  {}", f.name);
        }

        println!();
    }
}
