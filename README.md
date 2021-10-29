# Ticket2ride project

## Purpose

This project started as a mind exercise, when I asked myself what the
largest theoretical score in this game could be. Before I started
this, I read in a few places opinions about high score routes, but
there was no evidence if these where the optimum solutions. So I told
myself, this is a nice project to work on my programming skills in
rust and in exploring algorithms for solving this.

This is the first attempt and there is no logic, only brute force is
used. In the first step, all possible routes are explored (starting
from Edinburgh if I remember correctly) and then all these possible
routes are scored, and then I get the highest score. I' not sure that
this is correct, it seems to bring routes that are valid and that have
large scores, but each time I run it I get a different result of
maximum number of routes.

Also one this that is odd is that the number of routes and the maximum
score route is different if I run the program in debug mode or in
release mode, I'm not sure if something like that should happen.

## Future considerations

After tackling this problem, I am thinking of checking solutions in
other games too, or tackling this same problem with different
algorithms, maybe genetic algorithms?
