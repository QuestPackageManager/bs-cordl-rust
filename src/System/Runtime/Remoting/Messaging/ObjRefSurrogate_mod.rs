#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjRefSurrogate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ObjRefSurrogate =>
    "System.Runtime.Remoting.Messaging"."ObjRefSurrogate"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    pub fn GetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        si: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        sc: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (obj, si, sc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        si: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        sc: crate::System::Runtime::Serialization::StreamingContext,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("SetObjectData", (obj, si, sc, selector))?;
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl AsRef<crate::System::Runtime::Serialization::ISerializationSurrogate>
for crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ObjRefSurrogate")]
impl AsMut<crate::System::Runtime::Serialization::ISerializationSurrogate>
for crate::System::Runtime::Remoting::Messaging::ObjRefSurrogate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
