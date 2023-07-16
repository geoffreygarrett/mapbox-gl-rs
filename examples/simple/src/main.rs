use mapboxgl::{LngLat, Map, MapOptions};
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

#[hook]
fn use_map() -> Rc<RefCell<Option<Rc<Map>>>> {
    let map = use_mut_ref(|| Option::<Rc<Map>>::None);

    {
        let map = map.clone();
        use_effect_with_deps(
            move |_| {
                if let Ok(mut map) = map.try_borrow_mut() {
                    map.replace(create_map());
                } else {
                    eprintln!("Failed to create Map");
                }
                || {}
            },
            (),
        );
    }

    map
}

#[function_component(App)]
fn app() -> Html {
    let _map = use_map();

    html! {
      <div id="map" style="width: 100vw; height: 100vh;"></div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = std::env!("MAPBOX_TOKEN");

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(139.7647863, 35.6812373))
        .zoom(15.0);

    Map::new(opts).unwrap()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
