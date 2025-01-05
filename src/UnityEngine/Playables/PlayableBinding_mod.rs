#[cfg(feature = "UnityEngine+Playables+PlayableBinding")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlayableBinding {
    pub m_StreamName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_SourceObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub m_SourceBindingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_CreateOutputMethod: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod,
    >,
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableBinding =>
    "UnityEngine.Playables"."PlayableBinding"
);
#[cfg(feature = "UnityEngine+Playables+PlayableBinding")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::PlayableBinding {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding")]
impl crate::UnityEngine::Playables::PlayableBinding {
    #[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
    pub type CreateOutputMethod = crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod;
    pub fn CreateInternal(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        createFunction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableBinding> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableBinding = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInternal", (name, sourceObject, sourceType, createFunction))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOutput(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateOutput",
            (graph),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sourceObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sourceObject",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_streamName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_streamName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableBinding_CreateOutputMethod {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Playables::PlayableBinding_CreateOutputMethod =>
    "UnityEngine.Playables"."PlayableBinding/CreateOutputMethod"
);
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
impl std::ops::Deref
for crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
impl std::ops::DerefMut
for crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
impl crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod {
    pub fn Invoke(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = __cordl_object
            .invoke("Invoke", (graph, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBinding+CreateOutputMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableBinding_CreateOutputMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
