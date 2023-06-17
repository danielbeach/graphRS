## graphs-rs

Using `Rust` to solve some real life `graph` problems. 

This is a exercise to learn more `Rust` and also better understand `graphs` of data
in real life situations, and what it looks like to solve those problems with `Rust`.

### Data Set
To emmulate real life data of `graphs`, we will use the Divvy Bike Trips open source
data set. It includes a few key data points that we can turn into a `graph`.

Mainly the `start` and `end` `_station_id` data points. These gives us `Nodes`, along with
telling us these two `Nodes` are conncted together since someome road a bike inbetween them.
Also, even better, we have the `time` / `duration` of that bike trip.

This will allow us to explore weighted `graph` problems. Since we can say it 
took `x` or `y` time to travel from one `node` to another `node`.

This is a real life data set and will allow us to ask interesting questions,
that hopefully we can use `Rust` to sovle, while at the same time working
on basic `DSA` skills.

![alt text](https://github.com/danielbeach/graphRS/imgs/graph.png?raw=true)
