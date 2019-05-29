mod reader_tests {
    use ParamType;
    use param_type::Reader;

    #[test]
    fn test_read_param() {
        assert_eq!(Reader::read("uint256").unwrap(), ParamType::Uint(256));
        assert_eq!(Reader::read("int64").unwrap(), ParamType::Int(64));
        assert_eq!(Reader::read("dint").unwrap(), ParamType::Dint);
        assert_eq!(Reader::read("duint").unwrap(), ParamType::Duint);
        assert_eq!(Reader::read("bool").unwrap(), ParamType::Bool);
        assert_eq!(Reader::read("bool[]").unwrap(), ParamType::Array(Box::new(ParamType::Bool)));
        
        assert_eq!(
            Reader::read("int33[2]").unwrap(),
            ParamType::FixedArray(Box::new(ParamType::Int(33)), 2));
        
        assert_eq!(
            Reader::read("bool[][2]").unwrap(),
            ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2));
        
        assert_eq!(Reader::read("tuple").unwrap(), ParamType::Tuple(vec![]));
        
        assert_eq!(
            Reader::read("tuple[]").unwrap(),
            ParamType::Array(Box::new(ParamType::Tuple(vec![]))));
        
        assert_eq!(
            Reader::read("tuple[4]").unwrap(),
            ParamType::FixedArray(Box::new(ParamType::Tuple(vec![])), 4));

        assert_eq!(Reader::read("bits256").unwrap(), ParamType::Bits(256));
        assert_eq!(Reader::read("bitstring").unwrap(), ParamType::Bitstring);
    }
}

mod param_type_tests {
    use ParamType;
    use Param;

    #[test]
    fn test_param_type_signature() {
        assert_eq!(ParamType::Uint(256).type_signature(), "uint256".to_owned());
        assert_eq!(ParamType::Int(64).type_signature(), "int64".to_owned());
        assert_eq!(ParamType::Dint.type_signature(), "dint".to_owned());
        assert_eq!(ParamType::Duint.type_signature(), "duint".to_owned());
        assert_eq!(ParamType::Bool.type_signature(), "bool".to_owned());

        assert_eq!(
            ParamType::Array(Box::new(ParamType::Bool)).type_signature(),
            "bool[]".to_owned());

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Int(33)), 2).type_signature(),
            "int33[2]".to_owned());

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2)
                .type_signature(),
            "bool[][2]".to_owned());

        assert_eq!(ParamType::Bits(256).type_signature(), "bits256".to_owned());
        assert_eq!(ParamType::Bitstring.type_signature(), "bitstring".to_owned());

        let mut tuple_params = vec![];
        tuple_params.push(Param {name: "a".to_owned(), kind: ParamType::Uint(123)});
        tuple_params.push(Param {name: "b".to_owned(), kind: ParamType::Dint});

        assert_eq!(
            ParamType::Tuple(tuple_params.clone()).type_signature(),
            "(uint123,dint)".to_owned());

        assert_eq!(
            ParamType::Array(Box::new(ParamType::Tuple(tuple_params.clone()))).type_signature(),
            "(uint123,dint)[]".to_owned());

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Tuple(tuple_params)), 4).type_signature(),
            "(uint123,dint)[4]".to_owned());
    }
}

mod deserialize_tests {
    use serde_json;
    use ParamType;

    #[test]
    fn param_type_deserialization() {
        let s = r#"["uint256", "int64", "dint", "duint", "bool", "bool[]", "int33[2]", "bool[][2]",
            "tuple", "tuple[]", "tuple[4]", "bits256", "bitstring"]"#;
        let deserialized: Vec<ParamType> = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized, vec![
            ParamType::Uint(256),
            ParamType::Int(64),
            ParamType::Dint,
            ParamType::Duint,
            ParamType::Bool,
            ParamType::Array(Box::new(ParamType::Bool)),
            ParamType::FixedArray(Box::new(ParamType::Int(33)), 2),
            ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2),
            ParamType::Tuple(vec![]),
            ParamType::Array(Box::new(ParamType::Tuple(vec![]))),
            ParamType::FixedArray(Box::new(ParamType::Tuple(vec![])), 4),
            ParamType::Bits(256),
            ParamType::Bitstring
        ]);
    }
}