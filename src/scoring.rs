use ticket2ride::{get_big_tickets, get_tickets, City};

pub fn get_scores(route: Vec<City>) -> u8 {
    let mut score: u8 = 0;
    let big_tickets = get_big_tickets();
    let small_tickets = get_tickets();
    for ticket in big_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value;
        }
    }
    for ticket in small_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value;
        }
    }
    score
}
