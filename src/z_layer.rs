// TODO: is is possible to bind these into a single parent object?
pub const Z_LAYER_GAME_AREA: f32 = 1.;
pub const Z_LAYER_DEBUG_HIT_CIRCLES: f32 = Z_LAYER_GAME_AREA + 1.;
pub const Z_LAYER_SPRITES_COINS: f32 = Z_LAYER_DEBUG_HIT_CIRCLES + 1.;
pub const Z_LAYER_SPRITES_PLAYER: f32 = Z_LAYER_SPRITES_COINS + 1.;
pub const Z_LAYER_DEBUG_SPRITE_BOUNDARIES: f32 = Z_LAYER_SPRITES_PLAYER + 1.;
