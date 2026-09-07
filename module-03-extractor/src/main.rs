use axum::{
    body::Bytes,
    extract::{FromRequest,FromRequestParts,Query,Path,Request,State},
    http::{header::HeaderMap,request::Parts,StatusCode},
    response::{IntoResponse,Response},
    routing::{get,post},
    Json,Router
};
use serde::{Serialize,Deserialize};
use std::{process::Output, sync::Arc};

// Path extractor
// GET : /users/{id}
async fn get_user(
    Path(id): Path<u64>
) -> String {
    format!("user id : {}", id)
}

// Query Extractor : ?page=1&limit=100&sort=ASC
#[derive(Debug,Deserialize)]
struct ParamsList {
    page: Option<u32>,
    limit : Option<u32>,
    sort : Option<String>
}
async fn get_list_user(
    Query(params): Query<ParamsList>
) -> String {
    format!(
        "Page : {} , Limit : {} , Sort : {}",
        params.page.unwrap_or(1),
        params.limit.unwrap_or(100),
        params.sort.unwrap_or_else(|| "id".to_string())
    )
}

// json extractor : body : {"id":62","name":"somename","email":"somemail@mail"}

//request menggunakan deserialize
#[derive(Debug,Deserialize)]
struct ReqCreateUser {
    name: String,
    email : String
}

// response menggunakan serialize
#[derive(Debug,Serialize)]
struct ResCreateUser {
    id: u64,
    name: String,
    email : String
}

async fn create_user(
    Json(payload) : Json<ReqCreateUser>
) -> Json<ResCreateUser> {
    Json(ResCreateUser { id: 1, name: payload.name, email: payload.email })
}

// headers extractor 
// " -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:146.0)"
// " 'Content-Length: 0' \"

async fn headers(
    header: HeaderMap
) -> String {
    let user_agent = header
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");

    let content_type = header
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Not Specified");
    format!("User-Agent: {}\nContent-Type: {}", user_agent, content_type)
}

// Raw body extractor
async fn bodies(
    body:Bytes
) -> String {
    format!("Received {} bytes ", body.len())
}


// multiple extractor itu single handler
// urutan itu penting. 
// body yang consume (Json, String, Bytes)
// harus ada di akhir
async fn combined_extractors(
    Path(id) : Path<u64>,
    Query(params) : Query<ParamsList>,
    headers: HeaderMap,
    Json(body) : Json<ReqCreateUser>,
) -> String {
    format!(
        " ID: {}\nPage: {:?}\nUser-Agent: {:?}\nName: {}",
        id,
        params.page,
        headers.get("user-agent"),
        body.name
    )
}

// Optional Extractor
// Option<T> sebagai extractor membutuhkan T untuk implemen
/// `OptionalFromRequestParts` atau `OptionalFromRequest`
// ini menghasilkan error handling yang baik, dan tidak silent error.
// async fn optional_query(
//     Query(params): Query<Option<ParamsList>>
// ) -> String {
//     match params {
//         Some(p) => format!("Params : page={:?}", Some(p.page)),
//         None => "No Query params provided".to_string(),
//     }
// }
async fn optional_query(Query(params): Query<ParamsList>) -> String {
    if params.page.is_none() && params.limit.is_none() && params.sort.is_none() {
        "No Query params provided".to_string()
    } else {
        format!("Params : page={:?}", params.page.unwrap())
    }
}


// Custom Extractor (tidak lagi menggunakan #[async_trait])
/// Rust now supports `impl Future<Output = _>` in traits natively.

// custom extractor untuk apikey
struct ApiKey(String);
// custom error untuk custom extractor
#[derive(Debug)]
struct ApiKeyErr;

impl IntoResponse for ApiKeyErr {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            "missing or invalid API key. Provided X-API-Key header",
        )
            .into_response()
    }
}

impl<S> FromRequestParts <S> for ApiKey
where
    S:Send + Sync,
    {
        type Rejection = ApiKeyErr;

        fn from_request_parts(
            parts: &mut Parts,
            state: &S,
        ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
            let api_key = parts
                .headers
                .get("x-api-key")
                .and_then(|v| v.to_str().ok())
                .map(|s|s.to_string());

            async move {
                match api_key{
                    Some(key) if !key.is_empty() => Ok(ApiKey(key)),
                    _ => Err(ApiKeyErr),
                }
            }
        }
    }

async fn protected_endpoint(
    ApiKey(key): ApiKey
) -> String {
    format!("Access granted! Your Api Key : {}", key)
}

// Custom Extractor dengan body
// custom extractor yang valdasi json Body
#[derive(Debug, Deserialize)]
struct ValidatedUser {
    name : String,
    email : String
}

struct ValidatedJson<T>(T);

#[derive(Debug)]
enum ValidationErr{
    InvalidJson(String),
    InvalidEmail,
    NameTooShort
}

impl IntoResponse for ValidationErr {
    fn into_response(self) -> Response {
        let (status,msg) = match self {
            ValidationErr::InvalidJson(e) => {
                (StatusCode::BAD_REQUEST, format!("invalid JSON: {}", e))
            }
            ValidationErr::InvalidEmail => {
                (StatusCode::BAD_REQUEST, "invalid email".to_string())
            }
            ValidationErr::NameTooShort => {
                (StatusCode::BAD_REQUEST, "name must be at least 2 chars".to_string())
            }
        };
        (status,msg).into_response()
    }
}

// custom extractor yang validasi request body
// note untuk body extractor implementasi FromRequest 
// bukan FromRequestParts
impl<S> FromRequest <S> for ValidatedJson<ValidatedUser>
where
    S: Send + Sync,
    {
        type Rejection = ValidationErr;
        fn from_request(
            req: Request,
            state: &S,
        ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send{
            async move {
                // extract json dulu
                let Json(user): Json<ValidatedUser> = Json::from_request(req, state)
                    .await
                    .map_err(|e| ValidationErr::InvalidJson(e.to_string()))?;

                // validasi panjang nama
                if user.name.len() < 2 {
                    return Err(ValidationErr::NameTooShort);
                }

                // validasi email
                if !user.email.contains('@') {
                    return Err(ValidationErr::InvalidEmail);
                }

                Ok(ValidatedJson(user))
            }
        }
    }

async fn create_validated_user(
    ValidatedJson(user): ValidatedJson<ValidatedUser>
) -> String {
    format!("Created User : {} <{}>", user.name, user.email)
}


// state sebagai extractor
#[derive(Clone)]
struct AppState {
    db_pool : String, // aslinya ini akan menjadi database conn
    api_version : String
}
async fn with_state(
    State(state) : State<Arc<AppState>> 
) -> String {
    format!(" API Version :{}, DB : {}", state.api_version, state.db_pool)
}


#[tokio::main]
async fn main (){
    let state = Arc::new(AppState {
        db_pool : "Postgres//localhost/db".to_string(),
        api_version : "v.1.1.0".to_string(),
    });
    let app = Router::new()
        // Built-in extractor
        .route("/users/{id}", get(get_user))
        .route("/users", get(get_list_user).post(create_user))
        .route("/headers", get(headers))
        .route("/raw", post(bodies))

        // multiple extractors
        .route("/optional", get(optional_query))
        // custom extractor
        .route("/protected", get(protected_endpoint))
        .route("/validated", post(create_validated_user))
        // state extractor
        .route("/state", get(with_state))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");
    axum::serve(listener, app)
        .await
        .expect("failed start server");
}