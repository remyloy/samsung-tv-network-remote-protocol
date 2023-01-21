use crate::protocol::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// All the possible key codes.
/// See also [Samygo](http://wiki.samygo.tv/index.php?title=Key_codes).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    #[doc = "KEY_MENU"]
    Menu,
    #[doc = "KEY_UP"]
    Up,
    #[doc = "KEY_DOWN"]
    Down,
    #[doc = "KEY_LEFT"]
    Left,
    #[doc = "KEY_RIGHT"]
    Right,
    #[doc = "KEY_3"]
    D3,
    #[doc = "KEY_VOLUP"]
    Volup,
    #[doc = "KEY_4"]
    D4,
    #[doc = "KEY_5"]
    D5,
    #[doc = "KEY_6"]
    D6,
    #[doc = "KEY_VOLDOWN"]
    Voldown,
    #[doc = "KEY_7"]
    D7,
    #[doc = "KEY_8"]
    D8,
    #[doc = "KEY_9"]
    D9,
    #[doc = "KEY_MUTE"]
    Mute,
    #[doc = "KEY_CHDOWN"]
    Chdown,
    #[doc = "KEY_0"]
    D0,
    #[doc = "KEY_CHUP"]
    Chup,
    #[doc = "KEY_PRECH"]
    Prech,
    #[doc = "KEY_GREEN"]
    Green,
    #[doc = "KEY_YELLOW"]
    Yellow,
    #[doc = "KEY_CYAN"]
    Cyan,
    #[doc = "KEY_ADDDEL"]
    Adddel,
    #[doc = "KEY_SOURCE"]
    Source,
    #[doc = "KEY_INFO"]
    Info,
    #[doc = "KEY_PIP_ONOFF"]
    PipOnoff,
    #[doc = "KEY_PIP_SWAP"]
    PipSwap,
    #[doc = "KEY_PLUS100"]
    Plus100,
    #[doc = "KEY_CAPTION"]
    Caption,
    #[doc = "KEY_PMODE"]
    Pmode,
    #[doc = "KEY_TTX_MIX"]
    TtxMix,
    #[doc = "KEY_TV"]
    Tv,
    #[doc = "KEY_PICTURE_SIZE"]
    PictureSize,
    #[doc = "KEY_AD"]
    Ad,
    #[doc = "KEY_PIP_SIZE"]
    PipSize,
    #[doc = "KEY_MAGIC_CHANNEL"]
    MagicChannel,
    #[doc = "KEY_PIP_SCAN"]
    PipScan,
    #[doc = "KEY_PIP_CHUP"]
    PipChup,
    #[doc = "KEY_PIP_CHDOWN"]
    PipChdown,
    #[doc = "KEY_DEVICE_CONNECT"]
    DeviceConnect,
    #[doc = "KEY_HELP"]
    Help,
    #[doc = "KEY_ANTENA"]
    Antena,
    #[doc = "KEY_CONVERGENCE"]
    Convergence,
    #[doc = "KEY_11"]
    D11,
    #[doc = "KEY_12"]
    D12,
    #[doc = "KEY_AUTO_PROGRAM"]
    AutoProgram,
    #[doc = "KEY_FACTORY"]
    Factory,
    #[doc = "KEY_3SPEED"]
    Speed3,
    #[doc = "KEY_RSURF"]
    Rsurf,
    #[doc = "KEY_ASPECT"]
    Aspect,
    #[doc = "KEY_TOPMENU"]
    Topmenu,
    #[doc = "KEY_GAME"]
    Game,
    #[doc = "KEY_QUICK_REPLAY"]
    QuickReplay,
    #[doc = "KEY_STILL_PICTURE"]
    StillPicture,
    #[doc = "KEY_DTV"]
    Dtv,
    #[doc = "KEY_FAVCH"]
    Favch,
    #[doc = "KEY_REWIND"]
    Rewind,
    #[doc = "KEY_STOP"]
    Stop,
    #[doc = "KEY_PLAY"]
    Play,
    #[doc = "KEY_FF"]
    Ff,
    #[doc = "KEY_REC"]
    Rec,
    #[doc = "KEY_PAUSE"]
    Pause,
    #[doc = "KEY_TOOLS"]
    Tools,
    #[doc = "KEY_INSTANT_REPLAY"]
    InstantReplay,
    #[doc = "KEY_LINK"]
    Link,
    #[doc = "KEY_FF_"]
    Ff_,
    #[doc = "KEY_GUIDE"]
    Guide,
    #[doc = "KEY_REWIND_"]
    Rewind_,
    #[doc = "KEY_ANGLE"]
    Angle,
    #[doc = "KEY_RESERVED1"]
    Reserved1,
    #[doc = "KEY_ZOOM1"]
    Zoom1,
    #[doc = "KEY_PROGRAM"]
    Program,
    #[doc = "KEY_BOOKMARK"]
    Bookmark,
    #[doc = "KEY_DISC_MENU"]
    DiscMenu,
    #[doc = "KEY_PRINT"]
    Print,
    #[doc = "KEY_RETURN"]
    Return,
    #[doc = "KEY_SUB_TITLE"]
    SubTitle,
    #[doc = "KEY_CLEAR"]
    Clear,
    #[doc = "KEY_VCHIP"]
    Vchip,
    #[doc = "KEY_REPEAT"]
    Repeat,
    #[doc = "KEY_DOOR"]
    Door,
    #[doc = "KEY_OPEN"]
    Open,
    #[doc = "KEY_WHEEL_LEFT"]
    WheelLeft,
    #[doc = "KEY_POWER"]
    Power,
    #[doc = "KEY_SLEEP"]
    Sleep,
    #[doc = "KEY_2"]
    D2,
    #[doc = "KEY_DMA"]
    Dma,
    #[doc = "KEY_TURBO"]
    Turbo,
    #[doc = "KEY_1"]
    D1,
    #[doc = "KEY_FM_RADIO"]
    FmRadio,
    #[doc = "KEY_DVR_MENU"]
    DvrMenu,
    #[doc = "KEY_MTS"]
    Mts,
    #[doc = "KEY_PCMODE"]
    Pcmode,
    #[doc = "KEY_TTX_SUBFACE"]
    TtxSubface,
    #[doc = "KEY_CH_LIST"]
    ChList,
    #[doc = "KEY_RED"]
    Red,
    #[doc = "KEY_DNIe"]
    Dnie,
    #[doc = "KEY_SRS"]
    Srs,
    #[doc = "KEY_CONVERT_AUDIO_MAINSUB"]
    ConvertAudioMainsub,
    #[doc = "KEY_MDC"]
    Mdc,
    #[doc = "KEY_SEFFECT"]
    Seffect,
    #[doc = "KEY_DVR"]
    Dvr,
    #[doc = "KEY_DTV_SIGNAL"]
    DtvSignal,
    #[doc = "KEY_LIVE"]
    Live,
    #[doc = "KEY_PERPECT_FOCUS"]
    PerpectFocus,
    #[doc = "KEY_HOME"]
    Home,
    #[doc = "KEY_ESAVING"]
    Esaving,
    #[doc = "KEY_WHEEL_RIGHT"]
    WheelRight,
    #[doc = "KEY_CONTENTS"]
    Contents,
    #[doc = "KEY_VCR_MODE"]
    VcrMode,
    #[doc = "KEY_CATV_MODE"]
    CatvMode,
    #[doc = "KEY_DSS_MODE"]
    DssMode,
    #[doc = "KEY_TV_MODE"]
    TvMode,
    #[doc = "KEY_DVD_MODE"]
    DvdMode,
    #[doc = "KEY_STB_MODE"]
    StbMode,
    #[doc = "KEY_CALLER_ID"]
    CallerId,
    #[doc = "KEY_SCALE"]
    Scale,
    #[doc = "KEY_ZOOM_MOVE"]
    ZoomMove,
    #[doc = "KEY_CLOCK_DISPLAY"]
    ClockDisplay,
    #[doc = "KEY_AV1"]
    Av1,
    #[doc = "KEY_SVIDEO1"]
    Svideo1,
    #[doc = "KEY_COMPONENT1"]
    Component1,
    #[doc = "KEY_SETUP_CLOCK_TIMER"]
    SetupClockTimer,
    #[doc = "KEY_COMPONENT2"]
    Component2,
    #[doc = "KEY_MAGIC_BRIGHT"]
    MagicBright,
    #[doc = "KEY_DVI"]
    Dvi,
    #[doc = "KEY_HDMI"]
    Hdmi,
    #[doc = "KEY_W_LINK"]
    WLink,
    #[doc = "KEY_DTV_LINK"]
    DtvLink,
    #[doc = "KEY_APP_LIST"]
    AppList,
    #[doc = "KEY_BACK_MHP"]
    BackMhp,
    #[doc = "KEY_ALT_MHP"]
    AltMhp,
    #[doc = "KEY_DNSe"]
    Dnse,
    #[doc = "KEY_RSS"]
    Rss,
    #[doc = "KEY_ENTERTAINMENT"]
    Entertainment,
    #[doc = "KEY_ID_INPUT"]
    IdInput,
    #[doc = "KEY_ID_SETUP"]
    IdSetup,
    #[doc = "KEY_ANYNET"]
    Anynet,
    #[doc = "KEY_POWEROFF"]
    Poweroff,
    #[doc = "KEY_POWERON"]
    Poweron,
    #[doc = "KEY_ANYVIEW"]
    Anyview,
    #[doc = "KEY_MS"]
    Ms,
    #[doc = "KEY_MORE"]
    More,
    #[doc = "KEY_PANNEL_POWER"]
    PannelPower,
    #[doc = "KEY_PANNEL_CHUP"]
    PannelChup,
    #[doc = "KEY_PANNEL_CHDOWN"]
    PannelChdown,
    #[doc = "KEY_PANNEL_VOLUP"]
    PannelVolup,
    #[doc = "KEY_PANNEL_VOLDOW"]
    PannelVoldow,
    #[doc = "KEY_PANNEL_ENTER"]
    PannelEnter,
    #[doc = "KEY_PANNEL_MENU"]
    PannelMenu,
    #[doc = "KEY_PANNEL_SOURCE"]
    PannelSource,
    #[doc = "KEY_AV2"]
    Av2,
    #[doc = "KEY_AV3"]
    Av3,
    #[doc = "KEY_SVIDEO2"]
    Svideo2,
    #[doc = "KEY_SVIDEO3"]
    Svideo3,
    #[doc = "KEY_ZOOM2"]
    Zoom2,
    #[doc = "KEY_PANORAMA"]
    Panorama,
    #[doc = "KEY_4_3"]
    Aspect4_3,
    #[doc = "KEY_16_9"]
    Aspect16_9,
    #[doc = "KEY_DYNAMIC"]
    Dynamic,
    #[doc = "KEY_STANDARD"]
    Standard,
    #[doc = "KEY_MOVIE1"]
    Movie1,
    #[doc = "KEY_CUSTOM"]
    Custom,
    #[doc = "KEY_AUTO_ARC_RESET"]
    AutoArcReset,
    #[doc = "KEY_AUTO_ARC_LNA_ON"]
    AutoArcLnaOn,
    #[doc = "KEY_AUTO_ARC_LNA_OFF"]
    AutoArcLnaOff,
    #[doc = "KEY_AUTO_ARC_ANYNET_MODE_OK"]
    AutoArcAnynetModeOk,
    #[doc = "KEY_AUTO_ARC_ANYNET_AUTO_START"]
    AutoArcAnynetAutoStart,
    #[doc = "KEY_AUTO_FORMAT"]
    AutoFormat,
    #[doc = "KEY_DNET"]
    Dnet,
    #[doc = "KEY_HDMI1"]
    Hdmi1,
    #[doc = "KEY_AUTO_ARC_CAPTION_ON"]
    AutoArcCaptionOn,
    #[doc = "KEY_AUTO_ARC_CAPTION_OFF"]
    AutoArcCaptionOff,
    #[doc = "KEY_AUTO_ARC_PIP_DOUBLE"]
    AutoArcPipDouble,
    #[doc = "KEY_AUTO_ARC_PIP_LARGE"]
    AutoArcPipLarge,
    #[doc = "KEY_AUTO_ARC_PIP_SMALL"]
    AutoArcPipSmall,
    #[doc = "KEY_AUTO_ARC_PIP_WIDE"]
    AutoArcPipWide,
    #[doc = "KEY_AUTO_ARC_PIP_LEFT_TOP"]
    AutoArcPipLeftTop,
    #[doc = "KEY_AUTO_ARC_PIP_RIGHT_TOP"]
    AutoArcPipRightTop,
    #[doc = "KEY_AUTO_ARC_PIP_LEFT_BOTTOM"]
    AutoArcPipLeftBottom,
    #[doc = "KEY_AUTO_ARC_PIP_RIGHT_BOTTOM"]
    AutoArcPipRightBottom,
    #[doc = "KEY_AUTO_ARC_PIP_CH_CHANGE"]
    AutoArcPipChChange,
    #[doc = "KEY_AUTO_ARC_AUTOCOLOR_SUCCESS"]
    AutoArcAutocolorSuccess,
    #[doc = "KEY_AUTO_ARC_AUTOCOLOR_FAIL"]
    AutoArcAutocolorFail,
    #[doc = "KEY_AUTO_ARC_C_FORCE_AGING"]
    AutoArcCForceAging,
    #[doc = "KEY_AUTO_ARC_USBJACK_INSPECT"]
    AutoArcUsbjackInspect,
    #[doc = "KEY_AUTO_ARC_JACK_IDENT"]
    AutoArcJackIdent,
    #[doc = "KEY_NINE_SEPERATE"]
    NineSeperate,
    #[doc = "KEY_ZOOM_IN"]
    ZoomIn,
    #[doc = "KEY_ZOOM_OUT"]
    ZoomOut,
    #[doc = "KEY_MIC"]
    Mic,
    #[doc = "KEY_HDMI2"]
    Hdmi2,
    #[doc = "KEY_HDMI3"]
    Hdmi3,
    #[doc = "KEY_AUTO_ARC_CAPTION_KOR"]
    AutoArcCaptionKor,
    #[doc = "KEY_AUTO_ARC_CAPTION_ENG"]
    AutoArcCaptionEng,
    #[doc = "KEY_AUTO_ARC_PIP_SOURCE_CHANGE"]
    AutoArcPipSourceChange,
    #[doc = "KEY_HDMI4"]
    Hdmi4,
    #[doc = "KEY_AUTO_ARC_ANTENNA_AIR"]
    AutoArcAntennaAir,
    #[doc = "KEY_AUTO_ARC_ANTENNA_CABLE"]
    AutoArcAntennaCable,
    #[doc = "KEY_AUTO_ARC_ANTENNA_SATELLITE"]
    AutoArcAntennaSatellite,
    #[doc = "KEY_EXT1"]
    Ext1,
    #[doc = "KEY_EXT2"]
    Ext2,
    #[doc = "KEY_EXT3"]
    Ext3,
    #[doc = "KEY_EXT4"]
    Ext4,
    #[doc = "KEY_EXT5"]
    Ext5,
    #[doc = "KEY_EXT6"]
    Ext6,
    #[doc = "KEY_EXT7"]
    Ext7,
    #[doc = "KEY_EXT8"]
    Ext8,
    #[doc = "KEY_EXT9"]
    Ext9,
    #[doc = "KEY_EXT10"]
    Ext10,
    #[doc = "KEY_EXT11"]
    Ext11,
    #[doc = "KEY_EXT12"]
    Ext12,
    #[doc = "KEY_EXT13"]
    Ext13,
    #[doc = "KEY_EXT14"]
    Ext14,
    #[doc = "KEY_EXT15"]
    Ext15,
    #[doc = "KEY_EXT16"]
    Ext16,
    #[doc = "KEY_EXT17"]
    Ext17,
    #[doc = "KEY_EXT18"]
    Ext18,
    #[doc = "KEY_EXT19"]
    Ext19,
    #[doc = "KEY_EXT20"]
    Ext20,
    #[doc = "KEY_EXT21"]
    Ext21,
    #[doc = "KEY_EXT22"]
    Ext22,
    #[doc = "KEY_EXT23"]
    Ext23,
    #[doc = "KEY_EXT24"]
    Ext24,
    #[doc = "KEY_EXT25"]
    Ext25,
    #[doc = "KEY_EXT26"]
    Ext26,
    #[doc = "KEY_EXT27"]
    Ext27,
    #[doc = "KEY_EXT28"]
    Ext28,
    #[doc = "KEY_EXT29"]
    Ext29,
    #[doc = "KEY_EXT30"]
    Ext30,
    #[doc = "KEY_EXT31"]
    Ext31,
    #[doc = "KEY_EXT32"]
    Ext32,
    #[doc = "KEY_EXT33"]
    Ext33,
    #[doc = "KEY_EXT34"]
    Ext34,
    #[doc = "KEY_EXT35"]
    Ext35,
    #[doc = "KEY_EXT36"]
    Ext36,
    #[doc = "KEY_EXT37"]
    Ext37,
    #[doc = "KEY_EXT38"]
    Ext38,
    #[doc = "KEY_EXT39"]
    Ext39,
    #[doc = "KEY_EXT40"]
    Ext40,
    #[doc = "KEY_EXT41"]
    Ext41,
}

static KEY_CODES: [&str; 241] = [
    "KEY_MENU",
    "KEY_UP",
    "KEY_DOWN",
    "KEY_LEFT",
    "KEY_RIGHT",
    "KEY_3",
    "KEY_VOLUP",
    "KEY_4",
    "KEY_5",
    "KEY_6",
    "KEY_VOLDOWN",
    "KEY_7",
    "KEY_8",
    "KEY_9",
    "KEY_MUTE",
    "KEY_CHDOWN",
    "KEY_0",
    "KEY_CHUP",
    "KEY_PRECH",
    "KEY_GREEN",
    "KEY_YELLOW",
    "KEY_CYAN",
    "KEY_ADDDEL",
    "KEY_SOURCE",
    "KEY_INFO",
    "KEY_PIP_ONOFF",
    "KEY_PIP_SWAP",
    "KEY_PLUS100",
    "KEY_CAPTION",
    "KEY_PMODE",
    "KEY_TTX_MIX",
    "KEY_TV",
    "KEY_PICTURE_SIZE",
    "KEY_AD",
    "KEY_PIP_SIZE",
    "KEY_MAGIC_CHANNEL",
    "KEY_PIP_SCAN",
    "KEY_PIP_CHUP",
    "KEY_PIP_CHDOWN",
    "KEY_DEVICE_CONNECT",
    "KEY_HELP",
    "KEY_ANTENA",
    "KEY_CONVERGENCE",
    "KEY_11",
    "KEY_12",
    "KEY_AUTO_PROGRAM",
    "KEY_FACTORY",
    "KEY_3SPEED",
    "KEY_RSURF",
    "KEY_ASPECT",
    "KEY_TOPMENU",
    "KEY_GAME",
    "KEY_QUICK_REPLAY",
    "KEY_STILL_PICTURE",
    "KEY_DTV",
    "KEY_FAVCH",
    "KEY_REWIND",
    "KEY_STOP",
    "KEY_PLAY",
    "KEY_FF",
    "KEY_REC",
    "KEY_PAUSE",
    "KEY_TOOLS",
    "KEY_INSTANT_REPLAY",
    "KEY_LINK",
    "KEY_FF_",
    "KEY_GUIDE",
    "KEY_REWIND_",
    "KEY_ANGLE",
    "KEY_RESERVED1",
    "KEY_ZOOM1",
    "KEY_PROGRAM",
    "KEY_BOOKMARK",
    "KEY_DISC_MENU",
    "KEY_PRINT",
    "KEY_RETURN",
    "KEY_SUB_TITLE",
    "KEY_CLEAR",
    "KEY_VCHIP",
    "KEY_REPEAT",
    "KEY_DOOR",
    "KEY_OPEN",
    "KEY_WHEEL_LEFT",
    "KEY_POWER",
    "KEY_SLEEP",
    "KEY_2",
    "KEY_DMA",
    "KEY_TURBO",
    "KEY_1",
    "KEY_FM_RADIO",
    "KEY_DVR_MENU",
    "KEY_MTS",
    "KEY_PCMODE",
    "KEY_TTX_SUBFACE",
    "KEY_CH_LIST",
    "KEY_RED",
    "KEY_DNIe",
    "KEY_SRS",
    "KEY_CONVERT_AUDIO_MAINSUB",
    "KEY_MDC",
    "KEY_SEFFECT",
    "KEY_DVR",
    "KEY_DTV_SIGNAL",
    "KEY_LIVE",
    "KEY_PERPECT_FOCUS",
    "KEY_HOME",
    "KEY_ESAVING",
    "KEY_WHEEL_RIGHT",
    "KEY_CONTENTS",
    "KEY_VCR_MODE",
    "KEY_CATV_MODE",
    "KEY_DSS_MODE",
    "KEY_TV_MODE",
    "KEY_DVD_MODE",
    "KEY_STB_MODE",
    "KEY_CALLER_ID",
    "KEY_SCALE",
    "KEY_ZOOM_MOVE",
    "KEY_CLOCK_DISPLAY",
    "KEY_AV1",
    "KEY_SVIDEO1",
    "KEY_COMPONENT1",
    "KEY_SETUP_CLOCK_TIMER",
    "KEY_COMPONENT2",
    "KEY_MAGIC_BRIGHT",
    "KEY_DVI",
    "KEY_HDMI",
    "KEY_W_LINK",
    "KEY_DTV_LINK",
    "KEY_APP_LIST",
    "KEY_BACK_MHP",
    "KEY_ALT_MHP",
    "KEY_DNSe",
    "KEY_RSS",
    "KEY_ENTERTAINMENT",
    "KEY_ID_INPUT",
    "KEY_ID_SETUP",
    "KEY_ANYNET",
    "KEY_POWEROFF",
    "KEY_POWERON",
    "KEY_ANYVIEW",
    "KEY_MS",
    "KEY_MORE",
    "KEY_PANNEL_POWER",
    "KEY_PANNEL_CHUP",
    "KEY_PANNEL_CHDOWN",
    "KEY_PANNEL_VOLUP",
    "KEY_PANNEL_VOLDOW",
    "KEY_PANNEL_ENTER",
    "KEY_PANNEL_MENU",
    "KEY_PANNEL_SOURCE",
    "KEY_AV2",
    "KEY_AV3",
    "KEY_SVIDEO2",
    "KEY_SVIDEO3",
    "KEY_ZOOM2",
    "KEY_PANORAMA",
    "KEY_4_3",
    "KEY_16_9",
    "KEY_DYNAMIC",
    "KEY_STANDARD",
    "KEY_MOVIE1",
    "KEY_CUSTOM",
    "KEY_AUTO_ARC_RESET",
    "KEY_AUTO_ARC_LNA_ON",
    "KEY_AUTO_ARC_LNA_OFF",
    "KEY_AUTO_ARC_ANYNET_MODE_OK",
    "KEY_AUTO_ARC_ANYNET_AUTO_START",
    "KEY_AUTO_FORMAT",
    "KEY_DNET",
    "KEY_HDMI1",
    "KEY_AUTO_ARC_CAPTION_ON",
    "KEY_AUTO_ARC_CAPTION_OFF",
    "KEY_AUTO_ARC_PIP_DOUBLE",
    "KEY_AUTO_ARC_PIP_LARGE",
    "KEY_AUTO_ARC_PIP_SMALL",
    "KEY_AUTO_ARC_PIP_WIDE",
    "KEY_AUTO_ARC_PIP_LEFT_TOP",
    "KEY_AUTO_ARC_PIP_RIGHT_TOP",
    "KEY_AUTO_ARC_PIP_LEFT_BOTTOM",
    "KEY_AUTO_ARC_PIP_RIGHT_BOTTOM",
    "KEY_AUTO_ARC_PIP_CH_CHANGE",
    "KEY_AUTO_ARC_AUTOCOLOR_SUCCESS",
    "KEY_AUTO_ARC_AUTOCOLOR_FAIL",
    "KEY_AUTO_ARC_C_FORCE_AGING",
    "KEY_AUTO_ARC_USBJACK_INSPECT",
    "KEY_AUTO_ARC_JACK_IDENT",
    "KEY_NINE_SEPERATE",
    "KEY_ZOOM_IN",
    "KEY_ZOOM_OUT",
    "KEY_MIC",
    "KEY_HDMI2",
    "KEY_HDMI3",
    "KEY_AUTO_ARC_CAPTION_KOR",
    "KEY_AUTO_ARC_CAPTION_ENG",
    "KEY_AUTO_ARC_PIP_SOURCE_CHANGE",
    "KEY_HDMI4",
    "KEY_AUTO_ARC_ANTENNA_AIR",
    "KEY_AUTO_ARC_ANTENNA_CABLE",
    "KEY_AUTO_ARC_ANTENNA_SATELLITE",
    "KEY_EXT1",
    "KEY_EXT2",
    "KEY_EXT3",
    "KEY_EXT4",
    "KEY_EXT5",
    "KEY_EXT6",
    "KEY_EXT7",
    "KEY_EXT8",
    "KEY_EXT9",
    "KEY_EXT10",
    "KEY_EXT11",
    "KEY_EXT12",
    "KEY_EXT13",
    "KEY_EXT14",
    "KEY_EXT15",
    "KEY_EXT16",
    "KEY_EXT17",
    "KEY_EXT18",
    "KEY_EXT19",
    "KEY_EXT20",
    "KEY_EXT21",
    "KEY_EXT22",
    "KEY_EXT23",
    "KEY_EXT24",
    "KEY_EXT25",
    "KEY_EXT26",
    "KEY_EXT27",
    "KEY_EXT28",
    "KEY_EXT29",
    "KEY_EXT30",
    "KEY_EXT31",
    "KEY_EXT32",
    "KEY_EXT33",
    "KEY_EXT34",
    "KEY_EXT35",
    "KEY_EXT36",
    "KEY_EXT37",
    "KEY_EXT38",
    "KEY_EXT39",
    "KEY_EXT40",
    "KEY_EXT41",
];

/// Sends a single key press to the TV.
///
/// This struct is used by the [`create_send_key_msg`] function of this module to
/// create the message, which needs to be sent to the TV.
///
/// The client needs to be already authenticated, before a send key request is sent.
#[derive(Debug)]
pub struct SendKeyRequest {
    /// The key to send.
    pub key_code: KeyCode,
}

/// The parsed contents of the response message to an send key request.
///
/// This struct is returned by the [`parse_send_key_rsp`] function of this module after
/// the message was successfully parsed.
#[derive(Debug)]
pub struct SendKeyResponse {
    /// The device name of the sender, e.g. iapp.samsung or unknown.livingroom.samsung.
    /// Mostly included for debugging purposes.
    pub device_name: String,
}

/// Create a send key request.
pub fn create_send_key_msg(SendKeyRequest { key_code }: SendKeyRequest) -> Result<Vec<u8>> {
    let key_code: &str = key_code.try_into()?;
    let mut payload: Vec<u8> = vec![0, 0, 0];
    extend_with_encoded_string(&mut payload, key_code);
    Ok(create_request(&payload))
}

/// Parses the send key response.
pub fn parse_send_key_rsp(buf: &[u8]) -> Result<SendKeyResponse> {
    const PAYLOAD_OFFSET: usize = 0x0F;
    let prelude: [u8; PAYLOAD_OFFSET] = buf[0..PAYLOAD_OFFSET].try_into()?;
    let reserved = prelude[0];
    assert!(reserved == 0);
    let device_name = parse_string(&prelude[1..])?;
    let kind = buf[PAYLOAD_OFFSET];
    match kind {
        0x04 => {
            let prelude: [u8; 6] = buf[PAYLOAD_OFFSET..PAYLOAD_OFFSET + 6].try_into()?;
            match prelude {
                [4, 0, 0, 0, 0, 0] => Ok(()),
                _ => Err(invalid_data(format!(
                    "unexpected payload for send key {:?}",
                    prelude
                ))),
            }
        }
        _ => Err(invalid_data(format!(
            "unexpected discriminator for payload {}",
            kind
        ))),
    }?;
    Ok(SendKeyResponse { device_name })
}

impl TryInto<&str> for KeyCode {
    type Error = std::io::Error;

    fn try_into(self) -> std::result::Result<&'static str, Self::Error> {
        KEY_CODES
            .get(self as usize)
            .copied()
            .ok_or(invalid_data(format!(
                "Key code {:?} cannot be mapped",
                self
            )))
    }
}
