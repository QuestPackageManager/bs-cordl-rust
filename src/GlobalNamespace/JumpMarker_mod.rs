#[cfg(feature = "JumpMarker")]
#[repr(C)]
#[derive(Debug)]
pub struct JumpMarker {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::Marker>,
    pub _destination: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::JumpDestinationMarker,
    >,
    pub _id_k__BackingField: crate::UnityEngine::PropertyName,
}
#[cfg(feature = "JumpMarker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::JumpMarker => ""."JumpMarker"
);
#[cfg(feature = "JumpMarker")]
impl std::ops::Deref for crate::GlobalNamespace::JumpMarker {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::Marker>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JumpMarker")]
impl std::ops::DerefMut for crate::GlobalNamespace::JumpMarker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JumpMarker")]
impl crate::GlobalNamespace::JumpMarker {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PropertyName = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jumpDestination(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::JumpDestinationMarker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::JumpDestinationMarker,
        > = __cordl_object.invoke("get_jumpDestination", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "JumpMarker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::JumpMarker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "JumpMarker")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification>>
for crate::GlobalNamespace::JumpMarker {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "JumpMarker")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification>>
for crate::GlobalNamespace::JumpMarker {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification> {
        unsafe { std::mem::transmute(self) }
    }
}
