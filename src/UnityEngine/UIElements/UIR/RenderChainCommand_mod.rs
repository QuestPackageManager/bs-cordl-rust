#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderChainCommand {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    >,
    pub owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub prev: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub next: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub closing: bool,
    pub _cordl_type: crate::UnityEngine::UIElements::UIR::CommandType,
    pub state: crate::UnityEngine::UIElements::UIR::State,
    pub mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    pub indexOffset: i32,
    pub indexCount: i32,
    pub callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChainCommand {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChainCommand";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::RenderChainCommand {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::RenderChainCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
impl crate::UnityEngine::UIElements::UIR::RenderChainCommand {
    pub fn Blit(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blit", (source, destination, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineScissorRects(
        r0: crate::UnityEngine::Rect,
        r1: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineScissorRects", (r0, r1))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteNonDrawMesh(
        &mut self,
        drawParams: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::DrawParams,
        >,
        pixelsPerPoint: f32,
        immediateException: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ExecuteNonDrawMesh",
                (drawParams, pixelsPerPoint, immediateException),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RectPointsToPixelsAndFlipYAxis(
        rect: crate::UnityEngine::Rect,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RectInt> {
        let __cordl_ret: crate::UnityEngine::RectInt = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectPointsToPixelsAndFlipYAxis", (rect, pixelsPerPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::RenderChainCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
