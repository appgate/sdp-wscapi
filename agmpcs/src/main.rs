use std::collections::HashMap;
use serde::Serialize;
use wmi::*;
use wmi::COMLibrary;

#[derive(Serialize)]
struct Output<'a> {
    MpComputerStatus: &'a HashMap<String, Variant>,
}

fn main() -> Result<(), WMIError> {
    let wmi_con = WMIConnection::with_namespace_path("root\\microsoft\\windows\\defender", COMLibrary::new()?)?;
    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query("SELECT * FROM MSFT_MpComputerStatus")
        .unwrap();
    let o = Output { MpComputerStatus: &results[0] };
    let serialized = serde_json::to_string(&o).unwrap_or("{\"MpComputerStatus\":{}}".to_string());
    println!("{}", serialized);
    Ok(())
}