pub struct SucursalFilters{
  pub list: Vec<(String,String)>,
}

impl SucursalFilters{
  pub fn new() -> Self {
    SucursalFilters{
      list: Vec::new()
    }
  }

  pub fn add(mut self, filter: impl Into<(String, String)>) -> Self{
    self.list.push(filter.into());
    self
  }

  pub fn offset(mut self, offset: impl Into<usize>) -> Self{
    self.list.push(("offset".to_string(), format!("{}", offset.into())));
    self
  }

  pub fn size(mut self, size: impl Into<usize>) -> Self{
    self.list.push(("size".to_string(), format!("{}", size.into())));
    self
  }

  pub fn emisor(mut self, emisor: impl Into<String>) -> Self{
    self.list.push(("filter".to_string(), format!("empresa.rfc:eq!{}",emisor.into())));
    self
  }

}

