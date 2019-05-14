// Request the data needed for the get_statistics_connection routes from the database.
// fn get_statistics_connection_data(
//     conn: &diesel::SqliteConnection,
//     airport1: String,
//     airport2: String,
//     carrier: Option<String>,
// ) -> Result<Vec<HashMap<String, f64>>, NotFound<String>> {
//     use crate::database::{
//         models::{Airport, Carrier},
//         schema::{airports, carriers, statistics},
//     };

//     let airport1 = airports::table
//         .filter(airports::code.eq(airport1))
//         .first::<Airport>(conn)
//         .expect("Error loading airport");

//     let airport2 = airports::table
//         .filter(airports::code.eq(airport2))
//         .first::<Airport>(conn)
//         .expect("Error loading airport");

//     use diesel::dsl::avg;

//     match carrier {
//         Some(carrier) => {
//             let carrier = carriers::table
//                 .filter(carriers::code.eq(carrier))
//                 .first::<Carrier>(conn)
//                 .expect("Error loading carrier");

//             let mean1_late_aircraft: Option<BigDecimal> = statistics::table
//                 .filter(statistics::airport_id.eq(airport1.id))
//                 .filter(statistics::carrier_id.eq(carrier.id))
//                 .select(avg(statistics::minutes_delayed_late_aircraft))
//                 .get_result(conn)
//                 .unwrap();

//             let mean1_delayed_carrier: BigDecimal = statistics::table
//                 .filter(statistics::airport_id.eq(airport1.id))
//                 .filter(statistics::carrier_id.eq(carrier.id))
//                 .select(avg(statistics::minutes_delayed_carrier))
//                 .get_result(conn)
//                 .unwrap()
//                 .unwrap();

//             let mean2_late_aircraft: BigDecimal = statistics::table
//                 .filter(statistics::airport_id.eq(airport2.id))
//                 .filter(statistics::carrier_id.eq(carrier.id))
//                 .select(avg(statistics::minutes_delayed_late_aircraft))
//                 .get_result(conn)
//                 .unwrap()
//                 .unwrap();
//             let mean2_delayed_carrier: BigDecimal = statistics::table
//                 .filter(statistics::airport_id.eq(airport2.id))
//                 .filter(statistics::carrier_id.eq(carrier.id))
//                 .select(avg(statistics::minutes_delayed_carrier))
//                 .get_result(conn)
//                 .unwrap()
//                 .unwrap();

//             println!(
//                 "{} {} {} {}",
//                 mean1_late_aircraft,
//                 mean2_late_aircraft,
//                 mean1_delayed_carrier,
//                 mean2_delayed_carrier
//             );

//             // let carrier_delays = DescriptiveStatistics {
//             //     mean: (mean1_delayed_carrier + mean2_delayed_carrier) / 2.,
//             // };
//             // let late_aircraft_delays = DescriptiveStatistics {
//             //     mean: (mean1_late_aircraft + mean2_late_aircraft) / 2.,
//             // };

//             // Ok(vec![ConnectionStatistics {
//             //     carrier_delays,
//             //     late_aircraft_delays,
//             // }])
//             unimplemented!();
//         }
//         None => {
//             unimplemented!();
//         }
//     }
// }

// /// Get the JSON representation of the connections statistics between 2 airports.
// #[get(
//     "/connection?<airport1>&<airport2>&<carrier>",
//     format = "application/json",
//     rank = 1
// )]
// pub fn get_statistics_connection_json(
//     conn: CorgisDbConn,
//     airport1: String,
//     airport2: String,
//     carrier: Option<String>,
// ) -> Result<Json<HashSet<String, f64>>, diesel::result::Error> {
//     get_statistics_connection_data(&conn, airport1, airport2, carrier).map(Json)
// }

// /// Get the CSV representation of the connections statistics between 2 airports.
// #[get(
//     "/connection?<airport1>&<airport2>&<carrier>",
//     format = "text/csv",
//     rank = 2
// )]
// pub fn get_statistics_connection_csv(
//     conn: CorgisDbConn,
//     airport1: String,
//     airport2: String,
//     carrier: Option<String>,
// ) -> Result<Csv<HashMap<String, f64>>, diesel::result::Error> {
//     fn convertor(statistics: &Vec<ConnectionStatistics>) -> String {
//         let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
//         for statistic in statistics {
//             wtr.serialize(statistic).unwrap();
//         }
//         String::from_utf8(wtr.into_inner().unwrap()).unwrap()
//     };
//     get_statistics_connection_data(&conn, airport1, airport2, carrier)
//         .map(|data| Csv(data, convertor))
// }

// /// Get the HAL representation of the connections statistics between 2 airports.
// #[get(
//     "/connection?<airport1>&<airport2>&<carrier>",
//     format = "application/hal+json",
//     rank = 3
// )]
// pub fn get_statistics_connection_hal(
//     conn: CorgisDbConn,
//     airport1: String,
//     airport2: String,
//     carrier: Option<String>,
// ) -> Result<Json<HalResource>, Error> {
//     let data = get_statistics_data(&conn, statistics)?;
//     let result = HalResource::new(&data)
//         .with_link("self", format!("/statistics/{}", data.id))
//         .with_link("delete", format!("/statistics/{}", data.id))
//         .with_link("patch", format!("/statistics/{}", data.id))
//         .with_link("put", format!("/statistics/{}", data.id));

//     Ok(Json(result))
// }

// /// Get the default representation of the connections statistics between 2 airports. This is executed if the other routes are not matched.
// #[get("/connection?<airport1>&<airport2>&<carrier>", rank = 4)]
// pub fn get_statistics_connection_default(
//     conn: CorgisDbConn,
//     airport1: String,
//     airport2: String,
//     carrier: Option<String>,
// ) -> Result<Json<HashMap<String, f64>>, diesel::result::Error> {
//     get_statistics_connection_json(conn, airport1, airport2, carrier)
// }
