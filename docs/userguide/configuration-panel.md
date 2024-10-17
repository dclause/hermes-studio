---
outline: deep
---

# Configuration Panel

<img class="icon" style="float:right" alt="alt" src="/icons/cog-transfer.svg" width="100"/>

This panel lets you configure the _hardware_ that composes your robot.
1. First, we need to configure at least one board that is connected to the _backend_ machine.
2. Then, we need to configure the devices that compose your robot - connected to the configured boards
3. Finally, we can define logical groups to make easier use of our hardware.

## Prerequisites

In this page, we will use terms like _backend_, _hardware_, _boards_, _devices_ and associated vocabularies.
If you don't know what this means in the context of this application, read more about the concepts behind Hermes-Studio.

- You need at least one board connected to your _backend_ via one of the supported methods (currently Serial port). Those boards are referred to as "_the robot_".
- All connected boards MUST be configured with [StandardFirmataPlus.ino](https://github.com/firmata/arduino/blob/main/examples/StandardFirmataPlus/StandardFirmataPlus.ino) Arduino sketch installed.<br/>
_This code is available by default in Arduino IDE under the Firmata samples sketch menu._<br/>
_Uploading the sketch to the board needs to be done once only._

::: info EXAMPLE
For the purpose of example, we will consider the following (very simple) setup:
![basic-setup.png](/userguide/basic-setup.png)
- A computer (Ubuntu 22.04) running the Hermes-Studio application.
- An Arduino MEGA is connected to it by a serial USB cable on port `/dev/ttyUSB0`
- The board has an embedded red led on pin 13
- An orange LED is connected to pin 8
- A basic servo is connected to pin 22
  :::

## Configure board(s)

1. Click on the <img style="display:inline-block;vertical-align: middle;width:2em;" src="/icons/cog-transfer.svg"> icon to access the _Configuration Panel_.
![Icon access to Configuration Panel](/userguide/configuration_panel_icon.png)

2. Click on the _New Board_ button.
!['New Board' button](/userguide/create_board_button.png)

3. Configure the board according to your need.
![Create a new board form](/userguide/create_board_form.png){width=400 style="display: block; margin: 0 auto"}

4. The configured board must now be connected to the backend to be used.
To do so, toggle the _Status_ button.
![Status is a button to individually connect the board](/userguide/board_connection_button.png)

::: danger
Don't forget that your board MUST have [StandardFirmataPlus](https://github.com/firmata/arduino/blob/main/examples/StandardFirmataPlus/StandardFirmataPlus.ino) installed
:::

::: warning
The connection may need to be retried. We are working on this.
:::

::: tip
Notice how the color of the whole interface changes: 
- cyan blue: when no boards are yet connected
- orange: when some of the configured boards are connected
- deep blue: when all configured boards are properly connected
:::

## Configure device(s)

1. Click on the board to which the device is connected to.
![Click on a board to configure its Device(s)](/userguide/configure_board.png)

2. Click on the _New device_ button.
!['New Device' button](/userguide/create_device_button.png)

3. Configure the device according to your needs. See the list of supported devices for individual documentation.
![Create new Device form](/userguide/create_device_form.png){width=450 style="display: block; margin: 0 auto"}

::: info EXAMPLE
Try yourself to configure the devices according to our example:
- _Embedded Red Led_ on pin 13
- _Orange Led_ on pin 8
- _Basic Servo_ on pin 22
![Example of three different devices configured](/userguide/example_configured_devices.png)
:::

::: tip
Notice how all devices are marked "offline" in the screenshots ?<br/>
Try to connect the board and see what happens now: you can remote control the devices.
:::

## Configure Group(s)

Let's get back to the board list via the _Configuration Panel_ icon: <img style="display:inline-block;vertical-align: middle;width:2em;" src="/icons/cog-transfer.svg">

You have possibility to:
- create arbitrary number logical groups
- assign devices to groups regardless of which board it belongs
- nest the groups and devices arbitrarily.

1. Open the _Groups_ configuration tab from the _Configuration Panel_.
![Groups configuration panel](/userguide/configuration_groups.png)

2. Click on the _New Group_ button.
!['New Group' button](/userguide/create_group_button.png)

3. Add a group name.
![Create new Group form](/userguide/create_group_form.png)

4. Drag groups or devices into other groups. Don't hesitate to nest as needed.
![drag_device_group.png](/userguide/drag_device_group.png)

::: tip
Groups are specially useful when your robot gets complex.<br/>
For example, the [InMoov robot](https://inmoov.fr/) may have 2 Arduino MEGAs, one Arduino Nano and countless servos and sensors.
It is useful to build a structural representation of your robot.
![complex_nested_groups.png](/userguide/complex_nested_groups.png)
:::

## What's next?

Your robot configuration is now complete. It is time to play with it. Discover more about [how to control your robot](/userguide/configuration-panel).
