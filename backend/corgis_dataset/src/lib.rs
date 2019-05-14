#![deny(missing_docs)]

//! # Corgis - Core API Documentation
//!
//! A library providing access to the data from the [CORGIS Datasets Project](https://think.cs.vt.edu/corgis/). It wraps the data into Rust structs and uses [serde](https://github.com/serde-rs/serde) to extract the data from the [CORGIS JSON](https://think.cs.vt.edu/corgis/json/index.html).
//!
//! ## Wrapped Datasets
//!
//! | Dataset                                                                                                       | Supported Version |
//! | :--                                                                                                           | :--               |
//! | [Aids](https://think.cs.vt.edu/corgis/json/aids/aids.html)                                                    |                   |
//! | [Airlines](https://think.cs.vt.edu/corgis/json/airlines/airlines.html)                                        | Version 1         |
//! | [Billionaires](https://think.cs.vt.edu/corgis/json/billionaires/billionaires.html)                            |                   |
//! | [Broadway](https://think.cs.vt.edu/corgis/json/broadway/broadway.html)                                        |                   |
//! | [Business Dynamics](https://think.cs.vt.edu/corgis/json/business_dynamics/business_dynamics.html)             |                   |
//! | [Cancer](https://think.cs.vt.edu/corgis/json/cancer/cancer.html)                                              |                   |
//! | [Cars](https://think.cs.vt.edu/corgis/json/cars/cars.html)                                                    |                   |
//! | [Classics](https://think.cs.vt.edu/corgis/json/classics/classics.html)                                        |                   |
//! | [Construction Permits](https://think.cs.vt.edu/corgis/json/construction_permits/construction_permits.html)    |                   |
//! | [Construction Spending](https://think.cs.vt.edu/corgis/json/construction_spending/construction_spending.html) |                   |
//! | [County Crime](https://think.cs.vt.edu/corgis/json/county_crime/county_crime.html)                            |                   |
//! | [County Demographics](https://think.cs.vt.edu/corgis/json/county_demographics/county_demographics.html)       |                   |
//! | [Drug Bank](https://think.cs.vt.edu/corgis/json/drug_bank/drug_bank.html)                                     |                   |
//! | [Drugs](https://think.cs.vt.edu/corgis/json/drugs/drugs.html)                                                 |                   |
//! | [Earthquakes](https://think.cs.vt.edu/corgis/json/earthquakes/earthquakes.html)                               |                   |
//! | [Education](https://think.cs.vt.edu/corgis/json/education/education.html)                                     |                   |
//! | [Election](https://think.cs.vt.edu/corgis/json/election/election.html)                                        |                   |
//! | [Energy](https://think.cs.vt.edu/corgis/json/energy/energy.html)                                              |                   |
//! | [Finance](https://think.cs.vt.edu/corgis/json/finance/finance.html)                                           |                   |
//! | [Food](https://think.cs.vt.edu/corgis/json/food/food.html)                                                    |                   |
//! | [Food Access](https://think.cs.vt.edu/corgis/json/food_access/food_access.html)                               |                   |
//! | [Global Development](https://think.cs.vt.edu/corgis/json/global_development/global_development.html)          |                   |
//! | [Graduates](https://think.cs.vt.edu/corgis/json/graduates/graduates.html)                                     |                   |
//! | [Health](https://think.cs.vt.edu/corgis/json/health/health.html)                                              |                   |
//! | [Hospitals](https://think.cs.vt.edu/corgis/json/hospitals/hospitals.html)                                     |                   |
//! | [Hydropower](https://think.cs.vt.edu/corgis/json/hydropower/hydropower.html)                                  |                   |
//! | [Immigration](https://think.cs.vt.edu/corgis/json/immigration/immigration.html)                               |                   |
//! | [Injuries](https://think.cs.vt.edu/corgis/json/injuries/injuries.html)                                        |                   |
//! | [Labor](https://think.cs.vt.edu/corgis/json/labor/labor.html)                                                 |                   |
//! | [Medal of Honor](https://think.cs.vt.edu/corgis/json/medal_of_honor/medal_of_honor.html)                      |                   |
//! | [Music](https://think.cs.vt.edu/corgis/json/music/music.html)                                                 |                   |
//! | [Publishers](https://think.cs.vt.edu/corgis/json/publishers/publishers.html)                                  |                   |
//! | [Real Estate](https://think.cs.vt.edu/corgis/json/real_estate/real_estate.html)                               |                   |
//! | [Retail Services](https://think.cs.vt.edu/corgis/json/retail_services/retail_services.html)                   |                   |
//! | [School Scores](https://think.cs.vt.edu/corgis/json/school_scores/school_scores.html)                         |                   |
//! | [Skyscrapers](https://think.cs.vt.edu/corgis/json/skyscrapers/skyscrapers.html)                               |                   |
//! | [Slavery](https://think.cs.vt.edu/corgis/json/slavery/slavery.html)                                           |                   |
//! | [State Crime](https://think.cs.vt.edu/corgis/json/state_crime/state_crime.html)                               |                   |
//! | [State Demographics](https://think.cs.vt.edu/corgis/json/state_demographics/state_demographics.html)          |                   |
//! | [State Fragility](https://think.cs.vt.edu/corgis/json/state_fragility/state_fragility.html)                   |                   |
//! | [Suicide Attacks](https://think.cs.vt.edu/corgis/json/suicide_attacks/suicide_attacks.html)                   |                   |
//! | [Supreme Court](https://think.cs.vt.edu/corgis/json/supreme_court/supreme_court.html)                         |                   |
//! | [Tate](https://think.cs.vt.edu/corgis/json/tate/tate.html)                                                    |                   |
//! | [Weather](https://think.cs.vt.edu/corgis/json/weather/weather.html)                                           |                   |

pub mod airlines;
