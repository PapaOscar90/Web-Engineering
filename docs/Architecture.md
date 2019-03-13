# Architecture Design
This document serves to define the Architecture of the Corgi Flight Statistics webapp.
##### Version 0.0.1
## Milestone Checklist

- [ ] Provide a high-level architecture of the Web app
    - [ ] including the technologies tobe used and their justification.
- [ ] Identify one or more value-added features to be offered through the Web app(optional).
- [ ] The use of a database for persistence is optional but recommended.
    - [ ] We design with the intention of keeping the levels of effort to implement a database being only that of implementing the database itself.
- [ ] Include diagrams

# High Level Architecture
## Client Server Architecture
Clients fetch data from the server. Server can be split to handle scaling in future, predicated on having abstract data access layer.

## Thick Client
View, rendering of data in charts etc, sorting, occur (mostly) in client. Can optimize and refresh data that is out of date. Might smartly utilize caching to reduce fetching (check into this).

# Technology Selection
TODO: Insert DIAGRAM HERE

## Backend
We are using Rust for our backend. This language strives for the trifecta of concurrency, speed, and saftey. 
### Rocket (Rust)
Type saftey, speed, familiarity in team, General Saftey, Security

### rustic_hal


## Frontend

### ReasonML (OCAML)
Solves JS problem (AKA, don't use it). We do not like JS.

### React (ReasonReact)
Functional Reactive Programming. Messages, state, model, reactions.

