# ATSAMS70, ATSAME70, ATSAMV70, ATSAMV71 support for Rust

This repository holds various things that support/enable working with Microchip (formerly Atmel) SAMS70 family based
devices in Rust. Known boards using some of these chips:

* [ATSAME70-XPLD](https://www.microchip.com/DevelopmentTools/ProductDetails/PartNO/ATSAME70-XPLD)
* [SAM E70 Xplained Ultra Evaluation Kit](https://www.microchip.com/DevelopmentTools/ProductDetails/PartNO/DM320113)
* [SAM V71 Xplained Ultra Evaluation Kit](https://www.microchip.com/DevelopmentTools/ProductDetails/PartNO/ATSAMV71-XULT)
* [Tsunami Super WAV Trigger](https://www.sparkfun.com/products/13810)

## Status

This repository holds work in very early stage of development and doesn't provide any working support at the board level. Major pressure is put on support of [ATSAMS70Q21](https://www.microchip.com/wwwproducts/en/ATSAMS70Q21).

## Branches

<dl>
  <dt>development</dt>
  <dd>Main focus of progress</dd>
  <dt>nightly</dt>
  <dd>This branch holds code that requires nightly compiler to build.</dd>
</dl>

## Contributing

To clone the repository use `git clone --recursive git@github.com:atsams-rs/atsamx7x-rust.git` as it uses submodules.

Please pull request your contributions for review. By doing so you agree your work falls under the [License](#license) stated below.

People working on serious projects using any of MCUs from families covered by this repository may apply for membership in [atsams-rs](https://github.com/atsams-rs) organization. Please contact me via [LinkedIn](https://www.linkedin.com/in/michalfita/) or [discussions](https://github.com/atsams-rs/atsamx7x-rust/discussions).

## License

[BSD Zero Clause License](https://choosealicense.com/licenses/0bsd/)

## Disclaimer

This work is not endorsed, supported or even approved by any means by [Microchip Technology Inc.](https://www.microchip.com/about-us/company-information/about); any copyrighted names are used purely for informative purpose.
