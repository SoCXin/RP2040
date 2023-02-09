# [RP2040](https://doc.soc.xin/RP2040)

[![Build Status](https://github.com/SoCXin/RP2040/workflows/src/badge.svg)](https://github.com/SoCXin/RP2040/actions/workflows/src.yml)

* [Raspberry](https://www.raspberrypi.com/)：[Cortex-M0+](https://github.com/SoCXin/Cortex)
* [L2R2](https://github.com/SoCXin/Level)：2 x 133 MHz , [￥5.3](https://item.szlcsc.com/2392.html)


## [简介](https://github.com/SoCXin/RP2040/wiki)

[RP2040](https://www.raspberrypi.com/products/raspberry-pi-pico/) is a low-cost, high-performance microcontroller device with flexible digital interfaces. Key features: Dual Cortex M0+ processor cores, up to 133 MHz 264 kB of embedded SRAM in 6 banks 30 multifunction GPIO 6 dedicated IO for SPI Flash (supporting XIP) Dedicated hardware for commonly used peripherals Programmable IO for extended peripheral support 4 channel ADC with internal temperature sensor, 0.5 MSa/s, 12-bit conversion USB 1.1 Host/Device

[![sites](docs/RP2040.png)](https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html)

### 关键特性

* 133MHz Cortex-M0+ Dual Core
* 264KB RAM + 128KB/256KB Flash + 16MB QSPI Flash
* 2 × UART, 2 × SPI , 2 × I2C
* 16 × PWM
* USB1.1 OTG with UF2
* QFN56 (7*7mm), 30 GPIO, 其中的4个可以用于模拟输入
* 8 × PIO可以支持多种IO标准
    * DPI/VGA (利用电阻网络)
    * 8080/6080并行接口
    * SPI/DSPI/QSPI
    * I2C
    * I2S
    * SDIO
    * UART


## [资源收录](https://github.com/SoCXin)

* [参考资源](src/)
* [参考文档](docs/)
* [参考工程](project/)

## [选型建议](https://github.com/SoCXin)

2021年1月发布的[RP2040](https://item.szlcsc.com/2392.html)包含两个子系列，分别为支持无晶振USB FS device的RP2040xxU系列，不支持USB的 RP2040xxN(1.8V-3.6V)/RP2040x4NR(1.8V-5.5V)。

性价比非常高，开源资源丰富，在Arduino和Micropython生态资源上拥有无与伦比的优势。

采用TSMC 40nm制程工艺，极端情况下可超频到1GHz，芯片带有ADC精度高于ESP32，可用于替代ATmega328p，作为通用性很强的MCU，可广泛应用于各种外设驱动和处理场景。

软件开发可使用[platform-rp2040](https://github.com/OS-Q/platform-rp2040)

RP2040中有2个相同的PIO块,每个PIO块都有专用的连接到总线结构,GPIO和中断控制器,每个块有四个状态机,可以独立执行顺序程序来操作GPIO和传输数据。

与通用处理器不同的是,PIO状态机对IO的专业化程度很高(highly specialised),它注重确定性,精确的时序,并与固定功能硬件紧密结合。PIO状态机可以动态地配置和重新配置,以实现许多不同的接口,自由度很高。



## [www.SoC.xin](http://www.SoC.Xin)
