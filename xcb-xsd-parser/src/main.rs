use anyhow::Result;

use crate::generate::xsd::generate_xsd_spec;

mod generate;

fn main() -> Result<()> {
    gen_xsd()?;
    Ok(())
}

fn gen_xsd() -> Result<()> {
    let xsd = "xcb-xsd-parser/proto/xcb.xsd";
    let xsd_bytes = std::fs::read(xsd)?;
    let module_builder = generate_xsd_spec(&xsd_bytes)?;
    module_builder.write_to_disk("xcb-code-generator/src/generated_parse_template")?;
    Ok(())
}
