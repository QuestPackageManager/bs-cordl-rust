#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtensionDataSetter {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::ExtensionDataSetter =>
    "Newtonsoft.Json.Serialization"."ExtensionDataSetter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::ExtensionDataSetter {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::ExtensionDataSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
impl crate::Newtonsoft::Json::Serialization::ExtensionDataSetter {
    pub fn BeginInvoke(
        &mut self,
        o: *mut quest_hook::libil2cpp::Il2CppObject,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        callback: *mut crate::System::AsyncCallback,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (o, key, value, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        o: *mut quest_hook::libil2cpp::Il2CppObject,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (o, key, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ExtensionDataSetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::ExtensionDataSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
