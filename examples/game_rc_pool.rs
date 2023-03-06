use cellvec::{
    cell_set::ArrayCellSet,
    ptr::Ptr,
    rc_pool::{RcPool, StrongRef, VecRcPool, WeakRef},
};

struct Player<'t> {
    game:    GameRef<'t>,
    name:    String,
    health:  i32,
    friends: ArrayCellSet<PlayerWeak<'t>, 10>,
}

impl<'t> Player<'t> {
    fn new(game: GameRef<'t>, name: &str) -> Self {
        Self {
            game,
            name: name.to_owned(),
            health: Default::default(),
            friends: Default::default(),
        }
    }
}

type PlayerWeak<'t> = WeakRef<'t, Player<'t>>;
type PlayerStrong<'t> = StrongRef<'t, Player<'t>>;

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
        self.players.insert(Player::new(self.into(), name)).unwrap()
    }
}

fn main() {
    let game = Game::new(100);

    {
        let mut p1 = game.add_player("Sune");
        let p2 = game.add_player("Berra");

        let _ = p1.friends.insert(p2.weak()).unwrap();
        p2.friends.insert(p1.weak()).unwrap();
        assert_eq!(p1.game, p2.game);
        //assert_eq!(p1.friends.get(index), Some(p2));

        p1.borrow_mut().name += " Sunesson";
        p1.borrow_mut().health += 10;
    }

    for _ in 0..2 {
        for mut p in game.players.iter() {
            println!("{}, {}", p.name, p.health);
            p.borrow_mut().health = 2;

            for f in p.friends.iter_ref() {
                println!("  {}, {}", f.name, f.health);
            }

            println!();
        }
    }
}
