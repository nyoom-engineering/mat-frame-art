# mat-frame-art

Rust tool to generate framed and branded images for display on Samsung Frame TV's, "art" TV's, or as a wallpaper

<figure>
   <!--- <img alt="" src="https://github.com/user-attachments/assets/0cb2aa5e-81ed-4b60-bfac-4bdba8249592" /> --->
</figure>

<br>

<figure>
  <!--- <img alt="" src="https://github.com/user-attachments/assets/5b31e536-810e-44c1-814b-b3a99ae62bbe" /> --->
</figure>

<br>

## Usage

`Cargo/Rust` are required

A makefile is provided for your convenience, alongside a sample logo under `assets` and a few images by Nikolay Lobzov under `samples/`

The defult make task will render all images under sample with a logo at 4k

```sh
git clone --depth 1 https://github.com/nyoom-engineering/mat-frame-art.git && cd mat-frame-art
make all
```

Alternatively, you can specify the parameters yourself

```sh
make frame INPUT=path/to/input.jpg OUTPUT=path/to/output.png RESOLUTION=2560x1600 [WITH_LOGO=1]
```

Ratio of the white mat to the image can be specified with the `--mat-rato <float>` argument. By default, its a resonable `0.8`

You can also use `make build` to build the crate, and `make clean` to clean up the working directory. 

## Samples



## License

The project is vendored under the MIT license