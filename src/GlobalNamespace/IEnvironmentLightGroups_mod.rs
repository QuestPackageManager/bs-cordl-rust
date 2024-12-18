#[cfg(feature = "IEnvironmentLightGroups")]
#[repr(C)]
#[derive(Debug)]
pub struct IEnvironmentLightGroups {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IEnvironmentLightGroups")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IEnvironmentLightGroups => ""
    ."IEnvironmentLightGroups"
);
#[cfg(feature = "IEnvironmentLightGroups")]
impl std::ops::Deref for crate::GlobalNamespace::IEnvironmentLightGroups {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IEnvironmentLightGroups")]
impl std::ops::DerefMut for crate::GlobalNamespace::IEnvironmentLightGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IEnvironmentLightGroups")]
impl crate::GlobalNamespace::IEnvironmentLightGroups {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
#[cfg(feature = "IEnvironmentLightGroups")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IEnvironmentLightGroups {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
