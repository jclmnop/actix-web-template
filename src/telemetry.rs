use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    log_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    //TODO: learn about HRTBs
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let log_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(log_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    tracing::info!("Logger set.");
    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::info!("Subscriber set.");
}

/// Creates a unique ID for the current request, creates the `info_span` and
/// then enters the span to create the guard.
///
/// First arg is a string literal which is used as a "name" for the span.
/// The est of the arguments are fields to be displayed within the trace. `%`
/// can be used to automatically name the field. For example, `%field`, if field
/// had a value of 69, would be displayed as `field: 69`.
/// ```rust
/// # use actix_web::{web, HttpResponse};
/// # use sqlx::PgPool;
/// use tracing::Instrument;
/// # use actix_web_template::init_request_trace;
/// pub async fn example_get(
///     email: web::Path<String>,
///     pool: web::Data<PgPool>,
/// ) -> HttpResponse {
///     init_request_trace!("Processing new GET request", %email);
///     todo!()
/// }
#[macro_export]
macro_rules! init_request_trace {
    ($name:literal, $($field:tt)*) => {
        let request_id = ::uuid::Uuid::new_v4();
        let request_span = ::tracing::info_span!(
            $name,
            %request_id,
            $($field)*
        );
        let _request_span_guard = request_span.enter();
    };
    ($name:literal) => {
        let request_id = ::uuid::Uuid::new_v4();
        let request_span = ::tracing::info_span!(
            $name,
            %request_id,
        );
        let _request_span_guard = request_span.enter();
    };
}
