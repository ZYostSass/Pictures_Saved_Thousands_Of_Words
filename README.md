# Pictures_Saved_Thousands_Of_Words

The goal of the project originally was to let users take the nasa's APOD and edit it in some fun way, per my proposal.
As I feared in said proposal, coming into this class with 0 rust and 0 web dev experience made that a biiit of a stretch!

I used HW2 as a baseline. Main changes for the project as it pertains to NASA'S API are found in:
backend/src/models/apod.rs
backend/src/routes/main_routes.rs
backend/src/db.rs
backend/templates/apod_page.html
backend/src/handlers.rs

Amongst other smaller file changes.
Tried to organize apod changes modularly and grouped  together in their relevant files.

# WARNING: RUNNING THE TEST SUITE BRICKED MY DATABASE. 

Everything works fine and dandy until I add my 'APOD' table, which refers to users ids. Docker insists 'table users' does not exist.
Although when trying to revert migrations, I'm told "other tables rely on table users." 
I can view all of my tables with their expected populated data using docker and beekeeper studio. Thus I wasn't sure
if this was a real bug/error on my end, or something getting tripped up in the db (cookies?)

That being said...

# TESTING:
I run docker compose up 
CD and cargo run in my backend
I run through all the examples in examples.http.
I use "email12@email.com, password" to log in.
This should direct a user to a landing page which is what I am most proud of as a new web dev.
ApodDisplayData takes the key parts of nasas APOD API and through routing and handlers, takes a user
To a landing page with all the relevant data AND even renders the image! Which is a rather simple function,
though was quite exciting getting that up and running as a newcomer. 
It renders the page referring to the most recent APOD thanks to using my api key upon render time.

# Project Requirements Met (To some degree)
Uses our HW2 as a baseline so I was able to refer to the code side by side and insert/remove things as needed
Such as using Casey's login code to redirect to the current APOD.
- Thus has buitl in frontend/backend in rust, axum and postgres.

Queries APOD at runtime and returns that data.
Dummy simple frontend.
User accounts, thanks to initial framework, have passwords and authenticate using jwts.
- Currently can only do so from the command line / examples.html to create account.
- Use email12@email.com, password

It is possible to save an APOD to a database and tie it to a user ID
- Users can not do this manually from the front end. Incomplete.
- Deleteing APOD not implemented.

Making queries using the the post in examples.http, which is fully routed, does place entries into the database.

# Project Requirements Missing (Entirely/practically)
There are no roles for users.
Users can not delete APOD entries.
While the data is saved in my APOD table, it is not accessible in a way akin to a "cache" as mentioned in project parameters. 

# The process, the dids and didnts
I started by creating an apod.rs model and seeing what datapoints I'd need by referring to nasas APOD api at https://api.nasa.gov/
Tested to make sure I could use their example GET request, then formatted that into my struct.
Added various functionalities, and wanted to make sure I could make a database of APODs.
Ran into some design problems here, realizing the scope of web dev + new language proved to be a challenge. 
- Lacked the knowledge to formulate my questions or narrow down what I wanted to do and how
- Baby steps. Just store some made up data using example.http as a referrence.
- Realize I only get the json text out of this
- Slowly forge this text into something usable; figure out how to get an image to display on a webpage
- Narrow down bits and pieces I care about, modularize them to be workable (see "ApodDispalyData" as my 'aha' moment)

I think my biggest "what didnt work" things were related to me not knowing how to get from point A to point B.
I wanted users to be able to save an image, I'd make an add APOD function, but then struggle to try and figure out
what needed routing, handlers, what could be purely backend, etc. This was an incredible learning oppurtunity for me. 
And gave me a whole newfound respect for web dev, which I already regarded highly! 
