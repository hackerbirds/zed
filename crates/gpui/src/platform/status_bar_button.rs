use crate::SharedString;

/// Menu button for the status bar. Can either be a string or an icon.
pub enum StatusBarButton {
    /// The status bar button will be a string.
    Name(SharedString),
}
