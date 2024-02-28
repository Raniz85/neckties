use std::rc::Rc;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::Icon;
use icondata as i;
use rand::seq::IndexedRandom;
use crate::knot::{Knot, Knots};

const KNOTS: &str = include_str!("knots.toml");

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let knots: Knots = toml::from_str(KNOTS).unwrap();
    let knots2 = knots.clone();

    view! {
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
                        view=move || view!{ <KnotList knots={knots.clone()} /> }
                    />
                    <Route
                        path="/:slug"
                        view=move || view!{ <Knot knots={knots2.clone()} /> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// A list of knots
#[component]
fn KnotList(knots: Knots) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let random = knots.knots.choose(&mut rng).unwrap();

    view! {
        <h1>"Neckties!"</h1>
        <ul>
            <li>
                <a href={format!("/{}", &random.slug())}><Icon icon=i::BiDice3Regular /></a>
            </li>
            {
                knots.knots.iter()
                    .map(|knot| view!{
                        <li>
                            <a href={format!("/{}", &knot.slug())}>
                                <img src={format!("http://img.youtube.com/vi/{}/mqdefault.jpg", &knot.youtube)} />
                                {&knot.name}
                            </a>
                        </li>
                    })
                    .collect_view()
            }
        </ul>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <h1>"No such necktie for you"</h1>
    }
}

#[derive(Params, PartialEq, Eq)]
struct KnotParams {
    pub slug: String,
}

/// A knot
#[component]
fn Knot(knots: Knots) -> impl IntoView {
    let slug = use_params::<KnotParams>().with(|params| params.as_ref()
        .map(|params| params.slug.clone())
        .unwrap_or_default()
    );
    let knot = knots.knots.into_iter().find(|knot| knot.slug() == slug);

    view! {
        {
            move || if let Some(knot) = &knot {
                view! {
                    <h1>{knot.name.clone()}</h1>
                     <iframe width="1080" height="720" src={format!("https://www.youtube.com/embed/{}", knot.youtube.clone())}>

                    </iframe>

                }.into_view()
            } else {
                view! { <NotFound /> }.into_view()
            }
        }
    }
}

