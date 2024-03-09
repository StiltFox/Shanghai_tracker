# Stilt Fox&trade; Shanghai(上海) game tracker
This is a sample test project used to train those at Stilt Fox&trade; on how to use rust. This project lacks most things a set of our production code would contain. There's no unit tests, there's no verification of how well it works, simply put this was slapped together and shared for tinkering purposes.

## project goal
The goal is to track users over time playing a card game known as Shanghai(上海) over time. The database will track the user's final scores over games. This allows other data to be inferred like who won the most games over a period of time.

Things to add:
 - generic support for databases
    - right now this db only supports mariadb or mysql.
 -  better typing for the database
    - right now we're not using Dates or Uuid's in the database because of the difficulty supporting certain data types using the built in instance of Sqlx.
 - more endpoints that calculate player stats, or allow for selection of games based on a date range.

 ## Rocket.toml
 Although it's not included in this repo, adding a rocket.toml to the running directory of the program will allow you to change the settings of the server and the database it's connecting to. This is not compiled with the program and will be need to be deployed separately.