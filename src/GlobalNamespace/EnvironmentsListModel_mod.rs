#[cfg(feature = "EnvironmentsListModel")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentsListModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _envInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub _environmentSerializedNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _environmentNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _envInfoMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::EnvironmentInfoSO,
    >,
}
#[cfg(feature = "EnvironmentsListModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentsListModel => ""
    ."EnvironmentsListModel"
);
#[cfg(feature = "EnvironmentsListModel")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentsListModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentsListModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentsListModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentsListModel")]
impl crate::GlobalNamespace::EnvironmentsListModel {
    #[cfg(feature = "EnvironmentsListModel+__c")]
    pub type __c = crate::GlobalNamespace::EnvironmentsListModel___c;
    pub fn GetAllEnvironmentInfosWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("GetAllEnvironmentInfosWithType", (environmentType))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnvironmentInfoBySerializedName(
        &mut self,
        environmentSerializedName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("GetEnvironmentInfoBySerializedName", (environmentSerializedName))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnvironmentInfoBySerializedNameSafe(
        &mut self,
        environmentSerializedName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke(
                "GetEnvironmentInfoBySerializedNameSafe",
                (environmentSerializedName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetFirstEnvironmentInfoWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("GetFirstEnvironmentInfoWithType", (environmentType))?;
        Ok(__cordl_ret)
    }
    pub fn GetLastEnvironmentInfoWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("GetLastEnvironmentInfoWithType", (environmentType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        envInfoSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envInfoSOs))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        envInfoSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (envInfoSOs))?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("get_environmentInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_environmentNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentSerializedNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_environmentSerializedNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_loggerPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_loggerPrefix", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EnvironmentsListModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentsListModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
