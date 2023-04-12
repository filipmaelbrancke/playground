use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // GDPR PII 😇
    let request_span = tracing::info_span!(
        "Adding new subscriber",
        // prefix with % = use the Display implementation for logging purposes
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = form.name
    );
    // .enter() = returns Entered / guard: as long as guard variable is not dropped
    // -> all downstream spans & log events will be registered as children of entered span
    // Resource Acquisition Is Initialization pattern (compiler keeps track of lifetime of variables -
    // when going out of scope -> call to destructor inserted (Drop::drop))
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    // instrument = enter the span every time self, the future, is polled / exit the span every time the future is parked
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // {:?} = std::fmt::Debug format => capture query error
            tracing::error!(
                "request_id {} - Failed to execute query {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
