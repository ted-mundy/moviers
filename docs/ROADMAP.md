# movie-rs roadmap

This will serve as both an introduction and a roadmap of the project. If you're new, read this!

## Introduction

If you haven't read the [README](../README.md) already, check that out. That outlines a lot of what this should do.

**movie-rs** should provide an easy-to-use interface to create videos. How we will do this is still
up in the air, but it will be something similar to [MoviePy for Python](https://github.com/zulko/moviepy/), and I'm sure
many other libraries that do similar.

### Clips

We will have multiple types of "*clips*". A *clip* is our abstraction for a video, audio, a color, text, image - anything we can
put on the video will have a *clip*.

Clips will have varying properties - and this is still subject to change -
which define how they will appear in the final output. Here are some examples:

#### Example: Text Clip
- `content` - A text string containing whatever the text clip should say.
- `color` - A string, or int (hex value) which correlates to the color of the text.
- `stroke_width` - A number which will determine how thick the outline of the text is - if given.
- `stroke_color` - Same as color, but for the outline instead of the actual text.
- `x` - A number (and potentially a string? "center", "top" etc) which details the X position of the text on the [canvas](#canvas)
- `y` - Same as `x`, but for the Y axis.

#### Example: Video Clip

`width` and `height` should behave a little differently here. If one is given, we should auto-scale to keep the original
video's aspect ratio to avoid awkward stretching. If both are set, though, then we should just stretch it out.

- `filepath` - A text string containing the filepath of the video file to add.
- `x` - A number (and potentially a string? "center", "top" etc) which details the X position of the text on the [canvas](#canvas)
- `y` - Same as `x`, but for the Y axis.
- `trim_start` - A number which marks the point at which we should start playing the video file.
- `trim_end` - Similarly to `trim_start`, this number marks the point at which we should stop playing the file.
- `width` - An optional integer that sets the width of the video.
- `height` - An optional integer that will set the height of the video.

Again, this is all subject to change. Take for example `x` and `y` values which should change as the video plays. The clips should have
functions which get these values on a per-frame basis.

### Rendering

What good is a video library with no videos? This will stay brief - I'm very unsure of what the renderer will look like or do at this point in time,
but there should be a few definitions which we nail down now.

#### Canvas

Imagine the [clips](#clips) as post-it-notes, stickers, or whatever you want to put onto the canvas. We
add and remove clips from the base canvas, and out comes a video. The canvas is what all clips go onto, and whatever is on that canvas gets
outputted on the video.

When we render a video, we need to know the resolution of the video we're outputting. Unless explicitly set, the canvas should default to the
first clip's resolution in the array of clips to render. If we don't have one, we should error out.
