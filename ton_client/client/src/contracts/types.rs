#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContractABIParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub parameterType: String,
}


#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContractABIFunction {
    pub name: String,
    pub signed: bool,
    pub inputs: Vec<ContractABIParameter>,
    pub outputs: Vec<ContractABIParameter>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContractABI {
    #[serde(rename = "ABI version")]
    pub abiVersion: i32,
    pub functions: Vec<ContractABIFunction>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContractPackage {
    pub abi: ContractABI,
    pub imageBase64: String,
}