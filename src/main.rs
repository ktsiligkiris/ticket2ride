use ticket2ride::City;
mod experiment;
mod routing;
mod scoring;

fn main() {
    /* These functions come from the experiment.rs file where I keep
     * demo functions to test various functionality. In this way, I
     * keep main function more clean.*/
    // experiment::demo_bigtickets();

    // experiment::demo_normaltickets();

    // experiment::demo_network();

    //experiment::demo_dijkstra();

    let routes = routing::traverse(City::Edinburgh);
    for (id, route) in routes.iter() {
        println!("Route no. {:?} is {:?}", id, route);
    }

    // I'll try some routes that came up from initial traverse. I'll
    // need to add logic here that will compute the scores of all
    // routes computed by the traverse function and then I'll print
    // the route with the highest score, and the score of that route.
    let score1 = scoring::get_scores(vec![
        City::Edinburgh,
        City::London,
        City::Amsterdam,
        City::Essen,
        City::Berlin,
        City::Wien,
        City::Muenchen,
        City::Venezia,
        City::Roma,
        City::Palermo,
        City::Brindisi,
        City::Athina,
        City::Smyrna,
        City::Constantinople,
    ]);
    println!("The score is {:?}", score1);
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
