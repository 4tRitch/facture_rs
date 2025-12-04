pub trait FactureRequest {
  type Input;
  type Output;
  type TError;

  fn request(&self, input: Self::Input)
    -> impl std::future::Future<Output = Result<Self::Output, Self::TError>>;

}
