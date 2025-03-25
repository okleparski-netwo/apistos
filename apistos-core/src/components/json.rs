use crate::ApiComponent;
use actix_web::web::Json;
use apistos_models::ApistosSchema;
use apistos_models::reference_or::ReferenceOr;

impl<T> ApiComponent for Json<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: apistos_models::OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Json<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: apistos_models::OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}
