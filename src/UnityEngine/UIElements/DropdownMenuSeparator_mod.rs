#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownMenuSeparator {
    __cordl_parent: crate::UnityEngine::UIElements::DropdownMenuItem,
    pub _subMenuPath_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DropdownMenuSeparator
    => "UnityEngine.UIElements"."DropdownMenuSeparator"
);
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DropdownMenuSeparator {
    type Target = crate::UnityEngine::UIElements::DropdownMenuItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DropdownMenuSeparator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
impl crate::UnityEngine::UIElements::DropdownMenuSeparator {
    pub fn New(
        subMenuPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subMenuPath))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        subMenuPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subMenuPath))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownMenuSeparator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DropdownMenuSeparator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
