# mat-frame-art

Rust tool to generate framed and branded images for display on Samsung Frame TV's, "art" TV's, or as a wallpaper

<figure>
   <img width="768" alt="" src="https://github.com/user-attachments/assets/c35496da-352b-4443-861c-bc7e37541254" />
</figure>

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

![cooper_lobzov_3500_300_dpi_compressed](https://github.com/user-attachments/assets/e1d173b8-8501-44ae-ad3f-e3db43ae3b6a)
![cs2_print_300dpi_01_compressed](https://github.com/user-attachments/assets/a2b4f4ec-9caf-4847-a231-9aa80154c7b1)
![gta_lobzov_full_300dpi_boosty_compressed](https://github.com/user-attachments/assets/6e2d8e4f-edeb-45cf-aa66-b084eb61e74d)
![h3_print_300dpi_01_compressed](https://github.com/user-attachments/assets/050ecf70-4d18-471a-b7c4-3004257d122b)
![hl2_ep2_lobzov_print_01_compressed](https://github.com/user-attachments/assets/94441ae1-fa8a-4b0b-8ab9-c501ba0292ec)

## License

The project is vendored under the MIT license
