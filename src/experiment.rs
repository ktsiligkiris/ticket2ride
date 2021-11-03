#![allow(dead_code)]
use ticket2ride::routing;
use ticket2ride::scoring;
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

pub fn demo_dijkstra() {
    let big_tickets = get_big_tickets();

    for ticket in big_tickets.iter() {
        let (_, prev) = routing::dijkstra(ticket.depart, ticket.arrive, true);
        print!(
            "Total score of ticket from {:?} to {:?} is ",
            ticket.depart, ticket.arrive
        );
        let mut route: Vec<City> = Vec::new();
        let mut current = ticket.arrive;
        route.push(current);

        while current != ticket.depart {
            current = *prev.get(&current).unwrap();
            route.push(current);
        }
        let score = scoring::get_scores(route);
        println!("{}", score);
    }
    // Get from Dijkstra the largest route the includes Edinburgh and Athina
    let (distances, previous) = routing::dijkstra(City::Edinburgh, City::Athina, false);
    let distant_city = max_key(&distances).unwrap();
    println!(
        "The most distant city from Edinburgh is {:?} and it's distance is {:?}",
        distant_city,
        distances.get(&distant_city).unwrap()
    );
    let mut big_route: Vec<City> = Vec::new();
    big_route.push(*distant_city);
    let mut current_city = distant_city;
    while *current_city != City::Edinburgh {
        current_city = previous.get(&current_city).unwrap();
        big_route.push(*current_city);
    }
    println!("The big route is {:?}", big_route);
}
