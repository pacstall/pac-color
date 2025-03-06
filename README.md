# pac-color

Pac-color is a tiny web-app that dynamically generates solid-color images as well as getting JSON representation of color conversions to other formats.

We at pacstall use this tool to generate swatches for release notes (<https://github.com/pacstall/pacstall/releases>).

## Using pac-color

### Swatch

The most common usage is to get a color swatch, which can be done like this:

```bash
wget https://colors.pacstall.dev/E09540/preview
```

Which will download the swatch with the color #E09540.

You can also optionally change the size with `size` query parameter in the form: `${height}x${weight}`, such as `16x16`.

Finally, you can change the default generated filetype with the `type` query parameter. Currently, we accept the following:

* `png`
* `jpg`
* `gif`
* `ico`
* `webp`

### Conversion data

If you remove `/preview` from the slug, you will get a neatly formatted JSON response containing various color conversions.
