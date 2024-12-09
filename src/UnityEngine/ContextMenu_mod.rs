#[cfg(feature = "UnityEngine+ContextMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextMenu {
    __cordl_parent: crate::System::Attribute,
    pub menuItem: *mut quest_hook::libil2cpp::Il2CppString,
    pub validate: bool,
    pub priority: i32,
}
#[cfg(feature = "UnityEngine+ContextMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContextMenu => "UnityEngine"
    ."ContextMenu"
);
#[cfg(feature = "UnityEngine+ContextMenu")]
impl std::ops::Deref for crate::UnityEngine::ContextMenu {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ContextMenu")]
impl std::ops::DerefMut for crate::UnityEngine::ContextMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ContextMenu")]
impl crate::UnityEngine::ContextMenu {
    pub fn New_Il2CppString0(
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemName))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
        isValidateFunction: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemName, isValidateFunction))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_i32_2(
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
        isValidateFunction: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemName, isValidateFunction, priority))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
        isValidateFunction: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemName, isValidateFunction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_i32_2(
        &mut self,
        itemName: *mut quest_hook::libil2cpp::Il2CppString,
        isValidateFunction: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemName, isValidateFunction, priority))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ContextMenu")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ContextMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
