use leptos::html::Div;
use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::context::{LeafletMapContext, LeafletOverlayContainerContext};
use crate::components::position::Position;

#[component]
pub fn Tooltip(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<Position>,
    #[prop(into, optional)] permanent: MaybeSignal<bool>,
    #[prop(into, optional, default="auto".into())] direction: MaybeSignal<String>,
    #[prop(into, optional)] sticky: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay_context = use_context::<LeafletOverlayContainerContext>(cx);

    let content = create_node_ref::<Div>(cx);
    // let content = view! {cx, <div>{children(cx)}</div>};
    create_effect(cx, move |_| {
        let mut options = leaflet::TooltipOptions::default();
        options.permanent(permanent.get());
        options.direction(&direction.get());
        options.sticky(sticky.get());

        if let Some(overlay_context) = overlay_context.clone() {
            if let (Some(layer), Some(_map)) = (
                overlay_context.container::<leaflet::Layer>(),
                map_context.map(),
            ) {
                let tooltip = leaflet::Tooltip::new(&options, Some(layer.unchecked_ref()));
                let content = content.get_untracked().expect("content ref");
                tooltip.setContent(content.unchecked_ref());
                layer.bindTooltip(&tooltip);
                on_cleanup(cx, move || {
                    tooltip.remove();
                });
            }
        } else if let Some(map) = map_context.map() {
            let tooltip = leaflet::Tooltip::newWithLatLng(&position.get().into(), &options);
            let content = content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            tooltip.setContent(html_view);
            tooltip.openOn(&map);
            on_cleanup(cx, move || {
                tooltip.remove();
            });
        }
    });

    view! {cx, <div style="visibility:collapse"><div _ref=content>{children(cx)}</div></div> }
}
