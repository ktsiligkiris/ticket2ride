use flexi_logger::{Duplicate, FileSpec, Logger};
use log::{debug, info};
use std::collections::HashMap;
use std::str::FromStr;
use ticket2ride::{max_key, City};

mod cli;
mod experiment;
mod routing;
mod scoring;

fn main() {
    // Get command line options
    let matches = cli::get_cli().get_matches();

    // Setup logging first
    Logger::try_with_env_or_str(matches.value_of("logging").unwrap_or("none"))
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

    info!("Start traversing the network.");
    let routes = routing::traverse(City::from_str(startcity).unwrap());
    info!("Start computing scores for all routes.");
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
