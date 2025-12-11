# Shapeland
```
                      X                       
                     X X           X X X X X  
X X X X X X X       X   X         X         X 
                   X     X       X           X
X           X     X       X      X           X
                 X         X                  
X           X   X           X    X           X
               X             X    X         X 
X           X X               X    X       X  
                                              
                                              
  --------        Shapeland      ---------                

```

Shapeland park simulator: Where all the rides are shapes and all the guests are dots!

This is a class project for Portland State University CS523. It will create a 
simplified version of the Defunctland Shapeland simulation 
(see [57:30 of "Disney's FastPass: A Complicated History"](https://www.youtube.com/watch?v=9yjZpBq1XBE&t=3450s)E) 

---


All dot guests enter the park at the same time and stay for 9 hours. Dots have 3 possible states:
* In Queue
* Doing Activity
* Idle

Every 5 minute interval, all Idle dots enter the Decision Matrix, where the dot will decide what to do.
First there is a weighted coin flip (based on the dot's preferences) on whether they will try to ride and coaster or do an activity.
If they try to ride a ride, the dot will check the ride's current wait time. If it is under 30 minutes, they will automatically join the line
and their state will change to In Queue.

If the wait is more then 30 minutes, and fastpass is enabled,
they will check if a fastpass is available and that they do not already have a fastpass. 

Fastpass works by keeping a dot's place in the ride's virtual queue. If they succeed in getting a fastpass, they will be assigned a return time
equal to the wait time they would have spent in the line. Then they will return to the Decision Matrix to either ride a different ride or do an activity. Their state will remain Idle until another decision is made. 

If fastpass is not available, the dot will consider if the wait time is more than their Balking Point (aka, if the line is too long).
If so, they will balk, and return to the Decision Matrix to choose something else to do. If not, they will join the line. 

Activities always succeed, and will be assigned a random duration between 5 and 45 minutes, or a lesser amount if 
a Dot has a fastpass return time of less than 45 minutes. 

Each ride as its own service rate (# of guests that can ride the ride and exit the queue per 5 minutes) and popularity
(higher popularity = more likely for a Dot to choose to want ride it)

## User configurable settings:

* FastPass enabled: 
  ** Whether to implement the FastPass system in your park
    (See how this affects your guests' average time in a queue with option [4]! )
* Number of average guests in the park:
  ** Guests with a more even preference between riding rides and doing an activity (7:3)
* Number of ride enthusiast guests:
  ** Guests who would strongly prefer to ride rides (9:1), and have a higher balking point 
    to accommodate that preference
  ** Note: It is recommended to not have the total number of guests be below ~300
    As this would be unprofitable and make the shareholders very sad :( 
* Verbose mode:
  ** Print out all the information you could possibly want about every dot in the park
    at every time interval
* Stat tracking mode:
  ** Print out some extra aggregated stats about the dots, rides ridden, and activities at the end of the park day


## Assumptions and Limitations
* Changes in Dot state happen instantaneously 
  - rides are instant / less than the 5 minute interval mark
  - Dots teleport to the needed locations 
  - No time is spent doing anything other than doing activities or being in queue
* Rides have a constant service rate and never break down
* All Dots stay at the park for the entire duration of the park's open hours ( 9 hours )




---


Developer Comments:

I realized approximately 5 hours into this project that Rust is not a great language for data visualization (yet).
And everything interesting about this project idea relies on data visualization. 

I really wanted to include some cool coaster graphics and graphs showing the park over time, but was never able to
implement it is a satisfactory way

Almost all functionality is is within the main() function, as I started this project with a bad understanding of how to properly
borrow and clone variables properly. If I could go back, I would move functionality into more functions so that unit testing
would be more practical


