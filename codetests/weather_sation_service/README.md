# Weather Station Service

## Introduction 
The purpose of this test is for you to demonstrate your strengths. Don't spend more than a few hours working on this. You can code it in any language you are comfortable with 
Task 
Create an HTTP Service that reports on Melbourne weather. This service will source its information from either of the below providers: 
1. weatherstack (primary): 
curl "h ttp://api.weatherstack.com/current?access_key=YOUR_ACCESS_KEY&query=Melbourne" Documentation: https://weatherstack.com/documentation 
2. OpenWeatherMap (failover): 
curl "http://api.openweathermap.org/data/2.5/weather?q=melbourne,AU&appid=2326504fb9b100 bee 21400190e4dbe6d" 
Documentation: https://openweathermap.org/current Specs 
● The service can hard-code Melbourne as a city. 
● The service should return a JSON payload with a unified response containing 
temperature in degrees Celsius and wind speed. 
● If one of the provider goes down, your service can quickly failover to a different provider 
without affecting your customers. 
● Have scalability and reliability in mind when designing the solution. 
● Weather results are fine to be cached for up to 3 seconds on the server in normal 
behaviour to prevent hitting weather providers. Those results must be served as stale if all weather providers are down. 
● The proposed solution should allow new developers to make changes to the code safely. 
Expected Output 
Calling the service via curl (http://localhost:8080/v1/weather?city=melboune)​should output the following JSON payload { "wind_speed": 20, "temperature_degrees": 29 
} Please provide: 
● Working code and instructions provided as zip or hosted on private GitHub (share with kurl, mike-asm​and richarda2b) 
● Running code hosted or instructions to build and run locally provided. 
● Trade-offs you might have made, anything you left out, or what you might do differently if 
you were to spend additional time on the task. 
