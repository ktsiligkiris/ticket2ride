//! The scoring module contains all the functions that are needed for
//! computing the score of a route on the board of ticket to ride.

use crate::{create_network, get_big_tickets, get_tickets, City};

/// Receives a vector of cities that constitute a route, and returns
/// the score of this route, based on the big tickets and the regular
/// tickets completed, the trains used in city connections, 10 points
/// for the express bonus and 12 points for no stations used.
pub fn get_scores(route: Vec<City>) -> u16 {
    let mut score: u16 = 0;

    score += big_ticket_score(&route);
    score += ticket_score(&route);
    score += trains_score(&route);
    score += 10; // This is the express bonus
    score += 12; // This is the unused stations score
    score
}

#[test]
fn test_get_scores() {
    let test_route = vec![
        City::Edinburgh,
        City::London,
        City::Dieppe,
        City::Paris,
        City::Zuerich,
        City::Muenchen,
        City::Wien,
        City::Budapest,
        City::Bucuresti,
        City::Sofia,
        City::Athina,
        City::Smyrna,
    ];
    let test_score = get_scores(test_route);
    assert_eq!(test_score, 120);
}

/// Receives a vector of cities and returns the score from completing
/// a big ticket. If no big tickets are completed, then it returns
/// zero, so this is a nice way to prune routes in the routing module
/// that don't complete a big ticket (and therefore are not candidates
/// for the maximum theoretical score).
pub fn big_ticket_score(route: &[City]) -> u16 {
    let mut score: u16 = 0;
    let big_tickets = get_big_tickets();
    for ticket in big_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    score
}

#[test]
fn test_big_ticket_score() {
    let exact_route = vec![
        City::Lisboa,
        City::Madrid,
        City::Pamplona,
        City::Paris,
        City::Frankfurt,
        City::Berlin,
        City::Danzig,
    ];
    let exact_score = big_ticket_score(&exact_route);
    assert_eq!(exact_score, 20);
}

#[test]
fn test_big_ticket_extended_route() {
    let extended_route = vec![
        City::Cadiz,
        City::Lisboa,
        City::Madrid,
        City::Pamplona,
        City::Paris,
        City::Frankfurt,
        City::Berlin,
        City::Danzig,
        City::Riga,
    ];
    let extended_score = big_ticket_score(&extended_route);
    assert_eq!(extended_score, 20);
}

#[test]
fn test_big_ticket_only_start() {
    let non_route_start = vec![City::Lisboa, City::Madrid];
    let non_score_start = big_ticket_score(&non_route_start);
    assert_eq!(non_score_start, 0);
}

#[test]
fn test_big_ticket_only_end() {
    let non_route_end = vec![City::Berlin, City::Danzig];
    let non_score_end = big_ticket_score(&non_route_end);
    assert_eq!(non_score_end, 0);
}

fn ticket_score(route: &[City]) -> u16 {
    let mut score: u16 = 0;
    let small_tickets = get_tickets();

    for ticket in small_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    score
}

fn trains_score(route: &[City]) -> u16 {
    let mut score: u16 = 0;
    let routes = create_network();

    for (depart, arrive) in route
        .split_last()
        .unwrap()
        .1
        .iter()
        .zip(route.split_first().unwrap().1.iter())
    {
        score += match routes[depart][arrive] {
            1 => 1_u16,
            2 => 2_u16,
            3 => 4_u16,
            4 => 7_u16,
            6 => 15_u16,
            8 => 21_u16,
            // The default case that panics normally should never
            // happen, as the only train lengths between two cities in
            // the board game are the ones covered in the previous
            // cases. But rust wants me to cover all cases :)
            _ => panic!("Unsupported number of trains"),
        };
    }
    score
}
