# emmuoviti - the API part

This is the API part to fetch some amenity POIs from OpenStreetMap and to provide them to the frontend part. It includes a small process to handle a GeoJSON file exported from export.hotosm.org and to store the data in the Postgres database. It exposes the POIs with a paginated endpoint and a way to fetch them in a radius.

Other endpoints that I have in mind:

- `/pois-by-polygon` to fetch the POIs in a polygon
- `/pois-by-bbox` to fetch the POIs in a bounding box
- `/pois-by-tag` to fetch the POIs by tag

Probably should follow a better RESTful API design.

The frontend will be probably written in Leptos, adding some TailwindCSS. The mission is to do it in the most type-safe possible way. Perhaps the structs will move in a shared crate, or some other solution. Leptos will also make use of RPC to fetch the data from the API, in the style of server components. I might also implement the same frontend with React to compare the two, and then another modern solution like Solid.js or Svelte.

I also want to design the ma.

## How to run

1. Install it with cargo: `cargo install --path .`
2. Set the environment variables `DATABASE_URL` in a .env file.
3. Build and run it with `cargo run`.

## Some performance numbers

Here is what my laptop did, a Macbook Pro 2019 with 2.3 GHz 8-Core Intel Core i9 and 32 GB 2667 MHz DDR4.

```bash
➜  ~ autocannon -c 100 -d 5 -p 10 http://localhost:8080/pois\?page\=202 
Running 5s test @ http://localhost:8080/pois?page=202
100 connections with 10 pipelining factor


┌─────────┬────────┬────────┬────────┬────────┬───────────┬──────────┬────────┐
│ Stat    │ 2.5%   │ 50%    │ 97.5%  │ 99%    │ Avg       │ Stdev    │ Max    │
├─────────┼────────┼────────┼────────┼────────┼───────────┼──────────┼────────┤
│ Latency │ 107 ms │ 120 ms │ 179 ms │ 225 ms │ 126.06 ms │ 20.68 ms │ 283 ms │
└─────────┴────────┴────────┴────────┴────────┴───────────┴──────────┴────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 6.871   │ 6.871   │ 7.863   │ 8.479   │ 7.820   │ 526,57  │ 6.869   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 16.3 MB │ 16.3 MB │ 18.7 MB │ 20.2 MB │ 18.6 MB │ 1.25 MB │ 16.3 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 5

40k requests in 5.04s, 93 MB read
```

And here is the included node.js server with Express.js:

```bash
autocannon -c 100 -d 5 -p 10 http://localhost:3000/pois\?page\=202 
Running 5s test @ http://localhost:3000/pois?page=202
100 connections with 10 pipelining factor


┌─────────┬────────┬────────┬────────┬────────┬───────────┬─────────┬────────┐
│ Stat    │ 2.5%   │ 50%    │ 97.5%  │ 99%    │ Avg       │ Stdev   │ Max    │
├─────────┼────────┼────────┼────────┼────────┼───────────┼─────────┼────────┤
│ Latency │ 298 ms │ 314 ms │ 472 ms │ 559 ms │ 333.89 ms │ 51.7 ms │ 618 ms │
└─────────┴────────┴────────┴────────┴────────┴───────────┴─────────┴────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 1.976   │ 1.976   │ 3.173   │ 3.247   │ 2.904,4 │ 481,86  │ 1.976   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 4.96 MB │ 4.96 MB │ 7.97 MB │ 8.16 MB │ 7.29 MB │ 1.21 MB │ 4.96 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 5

16k requests in 5.03s, 36.5 MB read
```

Difference is quite big (40k vs 16k requests in 5 seconds). Latency is down in a big way (126 ms vs 333 ms). 

This is mostly interesting because writing Rust was harder, but not so hard. Took me around 5 hours in total. The Node.js example was just written out by Copilot, it's not particularly good, but it's fine.

## License

