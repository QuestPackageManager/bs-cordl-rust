#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OpacityIdAccelerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Jobs: crate::Unity::Collections::NativeArray_1<crate::Unity::Jobs::JobHandle>,
    pub m_NextJobIndex: i32,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::OpacityIdAccelerator => "UnityEngine.UIElements.UIR"
    ."OpacityIdAccelerator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    #[cfg(
        feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob"
    )]
    pub type OpacityIdUpdateJob = crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob;
    pub fn CompleteJobs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteJobs", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateJob(
        &mut self,
        oldVerts: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
        newVerts: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
        opacityData: crate::UnityEngine::Color32,
        vertexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateJob", (oldVerts, newVerts, opacityData, vertexCount))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OpacityIdAccelerator_OpacityIdUpdateJob {
    pub oldVerts: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub newVerts: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub opacityData: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob =>
    "UnityEngine.UIElements.UIR"."OpacityIdAccelerator/OpacityIdUpdateJob"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
impl crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
        Ok(__cordl_ret)
    }
}
