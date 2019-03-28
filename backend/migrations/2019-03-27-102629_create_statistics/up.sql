CREATE TABLE statistics (
  id SERIAL PRIMARY KEY NOT NULL,
  flights_cancelled INT NOT NULL,
  flights_delayed INT NOT NULL,
  flights_diverted INT NOT NULL,
  flights_on_time INT NOT NULL,
  flights_total INT NOT NULL AS (flights_cancelled + flights_delayed + flights_diverted + flights_on_time) STORED,
  minutes_delayed_carrier INT NOT NULL,
  minutes_delayed_late_aircraft INT NOT NULL,
  minutes_delayed_national_aviation_system INT NOT NULL,
  minutes_delayed_security INT NOT NULL,
  minutes_delayed_weather INT NOT NULL,
  minutes_delayed_total INT NOT NULL AS (minutes_delayed_carrier + minutes_delayed_late_aircraft + minutes_delayed_national_aviation_system + minutes_delayed_security + minutes_delayed_weather) STORED,
  number_of_delays_carrier INT NOT NULL,
  number_of_delays_late_aircraft INT NOT NULL,
  number_of_delays_national_aviation_system INT NOT NULL,
  number_of_delays_security INT NOT NULL,
  number_of_delays_weather INT NOT NULL,
  time DATE NOT NULL,
  carrier_id SERIAL NOT NULL,
  airport_id SERIAL NOT NULL,
  FOREIGN KEY (carrier_id)
  REFERENCES carriers (id),
  FOREIGN KEY (airport_id)
  REFERENCES airports (id),
  CONSTRAINT airport_carrier_time UNIQUE(time, carrier_id, airport_id)
)
