#[cfg(feature = "PlayerSensitivityDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSensitivityDropdown {
    __cordl_parent: ValueDropdownController_1<PlayerSensitivityFlag>,
}
#[cfg(feature = "PlayerSensitivityDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerSensitivityDropdown => ""
    ."PlayerSensitivityDropdown"
);
#[cfg(feature = "PlayerSensitivityDropdown")]
impl std::ops::Deref for PlayerSensitivityDropdown {
    type Target = ValueDropdownController_1<PlayerSensitivityFlag>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSensitivityDropdown")]
impl std::ops::DerefMut for PlayerSensitivityDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSensitivityDropdown")]
impl PlayerSensitivityDropdown {
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
    pub fn GetNamedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<
                PlayerSensitivityFlag,
                *mut crate::System::String,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<
                PlayerSensitivityFlag,
                *mut crate::System::String,
            >,
        > = __cordl_object.invoke("GetNamedValues", ())?;
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
#[cfg(feature = "PlayerSensitivityDropdown")]
impl quest_hook::libil2cpp::ObjectType for PlayerSensitivityDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
