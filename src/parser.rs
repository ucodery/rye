use pest::Parser;

#[derive(Parser)]
#[grammar = "python.pest"]
pub struct PythonParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_atoms() {
        for source in [
            String::from("1230"),
            String::from("1_023"),
            String::from("000"),
            String::from("0_00_0"),
            String::from("0b_0"),
            String::from("0b0101"),
            String::from("0B1111_0000"),
            String::from("0o_07"),
            String::from("0O_12_56"),
            String::from("0O0"),
            String::from("0xaBcDeF"),
            String::from("0x_aa_1234"),
            String::from("0XBA98"),
        ]
        .iter()
        {
            let number = PythonParser::parse(Rule::int, source).unwrap();
            assert_eq!(number.as_str(), source, "{} should be an int", source);
        }
    }

    #[test]
    fn not_int_atoms() {
        for source in [
            String::from("0123"),
            String::from("_123"),
            String::from("123_"),
            String::from("12__34"),
            String::from("0b"),
            String::from("0_b1"),
            String::from("0b02"),
            String::from("0o9"),
            String::from("_0o0"),
            String::from("0x12ABXY"),
        ]
        .iter()
        {
            match PythonParser::parse(Rule::int, source) {
                Ok(parse_result) => {
                    assert_ne!(
                        parse_result.as_str(),
                        source,
                        "{} shouldn't be an int",
                        source
                    );
                }
                Err(_) => assert!(true),
            }
        }
    }

    #[test]
    fn float_atoms() {
        for source in [
            String::from(".1230"),
            String::from("1230."),
            String::from("1.230"),
            String::from("0.0"),
            String::from("0e0"),
            String::from("12e30"),
            String::from(".12E30"),
            String::from("1.2e30"),
            String::from("12.E30"),
            String::from("12.e+30"),
            String::from("12.e-30"),
            String::from("1_2E30"),
            String::from("12e3_0"),
            String::from("1_2.3_4e+5_6"),
        ]
        .iter()
        {
            let number = PythonParser::parse(Rule::float, source).unwrap();
            assert_eq!(number.as_str(), source, "{} should be a float", source);
        }
    }

    #[test]
    fn not_float_atoms() {
        for source in [
            String::from("."),
            String::from("1._"),
            String::from("_.1"),
            String::from("1_.2"),
            String::from("1._2"),
            String::from(".e1"),
            String::from("1e.2"),
            String::from("1e2.3"),
            String::from("_1e2"),
            String::from("1_e2"),
            String::from("1e_2"),
            String::from("1e2_"),
        ]
        .iter()
        {
            match PythonParser::parse(Rule::float, source) {
                Ok(parse_result) => {
                    assert_ne!(
                        parse_result.as_str(),
                        source,
                        "{} shouldn't be a float",
                        source
                    );
                }
                Err(_) => assert!(true),
            }
        }
    }

    #[test]
    fn imaginary_atoms() {
        for source in [
            String::from("1j"),
            String::from("1.2J"),
            String::from(".1j"),
            String::from("1.J"),
            String::from("1e2j"),
            String::from("1.2e3J"),
            String::from("1_2.3e4_5j"),
        ]
        .iter()
        {
            let number = PythonParser::parse(Rule::imag, source).unwrap();
            assert_eq!(number.as_str(), source, "{} should be an imaginary number", source);
        }
    }

    #[test]
    fn not_imaginary_atoms() {
        for source in [
            String::from("1ej"),
            String::from("1e_j"),
            String::from("1e2_j"),
        ]
        .iter()
        {
            match PythonParser::parse(Rule::float, source) {
                Ok(parse_result) => {
                    assert_ne!(
                        parse_result.as_str(),
                        source,
                        "{} shouldn't be a float",
                        source
                    );
                }
                Err(_) => assert!(true),
            }
        }
    }
}
