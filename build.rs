use std::{
    error::Error,
    fs::File,
    io::Write,
    path::Path,
};

#[derive(Debug, serde::Deserialize)]
struct XYZRecord {
    wavelength: u16,
    x:          f64,
    y:          f64,
    z:          f64,
}

#[derive(Debug, serde::Deserialize)]
struct DaylightRecord {
    wavelength: u16,
    s0:         f64,
    s1:         f64,
    s2:         f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR is missing");

    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("cie/xyz.csv")?);

    let mut output_file = File::create(Path::new(&out_dir).join("cie.rs"))?;
    writeln!(
        &mut output_file,
        "pub const fn cie_xyz(wavelength: u16) -> Vec3<f64> {{"
    )?;
    writeln!(&mut output_file, "\tmatch wavelength {{")?;

    for record in csv.deserialize() {
        let XYZRecord {
            wavelength,
            x,
            y,
            z,
        } = record?;

        writeln!(
            &mut output_file,
            "\t\t{wavelength} => Vec3::new({x}f64, {y}f64, {z}f64),"
        )?;
    }

    writeln!(&mut output_file, "\t\t_ => Vec3::splat(0.0),")?;
    writeln!(&mut output_file, "\t}}")?;
    writeln!(&mut output_file, "}}")?;

    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("cie/daylight.csv")?);
    writeln!(
        &mut output_file,
        "pub fn cie_daylight(wavelength: u16) -> Vec3<f64> {{"
    )?;
    writeln!(&mut output_file, "\tmatch wavelength {{")?;

    for record in csv.deserialize() {
        let DaylightRecord {
            wavelength,
            s0,
            s1,
            s2,
        } = record?;

        writeln!(
            &mut output_file,
            "\t\t{wavelength} => Vec3::new({s0}f64, {s1}f64, {s2}f64),"
        )?;
    }

    writeln!(&mut output_file, "\t\t300..=780 => {{")?;
    writeln!(
        &mut output_file,
        "\t\t\tlet prev = wavelength.div_floor(5) * 5;"
    )?;
    writeln!(
        &mut output_file,
        "\t\t\tlet next = wavelength.div_ceil(5) * 5;"
    )?;
    writeln!(&mut output_file, "\t\t\tVec3::lerp(")?;
    writeln!(&mut output_file, "\t\t\t\t&cie_daylight(prev),")?;
    writeln!(&mut output_file, "\t\t\t\t&cie_daylight(next),")?;
    writeln!(
        &mut output_file,
        "\t\t\t\tf64::from(wavelength - prev) / f64::from(next - prev)"
    )?;
    writeln!(&mut output_file, "\t\t\t)")?;
    writeln!(&mut output_file, "\t\t}},")?;
    writeln!(&mut output_file, "\t\t_ => Vec3::splat(0.0),")?;
    writeln!(&mut output_file, "\t}}")?;
    writeln!(&mut output_file, "}}")?;

    Ok(())
}
