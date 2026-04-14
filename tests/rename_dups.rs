use chiptool::{ir::IR, svd2ir::convert_peripheral, transform::sanitize::Sanitize};
use svd_parser::ValidateLevel;

#[test]
fn duplicate_svd_fields_should_not_collide() -> Result<(), Box<dyn std::error::Error>> {
    let input_svd = r#"
        <device xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" schemaVersion="1.1" xs:noNamespaceSchemaLocation="CMSIS-SVD.xsd">
            <name>duplicate_fields_should_not_collide</name>
            <peripherals>
                <peripheral>
                    <name>PERIPHERAL</name>
                    <baseAddress>0x0</baseAddress>
                    <registers>
                        <register>
                            <name>Fieldset</name>
                            <addressOffset>0x41F</addressOffset>
                            <fields>
                                <field>
                                    <name>Reserved</name>
                                    <lsb>1</lsb>
                                    <msb>2</msb>
                                </field>
                                <field>
                                    <name>Reserved</name>
                                    <lsb>3</lsb>
                                    <msb>4</msb>
                                </field>
                            </fields>
                        </register>
                    </registers>
                </peripheral>
            </peripherals>
        </device> 
    "#;

    let config = svd_parser::Config::default()
        .expand_properties(true)
        .validate_level(ValidateLevel::Disabled);

    let device = svd_parser::parse_with_config(input_svd, &config)?;
    let mut ir = IR::new();
    convert_peripheral(&mut ir, device.peripherals.iter().next().unwrap())?;

    let (name, fieldset) = ir.fieldsets.iter().next().expect("Fieldset not found");

    assert_eq!(name, "Fieldset");

    let field_names = fieldset
        .fields
        .iter()
        .map(|field| &field.name)
        .collect::<Vec<_>>();

    assert_eq!(field_names, &["Reserved", "Reserved2"]);

    Ok(())
}

#[test]
fn duplicate_ir_fields_should_not_collide() -> Result<(), Box<dyn std::error::Error>> {
    let input_yaml = r#"
    block/BLOCK:
      items:
        - name: register
          byte_offset: 0
          bit_size: 32
          fieldset: regs::Fieldset
    fieldset/regs::Fieldset:
      fields:
      - name: reserved
        bit_offset: 0
        bit_size: 1
      - name: reserved
        bit_offset: 1
        bit_size: 1
    "#;
    let ir: IR = serde_yaml::from_slice(input_yaml.as_bytes())?;

    let (name, fieldset) = ir.fieldsets.iter().next().expect("Fieldset not found");

    assert_eq!(name, "regs::Fieldset");

    let field_names = fieldset
        .fields
        .iter()
        .map(|field| &field.name)
        .collect::<Vec<_>>();

    assert_eq!(field_names, &["reserved", "reserved2"]);

    Ok(())
}
