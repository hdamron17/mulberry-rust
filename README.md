[![crates.io](https://meritbadge.herokuapp.com/mulberry)](https://crates.io/crates/mulberry)
[![BSD-3 license](https://img.shields.io/badge/license-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

# mulberry
Coordinate transformation tree with a focus on efficiency

## Overview
The goal of this project is to provide the functionality of [ROS's TF2](http://wiki.ros.org/tf2)
but without the need for the ROS framework or intraprocess communication.
By removing that restriction, we aim to allow more efficient transform querying by allowing
truly static transforms, cached intermediate connections, and unrestricted tree merging.

## Contributing
This project is still in the very early stages, but any contributions would be appreciated.
In addition to code/testing/documentation contributions, please let us know if there are
any features which would make this tool more helpful. This will help direct the future
of the project.
