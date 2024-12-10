#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _intervalBit_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::RuntimeElement =>
    "UnityEngine.Timeline"."RuntimeElement"
);
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl std::ops::Deref for crate::UnityEngine::Timeline::RuntimeElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::RuntimeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl crate::UnityEngine::Timeline::RuntimeElement {
    pub fn DisableAt(
        &mut self,
        localTime: f64,
        rootDuration: f64,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAt", (localTime, rootDuration, frameData))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateAt(
        &mut self,
        localTime: f64,
        frameData: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateAt", (localTime, frameData))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_intervalBit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_intervalBit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intervalEnd(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_intervalEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intervalStart(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_intervalStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_intervalBit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_intervalBit", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::RuntimeElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl AsRef<crate::UnityEngine::Timeline::IInterval>
for crate::UnityEngine::Timeline::RuntimeElement {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::IInterval {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+RuntimeElement")]
impl AsMut<crate::UnityEngine::Timeline::IInterval>
for crate::UnityEngine::Timeline::RuntimeElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::IInterval {
        unsafe { std::mem::transmute(self) }
    }
}
