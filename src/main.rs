use std::collections::HashMap;
use std::str::FromStr;
use ticket2ride::{max_key, City};

mod cli;
mod experiment;
mod routing;
mod scoring;

fn main() {
    let matches = cli::get_cli().get_matches();

    let startcity = matches.value_of("city").unwrap_or("Edinburgh");
    /* These functions come from the experiment.rs file where I keep
     * demo functions to test various functionality. In this way, I
     * keep main function more clean.*/
    // experiment::demo_bigtickets();

    // experiment::demo_normaltickets();

    // experiment::demo_network();

    //experiment::demo_dijkstra();

    let routes = routing::traverse(City::from_str(startcity).unwrap());
    let mut scores: HashMap<u32, u16> = HashMap::new();
    for (id, route) in routes.iter() {
        let score: u16 = scoring::get_scores(route.to_vec());
        scores.entry(*id).or_insert(score);
    }
    let maxkey = max_key(&scores).unwrap();

    println!(
        "The route {:?} has maximum score {:?}",
        routes.get(&maxkey).unwrap(),
        scores.get(&maxkey).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use ticket2ride::{create_network, City};

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
