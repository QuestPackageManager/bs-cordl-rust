#[cfg(feature = "EnvironmentsListModel")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentsListModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _envInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::EnvironmentInfoSO,
        >,
    >,
    pub _normalEnvironmentSerializedNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _normalEnvironmentNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _envInfoMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
        >,
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
    pub fn CreateFromAddressables() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentsListModel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromAddressables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllEnvironmentInfosWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        > = __cordl_object.invoke("GetAllEnvironmentInfosWithType", (environmentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentInfoBySerializedName(
        &mut self,
        environmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object
            .invoke("GetEnvironmentInfoBySerializedName", (environmentSerializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentInfoBySerializedNameSafe(
        &mut self,
        environmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object
            .invoke(
                "GetEnvironmentInfoBySerializedNameSafe",
                (environmentSerializedName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstEnvironmentInfoWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("GetFirstEnvironmentInfoWithType", (environmentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastEnvironmentInfoWithType(
        &mut self,
        environmentType: crate::GlobalNamespace::EnvironmentType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object.invoke("GetLastEnvironmentInfoWithType", (environmentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        envInfoSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envInfoSOs))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        envInfoSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (envInfoSOs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
            >,
        > = __cordl_object.invoke("get_environmentInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loggerPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_loggerPrefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalEnvironmentNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_normalEnvironmentNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalEnvironmentSerializedNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_normalEnvironmentSerializedNames", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "EnvironmentsListModel")]
impl AsRef<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::EnvironmentsListModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentsListModel")]
impl AsMut<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::EnvironmentsListModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
