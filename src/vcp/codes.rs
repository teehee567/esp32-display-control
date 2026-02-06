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

#[rustfmt::skip]
CodeDef!([
    CODE_PAGE, 0x00, RW, T, "Code Page ID number",
    DEGAUSS, 0x01, W, NC, "Degauss the display",
    RESTORE_FACTORY_DEFAULTS, 0x04, W, NC, "Restores all factory presets (including luminance/geonetry, colour, TV defaults)",
    RESTORE_FACTORY_LUMINANCE, 0x05, W, NC, "Restores factory defaults for luminance and contrast",
    RESTORE_FACTORY_GEOMETRY, 0x06, W, NC, "Restores factory defaults for geometry",
    RESTORE_FACTORY_COLOUR, 0x08, W, NC, "Restores factory defaults for colour",
    RESTORE_FACTORY_TV, 0x0A, W, NC, "Restore factory defautls for TV functions",
    COLOR_TEMP_INCREMENT, 0x0B, R, NC, "Color temperature increment",
    COLOR_TEMP_REQUEST, 0x0C, RW, C, "Color temperature request",
    CLOCK, 0x0E, RW, C, "Clock frequency adjustment",
    LUMINANCE, 0x10, RW, C, "Luminance/brightness level",
    FLESH_TONE_ENHANCEMENT, 0x11, RW, C, "Flesh tone enhancement",
    CONTRAST, 0x12, RW, C, "Contrast level",
    SELECT_COLOR_PRESET, 0x14, RW, NC, "Select color temperature preset",
    VIDEO_GAIN_RED, 0x16, RW, C, "Video gain (drive): red",
    USER_VISION_COMPENSATION, 0x17, RW, C, "User vision compensation",
    VIDEO_GAIN_GREEN, 0x18, RW, C, "Video gain (drive): green",
    VIDEO_GAIN_BLUE, 0x1A, RW, C, "Video gain (drive): blue",
    FOCUS, 0x1C, RW, C, "Focus adjustment",
    AUTO_SETUP, 0x1E, RW, NC, "Perform auto setup",
    AUTO_COLOR_SETUP, 0x1F, RW, NC, "Auto color setup",
    HORIZONTAL_POSITION, 0x20, RW, C, "Horizontal position",
    HORIZONTAL_SIZE, 0x22, RW, C, "Horizontal size",
    HORIZONTAL_PINCUSHION, 0x24, RW, C, "Horizontal pincushion",
    HORIZONTAL_PINCUSHION_BALANCE, 0x26, RW, C, "Horizontal pincushion balance",
    HORIZONTAL_CONVERGENCE_RB, 0x28, RW, C, "Horizontal convergence R/B",
    HORIZONTAL_CONVERGENCE_MG, 0x29, RW, C, "Horizontal convergence M/G",
    HORIZONTAL_LINEARITY, 0x2A, RW, C, "Horizontal linearity",
    HORIZONTAL_LINEARITY_BALANCE, 0x2C, RW, C, "Horizontal linearity balance",
    GREY_SCALE_EXPANSION, 0x2E, RW, NC, "Grey scale expansion",
    VERTICAL_POSITION, 0x30, RW, C, "Vertical position",
    VERTICAL_SIZE, 0x32, RW, C, "Vertical size",
    VERTICAL_PINCUSHION, 0x34, RW, C, "Vertical pincushion",
    VERTICAL_PINCUSHION_BALANCE, 0x36, RW, C, "Vertical pincushion balance",
    VERTICAL_CONVERGENCE_RB, 0x38, RW, C, "Vertical convergence R/B",
    VERTICAL_CONVERGENCE_MG, 0x39, RW, C, "Vertical convergence M/G",
    VERTICAL_LINEARITY, 0x3A, RW, C, "Vertical linearity",
    VERTICAL_LINEARITY_BALANCE, 0x3C, RW, C, "Vertical linearity balance",
    CLOCK_PHASE, 0x3E, RW, C, "Clock phase adjustment",
    HORIZONTAL_PARALLELOGRAM, 0x40, RW, C, "Horizontal parallelogram",
    VERTICAL_PARALLELOGRAM, 0x41, RW, C, "Vertical parallelogram",
    HORIZONTAL_KEYSTONE, 0x42, RW, C, "Horizontal keystone",
    VERTICAL_KEYSTONE, 0x43, RW, C, "Vertical keystone",
    ROTATION, 0x44, RW, NC, "Display rotation",
    ACTIVE_CONTROL, 0x52, R, NC, "Active control status",
    PERFORMANCE_PRESERVATION, 0x54, RW, NC, "Performance preservation mode",
    H_MOIRE, 0x56, RW, C, "Horizontal moiré adjustment",
    V_MOIRE, 0x58, RW, C, "Vertical moiré adjustment",
    SIX_AXIS_SAT_RED, 0x59, RW, C, "6-axis saturation: red",
    SIX_AXIS_SAT_YELLOW, 0x5A, RW, C, "6-axis saturation: yellow",
    SIX_AXIS_SAT_GREEN, 0x5B, RW, C, "6-axis saturation: green",
    SIX_AXIS_SAT_CYAN, 0x5C, RW, C, "6-axis saturation: cyan",
    SIX_AXIS_SAT_BLUE, 0x5D, RW, C, "6-axis saturation: blue",
    SIX_AXIS_SAT_MAGENTA, 0x5E, RW, C, "6-axis saturation: magenta",
    AUDIO_SPEAKER_VOLUME, 0x62, RW, C, "Speaker volume",
    AUDIO_SPEAKER_PAIR_SELECT, 0x63, RW, NC, "Speaker pair selection",
    AUDIO_MICROPHONE_VOLUME, 0x64, RW, C, "Microphone volume",
    AUDIO_JACK_CONNECTION_STATUS, 0x65, R, NC, "Audio jack connection status",
    BACKLIGHT_LEVEL_WHITE, 0x6B, RW, C, "Backlight level for white",
    VIDEO_BLACK_LEVEL_RED, 0x6C, RW, C, "Video black level: red",
    BACKLIGHT_LEVEL_RED, 0x6D, RW, C, "Backlight level for red",
    VIDEO_BLACK_LEVEL_GREEN, 0x6E, RW, C, "Video black level: green",
    BACKLIGHT_LEVEL_GREEN, 0x6F, RW, C, "Backlight level for green",
    VIDEO_BLACK_LEVEL_BLUE, 0x70, RW, C, "Video black level: blue",
    BACKLIGHT_LEVEL_BLUE, 0x71, RW, C, "Backlight level for blue",
    GAMMA, 0x72, RW, NC, "Gamma selection",
    ADJUST_ZOOM, 0x7C, RW, C, "Zoom adjustment",
    HORIZONTAL_MIRROR, 0x82, RW, NC, "Horizontal mirror/flip",
    VERTICAL_MIRROR, 0x84, RW, NC, "Vertical mirror/flip",
    DISPLAY_SCALING, 0x86, RW, NC, "Display scaling mode",
    VELOCITY_SCAN_MODULATION, 0x88, RW, NC, "Velocity scan modulation",
    COLOR_SATURATION, 0x8A, RW, C, "Color saturation level",
    TV_CHANNEL_UP_DOWN, 0x8B, W, NC, "TV channel up/down",
    TV_SHARPNESS, 0x8C, RW, C, "TV sharpness level",
    AUDIO_MUTE, 0x8D, RW, NC, "Audio mute control",
    TV_CONTRAST, 0x8E, RW, C, "TV contrast level",
    AUDIO_TREBLE, 0x8F, RW, C, "Audio treble level",
    HUE, 0x90, RW, C, "Hue adjustment",
    AUDIO_BASS, 0x91, RW, C, "Audio bass level",
    TV_BLACK_LEVEL, 0x92, RW, C, "TV black level/luminance",
    WINDOW_POSITION_TL_X, 0x95, RW, C, "Window position top-left X",
    WINDOW_POSITION_TL_Y, 0x96, RW, C, "Window position top-left Y",
    WINDOW_POSITION_BR_X, 0x97, RW, C, "Window position bottom-right X",
    WINDOW_POSITION_BR_Y, 0x98, RW, C, "Window position bottom-right Y",
    WINDOW_BACKGROUND, 0x9A, RW, C, "Window background color",
    SIX_AXIS_HUE_RED, 0x9B, RW, C, "6-axis hue: red",
    SIX_AXIS_HUE_YELLOW, 0x9C, RW, C, "6-axis hue: yellow",
    SIX_AXIS_HUE_GREEN, 0x9D, RW, C, "6-axis hue: green",
    SIX_AXIS_HUE_CYAN, 0x9E, RW, C, "6-axis hue: cyan",
    SIX_AXIS_HUE_BLUE, 0x9F, RW, C, "6-axis hue: blue",
    SIX_AXIS_HUE_MAGENTA, 0xA0, RW, C, "6-axis hue: magenta",
    AUTO_SETUP_ON_OFF, 0xA2, W, NC, "Turn auto setup on/off",
    WINDOW_MASK_CONTROL, 0xA4, RW, NC, "Window mask control",
    WINDOW_SELECT, 0xA5, RW, C, "Window selection",
    SCREEN_ORIENTATION, 0xAA, R, NC, "Screen orientation status",
    STORE_RESTORE_SETTINGS, 0xB0, W, NC, "Store/restore settings",
    DPVL_MONITOR_STATUS, 0xB7, R, NC, "DPVL monitor status",
    DPVL_PACKET_COUNT, 0xB8, RW, C, "DPVL packet count",
    DPVL_MONITOR_X_ORIGIN, 0xB9, RW, C, "DPVL monitor X origin",
    DPVL_MONITOR_Y_ORIGIN, 0xBA, RW, C, "DPVL monitor Y origin",
    DPVL_HEADER_ERROR_COUNT, 0xBB, RW, C, "DPVL header error count",
    DPVL_BAD_CRC_ERROR_COUNT, 0xBC, RW, C, "DPVL bad CRC error count",
    DPVL_CLIENT_ID, 0xBD, RW, C, "DPVL client ID",
    DPVL_LINK_CONTROL, 0xBE, RW, NC, "DPVL link control",
    OSD, 0xCA, RW, NC, "OSD control",
    OSD_LANGUAGE, 0xCC, RW, NC, "OSD language selection",
    STEREO_VIDEO_MODE, 0xD4, RW, NC, "Stereo video mode",
    SCAN_MODE, 0xDA, RW, NC, "Scan mode selection",
    IMAGE_MODE, 0xDB, RW, NC, "Image mode selection",
    DISPLAY_APPLICATION, 0xDC, RW, NC, "Display application mode",
]);
