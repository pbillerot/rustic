
use actix_session::Session;
use serde::{Deserialize, Serialize};
use actix_web::error::Result;

const FLASH_KEY: &str = "flash";

pub fn set_flash(session: &Session, flash: FlashMessage) -> Result<()> {
    Ok(session.insert(FLASH_KEY, flash)?)
}

pub fn get_flash(session: &Session) -> Result<Option<FlashMessage>> {
    Ok(session.get::<FlashMessage>(FLASH_KEY)?)
}

pub fn clear_flash(session: &Session) {
    session.remove(FLASH_KEY);
}

// #[derive(Deserialize, Serialize, Clone)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FlashMessage {
    pub kind: String,
    pub message: String,
}

impl FlashMessage {
    pub fn success(message: &str) -> Self {
        Self {
            kind: "success".to_owned(),
            message: message.to_owned(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            kind: "error".to_owned(),
            message: message.to_owned(),
        }
    }
}

// // Session is set during operations on other endpoints that can redirect to index
// if let Some(flash) = session::get_flash(&session)? {
//     context.insert("msg", &(flash.kind, flash.message));
//     session::clear_flash(&session);
// }

// session::set_flash(&session, FlashMessage::success("Task successfully added"))?;

// Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))

// if params.description.is_empty() {
//     session::set_flash(&session, FlashMessage::error("Description cannot be empty"))?;
//     Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
// } else {
//     db::create_task(params.into_inner().description, &pool)
//         .await
//         .map_err(error::ErrorInternalServerError)?;

//     session::set_flash(&session, FlashMessage::success("Task successfully added"))?;

//     Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
// }

// #[derive(Debug, Clone, PartialEq)]
// struct FlagFromMiddleware(String);

// /// Use the `ReqData<T>` extractor to access request data in a handler.
// async fn handler(
//     req: HttpRequest,
//     opt_flag: Option<web::ReqData<FlagFromMiddleware>>,
// ) -> impl Responder {
//     // use an option extractor if middleware is not guaranteed to add this type of req data
//     if let Some(flag) = opt_flag {
//         assert_eq!(&flag.into_inner(), req.extensions().get::<FlagFromMiddleware>().unwrap());
//     }

//     HttpResponse::Ok()
// }