#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct MathUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::MathUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "MathUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::MathUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::MathUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
impl crate::Newtonsoft::Json::Utilities::MathUtils {
    pub fn ApproxEquals(d1: f64, d2: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApproxEquals", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntLength(i: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntLength", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToHex(n: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToHex", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Nullable_1_Nullable_1_0(
        val1: crate::System::Nullable_1<i32>,
        val2: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Nullable_1_Nullable_1_1(
        val1: crate::System::Nullable_1<f64>,
        val2: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_ret: crate::System::Nullable_1<f64> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min(
        val1: crate::System::Nullable_1<i32>,
        val2: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+MathUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::MathUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
