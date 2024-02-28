use std::rc::Rc;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::random;
use rand::seq::IndexedRandom;
use crate::knot::{Knot, Knots};

const KNOTS: &str = include_str!("knots.toml");

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let knots: Knots = toml::from_str(KNOTS).unwrap();
    let knots2 = knots.clone();

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/neckties.css"/>

        // sets the document title
        <Title text="Neckties!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view!{ cx, <KnotList knots={knots.clone()} /> }
                    />
                    <Route
                        path="/:slug"
                        view=move |cx| view!{ cx, <Knot knots={knots2.clone()} /> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// A list of knots
#[component]
fn KnotList(cx: Scope, knots: Knots) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let random = knots.knots.choose(&mut rng).unwrap();

    view! { cx,
        <h1>"Neckties!"</h1>
        <ul>
            <li>
                <a href={format!("/{}", &random.slug())}>Random</a>
            </li>
            {
                knots.knots.iter()
                    .map(|knot| view!{cx,
                        <li>
                            <a href={format!("/{}", &knot.slug())}>{&knot.name}</a>
                        </li>
                    })
                    .collect_view(cx)
            }
        </ul>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"No such necktie for you"</h1>
    }
}

#[derive(Params, PartialEq, Eq)]
struct KnotParams {
    pub slug: String,
}

/// A knot
#[component]
fn Knot(cx: Scope, knots: Knots) -> impl IntoView {
    let slug = use_params::<KnotParams>(cx).with(|params| params.as_ref()
        .map(|params| params.slug.clone())
        .unwrap_or_default()
    );
    let knot = knots.knots.into_iter().find(|knot| knot.slug() == slug);

    view! { cx,
        {
            move || if let Some(knot) = &knot {
                view! { cx,
                    <h1>{knot.name.clone()}</h1>
                     <iframe width="1080" height="720" src={format!("https://www.youtube.com/embed/{}", knot.youtube.clone())}>

                    </iframe>

                }.into_view(cx)
            } else {
                view! { cx, <NotFound /> }.into_view(cx)
            }
        }
    }
}

