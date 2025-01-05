#[cfg(feature = "UnityEngine+GUITargetAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct GUITargetAttribute {
    __cordl_parent: crate::System::Attribute,
    pub displayMask: i32,
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUITargetAttribute => "UnityEngine"
    ."GUITargetAttribute"
);
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl std::ops::Deref for crate::UnityEngine::GUITargetAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::GUITargetAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl crate::UnityEngine::GUITargetAttribute {
    pub fn GetGUITargetAttrValue(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGUITargetAttrValue", (klass, methodName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUITargetAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
