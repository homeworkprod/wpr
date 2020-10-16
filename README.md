# Wallpaper Randomizer

Sets a wallpaper (using the [*feh* image
viewer](https://feh.finalrewind.org/)) randomly chosen from a directory.


## Background

Since I like to change my wallpaper on the tap of a button and flip
through my collection until I find one that I feel comfortable with for
the moment (depending on time of day, lighting conditions, atmosphere,
mood, etc.), I ported my Python script to Rust to avoid booting up the
Python interpreter for each call.


## Usage

Just point it to a directory that contains images:

```sh
$ wpr ~/Wallpapers
```

- Considered image file extensions (case-insensitively) are "gif",
  "jpeg", "jpg", and "png".
- Sub-directories are not considered. This is so images can be moved
  into a sub-directory to exclude them from the random selection for the
  time being (in case you've seen them a bit too often recently) but
  still keep them nearby of the other wallpapers.


## License

Wallpaper Randomizer is licensed under the MIT license.
