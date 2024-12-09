#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
#[repr(C)]
#[derive(Debug)]
pub struct Lightmapping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::Lightmapping =>
    "UnityEngine.Experimental.GlobalIllumination"."Lightmapping"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
impl crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping {
    #[cfg(
        feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
    )]
    pub type RequestLightsDelegate = crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate;
    #[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+__c")]
    pub type __c = crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping___c;
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Lightmapping_RequestLightsDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate
    => "UnityEngine.Experimental.GlobalIllumination"."Lightmapping/RequestLightsDelegate"
);
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
impl crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate {
    pub fn Invoke(
        &mut self,
        requests: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Light,
        >,
        lightsOutput: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (requests, lightsOutput))?;
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
#[cfg(
    feature = "UnityEngine+Experimental+GlobalIllumination+Lightmapping+RequestLightsDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::GlobalIllumination::Lightmapping_RequestLightsDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
