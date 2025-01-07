#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeHelperEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AttributeHelperEngine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AttributeHelperEngine";
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
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl std::ops::Deref for crate::UnityEngine::AttributeHelperEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl std::ops::DerefMut for crate::UnityEngine::AttributeHelperEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl crate::UnityEngine::AttributeHelperEngine {
    pub fn CheckIsEditorScript(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIsEditorScript", (klass))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributeOfType<T>(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomAttributeOfType", (klass))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultExecutionOrderFor(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultExecutionOrderFor", (klass))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExecuteMode(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExecuteMode", (klass))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentTypeDisallowingMultipleInclusion(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentTypeDisallowingMultipleInclusion", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequiredComponents(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRequiredComponents", (klass))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AttributeHelperEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
