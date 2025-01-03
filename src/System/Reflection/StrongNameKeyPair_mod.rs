#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct StrongNameKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _publicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _keyPairContainer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _keyPairExported: bool,
    pub _keyPairArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::StrongNameKeyPair =>
    "System.Reflection"."StrongNameKeyPair"
);
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl std::ops::Deref for crate::System::Reflection::StrongNameKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl std::ops::DerefMut for crate::System::Reflection::StrongNameKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl crate::System::Reflection::StrongNameKeyPair {
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
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
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
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
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::StrongNameKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Reflection::StrongNameKeyPair {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Reflection::StrongNameKeyPair {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::StrongNameKeyPair {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+StrongNameKeyPair")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::StrongNameKeyPair {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
