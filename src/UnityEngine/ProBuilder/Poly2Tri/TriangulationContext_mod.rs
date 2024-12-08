#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct TriangulationContext {
    __cordl_parent: crate::System::Object,
    pub _DebugContext_k__BackingField: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext,
    pub Triangles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    >,
    pub Points: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    >,
    pub _TriangulationMode_k__BackingField: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode,
    pub _Triangulatable_k__BackingField: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
    pub _StepCount_k__BackingField: i32,
    pub _IsDebugEnabled_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext =>
    "UnityEngine.ProBuilder.Poly2Tri"."TriangulationContext"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Done(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Done", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NewConstraint(
        &mut self,
        a: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        b: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint = __cordl_object
            .invoke("NewConstraint", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn PrepareTriangulation(
        &mut self,
        t: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareTriangulation", (t))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (message))?;
        Ok(__cordl_ret)
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
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DTDebugContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext = __cordl_object
            .invoke("get_DTDebugContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DebugContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext = __cordl_object
            .invoke("get_DebugContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDebugEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDebugEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StepCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_StepCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Triangulatable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable = __cordl_object
            .invoke("get_Triangulatable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TriangulationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode = __cordl_object
            .invoke("get_TriangulationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DebugContext(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DebugContext", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDebugEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDebugEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_StepCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StepCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Triangulatable(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Triangulatable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TriangulationMode(
        &mut self,
        value: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TriangulationMode", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
