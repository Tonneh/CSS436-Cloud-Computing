/* Prog2 Tony Thanh Le */
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import java.io.*;
import java.net.HttpURLConnection;
import java.util.ArrayList;
import java.net.URL;


public class MyCity {

    public static void main(String[] args) {
        if (args.length != 1) {
            System.out.println("Please enter one city");
            return;
        }
        String city = args[0];
        // Replace all spaces with + since + means space in URLs
        city = city.replaceAll("\\s", "+");
        MyCity myCity = new MyCity();
        myCity.geoLocationAPI(city);
        myCity.cityAPI(city);
        myCity.weatherAPI(city);
    }

    /*
    * This function will get the JSON and return it, it'll also do retries on 500 codes
    * Up to 5 retries where the last retry will cap out at 8 seconds.
    *
    * Input:
    *   String : The URL String
    *   String : The Header Name - only needed if api needs key through header
    *   String : Api Key - only needed if api needs key through header
    *
    * Output:
    *   JsonElement : the json that it got from the call.
    * */
    private JsonElement getJSONFromURL(String URL, String headerName, String APIKey) {
        try {
            URL url = new URL(URL);
            HttpURLConnection connection = (HttpURLConnection) url.openConnection();
            connection.setRequestMethod("GET");
            // This is for APIs that want key in header
            if (!headerName.isEmpty()) {
                connection.setRequestProperty(headerName, APIKey);
            }
            int statusCode = connection.getResponseCode();
            // if code is 200 we're good to just return the json, otherwise we try again
            if (statusCode >= 200 && statusCode <= 299) {
                BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
                return JsonParser.parseReader(reader);
            } else if (statusCode >= 500 && statusCode <= 599) {
                int retryAttempt = 0;
                while (statusCode >= 500 && statusCode <= 599) {
                    System.out.printf("Retrying.....Status Code %d .... Waiting %d seconds\n", statusCode, retryAttempt);
                    // Stops the program for 1000 milliseconds * the retryAttempt variable
                    Thread.sleep(1000L * retryAttempt);
                    connection = (HttpURLConnection) url.openConnection();
                    statusCode = connection.getResponseCode();
                    if (statusCode == 200) {
                        BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
                        return JsonParser.parseReader(reader);
                    }
                    // If the retry attempt is greater than or equal 8 seconds, we can just print and error and return null
                    if (retryAttempt >= 8) {
                        System.out.println("Error: try again later");
                        return null;
                    }
                    retryAttempt = retryAttempt == 0 ? 1 : retryAttempt * 2;
                }
            }
        } catch (IOException | InterruptedException e) {
            System.out.println("Error");
        }
        return null;
    }

    /* Gets the Json from getJSONFromURL(), then gets the weather and temperature of the city and prints it out. */
    private void weatherAPI(String city) {
        String APIKey = "removed for safety";
        String URL = "http://api.openweathermap.org/data/2.5/weather?q=";
        JsonElement json = getJSONFromURL(URL + city + "&units=imperial" + "&APPID=" + APIKey, "", "");
        try {
            // Get the info about the city
            JsonObject cityObject = json.getAsJsonObject();

            // This is for getting the current weather description
            JsonArray weatherArray = cityObject.get("weather").getAsJsonArray();
            ArrayList<String> descriptionList = new ArrayList<>();
            for (JsonElement weatherElement : weatherArray) {
                JsonObject weatherObject = weatherElement.getAsJsonObject();
                descriptionList.add(weatherObject.get("description").getAsString());
            }

            // Get info of the city
            JsonObject temperatureInfo = cityObject.get("main").getAsJsonObject();
            Double temperature = temperatureInfo.get("temp").getAsDouble();
            Double feels_like = temperatureInfo.get("feels_like").getAsDouble();
            Double humidity = temperatureInfo.get("humidity").getAsDouble();
            Double maxTemp = temperatureInfo.get("temp_max").getAsDouble();
            Double minTemp = temperatureInfo.get("temp_min").getAsDouble();

            // Printing out the info
            System.out.printf(
                    "OpenWeatherAPI: \n" +
                            "   Temperature: %.2fF\n" +
                            "   Min Temperature: %.2fF\n" +
                            "   Max Temperature: %.2fF\n" +
                            "   Feels Like: %.2fF\n" +
                            "   Humidity: %.2f%% \n" +
                            "   Description:\n", temperature, minTemp, maxTemp, feels_like, humidity);
            for (String s : descriptionList) {
                System.out.printf("         %s\n", s);
            }
        } catch (NullPointerException e) {
            System.out.println("Error calling the OpenWeatherAPI");
        }
    }

    /* Gets the Json from getJSONFromURL(), then gets the population and country of the city. */
    private void cityAPI(String city) {
        String apiKey = "removed for safety";
        String URL = "https://api.api-ninjas.com/v1/city?name=";
        JsonElement json = getJSONFromURL(URL + city, "X-Api-Key", apiKey);
        try {
            JsonObject cityObject = json.getAsJsonArray().get(0).getAsJsonObject();
            Integer population = cityObject.get("population").getAsInt();
            String country = cityObject.get("country").getAsString();
            // Is capital city of country, doesn't really work for the US, can try Paris
            String isCapital = cityObject.get("is_capital").getAsString();
            System.out.printf(
                    "City API:\n" +
                            "   Population: %d\n" +
                            "   Country: %s\n" +
                            "   Capital?: %s\n", population, country, isCapital);
        } catch (IndexOutOfBoundsException | NullPointerException e) {
            System.out.println("Error calling the CityAPI");
        }
    }

    /* Gets the Json from getJSONFromURL(), then gets the date, time, timezone, and country of the city */
    private void geoLocationAPI(String city) {
        String apiKey = "removed for safety";
        String URL = "https://api.ipgeolocation.io/timezone?apiKey=";
        JsonElement json = getJSONFromURL(URL + apiKey + "&location=" + city + "&lang=en", "", "");
        try {
            JsonObject geoLocationObject = json.getAsJsonObject();
            String currentDate = geoLocationObject.get("date").getAsString();
            String currentTime = geoLocationObject.get("time_12").getAsString();
            String timeZone = geoLocationObject.get("timezone").getAsString();
            String country = geoLocationObject.get("geo").getAsJsonObject().get("country").getAsString();
            // The country might be outputed with random characters, this is due to it being in a language with non english characters
            System.out.printf("GeolocationAPI:\n" +
                    "   Current Date: %s\n" +
                    "   Current Time: %s\n" +
                    "   Timezone: %s\n" +
                    "   Country: %s\n", currentDate, currentTime, timeZone, country);
        } catch (NullPointerException e) {
            System.out.println("Error calling GeoLocationAPI");
        }
    }
}
