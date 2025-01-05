#[cfg(feature = "System+Security+IPermission")]
#[repr(C)]
#[derive(Debug)]
pub struct IPermission {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+IPermission")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::IPermission =>
    "System.Security"."IPermission"
);
#[cfg(feature = "System+Security+IPermission")]
impl std::ops::Deref for crate::System::Security::IPermission {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl std::ops::DerefMut for crate::System::Security::IPermission {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl crate::System::Security::IPermission {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::IPermission {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable>>
for crate::System::Security::IPermission {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+IPermission")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable>>
for crate::System::Security::IPermission {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Security::ISecurityEncodable> {
        unsafe { std::mem::transmute(self) }
    }
}
