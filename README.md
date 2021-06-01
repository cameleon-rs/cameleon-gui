# cameleon-gui

A standalone GUI application that allows you to take pictures and control GenAPI with [GenICam](https://www.emva.org/standards-technology/genicam/) compatible cameras.

![snapshot](snap.png)

## Prerequisites

* [libusb](https://libusb.info/)
* (optional) [OpenCV](https://opencv.org/)

## Installation

Install via cargo.

```shell
cargo install cameleon-gui
```

Install `camleon-gui` with color conversion using OpenCV  such that bayer conversion.

```shell
cargo install cameleon-gui --features cv
```

## FAQ

### USB3 Vision

#### Why isn't a camera found even though it is connected to the host?

It's probably due to permission issue for USB devices. You could add permissions by editing `udev` rules, a configuration example is found [here](misc/u3v.rules).

#### Why is frame rate so low?

Frame rate can be affected by several reasons.

1. Parameter settings of the camera

`AcquisitionFrameRate` and `ExposureTime` directly affect frame rate. So you need to setup the parameters first to improve frame rate.
Also, if `DeviceLinkThroughputLimitMode` is set to `On`, you would need to increase the value of `DeviceLinkThroughputLimit`.

2. Many devices are streaming simultaneously on the same USB host controller

In this case, it's recommended to allocate the equal throughput limit to the connected cameras,
making sure that the total throughput does not exceed the maximum bandwidth of the host controller.

3. `usbfs_memory_mb` is set to low value

If you use Linux, you may need to increase `usbfs_memory_mb` limit.
By default, USB-FS on Linux systems only allows 16 MB of buffer memory for all USB devices. This is quite low for high-resolution image streaming.
We recommend you to set the value to 1000MB. You could set the value as following:

```shell
echo 1000 > /sys/module/usbcore/parameters/usbfs_memory_mb
```

## License

This project is licenced under [MPL 2.0](LICENSE).
