use facture_api::{
  belling::{json::JsonBilling,
  utils::{core::comprobante::Comprobante, json::input::JsonInput}},
  core::request::FactureRequest, sucursal::{get::Sucursal, utils::{filters::SucursalFilters, input::SucursalInput}}
};


#[tokio::test]
async fn billing_json() {
  let bearer = "f71990fbc3225baaf714654746bed5e0".to_string();

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


  let path = std::path::Path::new("G:/hola/hola.txt");
  if !path.exists(){
    println!("This Path doesn't exists: {}", path.to_string_lossy());
  }

  let encode = tokio::fs::read_to_string(path).await.unwrap();


  let comprobante = Comprobante{
    encode: encode,
    request_uuid: "".to_string()
  };

  let input = JsonInput::new()
    .set_bearer(bearer)
    .set_comprobante(comprobante)
    .set_sucursal(sucursal_id);


  let response = JsonBilling.request(input).await.unwrap();

  println!(" code: {}",response.code);
  for item in response.result.items{
    println!("succed: {}", item.succeed);
    println!("message: {}",item.message);
  }
  println!("petition message: {}",response.message);
}
