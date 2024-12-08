#[cfg(feature = "EnvironmentLightGroups")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentLightGroups {
    __cordl_parent: crate::System::Object,
    pub _lightGroupSOList: *mut crate::System::Collections::Generic::List_1<
        *mut LightGroupSO,
    >,
    pub _lightGroupSODict: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut LightGroupSO,
    >,
    pub _lightGroupSOListForLightGroupDataDict: *mut crate::System::Collections::Generic::List_1<
        *mut LightGroupSO,
    >,
}
#[cfg(feature = "EnvironmentLightGroups")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentLightGroups => ""."EnvironmentLightGroups"
);
#[cfg(feature = "EnvironmentLightGroups")]
impl std::ops::Deref for EnvironmentLightGroups {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl std::ops::DerefMut for EnvironmentLightGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl EnvironmentLightGroups {
    #[cfg(feature = "EnvironmentLightGroups+__c")]
    pub type __c = crate::GlobalNamespace::EnvironmentLightGroups___c;
    pub fn GetDataForGroup(
        &mut self,
        groupId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut ILightGroup> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILightGroup = __cordl_object
            .invoke("GetDataForGroup", (groupId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lightGroups: *mut crate::System::Collections::Generic::List_1<*mut LightGroupSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroups))?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut ILightGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut ILightGroup,
        > = __cordl_object.invoke("get_lightGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn Sort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Sort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroupSOList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut LightGroupSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut LightGroupSO,
        > = __cordl_object.invoke("get_lightGroupSOList", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightGroups: *mut crate::System::Collections::Generic::List_1<*mut LightGroupSO>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroups))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentLightGroups {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
