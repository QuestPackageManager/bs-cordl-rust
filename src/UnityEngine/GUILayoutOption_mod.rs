#[cfg(feature = "UnityEngine+GUILayoutOption")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutOption {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::UnityEngine::GUILayoutOption_Type,
    pub value: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUILayoutOption")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutOption => "UnityEngine"
    ."GUILayoutOption"
);
#[cfg(feature = "UnityEngine+GUILayoutOption")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutOption {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutOption")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutOption")]
impl crate::UnityEngine::GUILayoutOption {
    #[cfg(feature = "UnityEngine+GUILayoutOption+Type")]
    pub type Type = crate::UnityEngine::GUILayoutOption_Type;
    pub fn New(
        _cordl_type: crate::UnityEngine::GUILayoutOption_Type,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::GUILayoutOption_Type,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayoutOption")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayoutOption {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUILayoutOption+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GUILayoutOption_Type {
    alignEnd = 10i32,
    alignJustify = 11i32,
    alignMiddle = 9i32,
    alignStart = 8i32,
    equalSize = 12i32,
    fixedHeight = 1i32,
    fixedWidth = 0i32,
    maxHeight = 5i32,
    maxWidth = 3i32,
    minHeight = 4i32,
    minWidth = 2i32,
    spacing = 13i32,
    stretchHeight = 7i32,
    stretchWidth = 6i32,
}
#[cfg(feature = "UnityEngine+GUILayoutOption+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutOption_Type =>
    "UnityEngine"."GUILayoutOption/Type"
);
