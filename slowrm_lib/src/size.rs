macro_rules! unit {
    ($unit:tt, $pow2:expr) => {
        paste::paste! {
            #[doc = concat!("Build Size from ", stringify!($unit), " knowledge")]
            #[must_use]
            pub fn $unit(value: f64) -> Self {
                let value_in_bytes = value * 2f64.powf($pow2 as f64);
                let clamped_value = value_in_bytes.clamp(0.0, u64::MAX as f64);
                if value < 0.0 {
                    log::warn!(
                        "Initialising Size::{} with a negative value ({}), updating to 0",
                        stringify!($unit),
                        value,
                    );
                }
                else if value_in_bytes > clamped_value {
                    log::warn!(
                        "Initialising Size::{} with a too high value of bytes ({}), restricting to {}",
                        stringify!($unit),
                        value,
                        clamped_value,
                    );
                }
                Self(clamped_value as u64)
            }

            #[doc = concat!("Convert size as ", stringify!($unit))]
            #[must_use]
            pub fn [< as_ $unit >](&self) -> f64 {
                self.0 as f64 / 2f64.powf($pow2 as f64)
            }
        }
    }
}

/// Size in Bytes
///
/// The struct provide simple helpers to build the desired size
/// instead of dealing with raw bytes
#[derive(Clone, Copy, Debug)]
pub struct Size(u64);
impl Size {
    /// Build Size from bytes knowledge
    #[must_use]
    pub const fn bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Convert size as bytes
    #[must_use]
    pub const fn as_bytes(&self) -> u64 {
        self.0
    }

    unit! {kilobytes, 10}
    unit! {megabytes, 20}
    unit! {gigabytes, 30}
    unit! {terabytes, 40}
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // TODO: Can we do it panic free?
        Self(self.0 + rhs.0)
    }
}

impl std::fmt::Display for Size {
    #[allow(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (value, unit) = {
            if self.0 > (1u64 << 40) - 1 {
                (self.as_terabytes(), "TB")
            } else if self.0 > (1u64 << 30) - 1 {
                (self.as_gigabytes(), "GB")
            } else if self.0 > (1u64 << 20) - 1 {
                (self.as_megabytes(), "MB")
            } else if self.0 > (1u64 << 10) - 1 {
                (self.as_kilobytes(), "KB")
            } else {
                (self.as_bytes() as f64, "B")
            }
        };
        write!(
            f,
            "{}{}",
            format!("{value:.3}")
                .trim_end_matches('0')
                .trim_end_matches('.'),
            unit
        )
    }
}

#[cfg(test)]
mod tests_size {
    use super::Size;

    #[test]
    fn bytes() {
        let size = Size::bytes(42);
        assert_eq!(size.0, 42);
        assert_eq!(size.to_string(), "42B");
    }

    #[test]
    fn kilobytes() {
        let size = Size::kilobytes(42.0);
        assert_eq!(size.as_kilobytes(), 42.0);
        assert_eq!(size.as_bytes(), 42 * 1024);
        assert_eq!(size.to_string(), "42KB");
    }

    #[test]
    fn megabytes() {
        let size = Size::megabytes(42.0);
        assert_eq!(size.as_megabytes(), 42.0);
        assert_eq!(size.as_kilobytes(), 42.0 * 1024.0);
        assert_eq!(size.as_bytes(), 42 * 1024 * 1024);
        assert_eq!(size.to_string(), "42MB");
    }

    #[test]
    fn gigabytes() {
        let size = Size::gigabytes(42.0);
        assert_eq!(size.as_gigabytes(), 42.0);
        assert_eq!(size.as_megabytes(), 42.0 * 1024.0);
        assert_eq!(size.as_kilobytes(), 42.0 * 1024.0 * 1024.0);
        assert_eq!(size.as_bytes(), 42 * 1024 * 1024 * 1024);
        assert_eq!(size.to_string(), "42GB");
    }

    #[test]
    fn terabytes() {
        let size = Size::terabytes(42.0);
        assert_eq!(size.as_terabytes(), 42.0);
        assert_eq!(size.as_gigabytes(), 42.0 * 1024.0);
        assert_eq!(size.as_megabytes(), 42.0 * 1024.0 * 1024.0);
        assert_eq!(size.as_kilobytes(), 42.0 * 1024.0 * 1024.0 * 1024.0);
        assert_eq!(size.as_bytes(), 42 * 1024 * 1024 * 1024 * 1024);
        assert_eq!(size.to_string(), "42TB");
    }

    #[test]
    fn to_string() {
        let size = Size::gigabytes(4.0) + Size::terabytes(5.0);
        assert_eq!(size.to_string(), "5.004TB");
    }
}
