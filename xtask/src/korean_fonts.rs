//! Reproducible Korean-only craft-fonts input; no font is stored in this repository.
use anyhow::{Context, Result, bail};
use std::{path::PathBuf, process::Command};
const SOURCE: &str = "https://github.com/naver/nanumfont/releases/download/VER2.5/NanumGothicCoding-2.5.zip";
const ZIP_SHA: &str = "f4c9b1082cb36d793ccf75a331316c880ef9e4bc229408c22f9d2703b7619a3f";
const LICENSE_URL: &str = "https://raw.githubusercontent.com/wiki/naver/nanumfont/Open-Font-License.md";
const LICENSE_SHA: &str = "4b13b7f15020898040e38ceca6648eb3c0f300ca06d45739af112f2fb4003a8f";
const FACES: &[(&str, &str, &str)] = &[
    ("NanumGothicCoding.ttf", "Regular", "0ef917c452fbcbd8107c69fcd0fe899b5ca6d09c42e9fdcc725cc0ed2c29aaa4"),
    ("NanumGothicCoding-Bold.ttf", "Bold", "5d15781e7a53b39130b9d87cb0681d7605ec380374cd0046210f61ce3ec070be"),
];
pub fn run(args: &[String]) -> Result<()> {
    if args.len() > 1 {
        bail!("usage: cargo xtask korean-fonts [DIR]");
    }
    let dir = args.first().map(PathBuf::from).unwrap_or_else(|| crate::gates::root().join("../craft-fonts-kr"));
    let mut manifest: String = FACES
        .iter()
        .map(|(name, style, hash)| {
            format!(
                "NanumGothicCoding | {style} | fonts/{name} | Kore,Latn | OFL-1.1 | {hash} | https://github.com/naver/nanumfont/releases/tag/VER2.5\n"
            )
        })
        .collect();
    manifest.push_str("Unifont | Regular | fonts/unifont-15.0.06.otf | Hang | OFL-1.1 | 7f0284ff6373347c9fd8253196aebd924a7e2f57a7ee56a3a63d7e84551299bd | https://unifoundry.com/pub/unifont/unifont-15.0.06/\n");
    let manifest_path = dir.join("fonts/manifest.txt");
    if manifest_path.exists() && std::fs::read_to_string(&manifest_path)? != manifest {
        bail!("{} already contains another font manifest; choose a separate empty directory", dir.display());
    }
    std::fs::create_dir_all(dir.join("fonts"))?;
    let archive = dir.join("NanumGothicCoding-2.5.zip");
    crate::assets::fetch_verified(SOURCE, &archive, ZIP_SHA)?;
    crate::assets::fetch_verified(LICENSE_URL, &dir.join("OFL.txt"), LICENSE_SHA)?;
    // UI-only final Hangul Jamo fallback: choose the font's OFL-1.1 dual-license option.
    // No GPL program source or font-generation utilities are included.
    crate::assets::fetch_verified(
        "https://unifoundry.com/pub/unifont/unifont-15.0.06/font-builds/unifont-15.0.06.otf",
        &dir.join("fonts/unifont-15.0.06.otf"),
        "7f0284ff6373347c9fd8253196aebd924a7e2f57a7ee56a3a63d7e84551299bd",
    )?;
    crate::assets::fetch_verified(
        "https://unifoundry.com/LICENSE.txt",
        &dir.join("OFL-Unifont.txt"),
        "1e74cb82bf476843e97c2596297b04219b1a7e51f7238944a8c031cb9401fa87",
    )?;
    for (name, _, hash) in FACES {
        // Only a fixed member of a hash-verified archive is read; no archive paths are extracted.
        let output = Command::new("unzip").arg("-p").arg(&archive).arg(name).output().context("running unzip")?;
        if !output.status.success() || crate::assets::sha256_hex(&output.stdout) != *hash {
            bail!("{name}: extraction or SHA-256 verification failed");
        }
        let path = dir.join("fonts").join(name);
        let temp = path.with_extension("part");
        std::fs::write(&temp, output.stdout)?;
        std::fs::rename(temp, path)?;
    }
    std::fs::write(manifest_path, manifest)?;
    println!("Verified Korean font input: {} (keep OFL.txt and OFL-Unifont.txt with every packaged build)", dir.display());
    Ok(())
}
