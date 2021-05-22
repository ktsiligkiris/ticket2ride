use ticket2ride::{create_network, get_big_tickets, get_tickets, City};

pub fn get_scores(route: Vec<City>) -> u16 {
    let mut score: u16 = 0;
    let routes = create_network();
    let big_tickets = get_big_tickets();
    let small_tickets = get_tickets();
    for ticket in big_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    for ticket in small_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    // Get score from used trains
    for (depart, arrive) in route
        .split_last()
        .unwrap()
        .1
        .iter()
        .zip(route.split_first().unwrap().1.iter())
    {
        score += routes[&depart][&arrive] as u16;
    }
    score
}
