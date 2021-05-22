use ticket2ride::*;

pub fn demo_bigtickets() {
    let big_tickets = get_big_tickets();

    println!("---- Big tickets ----");
    for ticket in big_tickets.iter() {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
}

pub fn demo_normaltickets() {
    let tickets = get_tickets();

    println!("---- Normal tickets ----");
    for ticket in tickets.iter() {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
}

pub fn demo_network() {
    let routes = create_network();

    println!("---- Routes ----");
    for (node, destinations) in routes.iter() {
        println!("Starting from {:?} we can get to", node);
        for (destination, distance) in destinations.iter() {
            println!("--> {:?} with {} trains", destination, distance);
        }
    }
}
