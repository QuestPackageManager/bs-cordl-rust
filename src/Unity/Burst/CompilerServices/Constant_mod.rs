#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
#[repr(C)]
#[derive(Debug)]
pub struct Constant {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::CompilerServices::Constant {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.CompilerServices";
    const CLASS_NAME: &'static str = "Constant";
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
#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::Constant {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::Constant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
impl crate::Unity::Burst::CompilerServices::Constant {
    pub fn IsConstantExpression_Il2CppObject1(
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConstantExpression", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsConstantExpression_T0<T>(t: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConstantExpression", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Constant")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::Constant {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
