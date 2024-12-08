#[cfg(feature = "System+Net+ServiceNameStore")]
#[repr(C)]
#[derive(Debug)]
pub struct ServiceNameStore {
    __cordl_parent: crate::System::Object,
    pub serviceNames: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub serviceNameCollection: *mut crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection,
}
#[cfg(feature = "System+Net+ServiceNameStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServiceNameStore => "System.Net"
    ."ServiceNameStore"
);
#[cfg(feature = "System+Net+ServiceNameStore")]
impl std::ops::Deref for crate::System::Net::ServiceNameStore {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServiceNameStore")]
impl std::ops::DerefMut for crate::System::Net::ServiceNameStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServiceNameStore")]
impl crate::System::Net::ServiceNameStore {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "System+Net+ServiceNameStore")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ServiceNameStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
