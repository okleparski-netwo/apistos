use crate::ApiErrorComponent;
#[cfg(feature = "actix")]
use crate::{PathItemDefinition, ResponseWrapper};
use netwopenapi_models::paths::{MediaType, Parameter, RequestBody, Response, Responses};
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::security::SecurityScheme;
use netwopenapi_models::Schema;
use std::collections::BTreeMap;
#[cfg(feature = "actix")]
use std::future::Future;

pub trait ApiComponent {
  fn content_type() -> String {
    "application/json".to_string()
  }

  fn required() -> bool {
    true
  }

  /// contains childs schemas recursively for this operation
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)>;

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    None
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)>;

  fn securities() -> BTreeMap<String, SecurityScheme> {
    Default::default()
  }

  fn security_requirement_name() -> Option<String> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    Self::schema().map(|(name, _)| RequestBody {
      content: BTreeMap::from_iter(vec![(
        Self::content_type(),
        MediaType {
          schema: Some(ReferenceOr::Reference {
            _ref: format!("#/components/schemas/{}", name),
          }),
          ..Default::default()
        },
      )]),
      required: Some(Self::required()),
      ..Default::default()
    })
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }

  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    BTreeMap::default()
  }

  fn responses(_content_type: Option<String>) -> Option<Responses> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![]
  }
}
impl<T> ApiComponent for Option<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }

  fn securities() -> BTreeMap<String, SecurityScheme> {
    T::securities()
  }

  fn security_requirement_name() -> Option<String> {
    T::security_requirement_name()
  }
}

impl<T> ApiComponent for Vec<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    true
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

impl<T, E> ApiComponent for Result<T, E>
where
  T: ApiComponent,
  E: ApiErrorComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }

  // We expect error to be present only for response part
  fn error_responses() -> Vec<(String, Response)> {
    E::error_responses()
  }

  // We expect error to be present only for response part
  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    E::schemas_by_status_code()
  }

  fn responses(content_type: Option<String>) -> Option<Responses> {
    T::responses(content_type)
  }
}

#[cfg(feature = "actix")]
impl<T> ApiComponent for actix_web::web::Data<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

#[cfg(feature = "actix")]
impl<T: Clone> ApiComponent for actix_web::web::ReqData<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

#[cfg(feature = "actix")]
impl<F, R, P> ApiComponent for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: actix_web::Responder + ApiComponent,
  P: PathItemDefinition,
{
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    R::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    R::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    R::schema()
  }

  fn error_responses() -> Vec<(String, Response)> {
    R::error_responses()
  }

  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    R::error_schemas()
  }

  fn responses(content_type: Option<String>) -> Option<Responses> {
    let mut responses = vec![];
    if let Some(response) = R::responses(content_type.clone()) {
      responses.append(
        &mut response
          .responses
          .into_iter()
          .collect::<Vec<(String, ReferenceOr<Response>)>>(),
      );
    } else if let Some((name, schema)) = Self::schema() {
      let _ref = match schema {
        ReferenceOr::Reference { _ref } => _ref,
        ReferenceOr::Object(_) => format!("#/components/schemas/{}", name),
      };
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(
            content_type.unwrap_or_else(Self::content_type),
            MediaType {
              schema: Some(ReferenceOr::Reference { _ref }),
              ..Default::default()
            },
          )]),
          ..Default::default()
        }),
      ));
    } else if let Some(content_type) = content_type {
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(content_type, MediaType::default())]),
          ..Default::default()
        }),
      ));
    } else {
      responses.push(("200".to_owned(), ReferenceOr::Object(Response::default())));
    }

    responses.append(
      &mut Self::error_responses()
        .into_iter()
        .map(|(status, schema)| (status, ReferenceOr::Object(schema)))
        .collect(),
    );
    Some(Responses {
      responses: BTreeMap::from_iter(responses),
      ..Default::default()
    })
  }
}
