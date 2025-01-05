#[cfg(feature = "UnityEngine+RequireComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct RequireComponent {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub m_Type0: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_Type1: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_Type2: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "UnityEngine+RequireComponent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RequireComponent => "UnityEngine"
    ."RequireComponent"
);
#[cfg(feature = "UnityEngine+RequireComponent")]
impl std::ops::Deref for crate::UnityEngine::RequireComponent {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RequireComponent")]
impl std::ops::DerefMut for crate::UnityEngine::RequireComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RequireComponent")]
impl crate::UnityEngine::RequireComponent {
    pub fn New_Gc0(
        requiredComponent: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requiredComponent))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc1(
        requiredComponent: quest_hook::libil2cpp::Gc<crate::System::Type>,
        requiredComponent2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requiredComponent, requiredComponent2))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        requiredComponent: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (requiredComponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        requiredComponent: quest_hook::libil2cpp::Gc<crate::System::Type>,
        requiredComponent2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (requiredComponent, requiredComponent2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RequireComponent")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RequireComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
