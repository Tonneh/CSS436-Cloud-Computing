/* Prog 1, Tony Le */

import java.io.*;
import java.net.HttpURLConnection;
import java.net.URL;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class WebCrawl {
    /* Stores Number of Hops */
    private Integer num_hops;
    /* Stores Queues of URLs */
    Stack<Queue<String>> URLStack;
    /* HashSet used to check for duplicates */
    private HashSet<String> seen;
    private WebCrawl() {
        num_hops = 0;
        URLStack = new Stack<>();
        seen = new HashSet<>();
    }

    public static void main(String[] args) throws IOException {
        // Initial check to make sure we have two inputs
        if (args.length != 2) {
            System.out.println("Please enter a url and number of hops from the URL");
            System.exit(1);
        }
        WebCrawl webCrawler = new WebCrawl();
        webCrawler.num_hops = Integer.parseInt(args[1]);

        // Parsing first URL
        Queue<String> q = new LinkedList<>();
        q.add(args[0]);
        webCrawler.URLStack.add(q);
        webCrawler.performHops();
        System.out.println("Finished");
    }

    /*
        This function gets a URL from the queue and calls readURL passing in the URL
     */
    private void performHops() throws IOException {
        for (int i = 0; i <= num_hops && !URLStack.isEmpty(); i++)
        {
            Queue<String> currQueue = URLStack.peek();
            // if the currentQueue is empty, we can just return since we just came from a successful access
            if (currQueue.size() == 0)
                return;
            String URL = currQueue.peek();
            // We need to get a URL we haven't seen before, if we run out of urls in current queue (current page)
            // we can just return
            while (seen.contains(URL)) {
                currQueue.remove();
                // This means the queue is empty AND the urlstack is empty, so no more links so we'll just finish
                if (currQueue.isEmpty())
                    return;
                URL = currQueue.peek();
            }
            // If we weren't able to read the URL, then we need to go to next link
            // If the current queue is empty, we need to get a new one from the stack.
            while (!readUrl(URL))
            {
                // IF the top of stack isn't the same queue, that means we've updated it through a redirect,
                // so now we must go to the redirect, we can do that by just updating the queue if it's different
                if (currQueue != URLStack.peek())
                    currQueue = URLStack.peek();
                else
                    currQueue.remove();
                // if the currentQueue is empty and URLstack isn't empty then we'll update the queue
                while (currQueue.isEmpty() && !URLStack.isEmpty())
                {
                    URLStack.pop();
                    if (!URLStack.isEmpty())
                        currQueue = URLStack.peek();
                }
                // This means the queue is empty AND the urlstack is empty, so no more links so we'll just finish
                if (currQueue.isEmpty())
                    return;
                URL = currQueue.peek();
            }
        }
    }

    /*
        This function reads each line of the Urls, then checks if its an <a href>.
        If it is, then it'll push the http link into the queue.

        Input:
            String : the URL string
        Output:
            Boolean : If it was able to read the URL, false if got a code 300/400
     */
    private boolean readUrl(String stringUrl) throws IOException {
        try {
            URL url = new URL(stringUrl);
            HttpURLConnection connection = (HttpURLConnection) url.openConnection();
            int statusCode = connection.getResponseCode();
            Queue<String> q = new LinkedList<>();
            if (statusCode == 200) {
                BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
                // Regex to get all the <a href>
                Matcher matcher = Pattern.compile("<a.*?href=\"(http.*?)\"").matcher("");
                String line;
                // We're going to grab all the links on the current page
                while ((line = reader.readLine()) != null) {
                    matcher.reset(line);
                    while (matcher.find()) {
                        String httpLink = matcher.group(1);
                        // Removing all spaces and then adding a slash at the end of the link if don't have one
                        httpLink = httpLink.replaceAll("\\s", "");
                        httpLink = httpLink.endsWith("/") ? httpLink : httpLink + '/';
                        q.add(httpLink);
                    }
                }
                System.out.printf("Visited: %s, Found: %d URLs\n", stringUrl, q.size());
                URLStack.add(q);
                // Mainly for the initial URL, we need to remove all spaces, then add / if URL doesn't have
                stringUrl = stringUrl.replaceAll("\\s", "");
                stringUrl = stringUrl.endsWith("/") ? stringUrl : stringUrl + '/';
                seen.add(stringUrl);
                // Successfully accessed, so return true
                return true;
            } else if (statusCode >= 300 && statusCode <= 399) {
                // get the redirectURL
                String RedirectURL = connection.getHeaderField("Location");
                q.add(RedirectURL);
                URLStack.add(q);
                System.out.printf("Redirect from: %s to %s\n", stringUrl, RedirectURL);
                return false;
            }
            // This is for the 400s and 500s
            System.out.printf("Unable to access %s, Code: %d\n", stringUrl, statusCode);
        } catch (IOException e) {
            // We couldn't access the URL for some reason
            System.out.printf("Unable to access %s\n", stringUrl);
        }
        seen.add(stringUrl);
        return false;
    }
}
