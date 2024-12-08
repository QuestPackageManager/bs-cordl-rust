#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateSerializationHolder_DelegateEntry {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: *mut crate::System::String,
    pub assembly: *mut crate::System::String,
    pub target: *mut crate::System::Object,
    pub targetTypeAssembly: *mut crate::System::String,
    pub targetTypeName: *mut crate::System::String,
    pub methodName: *mut crate::System::String,
    pub delegateEntry: *mut crate::System::DelegateSerializationHolder_DelegateEntry,
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::DelegateSerializationHolder_DelegateEntry => "System"
    ."DelegateSerializationHolder/DelegateEntry"
);
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
impl std::ops::Deref for crate::System::DelegateSerializationHolder_DelegateEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
impl std::ops::DerefMut for crate::System::DelegateSerializationHolder_DelegateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
impl crate::System::DelegateSerializationHolder_DelegateEntry {
    pub fn DeserializeDelegate(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Delegate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Delegate = __cordl_object
            .invoke("DeserializeDelegate", (info, index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        del: *mut crate::System::Delegate,
        targetLabel: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (del, targetLabel))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        del: *mut crate::System::Delegate,
        targetLabel: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (del, targetLabel))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::DelegateSerializationHolder_DelegateEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateSerializationHolder {
    __cordl_parent: crate::System::Object,
    pub _delegate: *mut crate::System::Delegate,
}
#[cfg(feature = "System+DelegateSerializationHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DelegateSerializationHolder => "System"
    ."DelegateSerializationHolder"
);
#[cfg(feature = "System+DelegateSerializationHolder")]
impl std::ops::Deref for crate::System::DelegateSerializationHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl std::ops::DerefMut for crate::System::DelegateSerializationHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl crate::System::DelegateSerializationHolder {
    #[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
    pub type DelegateEntry = crate::System::DelegateSerializationHolder_DelegateEntry;
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetRealObject", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, ctx))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, ctx))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DelegateSerializationHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
