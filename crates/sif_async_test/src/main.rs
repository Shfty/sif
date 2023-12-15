use async_std::path::PathBuf;
use ogre::parser_async::*;
use std::{error::Error, time::Instant};
use wad_source::{SourceStreamFile, SourceStreamSlice, SourceStreamVec};

const WAD_PATH: &str = "ogre/test_data/wad3/halflife.wad";

macro_rules ! profile {
    ($($tt:tt)*) => {
        {
            let start = std::time::Instant::now();

            {
                $($tt)*
            }

            let dur = Instant::now() - start;
            println!("Done in {:?}\n", dur);
        }
    };
}

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    print!("Parsing static WAD slice... ");
    profile! {
        const WAD_BYTES: &[u8] = include_bytes!("../../ogre/test_data/wad3/halflife.wad");
        let source = SourceStreamSlice::from(WAD_BYTES);
        let _wad = parse_wad(source).await?;
    }

    print!("Parsing {} from complete buffer... ", WAD_PATH);
    profile! {
        let wad_bytes = async_std::fs::read(WAD_PATH).await?;
        let source = SourceStreamVec::from(wad_bytes);
        let _wad = parse_wad(source).await?;
    }

    print!("Parsing {} directly from disk... ", WAD_PATH);
    profile! {
        let path = PathBuf::from(WAD_PATH);
        let source = SourceStreamFile::from(path);
        let _wad = parse_wad(source).await?;
    }

    Ok(())
}
