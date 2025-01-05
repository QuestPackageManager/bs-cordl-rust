#[cfg(feature = "UnityEngine+GL")]
#[repr(C)]
#[derive(Debug)]
pub struct GL {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+GL")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GL => "UnityEngine"."GL"
);
#[cfg(feature = "UnityEngine+GL")]
impl std::ops::Deref for crate::UnityEngine::GL {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GL")]
impl std::ops::DerefMut for crate::UnityEngine::GL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GL")]
impl crate::UnityEngine::GL {
    pub fn Begin(
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Begin", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear__cordl_bool__cordl_bool_Color1(
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (clearDepth, clearColor, backgroundColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_f32_0(
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (clearDepth, clearColor, backgroundColor, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn Color(
        c: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Color", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn End() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("End", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GLClear(
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GLClear", (clearDepth, clearColor, backgroundColor, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn GLClear_Injected(
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GLClear_Injected",
                (clearDepth, clearColor, backgroundColor, depth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GLIssuePluginEvent(
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GLIssuePluginEvent", (callback, eventID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GLLoadPixelMatrixScript(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GLLoadPixelMatrixScript", (left, right, bottom, top))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImmediateColor(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImmediateColor", (r, g, b, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEvent(
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IssuePluginEvent", (callback, eventID))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOrtho() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadOrtho", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadPixelMatrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadPixelMatrix", (left, right, bottom, top))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadProjectionMatrix(
        mat: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadProjectionMatrix", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadProjectionMatrix_Injected(
        mat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadProjectionMatrix_Injected", (mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopMatrix() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PushMatrix() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewMatrix(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetViewMatrix", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewMatrix_Injected(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetViewMatrix_Injected", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn TexCoord(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TexCoord", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn TexCoord2(
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TexCoord2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn TexCoord3(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TexCoord3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Vertex3(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Vertex3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Viewport(
        pixelRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Viewport", (pixelRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn Viewport_Injected(
        pixelRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Viewport_Injected", (pixelRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_invertCulling() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_invertCulling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sRGBWrite() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sRGBWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_invertCulling(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_invertCulling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_modelview(
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_modelview", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sRGBWrite(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sRGBWrite", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GL")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
