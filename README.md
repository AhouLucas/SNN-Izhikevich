# SNN-Izhikevich

Rust-based Spiking Neural Network (SNN) based on [Izhikevich 2003](https://www.izhikevich.org/publications/spikes.pdf). In this project, I will use my model of multiple populations to simulate, as an example, a decision-making process between two excitatory populations that aims to modelize a [Two-alternative forced choice (2AFC)](https://en.wikipedia.org/wiki/Two-alternative_forced_choice) task.

I decided to start this project after taking a course about mathematical models in neuroscience [(LGBIO2072)](https://uclouvain.be/cours-2025-lgbio2072) as part of my master's in Applied Mathematics at Université Catholique de Louvain.


## TODO

- [ ] Write step function for whole network (trivial)
- [ ] Create a GUI to initialize populations with user-defined configuration and allow to run the simulation from there
- [ ] In the GUI, implement a 2AFC to observe which population wins in a race to decision-making