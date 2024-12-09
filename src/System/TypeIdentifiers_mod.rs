#[cfg(feature = "System+TypeIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifiers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+TypeIdentifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeIdentifiers => "System"
    ."TypeIdentifiers"
);
#[cfg(feature = "System+TypeIdentifiers")]
impl std::ops::Deref for crate::System::TypeIdentifiers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers")]
impl std::ops::DerefMut for crate::System::TypeIdentifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers")]
impl crate::System::TypeIdentifiers {
    #[cfg(feature = "System+TypeIdentifiers+Display")]
    pub type Display = crate::System::TypeIdentifiers_Display;
}
#[cfg(feature = "System+TypeIdentifiers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeIdentifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeIdentifiers_Display {
    __cordl_parent: crate::System::TypeNames_ATypeName,
    pub displayName: *mut crate::System::String,
    pub internal_name: *mut crate::System::String,
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TypeIdentifiers_Display => "System"
    ."TypeIdentifiers/Display"
);
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl std::ops::Deref for crate::System::TypeIdentifiers_Display {
    type Target = crate::System::TypeNames_ATypeName;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl std::ops::DerefMut for crate::System::TypeIdentifiers_Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl crate::System::TypeIdentifiers_Display {
    pub fn GetInternalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetInternalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        displayName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        displayName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName))?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DisplayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InternalName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+TypeIdentifiers+Display")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TypeIdentifiers_Display {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
