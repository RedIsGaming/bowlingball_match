pub mod bowling;

#[cfg(test)]
mod tests {
    use crate::bowling::{game::BowlingGame, error::Error};

    #[test]
    fn test_new_game() {
        let game = BowlingGame::default();
        assert_eq!(game.end_score(), 0);
    }

    #[test]
    fn test_invalid_frame_1() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(11, 0), Err(Error::InvalidFrame));
    }

    #[test]
    fn test_invalid_frame_2() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(5, 6), Err(Error::InvalidFrame));
    }

    #[test]
    fn test_game_already_ended() {
        let mut game = BowlingGame::default();
        game.ended = true;
        assert_eq!(game.frame(1, 0), Err(Error::GameAlreadyEnded));
    }

    #[test]
    fn test_frame_1() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(1, 0), Ok(1));
    }

    #[test]
    fn test_frame_2() {
        let mut game = BowlingGame::default();
        for _ in 0..10 {
            game.frame(1, 0).unwrap();
        }
        assert_eq!(game.end_score(), 10);
    }

    #[test]
    fn test_game_not_ended() {
        let mut game = BowlingGame::default();
        game.frame(1, 0).unwrap();
        assert_eq!(game.end_score(), 0);
    }

    #[test]
    fn test_spare() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(5, 5), Ok(10));
        assert_eq!(game.frame(3, 2), Ok(18));
    }

    #[test]
    fn test_strike() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(10, 0), Ok(10));
        assert_eq!(game.frame(3, 2), Ok(20));
    }

    #[test]
    fn test_strikes() {
        let mut game = BowlingGame::default();
        assert_eq!(game.frame(10, 0), Ok(10));
        assert_eq!(game.frame(10, 0), Ok(30));
        assert_eq!(game.frame(3, 2), Ok(43));
    }

    #[test]
    fn test_last_illegal() {
        let mut game = BowlingGame::default();
        for _ in 0..10 {
            game.frame(1, 0).unwrap();
        }
        assert_eq!(game.frame(1, 0), Err(Error::GameAlreadyEnded));
    }

    #[test]
    fn test_last_spare_illegal() {
        let mut game = BowlingGame::default();
        for _ in 0..10 {
            game.frame(5, 5).unwrap();
        }
        assert_eq!(game.frame(5, 5), Err(Error::InvalidFrame));
    }

    #[test]
    fn test_last_spare() {
        let mut game = BowlingGame::default();
        for _ in 0..10 {
            game.frame(5, 5).unwrap();
        }
        assert_eq!(game.frame(5, 0), Ok(150));
    }

    #[test]
    fn test_last_strike() {
        let mut game = BowlingGame::default();
        for _ in 0..9 {
            game.frame(0, 0).unwrap();
        }
        assert_eq!(game.frame(10, 0), Ok(10));
        assert_eq!(game.frame(5, 5), Ok(20));
    }

    #[test]
    fn test_frame_max() {
        let mut game = BowlingGame::default();
        for _ in 0..10 {
            game.frame(10, 0).unwrap();
        }
        // extra rolls because of last strike
        game.frame(10, 10).unwrap();
        assert_eq!(game.end_score(), 300);
    }
}
