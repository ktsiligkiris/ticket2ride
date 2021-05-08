use std::collections::HashMap;
use ticket2ride::*;

fn dijkstra(source: City, arrival: City, stop: bool) -> (HashMap<City, u8>, HashMap<City, City>) {
    let graph = create_network();
    let mut dist: HashMap<City, u8> = HashMap::new();
    let mut prev: HashMap<City, City> = HashMap::new();
    let mut q: Vec<City> = Vec::new();
    let mut u: City;

    // initialize distances to 100 and set the distance of source to 0.
    // Since there are only 45 trains available in the game, the value of
    // 100 is basically infinity.
    for (city, _) in graph.iter() {
        dist.insert(city.clone(), 100);
        q.push(city.clone());
    }
    dist.insert(source, 0);

    // The main loop
    while !q.is_empty() {
        u = q[0];
        // Get the vertex with the smallest distance from q.
        for city in q.iter() {
            if dist.get(&city).unwrap() < dist.get(&u).unwrap() {
                u = city.clone();
            }
        }
        // the vertex with the shortest distance in q is our destination
        // then quit from the while loop prematurely.
        if u == arrival && stop == true {
            break;
        }

        // Remove this vertex from q
        let index = q.iter().position(|&x| x == u).unwrap();
        q.remove(index);

        // Check the neighbours of u for corrections
        let destinations = graph.get(&u).unwrap();
        for (destination, distance) in destinations.iter() {
            if q.iter().any(|i| i == destination) {
                let alt = dist.get(&u).unwrap() + *distance;
                // Maximum number of trains in game is 45, so you can't have a distance
                // greater than that.
                if alt < *dist.get(&destination).unwrap() && alt <= 45 {
                    dist.insert(destination.clone(), alt);
                    prev.insert(destination.clone(), u.clone());
                }
            }
        }
    }
    return (dist, prev);
}

fn get_scores(route: Vec<City>) -> u8 {
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

fn max_key<K, V>(hash: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    hash.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k)
}

fn main() {
    // let routes = create_network();
    let big_tickets = get_big_tickets();
    // let tickets = get_tickets();

    /* Just for demo
    println!("---- Big tickets ----");
    for ticket in big_tickets.iter() {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
     */
    /* Just for demo
    println!("---- Normal tickets ----");
    for ticket in tickets.iter() {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
     */
    /* Just for demo
    println!("---- Routes ----");
    for (node, destinations) in routes.iter() {
        println!("Starting from {:?} we can get to", node);
        for (destination, distance) in destinations.iter() {
            println!("--> {:?} with {} trains", destination, distance);
        }
    }
     */

    // Test Dijkstra
    for ticket in big_tickets.iter() {
        let (_, prev) = dijkstra(ticket.depart, ticket.arrive, true);
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
        let score = get_scores(route);
        println!("{}", score);
    }
    // Get from Dijkstra the largest route the includes Edinburgh and Athina
    let (distances, previous) = dijkstra(City::Edinburgh, City::Athina, false);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_kyiv_has_6_routes() {
        let routes = create_network();
        assert_eq!(routes[&City::Kyiv].len(), 6);
    }

    #[test]
    fn check_route_pairs() {
        let routes = create_network();
        for (city, destinations) in routes.iter() {
            for (destination, distance) in destinations.iter() {
                assert_eq!(*distance, routes[destination][city]);
            }
        }
    }
}
