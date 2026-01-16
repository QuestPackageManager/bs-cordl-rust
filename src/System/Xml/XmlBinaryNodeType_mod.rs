#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum XmlBinaryNodeType {
    #[default]
    Array = 3i32,
    Attribute = 5i32,
    BoolText = 180i32,
    BoolTextWithEndElement = 181i32,
    Bytes16Text = 160i32,
    Bytes16TextWithEndElement = 161i32,
    Bytes32Text = 162i32,
    Bytes32TextWithEndElement = 163i32,
    Bytes8Text = 158i32,
    Bytes8TextWithEndElement = 159i32,
    Chars16Text = 154i32,
    Chars16TextWithEndElement = 155i32,
    Chars32Text = 156i32,
    Chars32TextWithEndElement = 157i32,
    Chars8Text = 152i32,
    Chars8TextWithEndElement = 153i32,
    Comment = 2i32,
    DateTimeText = 150i32,
    DateTimeTextWithEndElement = 151i32,
    DecimalText = 148i32,
    DecimalTextWithEndElement = 149i32,
    DictionaryAttribute = 7i32,
    DictionaryElement = 67i32,
    DictionaryText = 170i32,
    DictionaryTextWithEndElement = 171i32,
    DictionaryXmlnsAttribute = 11i32,
    DoubleText = 146i32,
    DoubleTextWithEndElement = 147i32,
    Element = 65i32,
    EmptyText = 168i32,
    EmptyTextWithEndElement = 169i32,
    EndElement = 1i32,
    EndListText = 166i32,
    EndListTextWithEndElement = 167i32,
    FalseText = 132i32,
    FalseTextWithEndElement = 133i32,
    FloatText = 144i32,
    FloatTextWithEndElement = 145i32,
    GuidText = 176i32,
    GuidTextWithEndElement = 177i32,
    Int16Text = 138i32,
    Int16TextWithEndElement = 139i32,
    Int32Text = 140i32,
    Int32TextWithEndElement = 141i32,
    Int64Text = 142i32,
    Int64TextWithEndElement = 143i32,
    Int8Text = 136i32,
    Int8TextWithEndElement = 137i32,
    MaxAttribute = 63i32,
    MaxElement = 119i32,
    MaxText = 189i32,
    MinAttribute = 4i32,
    MinElement = 64i32,
    MinText = 128i32,
    OneText = 130i32,
    OneTextWithEndElement = 131i32,
    PrefixAttributeA = 38i32,
    PrefixAttributeB = 39i32,
    PrefixAttributeC = 40i32,
    PrefixAttributeD = 41i32,
    PrefixAttributeE = 42i32,
    PrefixAttributeF = 43i32,
    PrefixAttributeG = 44i32,
    PrefixAttributeH = 45i32,
    PrefixAttributeI = 46i32,
    PrefixAttributeJ = 47i32,
    PrefixAttributeK = 48i32,
    PrefixAttributeL = 49i32,
    PrefixAttributeM = 50i32,
    PrefixAttributeN = 51i32,
    PrefixAttributeO = 52i32,
    PrefixAttributeP = 53i32,
    PrefixAttributeQ = 54i32,
    PrefixAttributeR = 55i32,
    PrefixAttributeS = 56i32,
    PrefixAttributeT = 57i32,
    PrefixAttributeU = 58i32,
    PrefixAttributeV = 59i32,
    PrefixAttributeW = 60i32,
    PrefixAttributeX = 61i32,
    PrefixAttributeY = 62i32,
    PrefixDictionaryAttributeA = 12i32,
    PrefixDictionaryAttributeB = 13i32,
    PrefixDictionaryAttributeC = 14i32,
    PrefixDictionaryAttributeD = 15i32,
    PrefixDictionaryAttributeE = 16i32,
    PrefixDictionaryAttributeF = 17i32,
    PrefixDictionaryAttributeG = 18i32,
    PrefixDictionaryAttributeH = 19i32,
    PrefixDictionaryAttributeI = 20i32,
    PrefixDictionaryAttributeJ = 21i32,
    PrefixDictionaryAttributeK = 22i32,
    PrefixDictionaryAttributeL = 23i32,
    PrefixDictionaryAttributeM = 24i32,
    PrefixDictionaryAttributeN = 25i32,
    PrefixDictionaryAttributeO = 26i32,
    PrefixDictionaryAttributeP = 27i32,
    PrefixDictionaryAttributeQ = 28i32,
    PrefixDictionaryAttributeR = 29i32,
    PrefixDictionaryAttributeS = 30i32,
    PrefixDictionaryAttributeT = 31i32,
    PrefixDictionaryAttributeU = 32i32,
    PrefixDictionaryAttributeV = 33i32,
    PrefixDictionaryAttributeW = 34i32,
    PrefixDictionaryAttributeX = 35i32,
    PrefixDictionaryAttributeY = 36i32,
    PrefixDictionaryAttributeZ = 37i32,
    PrefixDictionaryElementA = 68i32,
    PrefixDictionaryElementB = 69i32,
    PrefixDictionaryElementC = 70i32,
    PrefixDictionaryElementD = 71i32,
    PrefixDictionaryElementE = 72i32,
    PrefixDictionaryElementF = 73i32,
    PrefixDictionaryElementG = 74i32,
    PrefixDictionaryElementH = 75i32,
    PrefixDictionaryElementI = 76i32,
    PrefixDictionaryElementJ = 77i32,
    PrefixDictionaryElementK = 78i32,
    PrefixDictionaryElementL = 79i32,
    PrefixDictionaryElementM = 80i32,
    PrefixDictionaryElementN = 81i32,
    PrefixDictionaryElementO = 82i32,
    PrefixDictionaryElementP = 83i32,
    PrefixDictionaryElementQ = 84i32,
    PrefixDictionaryElementR = 85i32,
    PrefixDictionaryElementS = 86i32,
    PrefixDictionaryElementT = 87i32,
    PrefixDictionaryElementU = 88i32,
    PrefixDictionaryElementV = 89i32,
    PrefixDictionaryElementW = 90i32,
    PrefixDictionaryElementX = 91i32,
    PrefixDictionaryElementY = 92i32,
    PrefixDictionaryElementZ = 93i32,
    PrefixElementA = 94i32,
    PrefixElementB = 95i32,
    PrefixElementC = 96i32,
    PrefixElementD = 97i32,
    PrefixElementE = 98i32,
    PrefixElementF = 99i32,
    PrefixElementG = 100i32,
    PrefixElementH = 101i32,
    PrefixElementI = 102i32,
    PrefixElementJ = 103i32,
    PrefixElementK = 104i32,
    PrefixElementL = 105i32,
    PrefixElementM = 106i32,
    PrefixElementN = 107i32,
    PrefixElementO = 108i32,
    PrefixElementP = 109i32,
    PrefixElementQ = 110i32,
    PrefixElementR = 111i32,
    PrefixElementS = 112i32,
    PrefixElementT = 113i32,
    PrefixElementU = 114i32,
    PrefixElementV = 115i32,
    PrefixElementW = 116i32,
    PrefixElementX = 117i32,
    PrefixElementY = 118i32,
    QNameDictionaryText = 188i32,
    ShortDictionaryAttribute = 6i32,
    ShortDictionaryElement = 66i32,
    ShortDictionaryXmlnsAttribute = 10i32,
    ShortXmlnsAttribute = 8i32,
    StartListText = 164i32,
    StartListTextWithEndElement = 165i32,
    TimeSpanText = 174i32,
    TimeSpanTextWithEndElement = 175i32,
    TrueText = 134i32,
    TrueTextWithEndElement = 135i32,
    UInt64Text = 178i32,
    UInt64TextWithEndElement = 179i32,
    UnicodeChars16Text = 184i32,
    UnicodeChars16TextWithEndElement = 185i32,
    UnicodeChars32Text = 186i32,
    UnicodeChars32TextWithEndElement = 187i32,
    UnicodeChars8Text = 182i32,
    UnicodeChars8TextWithEndElement = 183i32,
    UniqueIdText = 172i32,
    UniqueIdTextWithEndElement = 173i32,
    XmlnsAttribute = 9i32,
    ZeroTextWithEndElement = 129i32,
}
#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlBinaryNodeType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlBinaryNodeType";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::XmlBinaryNodeType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::XmlBinaryNodeType {
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
#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::XmlBinaryNodeType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Xml+XmlBinaryNodeType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::XmlBinaryNodeType {
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
