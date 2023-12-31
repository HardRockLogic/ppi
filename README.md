# ppi

PPI is a lightweight easy to use command line calculator for (who'd guessed) calculating pixel per inch value of a screen from a given diagonal value in inches and resolution either custom - passed by user, or chosen from available options. It is written in Rust and takes advantage of its intuitive arguments parsing, so it wont be an issue to use PPI for those who new to the command line interfaces.

If you like me and for some reason need a convenient instrument to calculate ppi and other screen related values - feel free to use it.

## Installation

### Building from source

To build this app you'll need a cargo installed. The easiest way to do so is to install latest stable release of [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Clone the project

```bash
git clone https://github.com/HardRockLogic/ppi.git
```

`cd` into directory you've cloned project to and run the installation script, by defalult it will copy compiled binary to `usr/bin/` as ppi, but you can
pass alternative dirs (note that they should be present in $PATH to be executed with just name) as an arguments.

```bash
# this will launch default script
chmod +x install_to.sh && ./install_to.sh
# enter your sudo password here
```

or just run it with `bash` or `sh` without separatly giving execution permission

```bash
bash install_to.sh
# enter your sudo password here
```

alternatively pass another destination path

```bash
sh install_to.sh /usr/local/bin/
# enter your sudo password here
```

**note** that script will not resolve tilda (~) so batter to write absolute path or relative to current working directory

From here you can delete file with source code and use standalone binary application.

### Pulling image form dockerhub

If you already have docker installed you can pull ppi from docker hub, this is as easy as:

```bash
docker pull hardrocklogic/ppi:musl
```

## Usage

You can run ppi --help to retrieve help message and view available options.

```bash
 ppi --help
Usage: ppi <diagonal> [-r <resolution>] [-h] [-f] [-q] [-u]

Screen data

Positional Arguments:
  diagonal          screen diagonal value

Options:
  -r, --resolution  custom resolution in format 1366x768 where x is any
                    alphabetic character
  -h, --hd          standart 16:9 hd resollution (1280x720)
  -f, --fhd         standart 16:9 full hd resollution (1920x1080)
  -q, --qhd         standart 16:9 quad hd resollution (2560x1440)
  -u, --uhd         standart 16:9 ultra hd resollution (3840x2160)
  --help            display usage information

```

```bash
$ ppi 14.2 -r 3024x1964
+--------------+------------------+
| Property     | Value            |
+--------------+------------------+
| PPI          |           253.93 |
+--------------+------------------+
| PPI²         |        64,480.62 |
+--------------+------------------+
| Total Px     |        5,939,136 |
+--------------+------------------+
| Aspect ratio | 756/491 (1.54:1) |
+--------------+------------------+
| Diagonal Px  |            3,606 |
+--------------+------------------+
| Dot pitch    |              0.1 |
+--------------+------------------+

$ ppi 27 --uhd
+--------------+---------------+
| Property     | Value         |
+--------------+---------------+
| PPI          |        163.18 |
+--------------+---------------+
| PPI²         |     26,627.16 |
+--------------+---------------+
| Total Px     |     8,294,400 |
+--------------+---------------+
| Aspect ratio | 16/9 (1.78:1) |
+--------------+---------------+
| Diagonal Px  |         4,406 |
+--------------+---------------+
| Dot pitch    |        0.1557 |
+--------------+---------------+

# using image from docker hub

[node1] (local) root@192.168.0.8 ~
$ docker images
REPOSITORY          TAG       IMAGE ID       CREATED        SIZE
hardrocklogic/ppi   musl      2c09c57f19eb   23 hours ago   74.3MB

[node1] (local) root@192.168.0.8 ~
$ docker run 2c09 32 -r 1366x768
+--------------+------------------+
| Property     | Value            |
+--------------+------------------+
| PPI          |            48.97 |
+--------------+------------------+
| PPI²         |         2,398.22 |
+--------------+------------------+
| Total Px     |        1,049,088 |
+--------------+------------------+
| Aspect ratio | 683/384 (1.78:1) |
+--------------+------------------+
| Diagonal Px  |            1,567 |
+--------------+------------------+
| Dot pitch    |           0.5187 |
+--------------+------------------+

```
For linux users, if you have xrandr installed you can use auto subcommand to resolve your ppi:
```bash
$ ppi auto -v
Session type: x11
Found resolution: 1920 x 1080
Found dimensions: 344mm x 194mm
Calculated diagonal: 15.5"
+--------------+---------------+
| Property     | Value         |
+--------------+---------------+
| PPI          |        141.68 |
+--------------+---------------+
| PPI²         |     20,073.04 |
+--------------+---------------+
| Total Px     |     2,073,600 |
+--------------+---------------+
| Aspect ratio | 16/9 (1.78:1) |
+--------------+---------------+
| Diagonal Px  |         2,203 |
+--------------+---------------+
| Dot pitch    |        0.1793 |
+--------------+---------------+

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.
