#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::TypeHelpers
    => "UnityEngine.InputSystem.Utilities"."TypeHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    pub fn As<TObject>(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("As", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGenericTypeArgumentFromHierarchy(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericTypeDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetGenericTypeArgumentFromHierarchy",
                (_cordl_type, genericTypeDefinition, argumentIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNiceTypeName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNiceTypeName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueType(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueType", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInt(
        _cordl_type: crate::System::TypeCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInt", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
