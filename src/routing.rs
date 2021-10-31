use crate::scoring;
use log::{debug, info};
use std::collections::HashMap;
use std::collections::HashSet;
use ticket2ride::{create_network, City};

/// # Considerations in planning traversal
///
/// We have a TTL value which is the number of *trains* which can't be
/// more than 45.
///
/// We start from source.
///
/// We check the first child, if it is visited. So one hashmap to add the
/// visited children.
///
/// If it's not visited and if the current number of trains is >= the
/// distance then we can go there.
///
/// So the child is added to *current vector* with the first id, it is
/// also added to visited, and it becomes the next current point in our
/// graph.
///
/// Then check children continuously, until we either reach a point where
/// no more trains are available, or no more non visited children are
/// available.
///
/// Then we need to back track, how do we do that? So there is a
/// solution. We have a mutable vector that new children are pushed there,
/// and when we reach a point that nothing can be done, then this vector
/// is cloned inside the hashmap, the id of the hashmap is incremented,
/// and we pop from the end of the vector, until we find a city that has
/// unvisited children.
///
/// In this way, we can have all the available courses starting from one
/// city. Also, the thing to do is to remove a child from visited when
/// backtracking, but to exclude it from the children, so that we don't
/// have a loop of going and returning.
pub fn traverse(source: City) -> HashMap<u32, Vec<City>> {
    let mut routes: HashMap<u32, Vec<City>> = HashMap::new();
    let mut route_id: u32 = 0;
    let mut current_route: Vec<City> = Vec::new();
    let mut trains: u8 = 45;
    let graph = create_network();
    let mut current_city: City = source.clone();
    debug!("Current city {:?}", current_city);
    let mut visited: HashMap<City, HashSet<City>> = HashMap::new();
    let mut backtracking: bool = false;

    current_route.push(current_city);
    debug!("current_route {:?}", current_route);
    visited.insert(current_city, HashSet::new());

    while !current_route.is_empty() {
        let mut destinations = graph
            .get(&current_city)
            .unwrap()
            .iter()
            .filter(|city| match city {
                (k, _) => {
                    !visited[&current_city].contains(k) && !current_route.iter().any(|i| i == *k)
                }
            });
        let dest = destinations.next();
        debug!("Destination: {:?}", dest);
        match dest {
            Some((next, distance)) => {
                backtracking = false;
                if trains >= *distance {
                    visited
                        .entry(*next)
                        .or_insert(HashSet::new())
                        .insert(current_city);
                    visited
                        .entry(current_city)
                        .or_insert(HashSet::new())
                        .insert(*next);
                    current_city = *next;
                    current_route.push(current_city);
                    debug!("current_route: {:?}", current_route);
                    trains -= *distance;
                } else {
                    visited
                        .entry(current_city)
                        .or_insert(HashSet::new())
                        .insert(*next);
                }
            }
            None => {
                if backtracking == false {
                    // Add only routes that complete any big ticket,
                    // otherwise don't bother :)
                    // And also don't include routes that are less
                    // than the largest ones (or with two trains less)
                    if scoring::big_ticket_score(&current_route) != 0 && trains <= 2 {
                        debug!("current_route before added to routes: {:?}", current_route);
                        routes.insert(route_id, current_route.clone());
                        route_id += 1;
                    }
                    backtracking = true;
                }
                let dead_end = current_route.pop().unwrap();
                visited.insert(dead_end, HashSet::new());
                match current_route.last() {
                    Some(city) => {
                        current_city = *city;
                        trains += graph.get(&current_city).unwrap()[&dead_end];
                    }
                    None => info!("Route {:?} is the last one", route_id),
                };
            }
        }
    }

    routes
}

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
