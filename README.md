# Scrapa

This is an extremely simple application to monitor changes to a website in Rust.

It loads a remote URL, creates a "result" out of it, and checks to see if it's any different from previous results.

More specifically, it monitors [Microsoft's SurFace Store](https://www.microsoft.com/en-us/surface) to detect when new devices are added to their inventory. I wanted to monitor the store to detect when the new Surface Laptop Studio 2 shows up for buying (see [this Mastodon thread](https://mastodon.gamedev.place/@zeh/111009505497061485) for context), and though this would be a good excuse for an exercise in using Rust to load remote data.

## Running

Running takes no arguments:

```
cargo run
```

It then starts monitoring the page, reloading every 5 minutes:

```
Results are the same.
Attempted 1 times; waiting 300 seconds until next request.
Results are the same.
Attempted 2 times; waiting 300 seconds until next request.
Results are the same.
Attempted 3 times; waiting 300 seconds until next request.
```

If new devices are detected (or any devices are removed), they show up as a diff. This hapens the first time the application is ran, or when the list changes afterwards.

```
New results detected!
 [2021] Surface Go 3 - $0 - https://www.microsoft.com/en-us/d/surface-go-3/904h27d0cbwn
 [2021] Surface Laptop 4 13.5” - $0 - https://www.microsoft.com/en-us/d/surface-laptop-4/946627fb12t1
+[2021] Surface Laptop 4 15” - $0 - https://www.microsoft.com/en-us/d/surface-laptop-4/946627fb12t1
 [2021] Surface Laptop Studio - $0 - https://www.microsoft.com/en-us/d/surface-laptop-studio/8srdf62swkpf
 [2021] Surface Pro 8 - $0 - https://www.microsoft.com/en-us/d/surface-pro-8/8qwcrtq8v8xg
+[2021] Surface Pro X - $0 - https://www.microsoft.com/en-us/d/surface-pro-x/8xtmb6c575md
 [2022] Surface Laptop 5 13.5” - $0 - https://www.microsoft.com/en-us/d/surface-laptop-5/8xn49v61s1bn
 [2022] Surface Laptop 5 15” - $0 - https://www.microsoft.com/en-us/d/surface-laptop-5/8xn49v61s1bn
 [2022] Surface Laptop Go 2 - $0 - https://www.microsoft.com/en-us/d/surface-laptop-go-2/8pglpv76mjhn
 [2022] Surface Pro 9 - $0 - https://www.microsoft.com/en-us/d/surface-pro-9/93vkd8np4fvk
 [2022] Surface Pro 9 with 5G - $0 - https://www.microsoft.com/en-us/d/surface-pro-9/93vkd8np4fvk
 [2022] Surface Studio 2+ - $0 - https://www.microsoft.com/en-us/d/surface-studio-2-plus/8vlfqc3597k4
(O)verwrite, (I)gnore, (Q)uit?
```

Overwriting rewrites the saved results. Ignoring continues (it will show up again next time it's loaded). Quitting terminates the process.
