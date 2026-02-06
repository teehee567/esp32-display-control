//https://milek7.pl/ddcbacklight/mccs.pdf

#[derive(Debug, Clone, Copy)]
pub enum Access {
    W,
    R,
    RW,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    C,
    NC,
    T,
}

#[derive(Debug)]
pub struct CodeDef {
    pub hex: u8,
    pub access: Access,
    pub mode: Mode,
}

macro_rules! CodeDef {
    ([
        $(
            $name:ident, $hex:expr, $access:ident, $mode:ident, $desc:expr
        ),* $(,)?
    ]) => {
        $(
            #[doc = $desc]
            pub const $name: CodeDef = CodeDef {
                hex: $hex,
                access: Access::$access,
                mode: Mode::$mode,
            };
        )*
    };
}

CodeDef!([
    CODE_PAGE,
    0x00,
    RW,
    T,
    "Code Page ID number",
    RESTORE_FACTORY_DEFAULTS,
    0x04,
    W,
    NC,
    "Restores all factory presets (including luminance/geonetry, colour, TV defaults)",
    RESTORE_FACTORY_LUMINANCE,
    0x05,
    W,
    NC,
    "Restores factory defaults for luminance and contrast",
    RESTORE_FACTORY_GEOMETRY,
    0x06,
    W,
    NC,
    "Restores factory defaults for geometry",
    RESTORE_FACTORY_COLOUR,
    0x08,
    W,
    NC,
    "Restores factory defaults for colour",
    RESTORE_FACTORY_TV,
    0x0A,
    W,
    NC,
    "Restore factory defautls for TV functions",
    // HEHEHEHAR
]);
