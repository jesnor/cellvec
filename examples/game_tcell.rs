use qcell::{TCell, TCellOwner};
use std::{ops::Deref, rc::Rc};

struct RcTCell<Q, T: ?Sized> {
    rc: Rc<TCell<Q, T>>,
}

impl<Q, T> RcTCell<Q, T> {
    fn new(value: T) -> Self {
        Self {
            rc: Rc::new(TCell::new(value)),
        }
    }

    pub fn ro<'a>(&'a self, owner: &'a Owner<Q>) -> &'a T { owner.owner.ro(self.rc.deref()) }
    pub fn rw<'a>(&'a self, owner: &'a mut Owner<Q>) -> &'a mut T { owner.owner.rw(self.rc.deref()) }
}

impl<Q, T> Clone for RcTCell<Q, T> {
    fn clone(&self) -> Self { Self { rc: self.rc.clone() } }
}

struct Owner<Q: 'static> {
    owner: TCellOwner<Q>,
}

impl<Q: 'static> Owner<Q> {
    fn new() -> Self {
        Self {
            owner: TCellOwner::new(),
        }
    }

    pub fn ro<'a, T: ?Sized>(&'a self, tc: &'a RcTCell<Q, T>) -> &'a T { self.owner.ro(tc.rc.deref()) }
    pub fn rw<'a, T: ?Sized>(&'a mut self, tc: &'a RcTCell<Q, T>) -> &'a mut T { self.owner.rw(tc.rc.deref()) }
}

struct Player {
    game:    GameRef,
    name:    String,
    health:  i32,
    friends: Vec<PlayerRef>,
}

type PlayerRef = RcTCell<Player, Player>;
type PlayerOwner = Owner<Player>;

#[derive(Default)]
struct Game {
    players: Vec<PlayerRef>,
}

type GameRef = &'static TCell<Game, Game>;
type GameOwner = TCellOwner<Game>;

fn add_player(game_ref: GameRef, go: &mut GameOwner, name: &str) -> PlayerRef {
    let game = game_ref.rw(go);

    let p: PlayerRef = PlayerRef::new(Player {
        game:    game_ref,
        name:    name.into(),
        health:  4,
        friends: Default::default(),
    });

    game.players.push(p.clone());
    p
}

fn main() {
    let mut go = GameOwner::new();
    let mut po = PlayerOwner::new();

    let game = Box::leak(Box::new(TCell::<Game, Game>::new(Game::default())));
    let p1 = &add_player(game, &mut go, "Sune");
    let p2 = &add_player(game, &mut go, "Berra");
    let index = p1.rw(&mut po).friends.push(p2.clone());
    p2.rw(&mut po).friends.push(p1.clone());
    //assert_eq!(p1.ro(&po).game, p2.ro(&po).game);
    //assert_eq!(p1.friends.get(index), Some(p2));

    po.rw(p1).health = 10;

    for p in go.ro(game).players.iter() {
        println!("{}", po.ro(p).name);

        for f in po.ro(p).friends.iter() {
            println!("  {}", po.ro(f).name);
        }

        println!();
    }
}
