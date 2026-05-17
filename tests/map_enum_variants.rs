use chiptool::{ir::IR, transform::map_enum_variants::MapEnumVariants};

#[test]
fn duplicate_ir_enums_should_be_updated() -> Result<(), Box<dyn std::error::Error>> {
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
        bit_size: 3
        enum: vals::Enumm
    enum/vals::Enumm:
      bit_size: 3
      variants:
        - { name: VariantOne, value: 0b001 }
        - { name: VariantTwo, value: 0b010 }
        - { name: VariantThree, value: 0b011 }
    "#;

    let transform_yaml = r#"
      !MapEnumVariants
      enum: vals::Enumm
      variants:
        VariantOne: _1
        VariantTwo: _2
        VariantThree: _3
    "#;

    let mut ir: IR = serde_yaml::from_slice(input_yaml.as_bytes())?;

    let transform = serde_yaml::from_slice::<MapEnumVariants>(transform_yaml.as_bytes())?;
    transform.run(&mut ir)?;

    let enumm = ir.enums.get("vals::Enumm").expect("Enum not found");

    let variant_names = enumm
        .variants
        .iter()
        .map(|field| &field.name)
        .collect::<Vec<_>>();

    assert_eq!(variant_names, &["_1", "_2", "_3"]);

    Ok(())
}
