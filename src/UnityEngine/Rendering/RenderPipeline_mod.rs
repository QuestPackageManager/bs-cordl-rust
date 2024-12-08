#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderPipeline {
    __cordl_parent: crate::System::Object,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderPipeline =>
    "UnityEngine.Rendering"."RenderPipeline"
);
#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderPipeline {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
impl crate::UnityEngine::Rendering::RenderPipeline {
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
    pub fn InternalProcessRenderRequests<RequestData>(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: *mut crate::UnityEngine::Camera,
        renderRequest: RequestData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        RequestData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalProcessRenderRequests", (context, camera, renderRequest))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRender(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameras: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Camera,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRender", (context, cameras))?;
        Ok(__cordl_ret)
    }
    pub fn IsRenderRequestSupported<RequestData>(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        data: RequestData,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        RequestData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRenderRequestSupported", (camera, data))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRenderRequests<RequestData>(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: *mut crate::UnityEngine::Camera,
        renderRequest: RequestData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        RequestData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRenderRequests", (context, camera, renderRequest))?;
        Ok(__cordl_ret)
    }
    pub fn Render_Il2CppArray0(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameras: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (context, cameras))?;
        Ok(__cordl_ret)
    }
    pub fn Render_List_1_1(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameras: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Camera,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (context, cameras))?;
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
#[cfg(feature = "UnityEngine+Rendering+RenderPipeline")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderPipeline {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
