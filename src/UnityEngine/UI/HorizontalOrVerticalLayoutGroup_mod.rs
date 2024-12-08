#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct HorizontalOrVerticalLayoutGroup {
    __cordl_parent: crate::UnityEngine::UI::LayoutGroup,
    pub m_Spacing: f32,
    pub m_ChildForceExpandWidth: bool,
    pub m_ChildForceExpandHeight: bool,
    pub m_ChildControlWidth: bool,
    pub m_ChildControlHeight: bool,
    pub m_ChildScaleWidth: bool,
    pub m_ChildScaleHeight: bool,
    pub m_ReverseArrangement: bool,
}
#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::HorizontalOrVerticalLayoutGroup
    => "UnityEngine.UI"."HorizontalOrVerticalLayoutGroup"
);
#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
impl std::ops::Deref for crate::UnityEngine::UI::HorizontalOrVerticalLayoutGroup {
    type Target = crate::UnityEngine::UI::LayoutGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
impl std::ops::DerefMut for crate::UnityEngine::UI::HorizontalOrVerticalLayoutGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
impl crate::UnityEngine::UI::HorizontalOrVerticalLayoutGroup {
    pub fn CalcAlongAxis(
        &mut self,
        axis: i32,
        isVertical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcAlongAxis", (axis, isVertical))?;
        Ok(__cordl_ret)
    }
    pub fn GetChildSizes(
        &mut self,
        child: *mut crate::UnityEngine::RectTransform,
        axis: i32,
        controlSize: bool,
        childForceExpand: bool,
        min: quest_hook::libil2cpp::ByRefMut<f32>,
        preferred: quest_hook::libil2cpp::ByRefMut<f32>,
        flexible: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetChildSizes",
                (child, axis, controlSize, childForceExpand, min, preferred, flexible),
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
    pub fn SetChildrenAlongAxis(
        &mut self,
        axis: i32,
        isVertical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChildrenAlongAxis", (axis, isVertical))?;
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
    pub fn get_childControlHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childControlHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childControlWidth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childControlWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childForceExpandHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childForceExpandHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childForceExpandWidth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childForceExpandWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childScaleHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childScaleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childScaleWidth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_childScaleWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_reverseArrangement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reverseArrangement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spacing(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_childControlHeight(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childControlHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_childControlWidth(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childControlWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_childForceExpandHeight(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childForceExpandHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_childForceExpandWidth(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childForceExpandWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_childScaleHeight(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childScaleHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_childScaleWidth(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_childScaleWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_reverseArrangement(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reverseArrangement", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_spacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spacing", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+HorizontalOrVerticalLayoutGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::HorizontalOrVerticalLayoutGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
