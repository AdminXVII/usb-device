usb-device
==========

Experimental device-side USB stack for embedded devices in Rust.

This crate is still under development and should not be considered production ready or even USB
compliant.

The UsbDevice object represents a composite USB device and is the most important object for
application implementors. The UsbDevice combines a number of UsbClasses (either custom ones, or
pre-existing ones provided by other crates) and a UsbBus device drives to implement the USB device.

The UsbClass trait can be used to implemented USB classes such as a HID device or a serial port. An
implementation may also use a custom class if the required functionality isn't covered by a standard
class.

The UsbBus trait is intended to be implemented by device-specific crates to provide a driver for
each device's USB peripheral.

Hardware driver crates
----------------------

* [stm32f103xx-usb](https://github.com/mvirkkunen/stm32f103xx-usb) - device-driver implementation
  for STM32F103 microcontrollers. Also contains runnable examples.
* [stm32l43x-usbd](https://github.com/nickray/stm32l43x-usbd) - device-driver implementation
  for STM32L43x microcontrollers. Also contains runnable examples.
* [stm32-usbd](https://github.com/Disasm/stm32-usbd) - device-driver implementation
  for multiple STM32 microcontroller families. Also contains runnable examples.

Class crates
------------

* [usbd-serial](https://github.com/mvirkkunen/usbd-serial) - CDC-ACM serial port class

TODO
----

Features planned but not implemented yet:

- Interface alternate settings
- Multilingual string descriptors
- Isochronous endpoints

Features not planning to support at the moment:

- More than one configuration descriptor (uncommon in practice)
