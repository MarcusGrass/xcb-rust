# 
A worse version of [x11rb](https://github.com/psychon/x11rb) trying to generate 
more optimizable code.

## Licensing
The project is licensed under [Mpl v2](LICENSE)

The project uses a lot of copied and modified code from [x11rb](https://github.com/psychon/x11rb), 
those parts are concentrated at helpers in [xcb-rust-connection/helpers](xcb-rust-connection/src/helpers), 
and [xcb-rust-protocol/helpers](xcb-rust-protocol/src/helpers). While that project is an obvious influence on the 
entirety of those two crates.
The license for x11rb can be found [at that repo here](https://github.com/psychon/x11rb/blob/33a438c93c3cbf7dc2b25bbc2f4b726b800c3117/LICENSE-MIT) 
and included in [xcb-rust-connection here](xcb-rust-connection/src/helpers/x11rb-MIT-LICENSE) as well as in [xcb-rust-protocol here](xcb-rust-protocol/src/helpers/x11rb-MIT-LICENSE).