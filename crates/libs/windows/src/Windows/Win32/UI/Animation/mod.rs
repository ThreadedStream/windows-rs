#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationInterpolator(::windows::core::IUnknown);
impl IUIAnimationInterpolator {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialValueAndVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(initialvelocity)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn InterpolateValue(&self, offset: f64) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterpolateValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn InterpolateVelocity(&self, offset: f64) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterpolateVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDependencies)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvaluedependencies), ::core::mem::transmute(initialvelocitydependencies), ::core::mem::transmute(durationdependencies)).ok()
    }
}
impl ::core::convert::From<IUIAnimationInterpolator> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationInterpolator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationInterpolator> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationInterpolator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationInterpolator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationInterpolator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationInterpolator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationInterpolator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationInterpolator {}
impl ::core::fmt::Debug for IUIAnimationInterpolator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationInterpolator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationInterpolator {
    type Vtable = IUIAnimationInterpolator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7815cbba_ddf7_478c_a46c_7b6c738b7978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationInterpolator2(::windows::core::IUnknown);
impl IUIAnimationInterpolator2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDimension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: &[f64], initialvelocity: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialValueAndVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(initialvalue)), ::core::mem::transmute(::windows::core::as_ptr_or_null(initialvelocity)), initialvelocity.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalValue(&self, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFinalValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(value)), value.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn InterpolateValue(&self, offset: f64, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InterpolateValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(value)), value.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn InterpolateVelocity(&self, offset: f64, velocity: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InterpolateVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(velocity)), velocity.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPrimitiveInterpolation<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPrimitiveInterpolation>>(&self, interpolation: Param0, cdimension: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrimitiveInterpolation)(::core::mem::transmute_copy(self), interpolation.into_param().abi(), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDependencies)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvaluedependencies), ::core::mem::transmute(initialvelocitydependencies), ::core::mem::transmute(durationdependencies)).ok()
    }
}
impl ::core::convert::From<IUIAnimationInterpolator2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationInterpolator2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationInterpolator2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationInterpolator2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationInterpolator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationInterpolator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationInterpolator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationInterpolator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationInterpolator2 {}
impl ::core::fmt::Debug for IUIAnimationInterpolator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationInterpolator2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationInterpolator2 {
    type Vtable = IUIAnimationInterpolator2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea76aff8_ea22_4a23_a0ef_a6a966703518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPrimitiveInterpolation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolation: ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationLoopIterationChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationLoopIterationChangeHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnLoopIterationChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLoopIterationChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(newiterationcount), ::core::mem::transmute(olditerationcount)).ok()
    }
}
impl ::core::convert::From<IUIAnimationLoopIterationChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationLoopIterationChangeHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationLoopIterationChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationLoopIterationChangeHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationLoopIterationChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationLoopIterationChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationLoopIterationChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationLoopIterationChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationLoopIterationChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationLoopIterationChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationLoopIterationChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationLoopIterationChangeHandler2 {
    type Vtable = IUIAnimationLoopIterationChangeHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3b15a4_4762_47ab_a030_b23221df3ae0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnLoopIterationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationManager(::windows::core::IUnknown);
impl IUIAnimationManager {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAnimationVariable)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationVariable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn ScheduleTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition>>(&self, variable: Param0, transition: Param1, timenow: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScheduleTransition)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), ::core::mem::transmute(timenow)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStoryboard)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinishAllStoryboards)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbandonAllStoryboards)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Update(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: UI_ANIMATION_UPDATE_RESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Update)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetVariableFromTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<IUIAnimationVariable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVariableFromTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationVariable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStoryboardFromTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStoryboardFromTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__: UI_ANIMATION_MANAGER_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_MANAGER_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAnimationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetManagerEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationManagerEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManagerEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetCancelPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCancelPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTrimPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrimPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetCompressPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompressPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetConcludePriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConcludePriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultLongestAcceptableDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUIAnimationManager> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationManager> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManager {}
impl ::core::fmt::Debug for IUIAnimationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationManager {
    type Vtable = IUIAnimationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9169896c_ac8d_4e7d_94e5_67fa4dc2f2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationManager2(::windows::core::IUnknown);
impl IUIAnimationManager2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateAnimationVectorVariable(&self, initialvalue: &[f64]) -> ::windows::core::Result<IUIAnimationVariable2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAnimationVectorVariable)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(initialvalue)), initialvalue.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::core::Result<IUIAnimationVariable2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAnimationVariable)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn ScheduleTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition2>>(&self, variable: Param0, transition: Param1, timenow: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScheduleTransition)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), ::core::mem::transmute(timenow)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStoryboard)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinishAllStoryboards)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbandonAllStoryboards)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Update(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: UI_ANIMATION_UPDATE_RESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Update)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetVariableFromTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<IUIAnimationVariable2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVariableFromTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStoryboardFromTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<IUIAnimationStoryboard2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStoryboardFromTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn EstimateNextEventTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EstimateNextEventTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__: UI_ANIMATION_MANAGER_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_MANAGER_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAnimationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManagerEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationManagerEventHandler2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManagerEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetCancelPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCancelPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTrimPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrimPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetCompressPriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompressPriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetConcludePriorityComparison<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConcludePriorityComparison)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultLongestAcceptableDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUIAnimationManager2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationManager2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManager2 {}
impl ::core::fmt::Debug for IUIAnimationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationManager2 {
    type Vtable = IUIAnimationManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8b6f7d4_4109_4d3f_acee_879926968cb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateAnimationVectorVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, timenow: f64) -> ::windows::core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EstimateNextEventTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManagerEventHandler: usize,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler(::windows::core::IUnknown);
impl IUIAnimationManagerEventHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnManagerStatusChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
impl ::core::convert::From<IUIAnimationManagerEventHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationManagerEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationManagerEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationManagerEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationManagerEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationManagerEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationManagerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManagerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManagerEventHandler {}
impl ::core::fmt::Debug for IUIAnimationManagerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManagerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationManagerEventHandler {
    type Vtable = IUIAnimationManagerEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x783321ed_78a3_4366_b574_6af607a64788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationManagerEventHandler2(::windows::core::IUnknown);
impl IUIAnimationManagerEventHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnManagerStatusChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
impl ::core::convert::From<IUIAnimationManagerEventHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationManagerEventHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationManagerEventHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationManagerEventHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationManagerEventHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationManagerEventHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationManagerEventHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationManagerEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationManagerEventHandler2 {}
impl ::core::fmt::Debug for IUIAnimationManagerEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationManagerEventHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationManagerEventHandler2 {
    type Vtable = IUIAnimationManagerEventHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e022ba_bff3_42ec_9033_e073f33e83c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationPrimitiveInterpolation(::windows::core::IUnknown);
impl IUIAnimationPrimitiveInterpolation {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddCubic)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimension), ::core::mem::transmute(beginoffset), ::core::mem::transmute(constantcoefficient), ::core::mem::transmute(linearcoefficient), ::core::mem::transmute(quadraticcoefficient), ::core::mem::transmute(cubiccoefficient)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSinusoidal)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimension), ::core::mem::transmute(beginoffset), ::core::mem::transmute(bias), ::core::mem::transmute(amplitude), ::core::mem::transmute(frequency), ::core::mem::transmute(phase)).ok()
    }
}
impl ::core::convert::From<IUIAnimationPrimitiveInterpolation> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationPrimitiveInterpolation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationPrimitiveInterpolation> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationPrimitiveInterpolation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationPrimitiveInterpolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationPrimitiveInterpolation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationPrimitiveInterpolation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPrimitiveInterpolation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPrimitiveInterpolation {}
impl ::core::fmt::Debug for IUIAnimationPrimitiveInterpolation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPrimitiveInterpolation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationPrimitiveInterpolation {
    type Vtable = IUIAnimationPrimitiveInterpolation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab20d63_4361_45da_a24f_ab8508846b5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPrimitiveInterpolation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison(::windows::core::IUnknown);
impl IUIAnimationPriorityComparison {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn HasPriority<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>>(&self, scheduledstoryboard: Param0, newstoryboard: Param1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HasPriority)(::core::mem::transmute_copy(self), scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), ::core::mem::transmute(priorityeffect)).ok()
    }
}
impl ::core::convert::From<IUIAnimationPriorityComparison> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationPriorityComparison) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationPriorityComparison> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationPriorityComparison) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationPriorityComparison {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationPriorityComparison {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationPriorityComparison {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPriorityComparison {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPriorityComparison {}
impl ::core::fmt::Debug for IUIAnimationPriorityComparison {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPriorityComparison").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationPriorityComparison {
    type Vtable = IUIAnimationPriorityComparison_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83fa9b74_5f86_4618_bc6a_a2fac19b3f44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationPriorityComparison2(::windows::core::IUnknown);
impl IUIAnimationPriorityComparison2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn HasPriority<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>>(&self, scheduledstoryboard: Param0, newstoryboard: Param1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HasPriority)(::core::mem::transmute_copy(self), scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), ::core::mem::transmute(priorityeffect)).ok()
    }
}
impl ::core::convert::From<IUIAnimationPriorityComparison2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationPriorityComparison2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationPriorityComparison2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationPriorityComparison2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationPriorityComparison2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationPriorityComparison2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationPriorityComparison2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationPriorityComparison2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationPriorityComparison2 {}
impl ::core::fmt::Debug for IUIAnimationPriorityComparison2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationPriorityComparison2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationPriorityComparison2 {
    type Vtable = IUIAnimationPriorityComparison2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b6d7a37_4621_467c_8b05_70131de62ddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: ::windows::core::RawPtr, newstoryboard: ::windows::core::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboard(::windows::core::IUnknown);
impl IUIAnimationStoryboard {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition>>(&self, variable: Param0, transition: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransition)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddKeyframeAtOffset<'a, Param0: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, existingkeyframe: Param0, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: UI_ANIMATION_KEYFRAME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddKeyframeAtOffset)(::core::mem::transmute_copy(self), existingkeyframe.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddKeyframeAfterTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationTransition>>(&self, transition: Param0) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: UI_ANIMATION_KEYFRAME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddKeyframeAfterTransition)(::core::mem::transmute_copy(self), transition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransitionAtKeyframe<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition>, Param2: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransitionAtKeyframe)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransitionBetweenKeyframes<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition>, Param2: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param3: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2, endkeyframe: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransitionBetweenKeyframes)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn RepeatBetweenKeyframes<'a, Param0: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param1: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, startkeyframe: Param0, endkeyframe: Param1, repetitioncount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RepeatBetweenKeyframes)(::core::mem::transmute_copy(self), startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), ::core::mem::transmute(repetitioncount)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn HoldVariable<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable>>(&self, variable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HoldVariable)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLongestAcceptableDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Schedule(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT> {
        let mut result__: UI_ANIMATION_SCHEDULING_RESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Schedule)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_SCHEDULING_RESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Conclude(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Conclude)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abandon)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__: UI_ANIMATION_STORYBOARD_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_STORYBOARD_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetElapsedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetElapsedTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetStoryboardEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboardEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStoryboardEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationStoryboard> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationStoryboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationStoryboard> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationStoryboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationStoryboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationStoryboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboard {}
impl ::core::fmt::Debug for IUIAnimationStoryboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboard {
    type Vtable = IUIAnimationStoryboard_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8ff128f_9bf9_4af1_9e67_e5e410defb84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboard2(::windows::core::IUnknown);
impl IUIAnimationStoryboard2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition2>>(&self, variable: Param0, transition: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransition)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddKeyframeAtOffset<'a, Param0: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, existingkeyframe: Param0, offset: f64) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: UI_ANIMATION_KEYFRAME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddKeyframeAtOffset)(::core::mem::transmute_copy(self), existingkeyframe.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddKeyframeAfterTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationTransition2>>(&self, transition: Param0) -> ::windows::core::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: UI_ANIMATION_KEYFRAME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddKeyframeAfterTransition)(::core::mem::transmute_copy(self), transition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransitionAtKeyframe<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition2>, Param2: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransitionAtKeyframe)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn AddTransitionBetweenKeyframes<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationTransition2>, Param2: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param3: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2, endkeyframe: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTransitionBetweenKeyframes)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RepeatBetweenKeyframes<'a, Param0: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param1: ::windows::core::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param4: ::windows::core::IntoParam<'a, IUIAnimationLoopIterationChangeHandler2>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, startkeyframe: Param0, endkeyframe: Param1, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: Param4, id: usize, fregisterfornextanimationevent: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RepeatBetweenKeyframes)(::core::mem::transmute_copy(self), startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), ::core::mem::transmute(crepetition), ::core::mem::transmute(repeatmode), piterationchangehandler.into_param().abi(), ::core::mem::transmute(id), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn HoldVariable<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>>(&self, variable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HoldVariable)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLongestAcceptableDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetSkipDuration(&self, secondsduration: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSkipDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(secondsduration)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Schedule(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_SCHEDULING_RESULT> {
        let mut result__: UI_ANIMATION_SCHEDULING_RESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Schedule)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_SCHEDULING_RESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Conclude(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Conclude)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abandon)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__: UI_ANIMATION_STORYBOARD_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_STORYBOARD_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetElapsedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetElapsedTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStoryboardEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboardEventHandler2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterstatuschangefornextanimationevent: Param1, fregisterupdatefornextanimationevent: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStoryboardEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterstatuschangefornextanimationevent.into_param().abi(), fregisterupdatefornextanimationevent.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationStoryboard2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationStoryboard2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationStoryboard2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationStoryboard2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationStoryboard2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationStoryboard2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationStoryboard2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboard2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboard2 {}
impl ::core::fmt::Debug for IUIAnimationStoryboard2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboard2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboard2 {
    type Vtable = IUIAnimationStoryboard2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae289cd2_12d4_4945_9419_9e41be034df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: ::windows::core::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr, transition: ::windows::core::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::core::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RepeatBetweenKeyframes: usize,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows::core::HRESULT,
    pub SetSkipDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows::core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStoryboardEventHandler: usize,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler(::windows::core::IUnknown);
impl IUIAnimationStoryboardEventHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnStoryboardStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>>(&self, storyboard: Param0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStoryboardStatusChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnStoryboardUpdated<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>>(&self, storyboard: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStoryboardUpdated)(::core::mem::transmute_copy(self), storyboard.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationStoryboardEventHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationStoryboardEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationStoryboardEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationStoryboardEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationStoryboardEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationStoryboardEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationStoryboardEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboardEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboardEventHandler {}
impl ::core::fmt::Debug for IUIAnimationStoryboardEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboardEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboardEventHandler {
    type Vtable = IUIAnimationStoryboardEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d5c9008_ec7c_4364_9f8a_9af3c58cbae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationStoryboardEventHandler2(::windows::core::IUnknown);
impl IUIAnimationStoryboardEventHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnStoryboardStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStoryboardStatusChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnStoryboardUpdated<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStoryboardUpdated)(::core::mem::transmute_copy(self), storyboard.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationStoryboardEventHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationStoryboardEventHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationStoryboardEventHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationStoryboardEventHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationStoryboardEventHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationStoryboardEventHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationStoryboardEventHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationStoryboardEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationStoryboardEventHandler2 {}
impl ::core::fmt::Debug for IUIAnimationStoryboardEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationStoryboardEventHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationStoryboardEventHandler2 {
    type Vtable = IUIAnimationStoryboardEventHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbac5f55a_ba7c_414c_b599_fbf850f553c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTimer(::windows::core::IUnknown);
impl IUIAnimationTimer {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTimerUpdateHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationTimerUpdateHandler>>(&self, updatehandler: Param0, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimerUpdateHandler)(::core::mem::transmute_copy(self), updatehandler.into_param().abi(), ::core::mem::transmute(idlebehavior)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTimerEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationTimerEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimerEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsEnabled)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetFrameRateThreshold(&self, framespersecond: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrameRateThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(framespersecond)).ok()
    }
}
impl ::core::convert::From<IUIAnimationTimer> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTimer> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimer {}
impl ::core::fmt::Debug for IUIAnimationTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTimer {
    type Vtable = IUIAnimationTimer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b0efad1_a053_41d6_9085_33a689144665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetTimerUpdateHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatehandler: ::windows::core::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::core::HRESULT,
    pub SetTimerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows::core::HRESULT,
    pub SetFrameRateThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTimerClientEventHandler(::windows::core::IUnknown);
impl IUIAnimationTimerClientEventHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTimerClientStatusChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
impl ::core::convert::From<IUIAnimationTimerClientEventHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTimerClientEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTimerClientEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTimerClientEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTimerClientEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTimerClientEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTimerClientEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerClientEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerClientEventHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerClientEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerClientEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerClientEventHandler {
    type Vtable = IUIAnimationTimerClientEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbedb4db6_94fa_4bfb_a47f_ef2d9e408c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerClientEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnTimerClientStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTimerEventHandler(::windows::core::IUnknown);
impl IUIAnimationTimerEventHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnPreUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPreUpdate)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnPostUpdate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPostUpdate)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnRenderingTooSlow(&self, framespersecond: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnRenderingTooSlow)(::core::mem::transmute_copy(self), ::core::mem::transmute(framespersecond)).ok()
    }
}
impl ::core::convert::From<IUIAnimationTimerEventHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTimerEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTimerEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTimerEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTimerEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTimerEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTimerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerEventHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerEventHandler {
    type Vtable = IUIAnimationTimerEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274a7dea_d771_4095_abbd_8df7abd23ce3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnPreUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPostUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnRenderingTooSlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTimerUpdateHandler(::windows::core::IUnknown);
impl IUIAnimationTimerUpdateHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnUpdate(&self, timenow: f64) -> ::windows::core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: UI_ANIMATION_UPDATE_RESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OnUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), ::core::mem::transmute(&mut result__)).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTimerClientEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationTimerClientEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimerClientEventHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn ClearTimerClientEventHandler(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearTimerClientEventHandler)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUIAnimationTimerUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTimerUpdateHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTimerUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTimerUpdateHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTimerUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTimerUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTimerUpdateHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTimerUpdateHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTimerUpdateHandler {}
impl ::core::fmt::Debug for IUIAnimationTimerUpdateHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTimerUpdateHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTimerUpdateHandler {
    type Vtable = IUIAnimationTimerUpdateHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x195509b7_5d5e_4e3e_b278_ee3759b367ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerUpdateHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::core::HRESULT,
    pub SetTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ClearTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransition(::windows::core::IUnknown);
impl IUIAnimationTransition {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn IsDurationKnown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDurationKnown)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransition> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransition> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransition {}
impl ::core::fmt::Debug for IUIAnimationTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransition {
    type Vtable = IUIAnimationTransition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc6ce252_f731_41cf_b610_614b6ca049ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransition2(::windows::core::IUnknown);
impl IUIAnimationTransition2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDimension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialVectorValue(&self, value: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(value)), value.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetInitialVectorVelocity(&self, velocity: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitialVectorVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(velocity)), velocity.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn IsDurationKnown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDurationKnown)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransition2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransition2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransition2 {}
impl ::core::fmt::Debug for IUIAnimationTransition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransition2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransition2 {
    type Vtable = IUIAnimationTransition2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62ff9123_a85a_4e9b_a218_435a93e268fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SetInitialVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows::core::HRESULT,
    pub SetInitialVectorVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory(::windows::core::IUnknown);
impl IUIAnimationTransitionFactory {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationInterpolator>>(&self, interpolator: Param0) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransition)(::core::mem::transmute_copy(self), interpolator.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransitionFactory> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransitionFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransitionFactory> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransitionFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransitionFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransitionFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionFactory {}
impl ::core::fmt::Debug for IUIAnimationTransitionFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionFactory {
    type Vtable = IUIAnimationTransitionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd91e03_3e3b_45ad_bbb1_6dfc8153743d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionFactory2(::windows::core::IUnknown);
impl IUIAnimationTransitionFactory2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateTransition<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationInterpolator2>>(&self, interpolator: Param0) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransition)(::core::mem::transmute_copy(self), interpolator.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransitionFactory2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransitionFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransitionFactory2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransitionFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransitionFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransitionFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransitionFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionFactory2 {}
impl ::core::fmt::Debug for IUIAnimationTransitionFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionFactory2 {
    type Vtable = IUIAnimationTransitionFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x937d4916_c1a6_42d5_88d8_30344d6efe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: ::windows::core::RawPtr, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary(::windows::core::IUnknown);
impl IUIAnimationTransitionLibrary {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstantaneousTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateConstantTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDiscreteTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(finalvalue), ::core::mem::transmute(hold), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearTransitionFromSpeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSinusoidalTransitionFromVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(period), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSinusoidalTransitionFromRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(minimumvalue), ::core::mem::transmute(maximumvalue), ::core::mem::transmute(period), ::core::mem::transmute(slope), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAccelerateDecelerateTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(accelerationratio), ::core::mem::transmute(decelerationratio), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReversalTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCubicTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSmoothStopTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(maximumduration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateParabolicTransitionFromAcceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(acceleration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransitionLibrary> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransitionLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransitionLibrary> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransitionLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransitionLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransitionLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransitionLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionLibrary {}
impl ::core::fmt::Debug for IUIAnimationTransitionLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionLibrary {
    type Vtable = IUIAnimationTransitionLibrary_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca5a14b1_d24f_48b8_8fe4_c78169ba954e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationTransitionLibrary2(::windows::core::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstantaneousTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateInstantaneousVectorTransition(&self, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstantaneousVectorTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), finalvalue.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateConstantTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDiscreteTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(finalvalue), ::core::mem::transmute(hold), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: &[f64], hold: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDiscreteVectorTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), finalvalue.len() as _, ::core::mem::transmute(hold), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearVectorTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), finalvalue.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearTransitionFromSpeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearVectorTransitionFromSpeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), finalvalue.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSinusoidalTransitionFromVelocity)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(period), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSinusoidalTransitionFromRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(minimumvalue), ::core::mem::transmute(maximumvalue), ::core::mem::transmute(period), ::core::mem::transmute(slope), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAccelerateDecelerateTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(accelerationratio), ::core::mem::transmute(decelerationratio), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReversalTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCubicTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: &[f64], finalvelocity: &[f64]) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCubicVectorTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvelocity)), finalvelocity.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSmoothStopTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(maximumduration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateParabolicTransitionFromAcceleration)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(acceleration), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCubicBezierLinearTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: &[f64], x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::core::Result<IUIAnimationTransition2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCubicBezierLinearVectorTransition)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(::windows::core::as_ptr_or_null(finalvalue)), finalvalue.len() as _, ::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationTransition2>(result__)
    }
}
impl ::core::convert::From<IUIAnimationTransitionLibrary2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationTransitionLibrary2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationTransitionLibrary2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationTransitionLibrary2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationTransitionLibrary2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationTransitionLibrary2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationTransitionLibrary2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationTransitionLibrary2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationTransitionLibrary2 {}
impl ::core::fmt::Debug for IUIAnimationTransitionLibrary2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationTransitionLibrary2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationTransitionLibrary2 {
    type Vtable = IUIAnimationTransitionLibrary2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03cfae53_9580_4ee3_b363_2ece51b4af6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstantaneousVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateDiscreteVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearVectorTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCubicVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCubicBezierLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCubicBezierLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariable(::windows::core::IUnknown);
impl IUIAnimationVariable {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentStoryboard)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowerBound)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpperBound)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRoundingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetVariableChangeHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariableChangeHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVariableChangeHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetVariableIntegerChangeHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariableIntegerChangeHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVariableIntegerChangeHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariable> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariable> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariable {}
impl ::core::fmt::Debug for IUIAnimationVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariable {
    type Vtable = IUIAnimationVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ceeb155_2849_4ce5_9448_91ff70e1e4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariable2(::windows::core::IUnknown);
impl IUIAnimationVariable2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetDimension(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDimension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetVectorValue(&self, value: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(value)), value.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Graphics_DirectComposition'*"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetCurve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectComposition::IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurve)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Graphics_DirectComposition'*"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetVectorCurve(&self, animation: &[::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVectorCurve)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(animation)), animation.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalVectorValue(&self, finalvalue: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFinalVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(finalvalue)), finalvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousValue(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousVectorValue(&self, previousvalue: &mut [f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPreviousVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(previousvalue)), previousvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetIntegerVectorValue(&self, value: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIntegerVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(value)), value.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFinalIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetFinalIntegerVectorValue(&self, finalvalue: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFinalIntegerVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(finalvalue)), finalvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviousIntegerValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetPreviousIntegerVectorValue(&self, previousvalue: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPreviousIntegerVectorValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(previousvalue)), previousvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::core::Result<IUIAnimationStoryboard2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentStoryboard)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowerBound)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetLowerBoundVector(&self, bound: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowerBoundVector)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(bound)), bound.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpperBound)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetUpperBoundVector(&self, bound: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpperBoundVector)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(bound)), bound.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRoundingMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::core::IUnknown>, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableChangeHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariableChangeHandler2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVariableChangeHandler)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableIntegerChangeHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariableIntegerChangeHandler2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVariableIntegerChangeHandler)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn SetVariableCurveChangeHandler<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariableCurveChangeHandler2>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVariableCurveChangeHandler)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariable2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariable2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariable2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariable2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariable2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariable2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariable2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariable2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariable2 {}
impl ::core::fmt::Debug for IUIAnimationVariable2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariable2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariable2 {
    type Vtable = IUIAnimationVariable2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4914b304_96ab_44d9_9e77_d5109b7e7466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows::core::HRESULT,
    pub GetVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetCurve: usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetVectorCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *const ::windows::core::RawPtr, cdimension: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetVectorCurve: usize,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetFinalVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows::core::HRESULT,
    pub GetPreviousVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub GetIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetFinalIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows::core::HRESULT,
    pub GetPreviousIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows::core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetLowerBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows::core::HRESULT,
    pub SetUpperBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableChangeHandler: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableIntegerChangeHandler: usize,
    pub SetVariableCurveChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler(::windows::core::IUnknown);
impl IUIAnimationVariableChangeHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnValueChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::core::IntoParam<'a, IUIAnimationVariable>>(&self, storyboard: Param0, variable: Param1, newvalue: f64, previousvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnValueChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue)).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariableChangeHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariableChangeHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariableChangeHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariableChangeHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariableChangeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariableChangeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariableChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableChangeHandler {}
impl ::core::fmt::Debug for IUIAnimationVariableChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableChangeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableChangeHandler {
    type Vtable = IUIAnimationVariableChangeHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6358b7ba_87d2_42d5_bf71_82e919dd5862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariableChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableChangeHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnValueChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationVariable2>>(&self, storyboard: Param0, variable: Param1, newvalue: &[f64], previousvalue: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnValueChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(newvalue)), ::core::mem::transmute(::windows::core::as_ptr_or_null(previousvalue)), previousvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariableChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariableChangeHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariableChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariableChangeHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariableChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariableChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariableChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableChangeHandler2 {
    type Vtable = IUIAnimationVariableChangeHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63acc8d2_6eae_4bb0_b879_586dd8cfbe42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariableCurveChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableCurveChangeHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnCurveChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationVariable2>>(&self, variable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCurveChanged)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariableCurveChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariableCurveChangeHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariableCurveChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariableCurveChangeHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariableCurveChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariableCurveChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariableCurveChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableCurveChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableCurveChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableCurveChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableCurveChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableCurveChangeHandler2 {
    type Vtable = IUIAnimationVariableCurveChangeHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72895e91_0145_4c21_9192_5aab40eddf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnCurveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler(::windows::core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnIntegerValueChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::core::IntoParam<'a, IUIAnimationVariable>>(&self, storyboard: Param0, variable: Param1, newvalue: i32, previousvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIntegerValueChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue)).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariableIntegerChangeHandler> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariableIntegerChangeHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariableIntegerChangeHandler> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariableIntegerChangeHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariableIntegerChangeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariableIntegerChangeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariableIntegerChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableIntegerChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableIntegerChangeHandler {}
impl ::core::fmt::Debug for IUIAnimationVariableIntegerChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableIntegerChangeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableIntegerChangeHandler {
    type Vtable = IUIAnimationVariableIntegerChangeHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3e1550_356e_44b0_99da_85ac6017865e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
pub struct IUIAnimationVariableIntegerChangeHandler2(::windows::core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler2 {
    #[doc = "*Required features: 'Win32_UI_Animation'*"]
    pub unsafe fn OnIntegerValueChanged<'a, Param0: ::windows::core::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::core::IntoParam<'a, IUIAnimationVariable2>>(&self, storyboard: Param0, variable: Param1, newvalue: &[i32], previousvalue: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIntegerValueChanged)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(newvalue)), ::core::mem::transmute(::windows::core::as_ptr_or_null(previousvalue)), previousvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IUIAnimationVariableIntegerChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: IUIAnimationVariableIntegerChangeHandler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIAnimationVariableIntegerChangeHandler2> for ::windows::core::IUnknown {
    fn from(value: &IUIAnimationVariableIntegerChangeHandler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIAnimationVariableIntegerChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUIAnimationVariableIntegerChangeHandler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIAnimationVariableIntegerChangeHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIAnimationVariableIntegerChangeHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAnimationVariableIntegerChangeHandler2 {}
impl ::core::fmt::Debug for IUIAnimationVariableIntegerChangeHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAnimationVariableIntegerChangeHandler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIAnimationVariableIntegerChangeHandler2 {
    type Vtable = IUIAnimationVariableIntegerChangeHandler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x829b6cf1_4f3a_4412_ae09_b243eb4c6b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: ::windows::core::RawPtr, variable: ::windows::core::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::core::HRESULT,
}
pub const UIAnimationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
pub const UIAnimationManager2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd25d8842_8884_4a4a_b321_091314379bdd);
pub const UIAnimationTimer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcd4a0c_06b6_4384_b768_0daa792c380e);
pub const UIAnimationTransitionFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9b1cdd_fcd7_419c_8b44_42fd17db1887);
pub const UIAnimationTransitionFactory2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84302f97_7f7b_4040_b190_72ac9d18e420);
pub const UIAnimationTransitionLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d6322ad_aa85_4ef5_a828_86d71067d145);
pub const UIAnimationTransitionLibrary2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x812f944a_c5c8_4cd9_b0a6_b3da802f228d);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_DEPENDENCIES(pub u32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0u32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1u32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2u32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4u32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8u32);
impl ::core::marker::Copy for UI_ANIMATION_DEPENDENCIES {}
impl ::core::clone::Clone for UI_ANIMATION_DEPENDENCIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_DEPENDENCIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_DEPENDENCIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_DEPENDENCIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_DEPENDENCIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
impl ::core::marker::Copy for UI_ANIMATION_IDLE_BEHAVIOR {}
impl ::core::clone::Clone for UI_ANIMATION_IDLE_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_IDLE_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_IDLE_BEHAVIOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_IDLE_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_IDLE_BEHAVIOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_KEYFRAME(pub isize);
impl UI_ANIMATION_KEYFRAME {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for UI_ANIMATION_KEYFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for UI_ANIMATION_KEYFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for UI_ANIMATION_KEYFRAME {}
impl ::core::fmt::Debug for UI_ANIMATION_KEYFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_KEYFRAME").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_KEYFRAME {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
impl ::core::marker::Copy for UI_ANIMATION_MANAGER_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_MANAGER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MANAGER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_MANAGER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_MANAGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MANAGER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_MODE(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
impl ::core::marker::Copy for UI_ANIMATION_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
impl ::core::marker::Copy for UI_ANIMATION_PRIORITY_EFFECT {}
impl ::core::clone::Clone for UI_ANIMATION_PRIORITY_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_PRIORITY_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_PRIORITY_EFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_PRIORITY_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_PRIORITY_EFFECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
impl ::core::marker::Copy for UI_ANIMATION_REPEAT_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_REPEAT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_REPEAT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_REPEAT_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_REPEAT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_REPEAT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
impl ::core::marker::Copy for UI_ANIMATION_ROUNDING_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_ROUNDING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_ROUNDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_ROUNDING_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_ROUNDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_ROUNDING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
impl ::core::marker::Copy for UI_ANIMATION_SCHEDULING_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_SCHEDULING_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SCHEDULING_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_SCHEDULING_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_SCHEDULING_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SCHEDULING_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_SLOPE(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
impl ::core::marker::Copy for UI_ANIMATION_SLOPE {}
impl ::core::clone::Clone for UI_ANIMATION_SLOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SLOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_SLOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_SLOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SLOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
impl ::core::marker::Copy for UI_ANIMATION_STORYBOARD_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_STORYBOARD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_STORYBOARD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_STORYBOARD_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_STORYBOARD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_STORYBOARD_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
impl ::core::marker::Copy for UI_ANIMATION_TIMER_CLIENT_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_TIMER_CLIENT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_TIMER_CLIENT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_UI_Animation'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
#[doc = "*Required features: 'Win32_UI_Animation'*"]
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);
impl ::core::marker::Copy for UI_ANIMATION_UPDATE_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_UPDATE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_UPDATE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_ANIMATION_UPDATE_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_ANIMATION_UPDATE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_UPDATE_RESULT").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
