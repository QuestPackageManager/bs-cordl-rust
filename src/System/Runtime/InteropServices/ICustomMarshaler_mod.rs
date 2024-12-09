#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
#[repr(C)]
#[derive(Debug)]
pub struct ICustomMarshaler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ICustomMarshaler =>
    "System.Runtime.InteropServices"."ICustomMarshaler"
);
#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::ICustomMarshaler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::ICustomMarshaler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
impl crate::System::Runtime::InteropServices::ICustomMarshaler {
    pub fn CleanUpManagedData(
        &mut self,
        ManagedObj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUpManagedData", (ManagedObj))?;
        Ok(__cordl_ret)
    }
    pub fn CleanUpNativeData(
        &mut self,
        pNativeData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUpNativeData", (pNativeData))?;
        Ok(__cordl_ret)
    }
    pub fn GetNativeDataSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNativeDataSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarshalManagedToNative(
        &mut self,
        ManagedObj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("MarshalManagedToNative", (ManagedObj))?;
        Ok(__cordl_ret)
    }
    pub fn MarshalNativeToManaged(
        &mut self,
        pNativeData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("MarshalNativeToManaged", (pNativeData))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ICustomMarshaler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ICustomMarshaler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
