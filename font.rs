use libc::{c_void, c_char, size_t};
use data_provider::CGDataProviderRef;

pub type CGFontRef = *c_void;
pub type CGGlyph = libc::c_ushort;

#[nolink]
#[link_args="-framework ApplicationServices"]
extern {
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    pub fn CGFontRelease(font: CGFontRef);
}
