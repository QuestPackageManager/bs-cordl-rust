#[cfg(feature = "EnvironmentLightGroups")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentLightGroups {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lightGroupSOList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightGroupSO,
        >,
    >,
    pub _lightGroupSODict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::GlobalNamespace::LightGroupSO,
        >,
    >,
    pub _lightGroupSOListForLightGroupDataDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightGroupSO,
        >,
    >,
}
#[cfg(feature = "EnvironmentLightGroups")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentLightGroups => ""
    ."EnvironmentLightGroups"
);
#[cfg(feature = "EnvironmentLightGroups")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentLightGroups {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentLightGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl crate::GlobalNamespace::EnvironmentLightGroups {
    pub fn GetDataForGroup(
        &mut self,
        groupId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILightGroup,
        > = __cordl_object.invoke("GetDataForGroup", (groupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LightGroupSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroups))?;
        Ok(__cordl_object.into())
    }
    pub fn Sort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Sort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LightGroupSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroups))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroupSOList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LightGroupSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::LightGroupSO,
            >,
        > = __cordl_object.invoke("get_lightGroupSOList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILightGroup,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILightGroup,
            >,
        > = __cordl_object.invoke("get_lightGroups", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentLightGroups {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl AsRef<crate::GlobalNamespace::IEnvironmentLightGroups>
for crate::GlobalNamespace::EnvironmentLightGroups {
    fn as_ref(&self) -> &crate::GlobalNamespace::IEnvironmentLightGroups {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentLightGroups")]
impl AsMut<crate::GlobalNamespace::IEnvironmentLightGroups>
for crate::GlobalNamespace::EnvironmentLightGroups {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IEnvironmentLightGroups {
        unsafe { std::mem::transmute(self) }
    }
}
