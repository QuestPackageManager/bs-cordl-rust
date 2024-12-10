#[cfg(feature = "System+Security+ISecurityEncodable")]
#[repr(C)]
#[derive(Debug)]
pub struct ISecurityEncodable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+ISecurityEncodable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::ISecurityEncodable =>
    "System.Security"."ISecurityEncodable"
);
#[cfg(feature = "System+Security+ISecurityEncodable")]
impl std::ops::Deref for crate::System::Security::ISecurityEncodable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+ISecurityEncodable")]
impl std::ops::DerefMut for crate::System::Security::ISecurityEncodable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+ISecurityEncodable")]
impl crate::System::Security::ISecurityEncodable {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Security+ISecurityEncodable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::ISecurityEncodable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
