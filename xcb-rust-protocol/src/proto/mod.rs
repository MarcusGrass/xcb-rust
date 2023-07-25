#[cfg(feature = "bigreq")]
pub mod bigreq;
#[cfg(feature = "composite")]
pub mod composite;
#[cfg(feature = "damage")]
pub mod damage;
#[cfg(feature = "dbe")]
pub mod dbe;
#[cfg(feature = "dpms")]
pub mod dpms;
#[cfg(feature = "dri2")]
pub mod dri2;
#[cfg(feature = "dri3")]
pub mod dri3;
#[cfg(feature = "ge")]
pub mod ge;
#[cfg(feature = "glx")]
pub mod glx;
#[cfg(feature = "present")]
pub mod present;
#[cfg(feature = "randr")]
pub mod randr;
#[cfg(feature = "record")]
pub mod record;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "res")]
pub mod res;
#[cfg(feature = "screensaver")]
pub mod screensaver;
#[cfg(feature = "shape")]
pub mod shape;
#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "xc_misc")]
pub mod xc_misc;
#[cfg(feature = "xevie")]
pub mod xevie;
#[cfg(feature = "xf86dri")]
pub mod xf86dri;
#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;
#[cfg(feature = "xfixes")]
pub mod xfixes;
#[cfg(feature = "xinerama")]
pub mod xinerama;
#[cfg(feature = "xinput")]
pub mod xinput;
#[cfg(feature = "xkb")]
pub mod xkb;
#[cfg(feature = "xprint")]
pub mod xprint;
#[cfg(feature = "xproto")]
pub mod xproto;
#[cfg(feature = "xselinux")]
pub mod xselinux;
#[cfg(feature = "xtest")]
pub mod xtest;
#[cfg(feature = "xv")]
pub mod xv;
#[cfg(feature = "xvmc")]
pub mod xvmc;
pub(crate) fn request_name(
    extension: Option<&str>,
    major_opcode: u8,
    minor_opcode: u16,
) -> Option<&'static str> {
    match major_opcode {
        #[cfg(feature = "xproto")]
        1 => return Some("CreateWindow"),
        #[cfg(feature = "xproto")]
        2 => return Some("ChangeWindowAttributes"),
        #[cfg(feature = "xproto")]
        3 => return Some("GetWindowAttributes"),
        #[cfg(feature = "xproto")]
        4 => return Some("DestroyWindow"),
        #[cfg(feature = "xproto")]
        5 => return Some("DestroySubwindows"),
        #[cfg(feature = "xproto")]
        6 => return Some("ChangeSaveSet"),
        #[cfg(feature = "xproto")]
        7 => return Some("ReparentWindow"),
        #[cfg(feature = "xproto")]
        8 => return Some("MapWindow"),
        #[cfg(feature = "xproto")]
        9 => return Some("MapSubwindows"),
        #[cfg(feature = "xproto")]
        10 => return Some("UnmapWindow"),
        #[cfg(feature = "xproto")]
        11 => return Some("UnmapSubwindows"),
        #[cfg(feature = "xproto")]
        12 => return Some("ConfigureWindow"),
        #[cfg(feature = "xproto")]
        13 => return Some("CirculateWindow"),
        #[cfg(feature = "xproto")]
        14 => return Some("GetGeometry"),
        #[cfg(feature = "xproto")]
        15 => return Some("QueryTree"),
        #[cfg(feature = "xproto")]
        16 => return Some("InternAtom"),
        #[cfg(feature = "xproto")]
        17 => return Some("GetAtomName"),
        #[cfg(feature = "xproto")]
        18 => return Some("ChangeProperty"),
        #[cfg(feature = "xproto")]
        19 => return Some("DeleteProperty"),
        #[cfg(feature = "xproto")]
        20 => return Some("GetProperty"),
        #[cfg(feature = "xproto")]
        21 => return Some("ListProperties"),
        #[cfg(feature = "xproto")]
        22 => return Some("SetSelectionOwner"),
        #[cfg(feature = "xproto")]
        23 => return Some("GetSelectionOwner"),
        #[cfg(feature = "xproto")]
        24 => return Some("ConvertSelection"),
        #[cfg(feature = "xproto")]
        25 => return Some("SendEvent"),
        #[cfg(feature = "xproto")]
        26 => return Some("GrabPointer"),
        #[cfg(feature = "xproto")]
        27 => return Some("UngrabPointer"),
        #[cfg(feature = "xproto")]
        28 => return Some("GrabButton"),
        #[cfg(feature = "xproto")]
        29 => return Some("UngrabButton"),
        #[cfg(feature = "xproto")]
        30 => return Some("ChangeActivePointerGrab"),
        #[cfg(feature = "xproto")]
        31 => return Some("GrabKeyboard"),
        #[cfg(feature = "xproto")]
        32 => return Some("UngrabKeyboard"),
        #[cfg(feature = "xproto")]
        33 => return Some("GrabKey"),
        #[cfg(feature = "xproto")]
        34 => return Some("UngrabKey"),
        #[cfg(feature = "xproto")]
        35 => return Some("AllowEvents"),
        #[cfg(feature = "xproto")]
        36 => return Some("GrabServer"),
        #[cfg(feature = "xproto")]
        37 => return Some("UngrabServer"),
        #[cfg(feature = "xproto")]
        38 => return Some("QueryPointer"),
        #[cfg(feature = "xproto")]
        39 => return Some("GetMotionEvents"),
        #[cfg(feature = "xproto")]
        40 => return Some("TranslateCoordinates"),
        #[cfg(feature = "xproto")]
        41 => return Some("WarpPointer"),
        #[cfg(feature = "xproto")]
        42 => return Some("SetInputFocus"),
        #[cfg(feature = "xproto")]
        43 => return Some("GetInputFocus"),
        #[cfg(feature = "xproto")]
        44 => return Some("QueryKeymap"),
        #[cfg(feature = "xproto")]
        45 => return Some("OpenFont"),
        #[cfg(feature = "xproto")]
        46 => return Some("CloseFont"),
        #[cfg(feature = "xproto")]
        47 => return Some("QueryFont"),
        #[cfg(feature = "xproto")]
        48 => return Some("QueryTextExtents"),
        #[cfg(feature = "xproto")]
        49 => return Some("ListFonts"),
        #[cfg(feature = "xproto")]
        50 => return Some("ListFontsWithInfo"),
        #[cfg(feature = "xproto")]
        51 => return Some("SetFontPath"),
        #[cfg(feature = "xproto")]
        52 => return Some("GetFontPath"),
        #[cfg(feature = "xproto")]
        53 => return Some("CreatePixmap"),
        #[cfg(feature = "xproto")]
        54 => return Some("FreePixmap"),
        #[cfg(feature = "xproto")]
        55 => return Some("CreateGC"),
        #[cfg(feature = "xproto")]
        56 => return Some("ChangeGC"),
        #[cfg(feature = "xproto")]
        57 => return Some("CopyGC"),
        #[cfg(feature = "xproto")]
        58 => return Some("SetDashes"),
        #[cfg(feature = "xproto")]
        59 => return Some("SetClipRectangles"),
        #[cfg(feature = "xproto")]
        60 => return Some("FreeGC"),
        #[cfg(feature = "xproto")]
        61 => return Some("ClearArea"),
        #[cfg(feature = "xproto")]
        62 => return Some("CopyArea"),
        #[cfg(feature = "xproto")]
        63 => return Some("CopyPlane"),
        #[cfg(feature = "xproto")]
        64 => return Some("PolyPoint"),
        #[cfg(feature = "xproto")]
        65 => return Some("PolyLine"),
        #[cfg(feature = "xproto")]
        66 => return Some("PolySegment"),
        #[cfg(feature = "xproto")]
        67 => return Some("PolyRectangle"),
        #[cfg(feature = "xproto")]
        68 => return Some("PolyArc"),
        #[cfg(feature = "xproto")]
        69 => return Some("FillPoly"),
        #[cfg(feature = "xproto")]
        70 => return Some("PolyFillRectangle"),
        #[cfg(feature = "xproto")]
        71 => return Some("PolyFillArc"),
        #[cfg(feature = "xproto")]
        72 => return Some("PutImage"),
        #[cfg(feature = "xproto")]
        73 => return Some("GetImage"),
        #[cfg(feature = "xproto")]
        74 => return Some("PolyText8"),
        #[cfg(feature = "xproto")]
        75 => return Some("PolyText16"),
        #[cfg(feature = "xproto")]
        76 => return Some("ImageText8"),
        #[cfg(feature = "xproto")]
        77 => return Some("ImageText16"),
        #[cfg(feature = "xproto")]
        78 => return Some("CreateColormap"),
        #[cfg(feature = "xproto")]
        79 => return Some("FreeColormap"),
        #[cfg(feature = "xproto")]
        80 => return Some("CopyColormapAndFree"),
        #[cfg(feature = "xproto")]
        81 => return Some("InstallColormap"),
        #[cfg(feature = "xproto")]
        82 => return Some("UninstallColormap"),
        #[cfg(feature = "xproto")]
        83 => return Some("ListInstalledColormaps"),
        #[cfg(feature = "xproto")]
        84 => return Some("AllocColor"),
        #[cfg(feature = "xproto")]
        85 => return Some("AllocNamedColor"),
        #[cfg(feature = "xproto")]
        86 => return Some("AllocColorCells"),
        #[cfg(feature = "xproto")]
        87 => return Some("AllocColorPlanes"),
        #[cfg(feature = "xproto")]
        88 => return Some("FreeColors"),
        #[cfg(feature = "xproto")]
        89 => return Some("StoreColors"),
        #[cfg(feature = "xproto")]
        90 => return Some("StoreNamedColor"),
        #[cfg(feature = "xproto")]
        91 => return Some("QueryColors"),
        #[cfg(feature = "xproto")]
        92 => return Some("LookupColor"),
        #[cfg(feature = "xproto")]
        93 => return Some("CreateCursor"),
        #[cfg(feature = "xproto")]
        94 => return Some("CreateGlyphCursor"),
        #[cfg(feature = "xproto")]
        95 => return Some("FreeCursor"),
        #[cfg(feature = "xproto")]
        96 => return Some("RecolorCursor"),
        #[cfg(feature = "xproto")]
        97 => return Some("QueryBestSize"),
        #[cfg(feature = "xproto")]
        98 => return Some("QueryExtension"),
        #[cfg(feature = "xproto")]
        99 => return Some("ListExtensions"),
        #[cfg(feature = "xproto")]
        100 => return Some("ChangeKeyboardMapping"),
        #[cfg(feature = "xproto")]
        101 => return Some("GetKeyboardMapping"),
        #[cfg(feature = "xproto")]
        102 => return Some("ChangeKeyboardControl"),
        #[cfg(feature = "xproto")]
        103 => return Some("GetKeyboardControl"),
        #[cfg(feature = "xproto")]
        104 => return Some("Bell"),
        #[cfg(feature = "xproto")]
        105 => return Some("ChangePointerControl"),
        #[cfg(feature = "xproto")]
        106 => return Some("GetPointerControl"),
        #[cfg(feature = "xproto")]
        107 => return Some("SetScreenSaver"),
        #[cfg(feature = "xproto")]
        108 => return Some("GetScreenSaver"),
        #[cfg(feature = "xproto")]
        109 => return Some("ChangeHosts"),
        #[cfg(feature = "xproto")]
        110 => return Some("ListHosts"),
        #[cfg(feature = "xproto")]
        111 => return Some("SetAccessControl"),
        #[cfg(feature = "xproto")]
        112 => return Some("SetCloseDownMode"),
        #[cfg(feature = "xproto")]
        113 => return Some("KillClient"),
        #[cfg(feature = "xproto")]
        114 => return Some("RotateProperties"),
        #[cfg(feature = "xproto")]
        115 => return Some("ForceScreenSaver"),
        #[cfg(feature = "xproto")]
        116 => return Some("SetPointerMapping"),
        #[cfg(feature = "xproto")]
        117 => return Some("GetPointerMapping"),
        #[cfg(feature = "xproto")]
        118 => return Some("SetModifierMapping"),
        #[cfg(feature = "xproto")]
        119 => return Some("GetModifierMapping"),
        #[cfg(feature = "xproto")]
        127 => return Some("NoOperation"),
        _ => {}
    }
    match (extension, minor_opcode) {
        #[cfg(feature = "bigreq")]
        (Some(bigreq::EXTENSION_NAME), 0) => Some("Enable"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 1) => Some("RedirectWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 2) => Some("RedirectSubwindows"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 3) => Some("UnredirectWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 4) => Some("UnredirectSubwindows"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 5) => Some("CreateRegionFromBorderClip"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 6) => Some("NameWindowPixmap"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 7) => Some("GetOverlayWindow"),
        #[cfg(feature = "composite")]
        (Some(composite::EXTENSION_NAME), 8) => Some("ReleaseOverlayWindow"),
        #[cfg(feature = "damage")]
        (Some(damage::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "damage")]
        (Some(damage::EXTENSION_NAME), 1) => Some("Create"),
        #[cfg(feature = "damage")]
        (Some(damage::EXTENSION_NAME), 2) => Some("Destroy"),
        #[cfg(feature = "damage")]
        (Some(damage::EXTENSION_NAME), 3) => Some("Subtract"),
        #[cfg(feature = "damage")]
        (Some(damage::EXTENSION_NAME), 4) => Some("Add"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 1) => Some("AllocateBackBuffer"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 2) => Some("DeallocateBackBuffer"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 3) => Some("SwapBuffers"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 4) => Some("BeginIdiom"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 5) => Some("EndIdiom"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 6) => Some("GetVisualInfo"),
        #[cfg(feature = "dbe")]
        (Some(dbe::EXTENSION_NAME), 7) => Some("GetBackBufferAttributes"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 0) => Some("GetVersion"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 1) => Some("Capable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 2) => Some("GetTimeouts"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 3) => Some("SetTimeouts"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 4) => Some("Enable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 5) => Some("Disable"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 6) => Some("ForceLevel"),
        #[cfg(feature = "dpms")]
        (Some(dpms::EXTENSION_NAME), 7) => Some("Info"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 1) => Some("Connect"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 2) => Some("Authenticate"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 3) => Some("CreateDrawable"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 4) => Some("DestroyDrawable"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 5) => Some("GetBuffers"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 6) => Some("CopyRegion"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 7) => Some("GetBuffersWithFormat"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 8) => Some("SwapBuffers"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 9) => Some("GetMSC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 10) => Some("WaitMSC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 11) => Some("WaitSBC"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 12) => Some("SwapInterval"),
        #[cfg(feature = "dri2")]
        (Some(dri2::EXTENSION_NAME), 13) => Some("GetParam"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 1) => Some("Open"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 2) => Some("PixmapFromBuffer"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 3) => Some("BufferFromPixmap"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 4) => Some("FenceFromFD"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 5) => Some("FDFromFence"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 6) => Some("GetSupportedModifiers"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 7) => Some("PixmapFromBuffers"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 8) => Some("BuffersFromPixmap"),
        #[cfg(feature = "dri3")]
        (Some(dri3::EXTENSION_NAME), 9) => Some("SetDRMDeviceInUse"),
        #[cfg(feature = "ge")]
        (Some(ge::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 1) => Some("Render"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 2) => Some("RenderLarge"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 3) => Some("CreateContext"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 4) => Some("DestroyContext"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 5) => Some("MakeCurrent"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 6) => Some("IsDirect"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 7) => Some("QueryVersion"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 8) => Some("WaitGL"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 9) => Some("WaitX"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 10) => Some("CopyContext"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 11) => Some("SwapBuffers"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 12) => Some("UseXFont"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 13) => Some("CreateGLXPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 14) => Some("GetVisualConfigs"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 15) => Some("DestroyGLXPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 16) => Some("VendorPrivate"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 17) => Some("VendorPrivateWithReply"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 18) => Some("QueryExtensionsString"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 19) => Some("QueryServerString"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 20) => Some("ClientInfo"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 21) => Some("GetFBConfigs"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 22) => Some("CreatePixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 23) => Some("DestroyPixmap"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 24) => Some("CreateNewContext"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 25) => Some("QueryContext"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 26) => Some("MakeContextCurrent"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 27) => Some("CreatePbuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 28) => Some("DestroyPbuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 29) => Some("GetDrawableAttributes"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 30) => Some("ChangeDrawableAttributes"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 31) => Some("CreateWindow"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 32) => Some("DeleteWindow"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 33) => Some("SetClientInfoARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 34) => Some("CreateContextAttribsARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 35) => Some("SetClientInfo2ARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 101) => Some("NewList"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 102) => Some("EndList"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 103) => Some("DeleteLists"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 104) => Some("GenLists"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 105) => Some("FeedbackBuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 106) => Some("SelectBuffer"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 107) => Some("RenderMode"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 108) => Some("Finish"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 109) => Some("PixelStoref"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 110) => Some("PixelStorei"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 111) => Some("ReadPixels"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 112) => Some("GetBooleanv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 113) => Some("GetClipPlane"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 114) => Some("GetDoublev"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 115) => Some("GetError"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 116) => Some("GetFloatv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 117) => Some("GetIntegerv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 118) => Some("GetLightfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 119) => Some("GetLightiv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 120) => Some("GetMapdv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 121) => Some("GetMapfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 122) => Some("GetMapiv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 123) => Some("GetMaterialfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 124) => Some("GetMaterialiv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 125) => Some("GetPixelMapfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 126) => Some("GetPixelMapuiv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 127) => Some("GetPixelMapusv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 128) => Some("GetPolygonStipple"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 129) => Some("GetString"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 130) => Some("GetTexEnvfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 131) => Some("GetTexEnviv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 132) => Some("GetTexGendv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 133) => Some("GetTexGenfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 134) => Some("GetTexGeniv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 135) => Some("GetTexImage"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 136) => Some("GetTexParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 137) => Some("GetTexParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 138) => Some("GetTexLevelParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 139) => Some("GetTexLevelParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 140) => Some("IsEnabled"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 141) => Some("IsList"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 142) => Some("Flush"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 143) => Some("AreTexturesResident"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 144) => Some("DeleteTextures"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 145) => Some("GenTextures"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 146) => Some("IsTexture"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 147) => Some("GetColorTable"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 148) => Some("GetColorTableParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 149) => Some("GetColorTableParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 150) => Some("GetConvolutionFilter"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 151) => Some("GetConvolutionParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 152) => Some("GetConvolutionParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 153) => Some("GetSeparableFilter"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 154) => Some("GetHistogram"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 155) => Some("GetHistogramParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 156) => Some("GetHistogramParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 157) => Some("GetMinmax"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 158) => Some("GetMinmaxParameterfv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 159) => Some("GetMinmaxParameteriv"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 160) => Some("GetCompressedTexImageARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 161) => Some("DeleteQueriesARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 162) => Some("GenQueriesARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 163) => Some("IsQueryARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 164) => Some("GetQueryivARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 165) => Some("GetQueryObjectivARB"),
        #[cfg(feature = "glx")]
        (Some(glx::EXTENSION_NAME), 166) => Some("GetQueryObjectuivARB"),
        #[cfg(feature = "present")]
        (Some(present::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "present")]
        (Some(present::EXTENSION_NAME), 1) => Some("Pixmap"),
        #[cfg(feature = "present")]
        (Some(present::EXTENSION_NAME), 2) => Some("NotifyMSC"),
        #[cfg(feature = "present")]
        (Some(present::EXTENSION_NAME), 3) => Some("SelectInput"),
        #[cfg(feature = "present")]
        (Some(present::EXTENSION_NAME), 4) => Some("QueryCapabilities"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 2) => Some("SetScreenConfig"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 4) => Some("SelectInput"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 5) => Some("GetScreenInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 6) => Some("GetScreenSizeRange"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 7) => Some("SetScreenSize"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 8) => Some("GetScreenResources"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 9) => Some("GetOutputInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 10) => Some("ListOutputProperties"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 11) => Some("QueryOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 12) => Some("ConfigureOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 13) => Some("ChangeOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 14) => Some("DeleteOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 15) => Some("GetOutputProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 16) => Some("CreateMode"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 17) => Some("DestroyMode"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 18) => Some("AddOutputMode"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 19) => Some("DeleteOutputMode"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 20) => Some("GetCrtcInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 21) => Some("SetCrtcConfig"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 22) => Some("GetCrtcGammaSize"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 23) => Some("GetCrtcGamma"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 24) => Some("SetCrtcGamma"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 25) => Some("GetScreenResourcesCurrent"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 26) => Some("SetCrtcTransform"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 27) => Some("GetCrtcTransform"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 28) => Some("GetPanning"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 29) => Some("SetPanning"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 30) => Some("SetOutputPrimary"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 31) => Some("GetOutputPrimary"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 32) => Some("GetProviders"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 33) => Some("GetProviderInfo"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 34) => Some("SetProviderOffloadSink"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 35) => Some("SetProviderOutputSource"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 36) => Some("ListProviderProperties"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 37) => Some("QueryProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 38) => Some("ConfigureProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 39) => Some("ChangeProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 40) => Some("DeleteProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 41) => Some("GetProviderProperty"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 42) => Some("GetMonitors"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 43) => Some("SetMonitor"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 44) => Some("DeleteMonitor"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 45) => Some("CreateLease"),
        #[cfg(feature = "randr")]
        (Some(randr::EXTENSION_NAME), 46) => Some("FreeLease"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 1) => Some("CreateContext"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 2) => Some("RegisterClients"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 3) => Some("UnregisterClients"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 4) => Some("GetContext"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 5) => Some("EnableContext"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 6) => Some("DisableContext"),
        #[cfg(feature = "record")]
        (Some(record::EXTENSION_NAME), 7) => Some("FreeContext"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 1) => Some("QueryPictFormats"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 2) => Some("QueryPictIndexValues"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 4) => Some("CreatePicture"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 5) => Some("ChangePicture"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 6) => Some("SetPictureClipRectangles"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 7) => Some("FreePicture"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 8) => Some("Composite"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 10) => Some("Trapezoids"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 11) => Some("Triangles"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 12) => Some("TriStrip"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 13) => Some("TriFan"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 17) => Some("CreateGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 18) => Some("ReferenceGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 19) => Some("FreeGlyphSet"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 20) => Some("AddGlyphs"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 22) => Some("FreeGlyphs"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 23) => Some("CompositeGlyphs8"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 24) => Some("CompositeGlyphs16"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 25) => Some("CompositeGlyphs32"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 26) => Some("FillRectangles"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 27) => Some("CreateCursor"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 28) => Some("SetPictureTransform"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 29) => Some("QueryFilters"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 30) => Some("SetPictureFilter"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 31) => Some("CreateAnimCursor"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 32) => Some("AddTraps"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 33) => Some("CreateSolidFill"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 34) => Some("CreateLinearGradient"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 35) => Some("CreateRadialGradient"),
        #[cfg(feature = "render")]
        (Some(render::EXTENSION_NAME), 36) => Some("CreateConicalGradient"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 1) => Some("QueryClients"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 2) => Some("QueryClientResources"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 3) => Some("QueryClientPixmapBytes"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 4) => Some("QueryClientIds"),
        #[cfg(feature = "res")]
        (Some(res::EXTENSION_NAME), 5) => Some("QueryResourceBytes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 1) => Some("QueryInfo"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 2) => Some("SelectInput"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 3) => Some("SetAttributes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 4) => Some("UnsetAttributes"),
        #[cfg(feature = "screensaver")]
        (Some(screensaver::EXTENSION_NAME), 5) => Some("Suspend"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 1) => Some("Rectangles"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 2) => Some("Mask"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 3) => Some("Combine"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 4) => Some("Offset"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 5) => Some("QueryExtents"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 6) => Some("SelectInput"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 7) => Some("InputSelected"),
        #[cfg(feature = "shape")]
        (Some(shape::EXTENSION_NAME), 8) => Some("GetRectangles"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 1) => Some("Attach"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 2) => Some("Detach"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 3) => Some("PutImage"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 4) => Some("GetImage"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 5) => Some("CreatePixmap"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 6) => Some("AttachFd"),
        #[cfg(feature = "shm")]
        (Some(shm::EXTENSION_NAME), 7) => Some("CreateSegment"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 0) => Some("Initialize"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 1) => Some("ListSystemCounters"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 2) => Some("CreateCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 3) => Some("SetCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 4) => Some("ChangeCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 5) => Some("QueryCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 6) => Some("DestroyCounter"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 7) => Some("Await"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 8) => Some("CreateAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 9) => Some("ChangeAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 10) => Some("QueryAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 11) => Some("DestroyAlarm"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 12) => Some("SetPriority"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 13) => Some("GetPriority"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 14) => Some("CreateFence"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 15) => Some("TriggerFence"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 16) => Some("ResetFence"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 17) => Some("DestroyFence"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 18) => Some("QueryFence"),
        #[cfg(feature = "sync")]
        (Some(sync::EXTENSION_NAME), 19) => Some("AwaitFence"),
        #[cfg(feature = "xc_misc")]
        (Some(xc_misc::EXTENSION_NAME), 0) => Some("GetVersion"),
        #[cfg(feature = "xc_misc")]
        (Some(xc_misc::EXTENSION_NAME), 1) => Some("GetXIDRange"),
        #[cfg(feature = "xc_misc")]
        (Some(xc_misc::EXTENSION_NAME), 2) => Some("GetXIDList"),
        #[cfg(feature = "xevie")]
        (Some(xevie::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xevie")]
        (Some(xevie::EXTENSION_NAME), 1) => Some("Start"),
        #[cfg(feature = "xevie")]
        (Some(xevie::EXTENSION_NAME), 2) => Some("End"),
        #[cfg(feature = "xevie")]
        (Some(xevie::EXTENSION_NAME), 3) => Some("Send"),
        #[cfg(feature = "xevie")]
        (Some(xevie::EXTENSION_NAME), 4) => Some("SelectInput"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 1) => Some("QueryDirectRenderingCapable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 2) => Some("OpenConnection"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 3) => Some("CloseConnection"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 4) => Some("GetClientDriverName"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 5) => Some("CreateContext"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 6) => Some("DestroyContext"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 7) => Some("CreateDrawable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 8) => Some("DestroyDrawable"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 9) => Some("GetDrawableInfo"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 10) => Some("GetDeviceInfo"),
        #[cfg(feature = "xf86dri")]
        (Some(xf86dri::EXTENSION_NAME), 11) => Some("AuthConnection"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 1) => Some("GetModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 2) => Some("ModModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 3) => Some("SwitchMode"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 4) => Some("GetMonitor"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 5) => Some("LockModeSwitch"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 6) => Some("GetAllModeLines"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 7) => Some("AddModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 8) => Some("DeleteModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 9) => Some("ValidateModeLine"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 10) => Some("SwitchToMode"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 11) => Some("GetViewPort"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 12) => Some("SetViewPort"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 13) => Some("GetDotClocks"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 14) => Some("SetClientVersion"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 15) => Some("SetGamma"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 16) => Some("GetGamma"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 17) => Some("GetGammaRamp"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 18) => Some("SetGammaRamp"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 19) => Some("GetGammaRampSize"),
        #[cfg(feature = "xf86vidmode")]
        (Some(xf86vidmode::EXTENSION_NAME), 20) => Some("GetPermissions"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 1) => Some("ChangeSaveSet"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 2) => Some("SelectSelectionInput"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 3) => Some("SelectCursorInput"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 4) => Some("GetCursorImage"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 5) => Some("CreateRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 6) => Some("CreateRegionFromBitmap"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 7) => Some("CreateRegionFromWindow"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 8) => Some("CreateRegionFromGC"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 9) => Some("CreateRegionFromPicture"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 10) => Some("DestroyRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 11) => Some("SetRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 12) => Some("CopyRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 13) => Some("UnionRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 14) => Some("IntersectRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 15) => Some("SubtractRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 16) => Some("InvertRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 17) => Some("TranslateRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 18) => Some("RegionExtents"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 19) => Some("FetchRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 20) => Some("SetGCClipRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 21) => Some("SetWindowShapeRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 22) => Some("SetPictureClipRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 23) => Some("SetCursorName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 24) => Some("GetCursorName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 25) => Some("GetCursorImageAndName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 26) => Some("ChangeCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 27) => Some("ChangeCursorByName"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 28) => Some("ExpandRegion"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 29) => Some("HideCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 30) => Some("ShowCursor"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 31) => Some("CreatePointerBarrier"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 32) => Some("DeletePointerBarrier"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 33) => Some("SetClientDisconnectMode"),
        #[cfg(feature = "xfixes")]
        (Some(xfixes::EXTENSION_NAME), 34) => Some("GetClientDisconnectMode"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 1) => Some("GetState"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 2) => Some("GetScreenCount"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 3) => Some("GetScreenSize"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 4) => Some("IsActive"),
        #[cfg(feature = "xinerama")]
        (Some(xinerama::EXTENSION_NAME), 5) => Some("QueryScreens"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 1) => Some("GetExtensionVersion"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 2) => Some("ListInputDevices"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 3) => Some("OpenDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 4) => Some("CloseDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 5) => Some("SetDeviceMode"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 6) => Some("SelectExtensionEvent"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 7) => Some("GetSelectedExtensionEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 8) => Some("ChangeDeviceDontPropagateList"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 9) => Some("GetDeviceDontPropagateList"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 10) => Some("GetDeviceMotionEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 11) => Some("ChangeKeyboardDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 12) => Some("ChangePointerDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 13) => Some("GrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 14) => Some("UngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 15) => Some("GrabDeviceKey"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 16) => Some("UngrabDeviceKey"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 17) => Some("GrabDeviceButton"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 18) => Some("UngrabDeviceButton"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 19) => Some("AllowDeviceEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 20) => Some("GetDeviceFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 21) => Some("SetDeviceFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 22) => Some("GetFeedbackControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 23) => Some("ChangeFeedbackControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 24) => Some("GetDeviceKeyMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 25) => Some("ChangeDeviceKeyMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 26) => Some("GetDeviceModifierMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 27) => Some("SetDeviceModifierMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 28) => Some("GetDeviceButtonMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 29) => Some("SetDeviceButtonMapping"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 30) => Some("QueryDeviceState"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 31) => Some("SendExtensionEvent"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 32) => Some("DeviceBell"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 33) => Some("SetDeviceValuators"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 34) => Some("GetDeviceControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 35) => Some("ChangeDeviceControl"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 36) => Some("ListDeviceProperties"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 37) => Some("ChangeDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 38) => Some("DeleteDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 39) => Some("GetDeviceProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 40) => Some("XIQueryPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 41) => Some("XIWarpPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 42) => Some("XIChangeCursor"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 43) => Some("XIChangeHierarchy"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 44) => Some("XISetClientPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 45) => Some("XIGetClientPointer"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 46) => Some("XISelectEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 47) => Some("XIQueryVersion"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 48) => Some("XIQueryDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 49) => Some("XISetFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 50) => Some("XIGetFocus"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 51) => Some("XIGrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 52) => Some("XIUngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 53) => Some("XIAllowEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 54) => Some("XIPassiveGrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 55) => Some("XIPassiveUngrabDevice"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 56) => Some("XIListProperties"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 57) => Some("XIChangeProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 58) => Some("XIDeleteProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 59) => Some("XIGetProperty"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 60) => Some("XIGetSelectedEvents"),
        #[cfg(feature = "xinput")]
        (Some(xinput::EXTENSION_NAME), 61) => Some("XIBarrierReleasePointer"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 0) => Some("UseExtension"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 1) => Some("SelectEvents"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 3) => Some("Bell"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 4) => Some("GetState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 5) => Some("LatchLockState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 6) => Some("GetControls"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 7) => Some("SetControls"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 8) => Some("GetMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 9) => Some("SetMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 10) => Some("GetCompatMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 11) => Some("SetCompatMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 12) => Some("GetIndicatorState"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 13) => Some("GetIndicatorMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 14) => Some("SetIndicatorMap"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 15) => Some("GetNamedIndicator"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 16) => Some("SetNamedIndicator"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 17) => Some("GetNames"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 18) => Some("SetNames"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 21) => Some("PerClientFlags"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 22) => Some("ListComponents"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 23) => Some("GetKbdByName"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 24) => Some("GetDeviceInfo"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 25) => Some("SetDeviceInfo"),
        #[cfg(feature = "xkb")]
        (Some(xkb::EXTENSION_NAME), 101) => Some("SetDebuggingFlags"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 0) => Some("PrintQueryVersion"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 1) => Some("PrintGetPrinterList"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 2) => Some("CreateContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 3) => Some("PrintSetContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 4) => Some("PrintGetContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 5) => Some("PrintDestroyContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 6) => Some("PrintGetScreenOfContext"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 7) => Some("PrintStartJob"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 8) => Some("PrintEndJob"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 9) => Some("PrintStartDoc"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 10) => Some("PrintEndDoc"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 11) => Some("PrintPutDocumentData"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 12) => Some("PrintGetDocumentData"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 13) => Some("PrintStartPage"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 14) => Some("PrintEndPage"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 15) => Some("PrintSelectInput"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 16) => Some("PrintInputSelected"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 17) => Some("PrintGetAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 18) => Some("PrintSetAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 19) => Some("PrintGetOneAttributes"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 20) => Some("PrintRehashPrinterList"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 21) => Some("PrintGetPageDimensions"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 22) => Some("PrintQueryScreens"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 23) => Some("PrintSetImageResolution"),
        #[cfg(feature = "xprint")]
        (Some(xprint::EXTENSION_NAME), 24) => Some("PrintGetImageResolution"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 1) => Some("SetDeviceCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 2) => Some("GetDeviceCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 3) => Some("SetDeviceContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 4) => Some("GetDeviceContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 5) => Some("SetWindowCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 6) => Some("GetWindowCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 7) => Some("GetWindowContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 8) => Some("SetPropertyCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 9) => Some("GetPropertyCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 10) => Some("SetPropertyUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 11) => Some("GetPropertyUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 12) => Some("GetPropertyContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 13) => Some("GetPropertyDataContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 14) => Some("ListProperties"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 15) => Some("SetSelectionCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 16) => Some("GetSelectionCreateContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 17) => Some("SetSelectionUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 18) => Some("GetSelectionUseContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 19) => Some("GetSelectionContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 20) => Some("GetSelectionDataContext"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 21) => Some("ListSelections"),
        #[cfg(feature = "xselinux")]
        (Some(xselinux::EXTENSION_NAME), 22) => Some("GetClientContext"),
        #[cfg(feature = "xtest")]
        (Some(xtest::EXTENSION_NAME), 0) => Some("GetVersion"),
        #[cfg(feature = "xtest")]
        (Some(xtest::EXTENSION_NAME), 1) => Some("CompareCursor"),
        #[cfg(feature = "xtest")]
        (Some(xtest::EXTENSION_NAME), 2) => Some("FakeInput"),
        #[cfg(feature = "xtest")]
        (Some(xtest::EXTENSION_NAME), 3) => Some("GrabControl"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 0) => Some("QueryExtension"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 1) => Some("QueryAdaptors"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 2) => Some("QueryEncodings"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 3) => Some("GrabPort"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 4) => Some("UngrabPort"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 5) => Some("PutVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 6) => Some("PutStill"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 7) => Some("GetVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 8) => Some("GetStill"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 9) => Some("StopVideo"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 10) => Some("SelectVideoNotify"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 11) => Some("SelectPortNotify"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 12) => Some("QueryBestSize"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 13) => Some("SetPortAttribute"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 14) => Some("GetPortAttribute"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 15) => Some("QueryPortAttributes"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 16) => Some("ListImageFormats"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 17) => Some("QueryImageAttributes"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 18) => Some("PutImage"),
        #[cfg(feature = "xv")]
        (Some(xv::EXTENSION_NAME), 19) => Some("ShmPutImage"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 0) => Some("QueryVersion"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 1) => Some("ListSurfaceTypes"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 2) => Some("CreateContext"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 3) => Some("DestroyContext"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 4) => Some("CreateSurface"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 5) => Some("DestroySurface"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 6) => Some("CreateSubpicture"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 7) => Some("DestroySubpicture"),
        #[cfg(feature = "xvmc")]
        (Some(xvmc::EXTENSION_NAME), 8) => Some("ListSubpictureTypes"),
        _ => None,
    }
}
#[inline]
pub fn find_extension(name: &str) -> Option<&'static str> {
    match name {
        #[cfg(feature = "bigreq")]
        bigreq::EXTENSION_NAME => Some(bigreq::EXTENSION_NAME),
        #[cfg(feature = "composite")]
        composite::EXTENSION_NAME => Some(composite::EXTENSION_NAME),
        #[cfg(feature = "damage")]
        damage::EXTENSION_NAME => Some(damage::EXTENSION_NAME),
        #[cfg(feature = "dbe")]
        dbe::EXTENSION_NAME => Some(dbe::EXTENSION_NAME),
        #[cfg(feature = "dpms")]
        dpms::EXTENSION_NAME => Some(dpms::EXTENSION_NAME),
        #[cfg(feature = "dri2")]
        dri2::EXTENSION_NAME => Some(dri2::EXTENSION_NAME),
        #[cfg(feature = "dri3")]
        dri3::EXTENSION_NAME => Some(dri3::EXTENSION_NAME),
        #[cfg(feature = "ge")]
        ge::EXTENSION_NAME => Some(ge::EXTENSION_NAME),
        #[cfg(feature = "glx")]
        glx::EXTENSION_NAME => Some(glx::EXTENSION_NAME),
        #[cfg(feature = "present")]
        present::EXTENSION_NAME => Some(present::EXTENSION_NAME),
        #[cfg(feature = "randr")]
        randr::EXTENSION_NAME => Some(randr::EXTENSION_NAME),
        #[cfg(feature = "record")]
        record::EXTENSION_NAME => Some(record::EXTENSION_NAME),
        #[cfg(feature = "render")]
        render::EXTENSION_NAME => Some(render::EXTENSION_NAME),
        #[cfg(feature = "res")]
        res::EXTENSION_NAME => Some(res::EXTENSION_NAME),
        #[cfg(feature = "screensaver")]
        screensaver::EXTENSION_NAME => Some(screensaver::EXTENSION_NAME),
        #[cfg(feature = "shape")]
        shape::EXTENSION_NAME => Some(shape::EXTENSION_NAME),
        #[cfg(feature = "shm")]
        shm::EXTENSION_NAME => Some(shm::EXTENSION_NAME),
        #[cfg(feature = "sync")]
        sync::EXTENSION_NAME => Some(sync::EXTENSION_NAME),
        #[cfg(feature = "xc_misc")]
        xc_misc::EXTENSION_NAME => Some(xc_misc::EXTENSION_NAME),
        #[cfg(feature = "xevie")]
        xevie::EXTENSION_NAME => Some(xevie::EXTENSION_NAME),
        #[cfg(feature = "xf86dri")]
        xf86dri::EXTENSION_NAME => Some(xf86dri::EXTENSION_NAME),
        #[cfg(feature = "xf86vidmode")]
        xf86vidmode::EXTENSION_NAME => Some(xf86vidmode::EXTENSION_NAME),
        #[cfg(feature = "xfixes")]
        xfixes::EXTENSION_NAME => Some(xfixes::EXTENSION_NAME),
        #[cfg(feature = "xinerama")]
        xinerama::EXTENSION_NAME => Some(xinerama::EXTENSION_NAME),
        #[cfg(feature = "xinput")]
        xinput::EXTENSION_NAME => Some(xinput::EXTENSION_NAME),
        #[cfg(feature = "xkb")]
        xkb::EXTENSION_NAME => Some(xkb::EXTENSION_NAME),
        #[cfg(feature = "xprint")]
        xprint::EXTENSION_NAME => Some(xprint::EXTENSION_NAME),
        #[cfg(feature = "xselinux")]
        xselinux::EXTENSION_NAME => Some(xselinux::EXTENSION_NAME),
        #[cfg(feature = "xtest")]
        xtest::EXTENSION_NAME => Some(xtest::EXTENSION_NAME),
        #[cfg(feature = "xv")]
        xv::EXTENSION_NAME => Some(xv::EXTENSION_NAME),
        #[cfg(feature = "xvmc")]
        xvmc::EXTENSION_NAME => Some(xvmc::EXTENSION_NAME),
        _ => None,
    }
}
#[derive(Debug, Clone)]
pub enum Event {
    Unknown(alloc::vec::Vec<u8>),
    #[cfg(feature = "xproto")]
    KeyPressEvent(xproto::KeyPressEvent),
    #[cfg(feature = "xproto")]
    KeyReleaseEvent(xproto::KeyReleaseEvent),
    #[cfg(feature = "xproto")]
    ButtonPressEvent(xproto::ButtonPressEvent),
    #[cfg(feature = "xproto")]
    ButtonReleaseEvent(xproto::ButtonReleaseEvent),
    #[cfg(feature = "xproto")]
    MotionNotifyEvent(xproto::MotionNotifyEvent),
    #[cfg(feature = "xproto")]
    EnterNotifyEvent(xproto::EnterNotifyEvent),
    #[cfg(feature = "xproto")]
    LeaveNotifyEvent(xproto::LeaveNotifyEvent),
    #[cfg(feature = "xproto")]
    FocusInEvent(xproto::FocusInEvent),
    #[cfg(feature = "xproto")]
    FocusOutEvent(xproto::FocusOutEvent),
    #[cfg(feature = "xproto")]
    KeymapNotifyEvent(xproto::KeymapNotifyEvent),
    #[cfg(feature = "xproto")]
    ExposeEvent(xproto::ExposeEvent),
    #[cfg(feature = "xproto")]
    GraphicsExposureEvent(xproto::GraphicsExposureEvent),
    #[cfg(feature = "xproto")]
    NoExposureEvent(xproto::NoExposureEvent),
    #[cfg(feature = "xproto")]
    VisibilityNotifyEvent(xproto::VisibilityNotifyEvent),
    #[cfg(feature = "xproto")]
    CreateNotifyEvent(xproto::CreateNotifyEvent),
    #[cfg(feature = "xproto")]
    DestroyNotifyEvent(xproto::DestroyNotifyEvent),
    #[cfg(feature = "xproto")]
    UnmapNotifyEvent(xproto::UnmapNotifyEvent),
    #[cfg(feature = "xproto")]
    MapNotifyEvent(xproto::MapNotifyEvent),
    #[cfg(feature = "xproto")]
    MapRequestEvent(xproto::MapRequestEvent),
    #[cfg(feature = "xproto")]
    ReparentNotifyEvent(xproto::ReparentNotifyEvent),
    #[cfg(feature = "xproto")]
    ConfigureNotifyEvent(xproto::ConfigureNotifyEvent),
    #[cfg(feature = "xproto")]
    ConfigureRequestEvent(xproto::ConfigureRequestEvent),
    #[cfg(feature = "xproto")]
    GravityNotifyEvent(xproto::GravityNotifyEvent),
    #[cfg(feature = "xproto")]
    ResizeRequestEvent(xproto::ResizeRequestEvent),
    #[cfg(feature = "xproto")]
    CirculateNotifyEvent(xproto::CirculateNotifyEvent),
    #[cfg(feature = "xproto")]
    CirculateRequestEvent(xproto::CirculateRequestEvent),
    #[cfg(feature = "xproto")]
    PropertyNotifyEvent(xproto::PropertyNotifyEvent),
    #[cfg(feature = "xproto")]
    SelectionClearEvent(xproto::SelectionClearEvent),
    #[cfg(feature = "xproto")]
    SelectionRequestEvent(xproto::SelectionRequestEvent),
    #[cfg(feature = "xproto")]
    SelectionNotifyEvent(xproto::SelectionNotifyEvent),
    #[cfg(feature = "xproto")]
    ColormapNotifyEvent(xproto::ColormapNotifyEvent),
    #[cfg(feature = "xproto")]
    ClientMessageEvent(xproto::ClientMessageEvent),
    #[cfg(feature = "xproto")]
    MappingNotifyEvent(xproto::MappingNotifyEvent),
    #[cfg(feature = "damage")]
    DamageNotifyEvent(damage::NotifyEvent),
    #[cfg(feature = "dri2")]
    Dri2BufferSwapCompleteEvent(dri2::BufferSwapCompleteEvent),
    #[cfg(feature = "dri2")]
    Dri2InvalidateBuffersEvent(dri2::InvalidateBuffersEvent),
    #[cfg(feature = "glx")]
    GlxPbufferClobberEvent(glx::PbufferClobberEvent),
    #[cfg(feature = "glx")]
    GlxBufferSwapCompleteEvent(glx::BufferSwapCompleteEvent),
    #[cfg(feature = "present")]
    PresentGenericEvent(present::GenericEvent),
    #[cfg(feature = "randr")]
    RandrScreenChangeNotifyEvent(randr::ScreenChangeNotifyEvent),
    #[cfg(feature = "randr")]
    RandrNotifyEvent(randr::NotifyEvent),
    #[cfg(feature = "screensaver")]
    ScreensaverNotifyEvent(screensaver::NotifyEvent),
    #[cfg(feature = "shape")]
    ShapeNotifyEvent(shape::NotifyEvent),
    #[cfg(feature = "shm")]
    ShmCompletionEvent(shm::CompletionEvent),
    #[cfg(feature = "sync")]
    SyncCounterNotifyEvent(sync::CounterNotifyEvent),
    #[cfg(feature = "sync")]
    SyncAlarmNotifyEvent(sync::AlarmNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesSelectionNotifyEvent(xfixes::SelectionNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesCursorNotifyEvent(xfixes::CursorNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceValuatorEvent(xinput::DeviceValuatorEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyPressEvent(xinput::DeviceKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyReleaseEvent(xinput::DeviceKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonPressEvent(xinput::DeviceButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonReleaseEvent(xinput::DeviceButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMotionNotifyEvent(xinput::DeviceMotionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusInEvent(xinput::DeviceFocusInEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusOutEvent(xinput::DeviceFocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputProximityInEvent(xinput::ProximityInEvent),
    #[cfg(feature = "xinput")]
    XinputProximityOutEvent(xinput::ProximityOutEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceStateNotifyEvent(xinput::DeviceStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMappingNotifyEvent(xinput::DeviceMappingNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceNotifyEvent(xinput::ChangeDeviceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyStateNotifyEvent(xinput::DeviceKeyStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonStateNotifyEvent(xinput::DeviceButtonStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePresenceNotifyEvent(xinput::DevicePresenceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePropertyNotifyEvent(xinput::DevicePropertyNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNewKeyboardNotifyEvent(xkb::NewKeyboardNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbMapNotifyEvent(xkb::MapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbStateNotifyEvent(xkb::StateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbControlsNotifyEvent(xkb::ControlsNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorStateNotifyEvent(xkb::IndicatorStateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorMapNotifyEvent(xkb::IndicatorMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNamesNotifyEvent(xkb::NamesNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbCompatMapNotifyEvent(xkb::CompatMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbBellNotifyEvent(xkb::BellNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbActionMessageEvent(xkb::ActionMessageEvent),
    #[cfg(feature = "xkb")]
    XkbAccessXNotifyEvent(xkb::AccessXNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbExtensionDeviceNotifyEvent(xkb::ExtensionDeviceNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintNotifyEvent(xprint::NotifyEvent),
    #[cfg(feature = "xprint")]
    XprintAttributNotifyEvent(xprint::AttributNotifyEvent),
    #[cfg(feature = "xv")]
    XvVideoNotifyEvent(xv::VideoNotifyEvent),
    #[cfg(feature = "xv")]
    XvPortNotifyEvent(xv::PortNotifyEvent),
}
impl Event {
    pub fn from_bytes<E>(raw: &[u8], ext_provider: &E) -> crate::error::Result<Event>
    where
        E: crate::util::ExtensionInfoProvider,
    {
        use crate::util::FixedLengthFromBytes;
        let opcode = crate::util::response_type(raw)?;
        match opcode {
            #[cfg(feature = "xproto")]
            2 => return Ok(Self::KeyPressEvent(xproto::KeyPressEvent::from_bytes(raw)?)),
            #[cfg(feature = "xproto")]
            3 => {
                return Ok(Self::KeyReleaseEvent(xproto::KeyReleaseEvent::from_bytes(
                    raw,
                )?))
            }
            #[cfg(feature = "xproto")]
            4 => {
                return Ok(Self::ButtonPressEvent(
                    xproto::ButtonPressEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            5 => {
                return Ok(Self::ButtonReleaseEvent(
                    xproto::ButtonReleaseEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            6 => {
                return Ok(Self::MotionNotifyEvent(
                    xproto::MotionNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            7 => {
                return Ok(Self::EnterNotifyEvent(
                    xproto::EnterNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            8 => {
                return Ok(Self::LeaveNotifyEvent(
                    xproto::LeaveNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            9 => return Ok(Self::FocusInEvent(xproto::FocusInEvent::from_bytes(raw)?)),
            #[cfg(feature = "xproto")]
            10 => return Ok(Self::FocusOutEvent(xproto::FocusOutEvent::from_bytes(raw)?)),
            #[cfg(feature = "xproto")]
            11 => {
                return Ok(Self::KeymapNotifyEvent(
                    xproto::KeymapNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            12 => return Ok(Self::ExposeEvent(xproto::ExposeEvent::from_bytes(raw)?)),
            #[cfg(feature = "xproto")]
            13 => {
                return Ok(Self::GraphicsExposureEvent(
                    xproto::GraphicsExposureEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            14 => {
                return Ok(Self::NoExposureEvent(xproto::NoExposureEvent::from_bytes(
                    raw,
                )?))
            }
            #[cfg(feature = "xproto")]
            15 => {
                return Ok(Self::VisibilityNotifyEvent(
                    xproto::VisibilityNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            16 => {
                return Ok(Self::CreateNotifyEvent(
                    xproto::CreateNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            17 => {
                return Ok(Self::DestroyNotifyEvent(
                    xproto::DestroyNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            18 => {
                return Ok(Self::UnmapNotifyEvent(
                    xproto::UnmapNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            19 => {
                return Ok(Self::MapNotifyEvent(xproto::MapNotifyEvent::from_bytes(
                    raw,
                )?))
            }
            #[cfg(feature = "xproto")]
            20 => {
                return Ok(Self::MapRequestEvent(xproto::MapRequestEvent::from_bytes(
                    raw,
                )?))
            }
            #[cfg(feature = "xproto")]
            21 => {
                return Ok(Self::ReparentNotifyEvent(
                    xproto::ReparentNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            22 => {
                return Ok(Self::ConfigureNotifyEvent(
                    xproto::ConfigureNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            23 => {
                return Ok(Self::ConfigureRequestEvent(
                    xproto::ConfigureRequestEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            24 => {
                return Ok(Self::GravityNotifyEvent(
                    xproto::GravityNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            25 => {
                return Ok(Self::ResizeRequestEvent(
                    xproto::ResizeRequestEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            26 => {
                return Ok(Self::CirculateNotifyEvent(
                    xproto::CirculateNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            27 => {
                return Ok(Self::CirculateRequestEvent(
                    xproto::CirculateRequestEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            28 => {
                return Ok(Self::PropertyNotifyEvent(
                    xproto::PropertyNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            29 => {
                return Ok(Self::SelectionClearEvent(
                    xproto::SelectionClearEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            30 => {
                return Ok(Self::SelectionRequestEvent(
                    xproto::SelectionRequestEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            31 => {
                return Ok(Self::SelectionNotifyEvent(
                    xproto::SelectionNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            32 => {
                return Ok(Self::ColormapNotifyEvent(
                    xproto::ColormapNotifyEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            33 => {
                return Ok(Self::ClientMessageEvent(
                    xproto::ClientMessageEvent::from_bytes(raw)?,
                ))
            }
            #[cfg(feature = "xproto")]
            34 => {
                return Ok(Self::MappingNotifyEvent(
                    xproto::MappingNotifyEvent::from_bytes(raw)?,
                ))
            }
            _ => {}
        }
        let ext_info = ext_provider.get_from_event_code(opcode);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::DamageNotifyEvent(damage::NotifyEvent::from_bytes(
                    raw,
                )?)),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "dri2")]
            Some((dri2::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::Dri2BufferSwapCompleteEvent(
                    dri2::BufferSwapCompleteEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::Dri2InvalidateBuffersEvent(
                    dri2::InvalidateBuffersEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "glx")]
            Some((glx::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::GlxPbufferClobberEvent(
                    glx::PbufferClobberEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::GlxBufferSwapCompleteEvent(
                    glx::BufferSwapCompleteEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "present")]
            Some((present::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::PresentGenericEvent(
                    present::GenericEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "randr")]
            Some((randr::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::RandrScreenChangeNotifyEvent(
                    randr::ScreenChangeNotifyEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::RandrNotifyEvent(randr::NotifyEvent::from_bytes(raw)?)),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "screensaver")]
            Some((screensaver::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::ScreensaverNotifyEvent(
                    screensaver::NotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "shape")]
            Some((shape::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::ShapeNotifyEvent(shape::NotifyEvent::from_bytes(raw)?)),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "shm")]
            Some((shm::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::ShmCompletionEvent(shm::CompletionEvent::from_bytes(
                    raw,
                )?)),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "sync")]
            Some((sync::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::SyncCounterNotifyEvent(
                    sync::CounterNotifyEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::SyncAlarmNotifyEvent(
                    sync::AlarmNotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "xfixes")]
            Some((xfixes::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::XfixesSelectionNotifyEvent(
                    xfixes::SelectionNotifyEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::XfixesCursorNotifyEvent(
                    xfixes::CursorNotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "xinput")]
            Some((xinput::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::XinputDeviceValuatorEvent(
                    xinput::DeviceValuatorEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::XinputDeviceKeyPressEvent(
                    xinput::DeviceKeyPressEvent::from_bytes(raw)?,
                )),
                2 => Ok(Self::XinputDeviceKeyReleaseEvent(
                    xinput::DeviceKeyReleaseEvent::from_bytes(raw)?,
                )),
                3 => Ok(Self::XinputDeviceButtonPressEvent(
                    xinput::DeviceButtonPressEvent::from_bytes(raw)?,
                )),
                4 => Ok(Self::XinputDeviceButtonReleaseEvent(
                    xinput::DeviceButtonReleaseEvent::from_bytes(raw)?,
                )),
                5 => Ok(Self::XinputDeviceMotionNotifyEvent(
                    xinput::DeviceMotionNotifyEvent::from_bytes(raw)?,
                )),
                6 => Ok(Self::XinputDeviceFocusInEvent(
                    xinput::DeviceFocusInEvent::from_bytes(raw)?,
                )),
                7 => Ok(Self::XinputDeviceFocusOutEvent(
                    xinput::DeviceFocusOutEvent::from_bytes(raw)?,
                )),
                8 => Ok(Self::XinputProximityInEvent(
                    xinput::ProximityInEvent::from_bytes(raw)?,
                )),
                9 => Ok(Self::XinputProximityOutEvent(
                    xinput::ProximityOutEvent::from_bytes(raw)?,
                )),
                10 => Ok(Self::XinputDeviceStateNotifyEvent(
                    xinput::DeviceStateNotifyEvent::from_bytes(raw)?,
                )),
                11 => Ok(Self::XinputDeviceMappingNotifyEvent(
                    xinput::DeviceMappingNotifyEvent::from_bytes(raw)?,
                )),
                12 => Ok(Self::XinputChangeDeviceNotifyEvent(
                    xinput::ChangeDeviceNotifyEvent::from_bytes(raw)?,
                )),
                13 => Ok(Self::XinputDeviceKeyStateNotifyEvent(
                    xinput::DeviceKeyStateNotifyEvent::from_bytes(raw)?,
                )),
                14 => Ok(Self::XinputDeviceButtonStateNotifyEvent(
                    xinput::DeviceButtonStateNotifyEvent::from_bytes(raw)?,
                )),
                15 => Ok(Self::XinputDevicePresenceNotifyEvent(
                    xinput::DevicePresenceNotifyEvent::from_bytes(raw)?,
                )),
                16 => Ok(Self::XinputDevicePropertyNotifyEvent(
                    xinput::DevicePropertyNotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "xkb")]
            Some((xkb::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::XkbNewKeyboardNotifyEvent(
                    xkb::NewKeyboardNotifyEvent::from_bytes(raw)?,
                )),
                1 => Ok(Self::XkbMapNotifyEvent(xkb::MapNotifyEvent::from_bytes(
                    raw,
                )?)),
                2 => Ok(Self::XkbStateNotifyEvent(
                    xkb::StateNotifyEvent::from_bytes(raw)?,
                )),
                3 => Ok(Self::XkbControlsNotifyEvent(
                    xkb::ControlsNotifyEvent::from_bytes(raw)?,
                )),
                4 => Ok(Self::XkbIndicatorStateNotifyEvent(
                    xkb::IndicatorStateNotifyEvent::from_bytes(raw)?,
                )),
                5 => Ok(Self::XkbIndicatorMapNotifyEvent(
                    xkb::IndicatorMapNotifyEvent::from_bytes(raw)?,
                )),
                6 => Ok(Self::XkbNamesNotifyEvent(
                    xkb::NamesNotifyEvent::from_bytes(raw)?,
                )),
                7 => Ok(Self::XkbCompatMapNotifyEvent(
                    xkb::CompatMapNotifyEvent::from_bytes(raw)?,
                )),
                8 => Ok(Self::XkbBellNotifyEvent(xkb::BellNotifyEvent::from_bytes(
                    raw,
                )?)),
                9 => Ok(Self::XkbActionMessageEvent(
                    xkb::ActionMessageEvent::from_bytes(raw)?,
                )),
                10 => Ok(Self::XkbAccessXNotifyEvent(
                    xkb::AccessXNotifyEvent::from_bytes(raw)?,
                )),
                11 => Ok(Self::XkbExtensionDeviceNotifyEvent(
                    xkb::ExtensionDeviceNotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "xprint")]
            Some((xprint::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::XprintNotifyEvent(xprint::NotifyEvent::from_bytes(
                    raw,
                )?)),
                1 => Ok(Self::XprintAttributNotifyEvent(
                    xprint::AttributNotifyEvent::from_bytes(raw)?,
                )),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            #[cfg(feature = "xv")]
            Some((xv::EXTENSION_NAME, info)) => match opcode - info.first_event {
                0 => Ok(Self::XvVideoNotifyEvent(xv::VideoNotifyEvent::from_bytes(
                    raw,
                )?)),
                1 => Ok(Self::XvPortNotifyEvent(xv::PortNotifyEvent::from_bytes(
                    raw,
                )?)),
                _ => Ok(Event::Unknown(raw.to_vec())),
            },
            _ => Ok(Event::Unknown(raw.to_vec())),
        }
    }
}
