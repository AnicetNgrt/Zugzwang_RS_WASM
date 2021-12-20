use log::debug;
use zzg::*;

fn main() {
    pretty_env_logger::init();
    let mut game = Game::new(15, 10, Rules { max_pawn_per_player: 4 });
    
    game.apply_current_turn();

    debug!("{:?}", game.state.pawns);

    game.rollback_last_turn();

    debug!("{:?}", game.state.pawns);
}
