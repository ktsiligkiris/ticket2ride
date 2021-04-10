use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum City {
    Edinburgh,
    London,
    Dieppe,
    Amsterdam,
    Brest,
    Paris,
    Bruxelles,
    Essen,
    Frankfurt,
    Pamplona,
    Zuerich,
    Marseille,
    Kobenhavn,
    Berlin,
    Muenchen,
    Madrid,
    Barcelona,
    Venezia,
    Roma,
    Stockholm,
    Danzig,
    Warszawa,
    Wien,
    Lisboa,
    Cadiz,
    Zagrab,
    Brindisi,
    Palermo,
    Petrograd,
    Riga,
    Wilno,
    Kyiv,
    Budapest,
    Sarajevo,
    Athina,
    Smyrna,
    Moskva,
    Smolensk,
    Kharkov,
    Bucuresti,
    Sofia,
    Constantinople,
    Angora,
    Rostov,
    Sevastopol,
    Erzurum,
    Sochi,
}

#[derive(Debug)]
struct Ticket {
    depart: City,
    arrive: City,
    value: u8,
}

impl Ticket {
    fn new(depart: City, arrive: City, value: u8) -> Ticket {
        return Ticket {
            depart,
            arrive,
            value,
        };
    }
}

fn create_network() -> HashMap<City, HashMap<City, u8>> {
    let mut network = HashMap::<City, HashMap<City, u8>>::new();

    let mut destination = HashMap::<City, u8>::new();

    destination.insert(City::London, 4);
    network.insert(City::Edinburgh, destination.clone());
    destination.clear();

    destination.insert(City::Edinburgh, 4);
    destination.insert(City::Dieppe, 2);
    destination.insert(City::Amsterdam, 2);
    network.insert(City::London, destination.clone());
    destination.clear();

    destination.insert(City::Paris, 1);
    destination.insert(City::London, 2);
    destination.insert(City::Brest, 2);
    destination.insert(City::Bruxelles, 2);
    network.insert(City::Dieppe, destination.clone());
    destination.clear();

    destination.insert(City::Essen, 3);
    destination.insert(City::Frankfurt, 2);
    destination.insert(City::London, 2);
    destination.insert(City::Bruxelles, 1);
    network.insert(City::Amsterdam, destination.clone());
    destination.clear();

    destination.insert(City::Dieppe, 2);
    destination.insert(City::Paris, 3);
    destination.insert(City::Pamplona, 4);
    network.insert(City::Brest, destination.clone());
    destination.clear();

    destination.insert(City::Brest, 3);
    destination.insert(City::Dieppe, 1);
    destination.insert(City::Bruxelles, 2);
    destination.insert(City::Frankfurt, 3);
    destination.insert(City::Zuerich, 3);
    destination.insert(City::Marseille, 4);
    destination.insert(City::Pamplona, 4);
    network.insert(City::Paris, destination.clone());
    destination.clear();

    destination.insert(City::Paris, 2);
    destination.insert(City::Dieppe, 2);
    destination.insert(City::Amsterdam, 1);
    destination.insert(City::Frankfurt, 2);
    network.insert(City::Bruxelles, destination.clone());
    destination.clear();

    destination.insert(City::Amsterdam, 3);
    destination.insert(City::Kobenhavn, 3);
    destination.insert(City::Berlin, 2);
    destination.insert(City::Frankfurt, 2);
    network.insert(City::Essen, destination.clone());
    destination.clear();

    destination.insert(City::Paris, 3);
    destination.insert(City::Bruxelles, 2);
    destination.insert(City::Amsterdam, 2);
    destination.insert(City::Berlin, 3);
    destination.insert(City::Muenchen, 2);
    destination.insert(City::Essen, 2);
    network.insert(City::Frankfurt, destination.clone());
    destination.clear();

    destination.insert(City::Madrid, 3);
    destination.insert(City::Brest, 4);
    destination.insert(City::Paris, 4);
    destination.insert(City::Marseille, 4);
    destination.insert(City::Barcelona, 2);
    network.insert(City::Pamplona, destination.clone());
    destination.clear();

    destination.insert(City::Paris, 3);
    destination.insert(City::Muenchen, 2);
    destination.insert(City::Venezia, 2);
    destination.insert(City::Marseille, 2);
    network.insert(City::Zuerich, destination.clone());
    destination.clear();

    destination.insert(City::Barcelona, 4);
    destination.insert(City::Pamplona, 4);
    destination.insert(City::Paris, 4);
    destination.insert(City::Zuerich, 2);
    destination.insert(City::Roma, 4);
    network.insert(City::Marseille, destination.clone());
    destination.clear();

    destination.insert(City::Essen, 3);
    destination.insert(City::Stockholm, 3);
    network.insert(City::Kobenhavn, destination.clone());
    destination.clear();

    destination.insert(City::Essen, 2);
    destination.insert(City::Danzig, 4);
    destination.insert(City::Warszawa, 4);
    destination.insert(City::Wien, 3);
    destination.insert(City::Frankfurt, 3);
    network.insert(City::Berlin, destination.clone());
    destination.clear();

    destination.insert(City::Zuerich, 2);
    destination.insert(City::Frankfurt, 2);
    destination.insert(City::Wien, 3);
    destination.insert(City::Venezia, 2);
    network.insert(City::Muenchen, destination.clone());
    destination.clear();

    destination.insert(City::Lisboa, 3);
    destination.insert(City::Pamplona, 3);
    destination.insert(City::Barcelona, 2);
    destination.insert(City::Cadiz, 3);
    network.insert(City::Madrid, destination.clone());
    destination.clear();

    destination.insert(City::Madrid, 2);
    destination.insert(City::Pamplona, 2);
    destination.insert(City::Marseille, 4);
    network.insert(City::Barcelona, destination.clone());
    destination.clear();

    destination.insert(City::Zuerich, 2);
    destination.insert(City::Muenchen, 2);
    destination.insert(City::Zagrab, 2);
    destination.insert(City::Roma, 2);
    network.insert(City::Venezia, destination.clone());
    destination.clear();

    destination.insert(City::Marseille, 4);
    destination.insert(City::Venezia, 2);
    destination.insert(City::Brindisi, 2);
    destination.insert(City::Palermo, 4);
    network.insert(City::Roma, destination.clone());
    destination.clear();

    destination.insert(City::Kobenhavn, 3);
    destination.insert(City::Petrograd, 8);
    network.insert(City::Stockholm, destination.clone());
    destination.clear();

    destination.insert(City::Berlin, 4);
    destination.insert(City::Riga, 3);
    destination.insert(City::Warszawa, 2);
    network.insert(City::Danzig, destination.clone());
    destination.clear();

    destination.insert(City::Berlin, 4);
    destination.insert(City::Danzig, 2);
    destination.insert(City::Wilno, 3);
    destination.insert(City::Kyiv, 4);
    destination.insert(City::Wien, 4);
    network.insert(City::Warszawa, destination.clone());
    destination.clear();

    destination.insert(City::Muenchen, 3);
    destination.insert(City::Berlin, 3);
    destination.insert(City::Warszawa, 4);
    destination.insert(City::Budapest, 1);
    destination.insert(City::Zagrab, 2);
    network.insert(City::Wien, destination.clone());
    destination.clear();

    destination.insert(City::Madrid, 3);
    destination.insert(City::Cadiz, 2);
    network.insert(City::Lisboa, destination.clone());
    destination.clear();

    destination.insert(City::Madrid, 3);
    destination.insert(City::Lisboa, 2);
    network.insert(City::Cadiz, destination.clone());
    destination.clear();

    destination.insert(City::Venezia, 2);
    destination.insert(City::Wien, 2);
    destination.insert(City::Budapest, 2);
    destination.insert(City::Sarajevo, 3);
    network.insert(City::Zagrab, destination.clone());
    destination.clear();

    destination.insert(City::Roma, 2);
    destination.insert(City::Athina, 4);
    destination.insert(City::Palermo, 3);
    network.insert(City::Brindisi, destination.clone());
    destination.clear();

    destination.insert(City::Roma, 4);
    destination.insert(City::Brindisi, 3);
    destination.insert(City::Smyrna, 6);
    network.insert(City::Palermo, destination.clone());
    destination.clear();

    destination.insert(City::Stockholm, 8);
    destination.insert(City::Riga, 4);
    destination.insert(City::Wilno, 4);
    destination.insert(City::Moskva, 4);
    network.insert(City::Petrograd, destination.clone());
    destination.clear();

    destination.insert(City::Danzig, 3);
    destination.insert(City::Petrograd, 4);
    destination.insert(City::Wilno, 4);
    network.insert(City::Riga, destination.clone());
    destination.clear();

    destination.insert(City::Warszawa, 3);
    destination.insert(City::Riga, 4);
    destination.insert(City::Petrograd, 4);
    destination.insert(City::Smolensk, 3);
    destination.insert(City::Kyiv, 2);
    network.insert(City::Wilno, destination.clone());
    destination.clear();

    destination.insert(City::Warszawa, 4);
    destination.insert(City::Wilno, 2);
    destination.insert(City::Smolensk, 3);
    destination.insert(City::Kharkov, 4);
    destination.insert(City::Bucuresti, 4);
    destination.insert(City::Budapest, 6);
    network.insert(City::Kyiv, destination.clone());
    destination.clear();

    destination.insert(City::Wien, 1);
    destination.insert(City::Kyiv, 6);
    destination.insert(City::Bucuresti, 4);
    destination.insert(City::Sarajevo, 3);
    destination.insert(City::Zagrab, 2);
    network.insert(City::Budapest, destination.clone());
    destination.clear();

    destination.insert(City::Zagrab, 3);
    destination.insert(City::Budapest, 3);
    destination.insert(City::Sofia, 2);
    destination.insert(City::Athina, 4);
    network.insert(City::Sarajevo, destination.clone());
    destination.clear();

    destination.insert(City::Brindisi, 4);
    destination.insert(City::Sarajevo, 4);
    destination.insert(City::Sofia, 3);
    destination.insert(City::Smyrna, 2);
    network.insert(City::Athina, destination.clone());
    destination.clear();

    destination.insert(City::Palermo, 6);
    destination.insert(City::Athina, 2);
    destination.insert(City::Constantinople, 2);
    destination.insert(City::Angora, 3);
    network.insert(City::Smyrna, destination.clone());
    destination.clear();

    destination.insert(City::Smolensk, 2);
    destination.insert(City::Petrograd, 4);
    destination.insert(City::Kharkov, 4);
    network.insert(City::Moskva, destination.clone());
    destination.clear();

    destination.insert(City::Wilno, 3);
    destination.insert(City::Moskva, 2);
    destination.insert(City::Kyiv, 3);
    network.insert(City::Smolensk, destination.clone());
    destination.clear();

    destination.insert(City::Kyiv, 4);
    destination.insert(City::Moskva, 4);
    destination.insert(City::Rostov, 2);
    network.insert(City::Kharkov, destination.clone());
    destination.clear();

    destination.insert(City::Budapest, 4);
    destination.insert(City::Kyiv, 4);
    destination.insert(City::Sevastopol, 4);
    destination.insert(City::Constantinople, 3);
    destination.insert(City::Sofia, 2);
    network.insert(City::Bucuresti, destination.clone());
    destination.clear();

    destination.insert(City::Sarajevo, 2);
    destination.insert(City::Bucuresti, 2);
    destination.insert(City::Constantinople, 3);
    destination.insert(City::Athina, 3);
    network.insert(City::Sofia, destination.clone());
    destination.clear();

    destination.insert(City::Sofia, 3);
    destination.insert(City::Bucuresti, 3);
    destination.insert(City::Sevastopol, 4);
    destination.insert(City::Angora, 2);
    destination.insert(City::Smyrna, 2);
    network.insert(City::Constantinople, destination.clone());
    destination.clear();

    destination.insert(City::Smyrna, 3);
    destination.insert(City::Constantinople, 2);
    destination.insert(City::Erzurum, 3);
    network.insert(City::Angora, destination.clone());
    destination.clear();

    destination.insert(City::Kharkov, 2);
    destination.insert(City::Sevastopol, 4);
    destination.insert(City::Sochi, 2);
    network.insert(City::Rostov, destination.clone());
    destination.clear();

    destination.insert(City::Bucuresti, 4);
    destination.insert(City::Rostov, 4);
    destination.insert(City::Sochi, 2);
    destination.insert(City::Erzurum, 4);
    destination.insert(City::Constantinople, 4);
    network.insert(City::Sevastopol, destination.clone());
    destination.clear();

    destination.insert(City::Sevastopol, 4);
    destination.insert(City::Sochi, 3);
    destination.insert(City::Angora, 3);
    network.insert(City::Erzurum, destination.clone());
    destination.clear();

    destination.insert(City::Sevastopol, 2);
    destination.insert(City::Rostov, 2);
    destination.insert(City::Erzurum, 3);
    network.insert(City::Sochi, destination.clone());
    destination.clear();

    network
}

fn get_big_tickets() -> Vec<Ticket> {
    let big_tickets = vec![
        Ticket::new(City::Brest, City::Petrograd, 20),
        Ticket::new(City::Cadiz, City::Stockholm, 21),
        Ticket::new(City::Edinburgh, City::Athina, 21),
        Ticket::new(City::Kobenhavn, City::Erzurum, 21),
        Ticket::new(City::Lisboa, City::Danzig, 20),
        Ticket::new(City::Palermo, City::Moskva, 20),
    ];

    big_tickets
}

fn get_tickets() -> Vec<Ticket> {
    let tickets = vec![
        Ticket::new(City::Amsterdam, City::Pamplona, 7),
        Ticket::new(City::Amsterdam, City::Wilno, 12),
        Ticket::new(City::Angora, City::Kharkov, 10),
        Ticket::new(City::Athina, City::Angora, 5),
        Ticket::new(City::Athina, City::Wilno, 11),
        Ticket::new(City::Barcelona, City::Bruxelles, 8),
        Ticket::new(City::Barcelona, City::Muenchen, 8),
        Ticket::new(City::Berlin, City::Bucuresti, 8),
        Ticket::new(City::Berlin, City::Moskva, 12),
        Ticket::new(City::Berlin, City::Roma, 9),
        Ticket::new(City::Brest, City::Marseille, 7),
        Ticket::new(City::Brest, City::Venezia, 8),
        Ticket::new(City::Bruxelles, City::Danzig, 9),
        Ticket::new(City::Budapest, City::Sofia, 5),
        Ticket::new(City::Edinburgh, City::Paris, 7),
        Ticket::new(City::Essen, City::Kyiv, 10),
        Ticket::new(City::Frankfurt, City::Kobenhavn, 5),
        Ticket::new(City::Frankfurt, City::Smolensk, 13),
        Ticket::new(City::Kyiv, City::Petrograd, 6),
        Ticket::new(City::Kyiv, City::Sochi, 8),
        Ticket::new(City::London, City::Berlin, 7),
        Ticket::new(City::London, City::Wien, 10),
        Ticket::new(City::Madrid, City::Dieppe, 8),
        Ticket::new(City::Madrid, City::Zuerich, 8),
        Ticket::new(City::Marseille, City::Essen, 8),
        Ticket::new(City::Palermo, City::Constantinople, 8),
        Ticket::new(City::Paris, City::Wien, 8),
        Ticket::new(City::Paris, City::Zagrab, 7),
        Ticket::new(City::Riga, City::Bucuresti, 10),
        Ticket::new(City::Roma, City::Smyrna, 8),
        Ticket::new(City::Rostov, City::Erzurum, 5),
        Ticket::new(City::Sarajevo, City::Sevastopol, 8),
        Ticket::new(City::Smolensk, City::Rostov, 8),
        Ticket::new(City::Sofia, City::Smyrna, 5),
        Ticket::new(City::Stockholm, City::Wien, 11),
        Ticket::new(City::Venezia, City::Constantinople, 10),
        Ticket::new(City::Warszawa, City::Smolensk, 6),
        Ticket::new(City::Zagrab, City::Brindisi, 6),
        Ticket::new(City::Zuerich, City::Brindisi, 6),
        Ticket::new(City::Zuerich, City::Budapest, 6),
    ];

    tickets
}

fn main() {
    let routes = create_network();
    let big_tickets = get_big_tickets();
    let tickets = get_tickets();

    println!("---- Big tickets ----");
    for ticket in big_tickets {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
    println!("---- Normal tickets ----");
    for ticket in tickets {
        println!(
            "Depart from {:?}, arrive at {:?} and get {} points",
            ticket.depart, ticket.arrive, ticket.value
        );
    }
    println!("---- Routes ----");
    for (node, destinations) in routes {
        println!("Starting from {:?} we can get to", node);
        for (destination, distance) in destinations {
            println!("--> {:?} with {} trains", destination, distance);
        }
    }
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
        for (city, destinations) in &routes {
            for (destination, distance) in destinations {
                assert_eq!(*distance, routes[destination][city]);
            }
        }
    }
}
