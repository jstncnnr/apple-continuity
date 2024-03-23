# Apple Continuity Messages
Apple devices will periodicly send messages out over BLE. These are shared
under the manufacturer specific advertisement data using Apple's company ID of 0x004c.
All messages start with a packet opcode and packet length with a variable body. More
information can be found in [Discontinued Privacy: Personal Data Leaks in Apple Bluetooth-Low-Energy Continuity Protocols](https://petsymposium.org/2020/files/papers/issue1/popets-2020-0003.pdf) by Guillaume Celosia and Mathieu Cunche.

This library will help decode those messages and where possible, interpret them.

For example:  
Proximity Pair includes status about the device. Charging state, model, color, battery levels, etc.
The battery state and charge state is sent as 2 bytes `battery1` and `battery2` but these contain
left battery, right battery, case battery, and charging status for all three. 

A `ProximityPairMessage` can be converted into a `ProximityDevice` which provides user friendly access
to these attributes. 

## Opcodes
| Message           | Opcode     | Expected Length |
|-------------------|------------|-----------------|
| Proximity Pairing | 0x07       | Variable        |
| AirPrint          | 0x03       | 0x16            |
| AirPlay Target    | 0x09       | 0x06            |
