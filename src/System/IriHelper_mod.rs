#[cfg(feature = "System+IriHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IriHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IriHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IriHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "IriHelper";
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
#[cfg(feature = "System+IriHelper")]
impl std::ops::Deref for crate::System::IriHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IriHelper")]
impl std::ops::DerefMut for crate::System::IriHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IriHelper")]
impl crate::System::IriHelper {
    pub fn CheckIriUnicodeRange__cordl_bool0(
        unicode: char,
        isQuery: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIriUnicodeRange", (unicode, isQuery))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIriUnicodeRange__cordl_char_ByRefMut__cordl_bool1(
        highSurr: char,
        lowSurr: char,
        surrogatePair: quest_hook::libil2cpp::ByRefMut<bool>,
        isQuery: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckIriUnicodeRange",
                (highSurr, lowSurr, surrogatePair, isQuery),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIsReserved(
        ch: char,
        component: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIsReserved", (ch, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapeUnescapeIri(
        pInput: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
        component: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EscapeUnescapeIri", (pInput, start, end, component))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IriHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IriHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
