#[cfg(feature = "System+Security+CodeAccessPermission")]
#[repr(C)]
#[derive(Debug)]
pub struct CodeAccessPermission {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::CodeAccessPermission =>
    "System.Security"."CodeAccessPermission"
);
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl std::ops::Deref for crate::System::Security::CodeAccessPermission {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl std::ops::DerefMut for crate::System::Security::CodeAccessPermission {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl crate::System::Security::CodeAccessPermission {
    pub fn CheckPermissionState(
        state: crate::System::Security::Permissions::PermissionState,
        allowUnrestricted: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Permissions::PermissionState,
    > {
        let __cordl_ret: crate::System::Security::Permissions::PermissionState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPermissionState", (state, allowUnrestricted))?;
        Ok(__cordl_ret.into())
    }
    pub fn Demand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Demand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSubsetOf(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Security::IPermission>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSubsetOf", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Security_IPermission_Demand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Security.IPermission.Demand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToXml(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::SecurityElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::SecurityElement,
        > = __cordl_object.invoke("ToXml", ())?;
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
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::CodeAccessPermission {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Security::IPermission>>
for crate::System::Security::CodeAccessPermission {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Security::IPermission> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Security::IPermission>>
for crate::System::Security::CodeAccessPermission {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Security::IPermission> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable>>
for crate::System::Security::CodeAccessPermission {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+CodeAccessPermission")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable>>
for crate::System::Security::CodeAccessPermission {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable> {
        unsafe { std::mem::transmute(self) }
    }
}
