use cellvec::{
    cell_set::ArrayCellSet,
    cell_trait::CellTrait,
    mcell::MCell,
    ptr::Ptr,
    slot_pool::{SlotPool, SlotPoolRef, VecSlotPool},
};
use std::cell::Cell;

struct Player<'t> {
    game:    GameRef<'t>,
    name:    MCell<String>,
    health:  Cell<i32>,
    friends: ArrayCellSet<PlayerRef<'t>, 10>,
}

impl<'t> Player<'t> {
    fn new(game: GameRef<'t>) -> Self {
        Self {
            game,
            name: Default::default(),
            health: Default::default(),
            friends: Default::default(),
        }
    }
}

type PlayerRef<'t> = SlotPoolRef<'t, Player<'t>>;

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
        let p = self.players.insert(Player::new(Ptr::new(self))).unwrap();
        p.name.set(name.into());
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

    p1.name.add(" Sunesson");
    p1.health.add(10);

    for _ in 0..2 {
        for p in game.players.iter() {
            println!("{}", p.name);

            for f in p.friends.iter() {
                println!("  {}", f.name);
            }

            println!();
        }
    }
}
