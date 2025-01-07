#[cfg(feature = "System+DelegateSerializationHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateSerializationHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _delegate: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
}
#[cfg(feature = "System+DelegateSerializationHolder")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DelegateSerializationHolder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DelegateSerializationHolder";
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
#[cfg(feature = "System+DelegateSerializationHolder")]
impl std::ops::Deref for crate::System::DelegateSerializationHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetDelegateData(
        instance: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDelegateData", (instance, info, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetRealObject", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, ctx))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, ctx))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+DelegateSerializationHolder")]
impl AsRef<crate::System::Runtime::Serialization::IObjectReference>
for crate::System::DelegateSerializationHolder {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::IObjectReference {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl AsMut<crate::System::Runtime::Serialization::IObjectReference>
for crate::System::DelegateSerializationHolder {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IObjectReference {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DelegateSerializationHolder {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DelegateSerializationHolder {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateSerializationHolder_DelegateEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub assembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub targetTypeAssembly: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub targetTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub delegateEntry: quest_hook::libil2cpp::Gc<
        crate::System::DelegateSerializationHolder_DelegateEntry,
    >,
}
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::DelegateSerializationHolder_DelegateEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DelegateEntry";
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
#[cfg(feature = "System+DelegateSerializationHolder+DelegateEntry")]
impl std::ops::Deref for crate::System::DelegateSerializationHolder_DelegateEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = __cordl_object
            .invoke("DeserializeDelegate", (info, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        del: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        targetLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (del, targetLabel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        del: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        targetLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (del, targetLabel))?;
        Ok(__cordl_ret.into())
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
