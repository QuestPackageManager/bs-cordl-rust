#[cfg(feature = "HMUI+CurvedTextMeshPro+CurvedMeshInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CurvedTextMeshPro_CurvedMeshInfo {
    pub uvs3: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
}
#[cfg(feature = "HMUI+CurvedTextMeshPro+CurvedMeshInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::CurvedTextMeshPro_CurvedMeshInfo => "HMUI"
    ."CurvedTextMeshPro/CurvedMeshInfo"
);
#[cfg(feature = "HMUI+CurvedTextMeshPro+CurvedMeshInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HMUI::CurvedTextMeshPro_CurvedMeshInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HMUI+CurvedTextMeshPro+CurvedMeshInfo")]
impl crate::HMUI::CurvedTextMeshPro_CurvedMeshInfo {}
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
#[repr(C)]
#[derive(Debug)]
pub struct CurvedTextMeshPro {
    __cordl_parent: crate::TMPro::TextMeshProUGUI,
    pub _useScriptableObjectColors: bool,
    pub _colorSo: *mut crate::GlobalNamespace::ColorSO,
    pub _curvedMeshInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HMUI::CurvedTextMeshPro_CurvedMeshInfo,
    >,
    pub _curvedCanvasSettingsHelper: *mut crate::HMUI::CurvedCanvasSettingsHelper,
}
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::CurvedTextMeshPro => "HMUI"
    ."CurvedTextMeshPro"
);
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
impl std::ops::Deref for crate::HMUI::CurvedTextMeshPro {
    type Target = crate::TMPro::TextMeshProUGUI;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
impl std::ops::DerefMut for crate::HMUI::CurvedTextMeshPro {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
impl crate::HMUI::CurvedTextMeshPro {
    #[cfg(feature = "HMUI+CurvedTextMeshPro+CurvedMeshInfo")]
    pub type CurvedMeshInfo = crate::HMUI::CurvedTextMeshPro_CurvedMeshInfo;
    pub fn FillColors(
        &mut self,
        meshIndex: i32,
        color32: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Color32,
        > = __cordl_object.invoke("FillColors", (meshIndex, color32))?;
        Ok(__cordl_ret)
    }
    pub fn FillUV3s(
        &mut self,
        meshIndex: i32,
        vertexCount: i32,
        curve: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector2,
        > = __cordl_object.invoke("FillUV3s", (meshIndex, vertexCount, curve))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTextMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTextMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMesh(
        &mut self,
        mesh: *mut crate::UnityEngine::Mesh,
        meshIndex: i32,
        curveUV: crate::UnityEngine::Vector2,
        color32: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMesh", (mesh, meshIndex, curveUV, color32))?;
        Ok(__cordl_ret)
    }
    pub fn __Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Refresh", ())?;
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useScriptableObjectColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useScriptableObjectColors", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useScriptableObjectColors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useScriptableObjectColors", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+CurvedTextMeshPro")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::CurvedTextMeshPro {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
