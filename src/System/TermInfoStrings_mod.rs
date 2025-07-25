#[cfg(feature = "System+TermInfoStrings")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TermInfoStrings {
    #[default]
    AcsChars = 146i32,
    AltScancodeEsc = 385i32,
    BackTab = 0i32,
    Bell = 1i32,
    BitImageCarriageReturn = 372i32,
    BitImageNewline = 371i32,
    BitImageRepeat = 370i32,
    CarriageReturn = 2i32,
    ChangeCharPitch = 304i32,
    ChangeLinePitch = 305i32,
    ChangeResHorz = 306i32,
    ChangeResVert = 307i32,
    ChangeScrollRegion = 3i32,
    CharPadding = 145i32,
    CharSetNames = 354i32,
    ClearAllTabs = 4i32,
    ClearMargins = 270i32,
    ClearScreen = 5i32,
    ClrBol = 269i32,
    ClrEol = 6i32,
    ClrEos = 7i32,
    CodeSetInit = 363i32,
    ColorNames = 373i32,
    ColumnAddress = 8i32,
    CommandCharacter = 9i32,
    CreateWindow = 277i32,
    CursorAddress = 10i32,
    CursorDown = 11i32,
    CursorHome = 12i32,
    CursorInvisible = 13i32,
    CursorLeft = 14i32,
    CursorMemAddress = 15i32,
    CursorNormal = 16i32,
    CursorRight = 17i32,
    CursorToLl = 18i32,
    CursorUp = 19i32,
    CursorVisible = 20i32,
    DefineBitImageRegion = 374i32,
    DefineChar = 308i32,
    DeleteCharacter = 21i32,
    DeleteLine = 22i32,
    DeviceType = 362i32,
    DialPhone = 280i32,
    DisStatusLine = 23i32,
    DisplayClock = 275i32,
    DisplayPcChar = 378i32,
    DownHalfLine = 24i32,
    EnaAcs = 155i32,
    EndBitImageRegion = 375i32,
    EnterAltCharsetMode = 25i32,
    EnterAmMode = 151i32,
    EnterBlinkMode = 26i32,
    EnterBoldMode = 27i32,
    EnterCaMode = 28i32,
    EnterDeleteMode = 29i32,
    EnterDimMode = 30i32,
    EnterDoublewideMode = 309i32,
    EnterDraftQuality = 310i32,
    EnterHorizontalHlMode = 386i32,
    EnterInsertMode = 31i32,
    EnterItalicsMode = 311i32,
    EnterLeftHlMode = 387i32,
    EnterLeftwardMode = 312i32,
    EnterLowHlMode = 388i32,
    EnterMicroMode = 313i32,
    EnterNearLetterQuality = 314i32,
    EnterNormalQuality = 315i32,
    EnterPcCharsetMode = 379i32,
    EnterProtectedMode = 33i32,
    EnterReverseMode = 34i32,
    EnterRightHlMode = 389i32,
    EnterScancodeMode = 381i32,
    EnterSecureMode = 32i32,
    EnterShadowMode = 316i32,
    EnterStandoutMode = 35i32,
    EnterSubscriptMode = 317i32,
    EnterSuperscriptMode = 318i32,
    EnterTopHlMode = 390i32,
    EnterUnderlineMode = 36i32,
    EnterUpwardMode = 319i32,
    EnterVerticalHlMode = 391i32,
    EnterXonMode = 149i32,
    EraseChars = 37i32,
    ExitAltCharsetMode = 38i32,
    ExitAmMode = 152i32,
    ExitAttributeMode = 39i32,
    ExitCaMode = 40i32,
    ExitDeleteMode = 41i32,
    ExitDoublewideMode = 320i32,
    ExitInsertMode = 42i32,
    ExitItalicsMode = 321i32,
    ExitLeftwardMode = 322i32,
    ExitMicroMode = 323i32,
    ExitPcCharsetMode = 380i32,
    ExitScancodeMode = 382i32,
    ExitShadowMode = 324i32,
    ExitStandoutMode = 43i32,
    ExitSubscriptMode = 325i32,
    ExitSuperscriptMode = 326i32,
    ExitUnderlineMode = 44i32,
    ExitUpwardMode = 327i32,
    ExitXonMode = 150i32,
    FixedPause = 285i32,
    FlashHook = 284i32,
    FlashScreen = 45i32,
    FormFeed = 46i32,
    FromStatusLine = 47i32,
    GetMouse = 358i32,
    GotoWindow = 278i32,
    Hangup = 279i32,
    Init1string = 48i32,
    Init2string = 49i32,
    Init3string = 50i32,
    InitFile = 51i32,
    InitProg = 138i32,
    InitializeColor = 299i32,
    InitializePair = 300i32,
    InsertCharacter = 52i32,
    InsertLine = 53i32,
    InsertPadding = 54i32,
    KeyA1 = 139i32,
    KeyA3 = 140i32,
    KeyB2 = 141i32,
    KeyBackspace = 55i32,
    KeyBeg = 158i32,
    KeyBtab = 148i32,
    KeyC1 = 142i32,
    KeyC3 = 143i32,
    KeyCancel = 159i32,
    KeyCatab = 56i32,
    KeyClear = 57i32,
    KeyClose = 160i32,
    KeyCommand = 161i32,
    KeyCopy = 162i32,
    KeyCreate = 163i32,
    KeyCtab = 58i32,
    KeyDc = 59i32,
    KeyDl = 60i32,
    KeyDown = 61i32,
    KeyEic = 62i32,
    KeyEnd = 164i32,
    KeyEnter = 165i32,
    KeyEol = 63i32,
    KeyEos = 64i32,
    KeyExit = 166i32,
    KeyF0 = 65i32,
    KeyF1 = 66i32,
    KeyF10 = 67i32,
    KeyF11 = 216i32,
    KeyF12 = 217i32,
    KeyF13 = 218i32,
    KeyF14 = 219i32,
    KeyF15 = 220i32,
    KeyF16 = 221i32,
    KeyF17 = 222i32,
    KeyF18 = 223i32,
    KeyF19 = 224i32,
    KeyF2 = 68i32,
    KeyF20 = 225i32,
    KeyF21 = 226i32,
    KeyF22 = 227i32,
    KeyF23 = 228i32,
    KeyF24 = 229i32,
    KeyF25 = 230i32,
    KeyF26 = 231i32,
    KeyF27 = 232i32,
    KeyF28 = 233i32,
    KeyF29 = 234i32,
    KeyF3 = 69i32,
    KeyF30 = 235i32,
    KeyF31 = 236i32,
    KeyF32 = 237i32,
    KeyF33 = 238i32,
    KeyF34 = 239i32,
    KeyF35 = 240i32,
    KeyF36 = 241i32,
    KeyF37 = 242i32,
    KeyF38 = 243i32,
    KeyF39 = 244i32,
    KeyF4 = 70i32,
    KeyF40 = 245i32,
    KeyF41 = 246i32,
    KeyF42 = 247i32,
    KeyF43 = 248i32,
    KeyF44 = 249i32,
    KeyF45 = 250i32,
    KeyF46 = 251i32,
    KeyF47 = 252i32,
    KeyF48 = 253i32,
    KeyF49 = 254i32,
    KeyF5 = 71i32,
    KeyF50 = 255i32,
    KeyF51 = 256i32,
    KeyF52 = 257i32,
    KeyF53 = 258i32,
    KeyF54 = 259i32,
    KeyF55 = 260i32,
    KeyF56 = 261i32,
    KeyF57 = 262i32,
    KeyF58 = 263i32,
    KeyF59 = 264i32,
    KeyF6 = 72i32,
    KeyF60 = 265i32,
    KeyF61 = 266i32,
    KeyF62 = 267i32,
    KeyF63 = 268i32,
    KeyF7 = 73i32,
    KeyF8 = 74i32,
    KeyF9 = 75i32,
    KeyFind = 167i32,
    KeyHelp = 168i32,
    KeyHome = 76i32,
    KeyIc = 77i32,
    KeyIl = 78i32,
    KeyLeft = 79i32,
    KeyLl = 80i32,
    KeyMark = 169i32,
    KeyMessage = 170i32,
    KeyMouse = 355i32,
    KeyMove = 171i32,
    KeyNext = 172i32,
    KeyNpage = 81i32,
    KeyOpen = 173i32,
    KeyOptions = 174i32,
    KeyPpage = 82i32,
    KeyPrevious = 175i32,
    KeyPrint = 176i32,
    KeyRedo = 177i32,
    KeyReference = 178i32,
    KeyRefresh = 179i32,
    KeyReplace = 180i32,
    KeyRestart = 181i32,
    KeyResume = 182i32,
    KeyRight = 83i32,
    KeySave = 183i32,
    KeySbeg = 186i32,
    KeyScancel = 187i32,
    KeyScommand = 188i32,
    KeyScopy = 189i32,
    KeyScreate = 190i32,
    KeySdc = 191i32,
    KeySdl = 192i32,
    KeySelect = 193i32,
    KeySend = 194i32,
    KeySeol = 195i32,
    KeySexit = 196i32,
    KeySf = 84i32,
    KeySfind = 197i32,
    KeyShelp = 198i32,
    KeyShome = 199i32,
    KeySic = 200i32,
    KeySleft = 201i32,
    KeySmessage = 202i32,
    KeySmove = 203i32,
    KeySnext = 204i32,
    KeySoptions = 205i32,
    KeySprevious = 206i32,
    KeySprint = 207i32,
    KeySr = 85i32,
    KeySredo = 208i32,
    KeySreplace = 209i32,
    KeySright = 210i32,
    KeySrsume = 211i32,
    KeySsave = 212i32,
    KeySsuspend = 213i32,
    KeyStab = 86i32,
    KeySundo = 214i32,
    KeySuspend = 184i32,
    KeyUndo = 185i32,
    KeyUp = 87i32,
    KeypadLocal = 88i32,
    KeypadXmit = 89i32,
    LabF0 = 90i32,
    LabF1 = 91i32,
    LabF10 = 92i32,
    LabF2 = 93i32,
    LabF3 = 94i32,
    LabF4 = 95i32,
    LabF5 = 96i32,
    LabF6 = 97i32,
    LabF7 = 98i32,
    LabF8 = 99i32,
    LabF9 = 100i32,
    LabelFormat = 273i32,
    LabelOff = 157i32,
    LabelOn = 156i32,
    Last = 394i32,
    MetaOff = 101i32,
    MetaOn = 102i32,
    MicroColumnAddress = 328i32,
    MicroDown = 329i32,
    MicroLeft = 330i32,
    MicroRight = 331i32,
    MicroRowAddress = 332i32,
    MicroUp = 333i32,
    MouseInfo = 356i32,
    Newline = 103i32,
    OrderOfPins = 334i32,
    OrigColors = 298i32,
    OrigPair = 297i32,
    PadChar = 104i32,
    ParmDch = 105i32,
    ParmDeleteLine = 106i32,
    ParmDownCursor = 107i32,
    ParmDownMicro = 335i32,
    ParmIch = 108i32,
    ParmIndex = 109i32,
    ParmInsertLine = 110i32,
    ParmLeftCursor = 111i32,
    ParmLeftMicro = 336i32,
    ParmRightCursor = 112i32,
    ParmRightMicro = 337i32,
    ParmRindex = 113i32,
    ParmUpCursor = 114i32,
    ParmUpMicro = 338i32,
    PcTermOptions = 383i32,
    PkeyKey = 115i32,
    PkeyLocal = 116i32,
    PkeyPlab = 361i32,
    PkeyXmit = 117i32,
    PlabNorm = 147i32,
    PrintScreen = 118i32,
    PrtrNon = 144i32,
    PrtrOff = 119i32,
    PrtrOn = 120i32,
    Pulse = 283i32,
    QuickDial = 281i32,
    RemoveClock = 276i32,
    RepeatChar = 121i32,
    ReqForInput = 215i32,
    ReqMousePos = 357i32,
    Reset1string = 122i32,
    Reset2string = 123i32,
    Reset3string = 124i32,
    ResetFile = 125i32,
    RestoreCursor = 126i32,
    RowAddress = 127i32,
    SaveCursor = 128i32,
    ScancodeEscape = 384i32,
    ScrollForward = 129i32,
    ScrollReverse = 130i32,
    SelectCharSet = 339i32,
    Set0DesSeq = 364i32,
    Set1DesSeq = 365i32,
    Set2DesSeq = 366i32,
    Set3DesSeq = 367i32,
    SetAAttributes = 392i32,
    SetABackground = 360i32,
    SetAForeground = 359i32,
    SetAttributes = 131i32,
    SetBackground = 303i32,
    SetBottomMargin = 340i32,
    SetBottomMarginParm = 341i32,
    SetClock = 274i32,
    SetColorBand = 376i32,
    SetColorPair = 301i32,
    SetForeground = 302i32,
    SetLeftMargin = 271i32,
    SetLeftMarginParm = 342i32,
    SetLrMargin = 368i32,
    SetPageLength = 377i32,
    SetPglenInch = 393i32,
    SetRightMargin = 272i32,
    SetRightMarginParm = 343i32,
    SetTab = 132i32,
    SetTbMargin = 369i32,
    SetTopMargin = 344i32,
    SetTopMarginParm = 345i32,
    SetWindow = 133i32,
    StartBitImage = 346i32,
    StartCharSetDef = 347i32,
    StopBitImage = 348i32,
    StopCharSetDef = 349i32,
    SubscriptCharacters = 350i32,
    SuperscriptCharacters = 351i32,
    Tab = 134i32,
    TheseCauseCr = 352i32,
    ToStatusLine = 135i32,
    Tone = 282i32,
    UnderlineChar = 136i32,
    UpHalfLine = 137i32,
    User0 = 287i32,
    User1 = 288i32,
    User2 = 289i32,
    User3 = 290i32,
    User4 = 291i32,
    User5 = 292i32,
    User6 = 293i32,
    User7 = 294i32,
    User8 = 295i32,
    User9 = 296i32,
    WaitTone = 286i32,
    XoffCharacter = 154i32,
    XonCharacter = 153i32,
    ZeroMotion = 353i32,
}
#[cfg(feature = "System+TermInfoStrings")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TermInfoStrings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TermInfoStrings";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+TermInfoStrings")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::TermInfoStrings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+TermInfoStrings")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::TermInfoStrings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+TermInfoStrings")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::TermInfoStrings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+TermInfoStrings")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::TermInfoStrings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
