# Track my macros

## Context

### Macros

- fats (9 kcal per gram)
- proteins (4 kcal per gram) 
- carbohydrates (4 kcal per gram)

### Goals

- calorie intake per day
- calorie repartition goal:
    - fats => 45%-65% of energy
    - prots => 10%-35% of energy
    - carbs => 20%-35% of energy

## App

### Initial set-up

1. Enter goal calorie intake:
- manually
- calculated through weight, height and goal

2. Enter repartition goal:
- manually
- based on goal/activity

### Macros input: Foods

Foods are composed of fats/100g, prots/100g, carbs/100g.

Possibly also more details such as fibers/100g, saturated vs monounsaturated vs polyunsaturated fats, different dietary carbs etc.

We should only add macros intake by selecting some existing Foods or creating a new one on the fly.

They should have tags (e.g. Meal, HelloFresh).

#### Adding foods

If possible, common Foods should be created from an existing database to simplify getting started with the app.
You should then be able to add custom foods if you want.

### Weight input

Renpho -> sync with Google Fit and then query the Google Fit API.

Depending on how the Google Fit API works, we might want to do it on demand or with a CRON.

### Viewing data

#### Today's macros

- calories eaten so far
- calories missing for today's goal (or calories over goal)
- same thing for each macro (fats/carbs/prots)

#### Past data

Visualisation: calories over time vs goal, macros over time vs goal


## Tech

Since this is a side project I'm just doing for fun, technical choices won't be made with productivity or fitness to context in mind,
but simply things I'd like to use and that can probably work.

### Overview

Backend is a GraphQL API with some database, that can authenticate users.
It is reponsible for making macros/calories calculations, holding Foods information, holding user information.

It could at some point ingest Weight data from public APIs.

### Things I want to use

They don't really match any technical requirements but I just want to use them anyway.

- Rust
- GraphQL
- Nomad
- Yew
- Waypoint?
- https://github.com/mit-pdos/noria ? Maybe ElasticSearch and some time-series database would make sense too (instead of, or on top of Noria).
- Maybe Flutter later?

### Things I'd like to avoid

They are tools I would normally strongly consider, but I want to see other options this time.

- Golang
- Docker
- Vue.js
