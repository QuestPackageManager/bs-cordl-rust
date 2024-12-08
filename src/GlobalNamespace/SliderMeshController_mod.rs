#[cfg(feature = "SliderMeshController")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMeshController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _numberOfFixedVertexPathSegments: i32,
    pub _controlPointDistancePerSqrtNotesDistance: f32,
    pub _middleAnchorPointOffsetAmount: f32,
    pub _middleControlPointZDistanceModifier: f32,
    pub _middleControlPointYDistanceModifier: f32,
    pub _middleControlPointXDistanceModifier: f32,
    pub _sliderMeshConstructor: *mut SliderMeshConstructor,
    pub _pathsHolder: *mut PathsHolder,
    pub _reusableAnchorsList: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub _reusableControlPointsArray4: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _reusableControlPointsArray2: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "SliderMeshController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderMeshController => ""."SliderMeshController"
);
#[cfg(feature = "SliderMeshController")]
impl std::ops::Deref for SliderMeshController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshController")]
impl std::ops::DerefMut for SliderMeshController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshController")]
impl SliderMeshController {
    pub const kDefaultGameNoteSize: f32 = 0.45f32;
    pub fn CreateBezierPathAndMesh(
        &mut self,
        sliderData: *mut SliderData,
        headNotePos: crate::UnityEngine::Vector3,
        tailNotePos: crate::UnityEngine::Vector3,
        jumpSpeed: f32,
        noteUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateBezierPathAndMesh",
                (sliderData, headNotePos, tailNotePos, jumpSpeed, noteUniformScale),
            )?;
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
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pathLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pathLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pathsHolder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PathsHolder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PathsHolder = __cordl_object
            .invoke("get_pathsHolder", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SliderMeshController")]
impl quest_hook::libil2cpp::ObjectType for SliderMeshController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
