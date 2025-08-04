# Arm CoreLink CCI-5x0 Cache Coherent Interconnect driver

Driver implementation for the [CCI-500](https://developer.arm.com/documentation/100023/0100/) and
[CCI-550](https://developer.arm.com/documentation/100282/0100) peripherals.

The Arm CCI-500 and CCI-550 interconnects enable coherent data sharing between multiple processor
clusters and I/O coherency with peripherals, ensuring efficient memory access and system
performance. The driver facilitates the components of the SoC to enter or exit coherency.

## Implemented features

* Enter and exit coherency

## License

The project is MIT and Apache-2.0 dual licensed, see `LICENSE-APACHE` and `LICENSE-MIT`.

## Maintainers

arm-cci is a trustedfirmware.org maintained project. All contributions are ultimately merged by the
maintainers listed below.

* Bálint Dobszay <balint.dobszay@arm.com>
  [balint-dobszay-arm](https://github.com/balint-dobszay-arm)
* Imre Kis <imre.kis@arm.com>
  [imre-kis-arm](https://github.com/imre-kis-arm)
* Sandrine Afsa <sandrine.afsa@arm.com>
  [sandrine-bailleux-arm](https://github.com/sandrine-bailleux-arm)

## Contributing

Please follow the directions of the [Trusted Firmware Processes](https://trusted-firmware-docs.readthedocs.io/en/latest/generic_processes/index.html)

Contributions are handled through [review.trustedfirmware.org](https://review.trustedfirmware.org/q/project:rust-spmc/arm-cci).

## Arm trademark notice

Arm is a registered trademark of Arm Limited (or its subsidiaries or affiliates).

This project uses some of the Arm product, service or technology trademarks, as listed in the
[Trademark List][1], in accordance with the [Arm Trademark Use Guidelines][2].

Subsequent uses of these trademarks throughout this repository do not need to be prefixed with the
Arm word trademark.

[1]: https://www.arm.com/company/policies/trademarks/arm-trademark-list
[2]: https://www.arm.com/company/policies/trademarks/guidelines-trademarks

--------------

*Copyright The arm-cci Contributors.*
