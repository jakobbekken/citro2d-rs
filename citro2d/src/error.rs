#[derive(Debug)]
pub enum Error {
    /// Failed to initialize the citro3d rendering backend.
    ///
    /// This typically means the GPU is already in use or unavailable system resources.
    C3dInitFailed,

    /// Failed to initialize citro2d.
    ///
    /// Usually occurs if [`C3dInitFailed`](Error::C3dInitFailed) was ignored and citro3d was never initialized.
    C2dInitFailed,

    /// Failed to allocate a [`TextBuf`](crate::TextBuf).
    ///
    /// The system ran out of memory for the glyph buffer.
    TextBufAllocFailed,
}
