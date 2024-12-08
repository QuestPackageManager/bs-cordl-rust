#[cfg(feature = "UnityEngine+ProBuilder+PolyShape+PolyEditMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyShape_PolyEditMode {
    Edit = 3i32,
    Height = 2i32,
    None = 0i32,
    Path = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape+PolyEditMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PolyShape_PolyEditMode
    => "UnityEngine.ProBuilder"."PolyShape/PolyEditMode"
);
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
#[repr(C)]
#[derive(Debug)]
pub struct PolyShape {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    pub m_Points: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub m_Extrude: f32,
    pub m_EditMode: crate::UnityEngine::ProBuilder::PolyShape_PolyEditMode,
    pub m_FlipNormals: bool,
    pub isOnGrid: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PolyShape =>
    "UnityEngine.ProBuilder"."PolyShape"
);
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::PolyShape {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::PolyShape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
impl crate::UnityEngine::ProBuilder::PolyShape {
    #[cfg(feature = "UnityEngine+ProBuilder+PolyShape+PolyEditMode")]
    pub type PolyEditMode = crate::UnityEngine::ProBuilder::PolyShape_PolyEditMode;
    pub fn IsSnapEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSnapEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetControlPoints(
        &mut self,
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetControlPoints", (points))?;
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
    pub fn get_controlPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            crate::UnityEngine::Vector3,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            crate::UnityEngine::Vector3,
        > = __cordl_object.invoke("get_controlPoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_extrude(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_extrude", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flipNormals(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipNormals", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_polyEditMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::PolyShape_PolyEditMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::PolyShape_PolyEditMode = __cordl_object
            .invoke("get_polyEditMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_extrude(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_extrude", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_flipNormals(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipNormals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_mesh(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mesh", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_polyEditMode(
        &mut self,
        value: crate::UnityEngine::ProBuilder::PolyShape_PolyEditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_polyEditMode", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PolyShape")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::PolyShape {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
