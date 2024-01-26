# Weather CLI App

## Overview

This is a simple Command Line Interface (CLI) application for fetching weather data. The application retrieves current weather information using the OpenWeatherMap API.

## Prerequisites

Before you run the application, you need to have an API key from OpenWeatherMap. You can obtain an API key by signing up at [OpenWeatherMap](https://openweathermap.org/appid).

## Configuration

Once you have your API key, you need to add it to your environment variables:

1. Create a `.env` file in the root directory of the project.
2. Add the following line to your `.env` file:

```
WEATHER_API_KEY=your_api_key_here
```

Replace `your_api_key_here` with your actual API key from OpenWeatherMap.

## Running the Application

From the root directory, run:

```
cargo run
```

The default city is Seattle, but you can change this with:

```
cargo run -- --city Reno
```

OR

```
cargo run -- -c Reno
```

Note: If you want to enter a city with multiple words, like "New York" you'll need to wrap the full city in quotations:

```
cargo run -- --city "New York"
```
