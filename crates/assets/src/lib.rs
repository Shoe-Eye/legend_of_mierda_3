pub mod loading;
pub mod sprites;

pub use loading::{AudioAssets, FontAssets, StaticSpriteAssets};

pub type TextureAtlasHandle = bevy::asset::Handle<bevy::image::TextureAtlasLayout>;

pub fn load_texture_atlas(
    path: String,
    asset_server: &bevy::asset::AssetServer,
    columns: u32,
    rows: u32,
    _padding: Option<bevy::math::Vec2>,
    sprite_size: bevy::math::Vec2,
    texture_atlasses: &mut bevy::asset::Assets<bevy::image::TextureAtlasLayout>,
) -> TextureAtlasHandle {
    use bevy::image::TextureAtlasLayout;
    use bevy::math::UVec2;
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(sprite_size.x as u32),
        columns,
        rows,
        Some(UVec2::ZERO),
        Some(UVec2::ZERO),
    );
    let handle = texture_atlasses.add(layout);
    let _img: bevy::asset::Handle<bevy::image::Image> = asset_server.load(path);
    handle
}

pub fn load_texture_atlas_layout(
    path: String,
    asset_server: &bevy::asset::AssetServer,
    sheet_columns: usize,
    sheet_rows: usize,
    sprite_size: bevy::math::Vec2,
) -> bevy::image::TextureAtlasLayout {
    use bevy::image::TextureAtlasLayout;
    use bevy::math::UVec2;
    let _texture_handle: bevy::asset::Handle<bevy::image::Image> = asset_server.load(path);
    TextureAtlasLayout::from_grid(
        UVec2::splat(sprite_size.x as u32),
        sheet_columns as u32,
        sheet_rows as u32,
        Some(UVec2::ZERO),
        Some(UVec2::ZERO),
    )
}
