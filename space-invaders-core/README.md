This crates contains the code for space invaders simulation which can be shared between the 
different implementations (with `gtk-rs` and `piston`) and embeds the rom in the crate.

Don't forget to add the space invaders rom as `resources/invaders.rom` in this crate. Otherwise, 
`resources/dummy.rom` is embedded.