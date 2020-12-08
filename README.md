# lecar
Reinforcement Learning Cache Replacement (LECAR) implemented in Rust
The paper, written by Vietri et. al., can be found [here](https://www.usenix.org/system/files/conference/hotstorage18/hotstorage18-paper-vietri.pdf) and the slides they wrote about the paper can be found [here](https://www.usenix.org/sites/default/files/conference/protected-files/hotstorage18_slides_martinez.pdf).

This cache utilizes a machine learner that informs the controller which policy to use at any given time there is a miss. Thus there are three caches that make up this one cache: the main cache, the LFU cache, and the LRU cache.

## This is still in development
While this is still in development, please feel free to play around with it.
