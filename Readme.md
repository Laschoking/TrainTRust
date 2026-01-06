# TrainTRust Project

Monitor real-time prices of train tickets in Germany

## Roadmap

1. Get core functionality running: request prices based on the parameters Origin, Destination, Date
2. Load existing trips from database & update price information
3. configure proxy for caching
4. add command line arguments for easier use
5. configure project as a chrono job, to update the entire database once per day
6. include pricing for other European countries (pref. France) -> waiting for the API key from SNCF
7. analyze existing data for trends
8. move code to server

## Status

- if the API does not respond, check this [Status Link](https://stats.uptimerobot.com/57wNLs39M)
