use gstd::Encode;
use gtest::System;
use io::*;

mod routines;
pub use routines::*;

#[test]
fn common_check() {
    let sys = System::new();

    let game = common_init(&sys);

    check_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
}

#[test]
fn check_greater_bet() {
    let sys = System::new();

    let game = common_init(&sys);

    check_user_move(&game, USERS[0], Move::Spock, 10_000_000);
}

#[test]
fn check_two_moves() {
    let sys = System::new();

    let game = common_init(&sys);

    check_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
    check_user_move(&game, USERS[1], Move::Spock, COMMON_BET);
}

#[test]
fn check_not_added_user() {
    let sys = System::new();

    let game = common_init(&sys);

    failure_user_move(&game, USERS[3], Move::Spock, COMMON_BET);
}

#[test]
fn check_wrong_bet() {
    let sys = System::new();

    let game = common_init(&sys);

    failure_user_move(&game, USERS[0], Move::Spock, 100_000);
}

#[test]
fn check_move_twice() {
    let sys = System::new();

    let game = common_init(&sys);

    check_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
    failure_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
}

#[test]
fn check_on_reveal_stage() {
    let sys = System::new();

    let game = common_init(&sys);

    check_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
    check_user_move(&game, USERS[1], Move::Spock, COMMON_BET);
    check_user_move(&game, USERS[2], Move::Spock, COMMON_BET);

    failure_user_move(&game, USERS[0], Move::Spock, COMMON_BET);
    failure_user_move(&game, USERS[1], Move::Spock, COMMON_BET);
    failure_user_move(&game, USERS[2], Move::Spock, COMMON_BET);
    failure_user_move(&game, USERS[3], Move::Spock, COMMON_BET);

    check_user_reveal_with_continue(&game, USERS[0], Move::Spock);
}

#[test]
fn check_move_in_second_round_without_bet() {
    let sys = System::new();
    let game = init_with_users(&sys, USERS);
    let moves = [Move::Rock, Move::Paper, Move::Scissors, Move::Spock];

    play_round(&game, USERS, &moves).contains(&(
        *USERS.last().unwrap(),
        Event::SuccessfulReveal(RevealResult::NextRoundStarted {
            players: USERS.iter().copied().map(|id| id.into()).collect(),
        })
        .encode(),
    ));

    let result = try_to_move(&game, USERS[0], Move::Rock, 0);

    assert!(!result.main_failed());
}

#[test]
fn check_move_in_second_round_with_bet() {
    let sys = System::new();
    let game = init_with_users(&sys, USERS);
    let moves = [Move::Rock, Move::Paper, Move::Scissors, Move::Spock];

    play_round(&game, USERS, &moves).contains(&(
        *USERS.last().unwrap(),
        Event::SuccessfulReveal(RevealResult::NextRoundStarted {
            players: USERS.iter().copied().map(|id| id.into()).collect(),
        })
        .encode(),
    ));

    let result = try_to_move(&game, USERS[0], Move::Rock, COMMON_BET);

    assert!(!result.main_failed());
}
