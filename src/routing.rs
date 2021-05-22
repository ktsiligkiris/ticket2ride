use std::collections::HashMap;
use ticket2ride::{create_network, City};

pub fn dijkstra(
    source: City,
    arrival: City,
    stop: bool,
) -> (HashMap<City, u8>, HashMap<City, City>) {
    let graph = create_network();
    let mut dist: HashMap<City, u8> = HashMap::new();
    let mut prev: HashMap<City, City> = HashMap::new();
    let mut q: Vec<City> = Vec::new();
    let mut u: City;

    // initialize distances to 100 and set the distance of source to 0.
    // Since there are only 45 trains available in the game, the value of
    // 100 is basically infinity.
    for (city, _) in graph.iter() {
        dist.insert(*city, 100);
        q.push(*city);
    }
    dist.insert(source, 0);

    // The main loop
    while !q.is_empty() {
        u = q[0];
        // Get the vertex with the smallest distance from q.
        for city in q.iter() {
            if dist.get(&city).unwrap() < dist.get(&u).unwrap() {
                u = *city;
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
                    dist.insert(*destination, alt);
                    prev.insert(*destination, u);
                }
            }
        }
    }
    return (dist, prev);
}
