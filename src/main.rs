use flexi_logger::{Duplicate, FileSpec, Logger};
use log::{debug, info};
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;
use ticket2ride::{max_key, routing, scoring, City};

mod cli;
mod experiment;

fn main() {
    // Get command line options
    let matches = cli::get_cli().get_matches();

    // Setup logging first
    Logger::try_with_str(matches.value_of("logging").unwrap_or("none"))
        .unwrap()
        .log_to_file(FileSpec::default())
        .duplicate_to_stderr(Duplicate::Info)
        .start()
        .unwrap();
    debug!("Debug reporting turned on!");

    let startcity = matches.value_of("city").unwrap_or("Edinburgh");
    /* These functions come from the experiment.rs file where I keep
     * demo functions to test various functionality. In this way, I
     * keep main function more clean.*/
    // experiment::demo_bigtickets();

    // experiment::demo_normaltickets();

    // experiment::demo_network();

    //experiment::demo_dijkstra();
    if let Some(matched) = matches.subcommand_matches("info") {
        if matched.is_present("big") {
            experiment::demo_bigtickets();
        }
        if matched.is_present("normal") {
            experiment::demo_normaltickets();
        }
    } else if let Some(_matched) = matches.subcommand_matches("solve") {
        let mut results: HashMap<City, u16> = HashMap::new();
        for city in City::iter() {
            let (_, score) = find_max(city);
            results.entry(city).or_insert(score);
        }
        let maxcity = max_key(&results).unwrap();
        println!(
            "the city {:?} has the maximum score of {:?}",
            maxcity,
            results.get(maxcity).unwrap()
        );
    } else {
        let (route, score) = find_max(City::from_str(startcity).unwrap());

        println!("The route {:?} has maximum score {:?}", route, score);
    }
}

fn find_max(start: City) -> (Vec<City>, u16) {
    info!("Start traversing the network from {:?}", start);
    let routes = routing::traverse(start);
    info!("Start computing scores for all routes.");
    let mut scores: HashMap<u32, u16> = HashMap::new();
    for (id, route) in routes.iter() {
        let score: u16 = scoring::get_scores(route.to_vec());
        scores.entry(*id).or_insert(score);
    }
    let maxkey = max_key(&scores).unwrap();
    let route: Vec<City> = routes.get(maxkey).unwrap().to_vec();
    let score: u16 = *scores.get(maxkey).unwrap();
    info!("Maximum score is {:?}", score);
    (route, score)
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
