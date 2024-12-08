#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct PreserveAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Scripting::PreserveAttribute =>
    "UnityEngine.Scripting"."PreserveAttribute"
);
#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
impl std::ops::Deref for crate::UnityEngine::Scripting::PreserveAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Scripting::PreserveAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
impl crate::UnityEngine::Scripting::PreserveAttribute {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Scripting+PreserveAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Scripting::PreserveAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
