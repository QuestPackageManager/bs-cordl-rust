#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SpanAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub _IsReadOnly_k__BackingField: bool,
    pub _SizeParameter_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::SpanAttribute =>
    "UnityEngine.Bindings"."SpanAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::SpanAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::SpanAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
impl crate::UnityEngine::Bindings::SpanAttribute {
    pub fn New(
        sizeParameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sizeParameter, isReadOnly))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sizeParameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sizeParameter, isReadOnly))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Bindings+SpanAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Bindings::SpanAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
