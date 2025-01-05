#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
#[repr(C)]
#[derive(Debug)]
pub struct G_FloatString {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Utils::NumString::G_FloatString =>
    "Tayx.Graphy.Utils.NumString"."G_FloatString"
);
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl std::ops::Deref for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    pub const m_floatFormat: &'static str = "0.0";
    pub fn Dispose() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromIndex(i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIndex", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        minNegativeValue: f32,
        maxPositiveValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", (minNegativeValue, maxPositiveValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFloat(i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFloat", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIndex(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIndex", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStringNonAlloc_Gc1(
        value: f32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToStringNonAlloc", (value, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStringNonAlloc_f32_0(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToStringNonAlloc", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxValue() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinValue() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MinValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
