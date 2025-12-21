use crate::{sucursal::utils::filters::SucursalFilters};

pub struct SucursalInput {
  pub bearer: String,
  pub filters: SucursalFilters
}

impl SucursalInput {
  pub fn new() -> Self{
    SucursalInput{
      bearer: String::new(),
      filters: SucursalFilters::new()
    }
  }
}

