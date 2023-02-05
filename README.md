# :seedling: GRDN

GRDN is a system for managing automatic data collection and watering of a small garden.

[Project board](https://www.notion.so/ardentforge/GRDN-bedc4a8400aa46009fe0b83b77abeae5)

It is designed in a client/server model where the client is co-located with the garden and the
server is somewhere safer inside, connected to the same network. I designed the system to work
with a Raspberry Pi Zero 2W running the client and a Raspberry Pi 4 running the server.

The client is responsible for collecting raw data from the sensors attached to it, turning on
and off the water, and sending data up to the server. The client is powered by a solar panel.

The server is responsible for receiving data from the client, creating a watering schedule, and
telling the client when it should turn the water on and off.
