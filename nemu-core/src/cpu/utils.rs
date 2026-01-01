#[derive(PartialEq, Eq)]
pub(crate) enum InterruptMode {
    Enabled,
    Disabled,
    Pending,
}