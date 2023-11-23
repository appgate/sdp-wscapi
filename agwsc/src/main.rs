use std::collections::HashMap;
use tinyjson::JsonValue;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::SecurityCenter::*;

fn unwrap(s: Result<windows::core::BSTR, windows::core::Error>) -> String {
    match s {
        Ok(s) => s.to_string().replace(|c: char| !c.is_ascii(), "?"),
        _ => "<Unknown>".into(),
    }
}

unsafe fn get_products(provider: WSC_SECURITY_PROVIDER) -> windows::core::Result<JsonValue> {
    let pl: IWSCProductList = CoCreateInstance(&WSCProductList, None, CLSCTX_ALL)?;
    pl.Initialize(provider)?;
    let n = pl.Count().unwrap_or(0) as u32;
    let mut products = Vec::new();
    for i in 0..n {
        let Ok(p) = pl.get_Item(i) else {
            continue;
        };

        let mut product: HashMap<String, JsonValue> = HashMap::new();
        product.insert("product_name".into(), unwrap(p.ProductName()).into());

        let state = match p.ProductState() {
            Ok(WSC_SECURITY_PRODUCT_STATE_OFF) => "Off",
            Ok(WSC_SECURITY_PRODUCT_STATE_ON) => "On",
            Ok(WSC_SECURITY_PRODUCT_STATE_SNOOZED) => "Snoozed",
            Ok(WSC_SECURITY_PRODUCT_STATE_EXPIRED) => "Expired",
            _ => "<Unknown>",
        };
        product.insert("product_state".into(), state.to_string().into());

        product.insert(
            "remediation_path".into(),
            unwrap(p.RemediationPath()).into(),
        );

        if provider != WSC_SECURITY_PROVIDER_FIREWALL {
            let status = match p.SignatureStatus() {
                Ok(WSC_SECURITY_PRODUCT_UP_TO_DATE) => "Up-to-date",
                _ => "Out-of-date",
            };
            product.insert("product_status".into(), status.to_string().into());
        }

        if provider == WSC_SECURITY_PROVIDER_ANTIVIRUS {
            product.insert(
                "product_state_timestamp".into(),
                unwrap(p.ProductStateTimestamp()).into(),
            );
        }

        products.push(JsonValue::from(product));
    }

    Ok(JsonValue::from(products))
}

fn usage() {
    println!(
        "Usage: agwsc.exe [-av | -as | -fw]
    av: Query Antivirus programs
    as: Query Antispyware programs
    fw: Query Firewall programs"
    );
}

fn main() -> windows::core::Result<()> {
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED)? };
    let mut json = HashMap::new();
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 {
        usage();
        return Ok(());
    }
    for arg in &args[1..] {
        let (key, val) = match arg.as_str() {
            "-av" => ("Antivirus", WSC_SECURITY_PROVIDER_ANTIVIRUS),
            "-as" => ("Antispyware", WSC_SECURITY_PROVIDER_ANTISPYWARE),
            "-fw" => ("Firewall", WSC_SECURITY_PROVIDER_FIREWALL),
            _ => {
                usage();
                return Ok(());
            }
        };
        if let Ok(val) = unsafe { get_products(val) } {
            json.insert(key.into(), val);
        }
    }

    print!("{}", JsonValue::from(json).stringify().unwrap());
    Ok(())
}
