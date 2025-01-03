#[cfg(feature = "UnityEngine+HelpURLAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpURLAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_Url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Dispatcher: bool,
    pub m_DispatchingFieldName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+HelpURLAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HelpURLAttribute => "UnityEngine"
    ."HelpURLAttribute"
);
#[cfg(feature = "UnityEngine+HelpURLAttribute")]
impl std::ops::Deref for crate::UnityEngine::HelpURLAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HelpURLAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::HelpURLAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HelpURLAttribute")]
impl crate::UnityEngine::HelpURLAttribute {
    pub fn New(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+HelpURLAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::HelpURLAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
