#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct ArcVisibilityTypeSettingsDropdown {
    __cordl_parent: ValueDropdownController_1<ArcVisibilityType>,
}
#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ArcVisibilityTypeSettingsDropdown => ""
    ."ArcVisibilityTypeSettingsDropdown"
);
#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
impl std::ops::Deref for ArcVisibilityTypeSettingsDropdown {
    type Target = ValueDropdownController_1<ArcVisibilityType>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
impl std::ops::DerefMut for ArcVisibilityTypeSettingsDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
impl ArcVisibilityTypeSettingsDropdown {
    pub fn GetNamedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<ArcVisibilityType, *mut crate::System::String>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<ArcVisibilityType, *mut crate::System::String>,
        > = __cordl_object.invoke("GetNamedValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "ArcVisibilityTypeSettingsDropdown")]
impl quest_hook::libil2cpp::ObjectType for ArcVisibilityTypeSettingsDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}