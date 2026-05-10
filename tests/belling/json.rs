use std::fs;

use base64::{Engine, engine::general_purpose};
use facture_rs::{
  belling::{json::JsonBilling,
  utils::{core::comprobante::Comprobante, json::input::JsonInput}},
  core::request::FactureRequest/* , sucursal::{get::Sucursal, utils::{filters::SucursalFilters, input::SucursalInput}} */
};


#[tokio::test]
async fn billing_json() {
  let bearer = "ea242e297578768dd8ae94ecbbde0497".to_string();


  /*
   // This code consulte the User Sucursals
   //
    let filters = SucursalFilters::new()
      .offset(0 as usize)
      .size(1 as usize)
      .emisor("EKU9003173C9");

    let sucursal_input  = SucursalInput{
      bearer: bearer.clone(),
      filters: filters
    };

    let sucursales = Sucursal.request(sucursal_input).await.unwrap();
    let sucursal_id = sucursales.pagination.items[0].id;
  */

 let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/simple_tax.json");

  let tax = fs::read_to_string(path)
    .expect("No se pudo leer examples/simple_tax.json");

  let encode = general_purpose::STANDARD.encode(tax);

  let comprobante = Comprobante{
    encode: encode,
    request_uuid: "123e4567-e89b-12d3-a456-426655440000".to_string()
  };

  let input = JsonInput::new()
    .set_bearer(bearer)
    .set_comprobante(comprobante)
    .set_emisor(61400 as usize)
    .set_sucursal(63073 as usize);


  let response = JsonBilling.request(input).await.unwrap();

  println!("code: {}",response.code);
  for item in response.result.items{
    println!("succed: {}", item.succeed);
    println!("message: {}",item.message);
  }
  println!("petition message: {}",response.message);
}
