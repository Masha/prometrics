#[cfg(target_os = "linux")]
use libc;
use std::time::SystemTime;
use std::vec;

use metric::Metric;
use Collect;

#[cfg(target_os = "linux")]
lazy_static! {
    static ref CLK_TCK: f64 = unsafe { libc::sysconf(libc::_SC_CLK_TCK) as f64 };
    static ref PAGESIZE: usize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
}

/// Process metrics collector.
///
/// # Notice
///
/// On non Linux platforms, the `collect` method always returns `None`.
///
/// # Reference
///
/// - [process metrics](https://prometheus.io/docs/instrumenting/writing_clientlibs/#process-metrics)
///
/// # Examples
///
/// ```
/// use prometrics::{default_gatherer, default_registry};
/// use prometrics::metrics::ProcessMetricsCollector;
///
/// // Register
/// default_registry().register(ProcessMetricsCollector::new());
///
/// // Gather
/// let _metrics = default_gatherer().lock().unwrap().gather();
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub struct ProcessMetricsCollector {
    start_time: SystemTime,
}
impl ProcessMetricsCollector {
    /// Makes a new `ProcessMetricsCollector` instance.
    pub fn new() -> Self {
        ProcessMetricsCollector {
            start_time: SystemTime::now(),
        }
    }
}
impl Default for ProcessMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
impl Collect for ProcessMetricsCollector {
    type Metrics = vec::IntoIter<Metric>;
    fn collect(&mut self) -> Option<Self::Metrics> {
        None
    }
}
