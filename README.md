# movie-rs

The eventual end goal for this project, is something akin to [MoviePy for Python](https://github.com/zulko/moviepy/), but for Rust.

## Fast

The aim is for it to be faster to do its job. I haven't looked into MoviePy under the hood too much, but what I'm aiming to do to improve speed is outlined below.
As obvious as the points may seem, it is essential they are implemented. From what I can tell, they aren't in MoviePy, so I think this will really just be the icing on the cake.

#### Avoidance of unnecessary work

Use FFMPEG where able, e.g for clips with no effects/modifications to them that would require us doing any real work, such as trimming a clip, or changing
output format. From what I can tell, MoviePy treats all clips the same, even when a more streamlined solution would do the trick. Their approach works great
for clips where custom effects are needed, but for simple operations that FFMPEG can do, why do the work?

#### GPU-Accelerated

MoviePy does not make use of a GPU, apart from when calling FFMPEG, in which the user can pass in some GPU settings. With the way that we handle video, we can very easily
distribute this across hundreds of processes, as each frame is isolated from each other, and any frame can be rendered at any point, with no issues present (in theory).

#### Written in Rust

...



## Other Features

Eventually it would be nice to have Python bindings, so that people using MoviePy can just chop and change packages quite easily, rather than uproot their whole project, or create a separate service in Rust.
