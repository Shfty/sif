use std::time::Instant;

use async_std::{
    fs::DirBuilder,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::{StreamExt, TryStreamExt};
use ogre::{
    parser_async::{parse_textures, wad_source::SourceStreamSlice},
    repr::{texture::Texture, Image, Palette},
};

const SINGLE_THREADED: bool = false;

#[async_std::main]
pub async fn main() -> Result<(), async_std::io::Error> {
    let palette = include_bytes!("../../ogre/test_data/palette/quake.lmp");

    //let wad = include_bytes!("../../ogre/test_data/wad2/medieval.wad");
    //let wad = include_bytes!("../../ogre/test_data/wad2/jr_med.wad");
    //let wad = include_bytes!("../../ogre/test_data/wad3/cs_dust.wad");
    //let wad = include_bytes!("../../ogre/test_data/wad3/gfx.wad");
    let wad = include_bytes!("../../ogre/test_data/wad3/halflife.wad");

    let texture_dumps_path = Path::new(".").join("texture_dumps");
    //let path = texture_dumps_path.join("medieval");
    //let path = texture_dumps_path.join("jr_med");
    //let path = texture_dumps_path.join("cs_dust");
    //let path = texture_dumps_path.join("gfx");
    let path = texture_dumps_path.join("halflife");

    if SINGLE_THREADED {
        let wad = include_bytes!("../../ogre/test_data/wad3/halflife.wad");
        println!("Parsing Half Life WAD...");
        let start = Instant::now();
        let (_, wad) = ogre::parser::parse_wad(wad).unwrap();
        println!("Done in {:?}", Instant::now() - start);
        assert!(wad.len() > 0);
        return Ok(());
    }

    println!("Parsing Quake Palette...");

    let palette = ogre::parser::parse_palette(palette)
        .expect("Invalid palette")
        .1;
    let palette = Arc::new(palette);

    println!("Parsing and dumping textures to {}", path.to_str().unwrap());

    let start = Instant::now();
    let source = SourceStreamSlice::from(&wad[..]);
    let textures = parse_textures(source).await?;
    println!("Parse done in {:?}", Instant::now() - start);

    let start = Instant::now();
    let stream = textures.and_then(|(name, texture)| {
        async_std::task::spawn(dump_texture(
            name,
            texture,
            Some(palette.clone()),
            path.clone(),
        ))
    });

    futures::pin_mut!(stream);
    while stream.next().await.is_some() {}
    println!("Dump done in {:?}", Instant::now() - start);


    Ok(())
}

async fn dump_texture(
    name: String,
    texture: Texture,
    palette: Option<Arc<Palette>>,
    path: PathBuf,
) -> Result<(), async_std::io::Error> {
    DirBuilder::new().recursive(true).create(&path).await?;
    let name = name.chars().filter(|char| *char != '*').collect::<String>() + ".png";
    let path = path.join(name);
    write_texture_png(texture, palette.as_deref(), &path).await
}

async fn write_texture_png(
    texture: Texture,
    palette: Option<&Palette>,
    path: &Path,
) -> Result<(), async_std::io::Error> {
    let palette = texture.palette().unwrap_or_else(|| {
        palette.expect("Failed to convert texture to RGB: No built-in palette or provided default")
    });
    let size = texture.size();
    write_image_png(size.width, size.height, texture.image(), palette, path).await?;

    Ok(())
}

async fn write_image_png(
    width: u32,
    height: u32,
    data: &Image,
    palette: &Palette,
    path: &Path,
) -> Result<(), async_std::io::Error> {
    let mut buf = Vec::with_capacity(width as usize * height as usize);
    {
        let mut encoder = png::Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let texels = data
            .texels_rgb(&palette)
            .flat_map(|c| vec![c.0, c.1, c.2])
            .collect::<Vec<_>>();

        writer.write_image_data(&texels)?;
    }

    async_std::fs::write(path, buf).await?;

    Ok(())
}
