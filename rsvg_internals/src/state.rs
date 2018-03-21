use cairo;
use glib::translate::*;
use glib_sys;
use libc::{self, c_int};
use pango;
use pango_sys;

use attributes::Attribute;
use color::{Color, ColorSpec};
use error::*;
use length::{RsvgLength, StrokeDasharray};
use node::RsvgNode;
use opacity::{Opacity, OpacitySpec};
use paint_server::PaintServer;
use parsers::{Parse, ParseError};
use util::utf8_cstr;

pub enum RsvgState {}

#[derive(Clone)]
pub struct State {
    join: StrokeLinejoin,
    has_join: bool,
}

impl Default for State {
    fn default() -> State {
        State {
            join: Default::default(),
            has_join: false,
        }
    }
}

impl State {
    fn parse_style_pair(&mut self, attr: Attribute, value: &str) -> Result<(), AttributeError> {
        match attr {
            _ => {
                // Maybe it's an attribute not parsed here, but in the
                // node implementations.
                Ok(())
            },
        }
    }
}

// Keep in sync with rsvg-styles.h:UnicodeBidi
#[repr(C)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Override,
}

// Keep in sync with rsvg-styles.h:TextDecoration
#[repr(C)]
#[derive(Copy, Clone)]
struct TextDecoration {
    overline: glib_sys::gboolean,
    underline: glib_sys::gboolean,
    strike: glib_sys::gboolean,
}

pub struct FontDecor {
    pub overline: bool,
    pub underline: bool,
    pub strike: bool,
}

impl From<TextDecoration> for FontDecor {
    fn from(td: TextDecoration) -> FontDecor {
        FontDecor {
            overline: from_glib(td.overline),
            underline: from_glib(td.underline),
            strike: from_glib(td.strike),
        }
    }
}

extern "C" {
    fn rsvg_state_new() -> *mut RsvgState;
    fn rsvg_state_free(state: *mut RsvgState);
    fn rsvg_state_reinit(state: *mut RsvgState);
    fn rsvg_state_reconstruct(state: *mut RsvgState, node: *const RsvgNode);
    fn rsvg_state_get_affine(state: *const RsvgState) -> cairo::Matrix;
    fn rsvg_state_is_overflow(state: *const RsvgState) -> glib_sys::gboolean;
    fn rsvg_state_has_overflow(state: *const RsvgState) -> glib_sys::gboolean;
    fn rsvg_state_get_cond_true(state: *const RsvgState) -> glib_sys::gboolean;
    fn rsvg_state_set_cond_true(state: *const RsvgState, cond_true: glib_sys::gboolean);
    fn rsvg_state_get_stop_color(state: *const RsvgState) -> *const ColorSpec;
    fn rsvg_state_get_stop_opacity(state: *const RsvgState) -> *const OpacitySpec;
    fn rsvg_state_get_stroke_dasharray(state: *const RsvgState) -> *const StrokeDasharray;
    fn rsvg_state_get_dash_offset(state: *const RsvgState) -> RsvgLength;
    fn rsvg_state_get_current_color(state: *const RsvgState) -> u32;
    fn rsvg_state_get_shape_rendering_type(state: *const RsvgState) -> cairo::Antialias;
    fn rsvg_state_get_text_rendering_type(state: *const RsvgState) -> cairo::Antialias;

    fn rsvg_state_get_stroke(state: *const RsvgState) -> *const PaintServer;
    fn rsvg_state_get_stroke_opacity(state: *const RsvgState) -> u8;
    fn rsvg_state_get_stroke_width(state: *const RsvgState) -> RsvgLength;
    fn rsvg_state_get_miter_limit(state: *const RsvgState) -> f64;
    fn rsvg_state_get_line_cap(state: *const RsvgState) -> cairo::LineCap;
    fn rsvg_state_get_line_join(state: *const RsvgState) -> cairo::LineJoin;
    fn rsvg_state_get_language(state: *const RsvgState) -> *const libc::c_char;
    fn rsvg_state_get_unicode_bidi(state: *const RsvgState) -> UnicodeBidi;
    fn rsvg_state_get_text_dir(state: *const RsvgState) -> pango_sys::PangoDirection;
    fn rsvg_state_get_text_gravity(state: *const RsvgState) -> pango_sys::PangoGravity;
    fn rsvg_state_get_font_family(state: *const RsvgState) -> *const libc::c_char;
    fn rsvg_state_get_font_style(state: *const RsvgState) -> pango_sys::PangoStyle;
    fn rsvg_state_get_font_variant(state: *const RsvgState) -> pango_sys::PangoVariant;
    fn rsvg_state_get_font_weight(state: *const RsvgState) -> pango_sys::PangoWeight;
    fn rsvg_state_get_font_stretch(state: *const RsvgState) -> pango_sys::PangoStretch;
    fn rsvg_state_get_letter_spacing(state: *const RsvgState) -> RsvgLength;
    fn rsvg_state_get_font_decor(state: *const RsvgState) -> *const TextDecoration;
    fn rsvg_state_get_clip_rule(state: *const RsvgState) -> cairo::FillRule;
    fn rsvg_state_get_fill(state: *const RsvgState) -> *const PaintServer;
    fn rsvg_state_get_fill_opacity(state: *const RsvgState) -> u8;
    fn rsvg_state_get_fill_rule(state: *const RsvgState) -> cairo::FillRule;

    fn rsvg_state_get_state_rust(state: *const RsvgState) -> *mut State;
}

pub fn new() -> *mut RsvgState {
    unsafe { rsvg_state_new() }
}

pub fn free(state: *mut RsvgState) {
    unsafe {
        rsvg_state_free(state);
    }
}

pub fn reinit(state: *mut RsvgState) {
    unsafe {
        rsvg_state_reinit(state);
    }
}

pub fn reconstruct(state: *mut RsvgState, node: *const RsvgNode) {
    unsafe {
        rsvg_state_reconstruct(state, node);
    }
}

pub fn get_affine(state: *const RsvgState) -> cairo::Matrix {
    unsafe { rsvg_state_get_affine(state) }
}

pub fn is_overflow(state: *const RsvgState) -> bool {
    unsafe { from_glib(rsvg_state_is_overflow(state)) }
}

pub fn has_overflow(state: *const RsvgState) -> bool {
    unsafe { from_glib(rsvg_state_has_overflow(state)) }
}

pub fn get_cond_true(state: *const RsvgState) -> bool {
    unsafe { from_glib(rsvg_state_get_cond_true(state)) }
}

pub fn set_cond_true(state: *const RsvgState, cond_true: bool) {
    unsafe {
        rsvg_state_set_cond_true(state, cond_true.to_glib());
    }
}

pub fn get_stop_color(state: *const RsvgState) -> Result<Option<Color>, AttributeError> {
    unsafe {
        let spec_ptr = rsvg_state_get_stop_color(state);

        if spec_ptr.is_null() {
            Ok(None)
        } else {
            Color::from_color_spec(&*spec_ptr).map(Some)
        }
    }
}

pub fn get_stop_opacity(state: *const RsvgState) -> Result<Option<Opacity>, AttributeError> {
    unsafe {
        let opacity_ptr = rsvg_state_get_stop_opacity(state);

        if opacity_ptr.is_null() {
            Ok(None)
        } else {
            Opacity::from_opacity_spec(&*opacity_ptr).map(Some)
        }
    }
}

pub fn get_stroke_dasharray<'a>(state: *const RsvgState) -> Option<&'a StrokeDasharray> {
    let dash = unsafe { rsvg_state_get_stroke_dasharray(state) };

    if dash.is_null() {
        None
    } else {
        unsafe { Some(&*dash) }
    }
}

pub fn get_dash_offset(state: *const RsvgState) -> RsvgLength {
    unsafe { rsvg_state_get_dash_offset(state) }
}

pub fn get_current_color(state: *const RsvgState) -> Color {
    let argb = unsafe { rsvg_state_get_current_color(state) };

    Color::from(argb)
}

pub fn get_shape_rendering_type(state: *const RsvgState) -> cairo::Antialias {
    unsafe { rsvg_state_get_shape_rendering_type(state) }
}

pub fn get_text_rendering_type(state: *const RsvgState) -> cairo::Antialias {
    unsafe { rsvg_state_get_text_rendering_type(state) }
}

pub fn get_stroke<'a>(state: *const RsvgState) -> Option<&'a PaintServer> {
    unsafe {
        let ps = rsvg_state_get_stroke(state);

        if ps.is_null() {
            None
        } else {
            Some(&*ps)
        }
    }
}

pub fn get_stroke_opacity(state: *const RsvgState) -> u8 {
    unsafe { rsvg_state_get_stroke_opacity(state) }
}

pub fn get_stroke_width(state: *const RsvgState) -> RsvgLength {
    unsafe { rsvg_state_get_stroke_width(state) }
}

pub fn get_miter_limit(state: *const RsvgState) -> f64 {
    unsafe { rsvg_state_get_miter_limit(state) }
}

pub fn get_line_cap(state: *const RsvgState) -> cairo::LineCap {
    unsafe { rsvg_state_get_line_cap(state) }
}

pub fn get_line_join(state: *const RsvgState) -> cairo::LineJoin {
    unsafe { rsvg_state_get_line_join(state) }
}

pub fn get_language(state: *const RsvgState) -> Option<String> {
    unsafe { from_glib_none(rsvg_state_get_language(state)) }
}

pub fn get_unicode_bidi(state: *const RsvgState) -> UnicodeBidi {
    unsafe { rsvg_state_get_unicode_bidi(state) }
}

pub fn get_text_dir(state: *const RsvgState) -> pango::Direction {
    unsafe { from_glib(rsvg_state_get_text_dir(state)) }
}

pub fn get_text_gravity(state: *const RsvgState) -> pango::Gravity {
    unsafe { from_glib(rsvg_state_get_text_gravity(state)) }
}

pub fn get_font_family(state: *const RsvgState) -> Option<String> {
    unsafe { from_glib_none(rsvg_state_get_font_family(state)) }
}

pub fn get_font_style(state: *const RsvgState) -> pango::Style {
    unsafe { from_glib(rsvg_state_get_font_style(state)) }
}

pub fn get_font_variant(state: *const RsvgState) -> pango::Variant {
    unsafe { from_glib(rsvg_state_get_font_variant(state)) }
}

pub fn get_font_weight(state: *const RsvgState) -> pango::Weight {
    unsafe { from_glib(rsvg_state_get_font_weight(state)) }
}

pub fn get_font_stretch(state: *const RsvgState) -> pango::Stretch {
    unsafe { from_glib(rsvg_state_get_font_stretch(state)) }
}

pub fn get_letter_spacing(state: *const RsvgState) -> RsvgLength {
    unsafe { rsvg_state_get_letter_spacing(state) }
}

pub fn get_font_decor(state: *const RsvgState) -> Option<FontDecor> {
    unsafe {
        let td = rsvg_state_get_font_decor(state);
        if td.is_null() {
            None
        } else {
            Some(FontDecor::from(*td))
        }
    }
}

pub fn get_clip_rule(state: *const RsvgState) -> cairo::FillRule {
    unsafe { rsvg_state_get_clip_rule(state) }
}

pub fn get_fill<'a>(state: *const RsvgState) -> Option<&'a PaintServer> {
    unsafe {
        let ps = rsvg_state_get_fill(state);

        if ps.is_null() {
            None
        } else {
            Some(&*ps)
        }
    }
}

pub fn get_fill_opacity(state: *const RsvgState) -> u8 {
    unsafe { rsvg_state_get_fill_opacity(state) }
}

pub fn get_fill_rule(state: *const RsvgState) -> cairo::FillRule {
    unsafe { rsvg_state_get_fill_rule(state) }
}

pub fn get_state_rust<'a>(state: *const RsvgState) -> &'a mut State {
    unsafe { &mut *rsvg_state_get_state_rust(state) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum StrokeLinejoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}

#[repr(C)]
pub struct StrokeLinejoinResult {
    valid: glib_sys::gboolean,
    linejoin: StrokeLinejoin,
}

impl Default for StrokeLinejoin {
    fn default() -> StrokeLinejoin {
        StrokeLinejoin::Miter
    }
}

impl Parse for StrokeLinejoin {
    type Data = ();
    type Err = AttributeError;

    fn parse(s: &str, _: Self::Data) -> Result<StrokeLinejoin, AttributeError> {
        match s.trim() {
            "miter" => Ok(StrokeLinejoin::Miter),
            "round" => Ok(StrokeLinejoin::Round),
            "bevel" => Ok(StrokeLinejoin::Bevel),
            "inherit" => Ok(StrokeLinejoin::Inherit),
            _ => Err(AttributeError::from(ParseError::new("expected miter|round|bevel|inherit"))),
        }
    }
}

impl From<Result<StrokeLinejoin, AttributeError>> for StrokeLinejoinResult {
    fn from(v: Result<StrokeLinejoin, AttributeError>) -> StrokeLinejoinResult {
        match v {
            Ok(j) => StrokeLinejoinResult {
                valid: true.to_glib(),
                linejoin: j,
            },

            Err(_) => StrokeLinejoinResult {
                valid: false.to_glib(),
                linejoin: StrokeLinejoin::default(),
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn rsvg_stroke_linejoin_get_default() -> StrokeLinejoin {
    StrokeLinejoin::default()
}

#[no_mangle]
pub extern "C" fn rsvg_stroke_linejoin_parse(s: *const libc::c_char) -> StrokeLinejoinResult {
    let s = unsafe { utf8_cstr(s) };

    StrokeLinejoinResult::from(StrokeLinejoin::parse(s, ()))
}

#[no_mangle]
pub extern "C" fn rsvg_state_rust_new() -> *mut State {
    Box::into_raw(Box::new(State::default()))
}

#[no_mangle]
pub extern "C" fn rsvg_state_rust_free(state: *mut State) {
    assert!(!state.is_null());

    unsafe { Box::from_raw(state); }
}

#[no_mangle]
pub extern "C" fn rsvg_state_rust_clone(state: *const State) -> *mut State {
    assert!(!state.is_null());

    unsafe { Box::into_raw(Box::new((*state).clone())) }
}

#[no_mangle]
pub extern "C" fn rsvg_state_rust_parse_style_pair(
    state: *mut State,
    attr: Attribute,
    value: *const libc::c_char,
) {
    assert!(!state.is_null());
    assert!(!value.is_null());

    let state = unsafe { &mut *state };
    let value = unsafe { utf8_cstr(value) };

    match state.parse_style_pair(attr, value) {
        _ => (), // FIXME: propagate errors
    }
}

#[no_mangle]
pub extern "C" fn rsvg_state_rust_inherit_run(
    dst: *mut State,
    src: *const State,
    inherit_fn: extern "C" fn (glib_sys::gboolean, glib_sys::gboolean) -> c_int,
) {
    assert!(!dst.is_null());
    assert!(!src.is_null());

    unsafe {
        let dst = &mut *dst;
        let src = &*src;

        }
    }
}
