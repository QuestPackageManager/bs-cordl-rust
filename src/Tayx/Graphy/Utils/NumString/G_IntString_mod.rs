#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
#[repr(C)]
#[derive(Debug)]
pub struct G_IntString {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Tayx::Graphy::Utils::NumString::G_IntString {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tayx.Graphy.Utils.NumString";
    const CLASS_NAME: &'static str = "G_IntString";
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
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
impl std::ops::Deref for crate::Tayx::Graphy::Utils::NumString::G_IntString {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Utils::NumString::G_IntString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
impl crate::Tayx::Graphy::Utils::NumString::G_IntString {
    pub fn Dispose() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        minNegativeValue: i32,
        maxPositiveValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", (minNegativeValue, maxPositiveValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStringNonAlloc(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToStringNonAlloc", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxValue() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinValue() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MinValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_IntString")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::Utils::NumString::G_IntString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
