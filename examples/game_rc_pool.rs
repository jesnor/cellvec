use cellvec::{
    cell_set::ArrayCellSet,
    clear::Clear,
    mcell::MCell,
    ptr::Ptr,
    rc_pool::{RcPool, StrongRef, VecRcPool, WeakRef},
    refs::StrongRefTrait,
};

struct Player<'t> {
    game:    GameRef<'t>,
    name:    MCell<String>,
    health:  MCell<i32>,
    friends: ArrayCellSet<PlayerWeak<'t>, 10>,
}

impl<'t> Player<'t> {
    fn new(game: GameRef<'t>, name: &str) -> Self {
        Self {
            game,
            name: name.to_owned().into(),
            health: Default::default(),
            friends: Default::default(),
        }
    }
}

type PlayerWeak<'t> = WeakRef<'t, Player<'t>>;
type PlayerStrong<'t> = StrongRef<'t, Player<'t>>;

impl<'t> Clear for Player<'t> {
    fn clear(&self) {
        self.name.clear();
        self.friends.clear();
    }
}

struct Game<'t> {
    players: VecRcPool<Player<'t>>,
}

type GameRef<'t> = Ptr<&'t Game<'t>>;

impl<'t> Game<'t> {
    fn new(player_cap: usize) -> Self {
        Self {
            players: RcPool::new_vec(player_cap),
        }
    }

    fn add_player(&'t self, name: &str) -> PlayerStrong<'t> {
        self.players.insert(Player::new(Ptr::new(self), name)).unwrap()
    }
}

fn main() {
    let game = Game::new(100);
    let p1 = game.add_player("Sune");
    let p2 = game.add_player("Berra");
    let index = p1.friends.insert(p2.downgrade()).unwrap();
    p2.friends.insert(p1.downgrade()).unwrap();
    assert_eq!(p1.game, p2.game);
    //assert_eq!(p1.friends.get(index), Some(p2));

    p1.name.add(" Sunesson");
    p1.health.add(10);

    for _ in 0..2 {
        for p in game.players.iter() {
            println!("{}, {}", p.name, p.health);
            p.health.set(2);

            for f in p.friends.iter_ref() {
                println!("  {}, {}", f.name, f.health);
            }

            println!();
        }
    }
}
