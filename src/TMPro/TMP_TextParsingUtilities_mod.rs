#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextParsingUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_TextParsingUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_TextParsingUtilities";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_TextParsingUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_TextParsingUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
impl crate::TMPro::TMP_TextParsingUtilities {
    pub const k_LookupStringL: &'static str = "-------------------------------- !-#$%&-()*+,-./0123456789:;<=>?@abcdefghijklmnopqrstuvwxyz[-]^_`abcdefghijklmnopqrstuvwxyz{|}~-";
    pub const k_LookupStringU: &'static str = "-------------------------------- !-#$%&-()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[-]^_`ABCDEFGHIJKLMNOPQRSTUVWXYZ{|}~-";
    pub fn ConvertToUTF32(
        highSurrogate: u32,
        lowSurrogate: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToUTF32", (highSurrogate, lowSurrogate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeCaseSensitive(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCodeCaseSensitive", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHighSurrogate(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHighSurrogate", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLowSurrogate(c: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLowSurrogate", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToLowerASCIIFast__cordl_char0(
        c: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLowerASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerASCIIFast_u32_1(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLowerASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast__cordl_char0(
        c: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast_u32_1(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextParsingUtilities>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::TMPro::TMP_TextParsingUtilities,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_TextParsingUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_TextParsingUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
