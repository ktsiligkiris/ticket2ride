use ticket2ride::{create_network, get_big_tickets, get_tickets, City};

pub fn get_scores(route: Vec<City>) -> u16 {
    let mut score: u16 = 0;

    score += big_ticket_score(&route);
    score += ticket_score(&route);
    score += trains_score(&route);
    score
}

fn big_ticket_score(route: &Vec<City>) -> u16 {
    let mut score: u16 = 0;
    let big_tickets = get_big_tickets();
    for ticket in big_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    score
}

fn ticket_score(route: &Vec<City>) -> u16 {
    let mut score: u16 = 0;
    let small_tickets = get_tickets();

    for ticket in small_tickets.iter() {
        if route.iter().any(|&i| i == ticket.depart) && route.iter().any(|&i| i == ticket.arrive) {
            score += ticket.value as u16;
        }
    }
    score
}

fn trains_score(route: &Vec<City>) -> u16 {
    let mut score: u16 = 0;
    let routes = create_network();

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
