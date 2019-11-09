use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::forms::profile_form::ProfileForm;

use {{crate_name}}_core::profile::UserProfile;
use {{crate_name}}_service::AppConfig;

/// Available at `/`
pub fn index(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {{crate_name}}_templates::home::index(&ctx))
}

/// Available at `/health`
pub fn health() -> HttpResponse {
  HttpResponse::Ok().finish()
}

/// Available at `/profile`
pub fn profile(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {{crate_name}}_templates::profile::profile(&ctx))
}

/// Available by posting to `/profile`
pub fn profile_post(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, form: Option<web::Form<ProfileForm>>) -> HttpResponse {
  match form {
    Some(f) => crate::redir(&session, &cfg, &req, |ctx| {
      let profile = UserProfile::new(
        String::from(f.username()),
        f.theme(),
        f.navbar_color().into(),
        f.link_color().into()
      );
      {{crate_name}}_service::profile::save(&cfg.files(), &ctx.user_id(), &profile)?;
      ctx.router().route_simple("profile")
    }),
    None => crate::redir(&session, &cfg, &req, |ctx| ctx.router().route_simple("profile"))
  }
}

/// Available at `/settings`
pub fn settings(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {{crate_name}}_templates::settings::settings(&ctx))
}

/// Available by posting to `/settings`
pub fn settings_post(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {{crate_name}}_templates::settings::settings(&ctx))
}