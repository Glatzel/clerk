use tracing_subscriber::Layer;
use tracing_subscriber::registry::LookupSpan;
/// Generate a terminal log layer for tracing.
///
/// # Arguments
///
/// - `level`: The desired log level filter to set.
///
/// # Example
///
/// ```
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// use tracing_subscriber::filter::LevelFilter;
/// use tracing_subscriber::EnvFilter;
/// tracing_subscriber::registry()
///         .with(
///             EnvFilter::builder()
///                 .with_default_directive(LevelFilter::TRACE.into())
///                 .from_env_lossy(),
///         )
///         .with(clerk::terminal_layer(true))
///         .init();
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn terminal_layer<S>(color: bool) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .event_format(crate::ClerkFormatter { color })
        .with_writer(std::io::stderr)
        .boxed()
}

#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, trace, warn};
    use tracing_core::LevelFilter;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use super::*;
    #[test]
    fn test_log() {
        tracing_subscriber::registry()
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::TRACE.into())
                    .from_env_lossy(),
            )
            .with(terminal_layer(true))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
