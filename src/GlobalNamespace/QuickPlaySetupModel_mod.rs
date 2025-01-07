#[cfg(feature = "QuickPlaySetupModel")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkConfig,
    >,
    pub _client: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpClient>,
    pub _request: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
        >,
    >,
    pub _lastRequestTime: crate::System::DateTime,
}
#[cfg(feature = "QuickPlaySetupModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::QuickPlaySetupModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl std::ops::Deref for crate::GlobalNamespace::QuickPlaySetupModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuickPlaySetupModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl crate::GlobalNamespace::QuickPlaySetupModel {
    pub const kRequestCacheTimeoutMinutes: i32 = 5i32;
    pub const kRequestTimeoutSeconds: i32 = 60i32;
    #[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
    pub type QuickPlaySetupDataFB = crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB;
    pub fn GetQuickPlaySetupAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        > = __cordl_object.invoke("GetQuickPlaySetupAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQuickPlaySetupInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        > = __cordl_object.invoke("GetQuickPlaySetupInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsQuickPlaySetupTaskValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsQuickPlaySetupTaskValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUrlValid(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsUrlValid", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::QuickPlaySetupModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl AsRef<crate::GlobalNamespace::IQuickPlaySetupModel>
for crate::GlobalNamespace::QuickPlaySetupModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IQuickPlaySetupModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl AsMut<crate::GlobalNamespace::IQuickPlaySetupModel>
for crate::GlobalNamespace::QuickPlaySetupModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IQuickPlaySetupModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupModel_QuickPlaySetupDataFB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
        >,
    >,
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupDataFB";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
