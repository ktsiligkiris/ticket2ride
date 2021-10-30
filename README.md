# Ticket2ride project

## Purpose

This project started as a mind exercise, when I asked myself what the
largest theoretical score in this game could be. Before I started
this, I read in a few places opinions about high score routes, but
there was no evidence if these where the optimum solutions. So I told
myself, this is a nice project to work on my programming skills in
rust and in exploring algorithms for solving this.

## Architecture

This is the first attempt and no specific algorithmic logic is used,
only brute force, basically.

The user, when executing the cli program, can provide the starting
city for the algorithm with the `-c` or `--city` option. If no city if
provided or if the city provided is non existent, then Edinburgh is
used by default.

The program execution consists of two steps. In the first step, all
possible routes are explored, starting from the initial city. In the
second step, all these possible routes are scored, and then the route
with the highest score is found.

The correctness of the program is not certain. It seems to create
valid routes and after fixing a logic bug, the score is correct,
taking into account:
* one of the big tickets
* all small tickets that are covered
* the score of the trains used to connect the route
* the score of 10 for the express bonus
* the score of 12 for three unused stations

Also, I guess because of the use of HashMaps, the maximum number of
routes computed each time differs. My reasoning is that because
HashMaps are not ordered and each time the way that are stored in
memory is kind of arbitrary, the computed routes differ also. Maybe
there is some bug in my logic there that I need to check.

One final consideration is that all routes start from the initial city
and no route is created where the initial city is in the middle of a
route.

## Future considerations

This that I could do in the future:
- [ ] Add color of routes
- [ ] Experiment with different algorithms for traversing the network
- [ ] Use stations for fulfilling more tickets
